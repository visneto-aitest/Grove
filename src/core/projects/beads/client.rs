

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::types::{api_base_url, BeadsTeam, BeadsWorkspace};
use crate::cache::Cache;
use crate::core::projects::helpers::{create_authenticated_client, AuthType};

/// Internal client for Beads API communication
pub struct BeadsClient {
    client: Client,
}

impl BeadsClient {
    /// Create a new Beads client with the given API token
    pub fn new(token: &str) -> Result<Self> {
        let client = create_authenticated_client(AuthType::Bearer, token, None)?;
        Ok(Self { client })
    }

    /// Fetch all workspaces accessible to the authenticated user
    pub async fn fetch_workspaces(&self) -> Result<Vec<BeadsWorkspace>> {
        let url = format!("{}/api/v1/workspaces", api_base_url());
        tracing::debug!("Beads fetch_workspaces: url={}", url);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to fetch Beads workspaces")?;

        let status = response.status();
        let response_text = response.text().await.unwrap_or_default();
        tracing::debug!(
            "Beads fetch_workspaces response: status={}, body={}",
            status,
            response_text
        );

        if !status.is_success() {
            anyhow::bail!("Beads API error: {} - {}", status, response_text);
        }

        #[derive(Deserialize)]
        struct BeadsWorkspaceResponse {
            data: Vec<BeadsWorkspace>,
        }

        let workspaces: BeadsWorkspaceResponse = serde_json::from_str(&response_text)
            .context("Failed to parse Beads workspaces response")?;

        Ok(workspaces.data)
    }

    /// Fetch all teams/databases accessible to the authenticated user
    pub async fn fetch_teams(&self) -> Result<Vec<BeadsTeam>> {
        let url = format!("{}/api/v1/teams", api_base_url());
        tracing::debug!("Beads fetch_teams: url={}", url);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to fetch Beads teams")?;

        let status = response.status();
        let response_text = response.text().await.unwrap_or_default();
        tracing::debug!(
            "Beads fetch_teams response: status={}, body={}",
            status,
            response_text
        );

        if !status.is_success() {
            anyhow::bail!("Beads API error: {} - {}", status, response_text);
        }

        #[derive(Deserialize)]
        struct BeadsTeamResponse {
            data: Vec<BeadsTeam>,
        }

        let teams: BeadsTeamResponse = serde_json::from_str(&response_text)
            .context("Failed to parse Beads teams response")?;

        Ok(teams.data)
    }

    /// Fetch a single issue by ID
    pub async fn fetch_issue(&self, team_id: &str, issue_id: &str) -> Result<Option<BeadsIssue>> {
        let url = format!(
            "{}/api/v1/teams/{}/issues/{}",
            api_base_url(),
            team_id,
            issue_id
        );
        tracing::debug!("Beads fetch_issue: url={}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch Beads issue")?;

        let status = response.status();
        let response_text = response.text().await.unwrap_or_default();

        if status.as_u16() == 404 {
            return Ok(None);
        }

        if !status.is_success() {
            anyhow::bail!("Beads API error: {} - {}", status, response_text);
        }

        #[derive(Deserialize)]
        struct BeadsIssueResponse {
            data: BeadsIssue,
        }

        let issue: BeadsIssueResponse = serde_json::from_str(&response_text)
            .context("Failed to parse Beads issue response")?;

        Ok(Some(issue.data))
    }

    /// Update an issue's status
    pub async fn update_issue_status(&self, team_id: &str, issue_id: &str, status_id: &str) -> Result<()> {
        let url = format!(
            "{}/api/v1/teams/{}/issues/{}",
            api_base_url(),
            team_id,
            issue_id
        );

        #[derive(Serialize)]
        struct UpdateRequest {
            status_id: String,
        }

        let response = self
            .client
            .patch(&url)
            .json(&UpdateRequest {
                status_id: status_id.to_string(),
            })
            .send()
            .await
            .context("Failed to update Beads issue status")?;

        let status = response.status();
        if !status.is_success() {
            let response_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Beads API error updating status: {} - {}", status, response_text);
        }

        Ok(())
    }
}



/// A Beads issue
#[derive(Debug, Clone, Deserialize)]
pub struct BeadsIssue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub status: BeadsIssueStatus,
    pub url: String,
}

/// Status of a Beads issue
#[derive(Debug, Clone, Deserialize)]
pub struct BeadsIssueStatus {
    pub id: String,
    pub name: String,
    #[serde(rename = "type", default)]
    pub status_type: String,
}

/// Workflow status option for a team
#[derive(Debug, Clone, Deserialize)]
pub struct BeadsStatusOption {
    pub id: String,
    pub name: String,
    #[serde(rename = "type", default)]
    pub status_type: String,
}

/// Optional Beads client that handles missing token gracefully
pub struct OptionalBeadsClient {
    client: RwLock<Option<BeadsClient>>,
    cached_workspaces: Cache<Vec<BeadsWorkspace>>,
    cached_teams: Cache<Vec<BeadsTeam>>,
}

impl OptionalBeadsClient {
    /// Create a new optional Beads client
    pub fn new(token: Option<&str>, cache_ttl_secs: u64) -> Self {
        let client = token.and_then(|tok| BeadsClient::new(tok).ok());
        Self {
            client: RwLock::new(client),
            cached_workspaces: Cache::new(cache_ttl_secs),
            cached_teams: Cache::new(cache_ttl_secs),
        }
    }

    /// Reconfigure the client with a new token
    pub async fn reconfigure(&self, token: Option<&str>) {
        let new_client = token.and_then(|tok| BeadsClient::new(tok).ok());
        let mut guard = self.client.write().await;
        *guard = new_client;
        self.invalidate_cache().await;
    }

    /// Check if Beads is configured
    pub async fn is_configured(&self) -> bool {
        self.client.read().await.is_some()
    }

    /// Fetch all workspaces
    pub async fn fetch_workspaces(&self) -> Result<Vec<BeadsWorkspace>> {
        // Check cache first
        if let Some(cached) = self.cached_workspaces.get().await {
            return Ok(cached);
        }

        let guard = self.client.read().await;
        match &*guard {
            Some(c) => {
                let workspaces = c.fetch_workspaces().await?;
                self.cached_workspaces.set(workspaces.clone()).await;
                Ok(workspaces)
            }
            None => anyhow::bail!("Beads not configured. Set BEADS_TOKEN environment variable."),
        }
    }

    /// Fetch all teams/databases
    pub async fn fetch_teams(&self) -> Result<Vec<BeadsTeam>> {
        // Check cache first
        if let Some(cached) = self.cached_teams.get().await {
            return Ok(cached);
        }

        let guard = self.client.read().await;
        match &*guard {
            Some(c) => {
                let teams = c.fetch_teams().await?;
                self.cached_teams.set(teams.clone()).await;
                Ok(teams)
            }
            None => anyhow::bail!("Beads not configured. Set BEADS_TOKEN environment variable."),
        }
    }

    /// Fetch a single issue
    pub async fn fetch_issue(&self, team_id: &str, issue_id: &str) -> Result<Option<BeadsIssue>> {
        let guard = self.client.read().await;
        match &*guard {
            Some(c) => c.fetch_issue(team_id, issue_id).await,
            None => anyhow::bail!("Beads not configured. Set BEADS_TOKEN environment variable."),
        }
    }

    /// Update an issue's status
    pub async fn update_issue_status(&self, team_id: &str, issue_id: &str, status_id: &str) -> Result<()> {
        let guard = self.client.read().await;
        match &*guard {
            Some(c) => c.update_issue_status(team_id, issue_id, status_id).await,
            None => anyhow::bail!("Beads not configured. Set BEADS_TOKEN environment variable."),
        }
    }

    /// Get cached workspaces
    pub async fn get_cached_workspaces(&self) -> Option<Vec<BeadsWorkspace>> {
        self.cached_workspaces.get().await
    }

    /// Get cached teams
    pub async fn get_cached_teams(&self) -> Option<Vec<BeadsTeam>> {
        self.cached_teams.get().await
    }

    /// Invalidate all caches
    pub async fn invalidate_cache(&self) {
        self.cached_workspaces.invalidate().await;
        self.cached_teams.invalidate().await;
    }

    /// Force refresh caches (bypass TTL)
    pub async fn refresh(&self) {
        self.invalidate_cache().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_beads_client_without_token() {
        let client = OptionalBeadsClient::new(None, 60);

        // Use block_on for async check
        let rt = tokio::runtime::Runtime::new().unwrap();
        let is_configured = rt.block_on(client.is_configured());
        assert!(!is_configured);
    }

    #[test]
    fn test_beads_issue_deserialization() {
        let json = r#"{
            "id": "issue-123",
            "identifier": "bd-abc123",
            "title": "Implement feature",
            "status": {
                "id": "status-1",
                "name": "In Progress",
                "type": "started"
            },
            "url": "https://beads.xyz/issue/bd-abc123"
        }"#;

        let issue: BeadsIssue = serde_json::from_str(json).unwrap();
        assert_eq!(issue.id, "issue-123");
        assert_eq!(issue.identifier, "bd-abc123");
        assert_eq!(issue.title, "Implement feature");
        assert_eq!(issue.status.name, "In Progress");
    }

    #[test]
    fn test_beads_workspace_deserialization() {
        let json = r#"{
            "id": "ws-123",
            "name": "My Workspace",
            "key": "myworkspace"
        }"#;

        let workspace: BeadsWorkspace = serde_json::from_str(json).unwrap();
        assert_eq!(workspace.id, "ws-123");
        assert_eq!(workspace.key, "myworkspace");
    }

    #[test]
    fn test_beads_team_deserialization() {
        let json = r#"{
            "id": "team-456",
            "name": "Backend Team",
            "key": "backend"
        }"#;

        let team: BeadsTeam = serde_json::from_str(json).unwrap();
        assert_eq!(team.id, "team-456");
        assert_eq!(team.key, "backend");
    }

    #[test]
    fn test_beads_issue_status_deserialization() {
        let json = r#"{
            "id": "status-1",
            "name": "Todo",
            "type": "backlog"
        }"#;

        let status: BeadsIssueStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.id, "status-1");
        assert_eq!(status.name, "Todo");
        assert_eq!(status.status_type, "backlog");
    }
}