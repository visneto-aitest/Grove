use serde::{Deserialize, Serialize};

use crate::core::projects::helpers::truncate_with_ellipsis;

/// A Beads workspace (user account or organization container)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BeadsWorkspace {
    pub id: String,
    pub name: String,
    pub key: String,
}

/// A Beads team/database (project-specific issue database)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BeadsTeam {
    pub id: String,
    pub name: String,
    pub key: String,
}

/// Global Beads configuration (user preferences)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeadsConfig {
    #[serde(default)]
    pub refresh_secs: u64,
    #[serde(default)]
    pub cache_ttl_secs: u64,
}

impl BeadsConfig {
    pub fn default_refresh_secs() -> u64 {
        120
    }

    pub fn default_cache_ttl_secs() -> u64 {
        60
    }
}

/// Per-project Beads configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RepoBeadsConfig {
    pub workspace_id: Option<String>,
    pub team_id: Option<String>,
}

/// Status of a Beads issue linked to an agent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BeadsTaskStatus {
    #[default]
    None,
    NotStarted {
        id: String,
        identifier: String,
        name: String,
        status_name: String,
        url: String,
    },
    InProgress {
        id: String,
        identifier: String,
        name: String,
        status_name: String,
        url: String,
    },
    Completed {
        id: String,
        identifier: String,
        name: String,
        status_name: String,
        url: String,
    },
    Error {
        id: String,
        message: String,
    },
}

impl BeadsTaskStatus {
    pub fn format_short(&self) -> String {
        match self {
            BeadsTaskStatus::None => "—".to_string(),
            BeadsTaskStatus::NotStarted { name, .. } => truncate_with_ellipsis(name, 24),
            BeadsTaskStatus::InProgress { name, .. } => truncate_with_ellipsis(name, 24),
            BeadsTaskStatus::Completed { name, .. } => truncate_with_ellipsis(name, 24),
            BeadsTaskStatus::Error { message, .. } => {
                format!("err: {}", truncate_with_ellipsis(message, 10))
            }
        }
    }

    pub fn format_status_name(&self) -> String {
        match self {
            BeadsTaskStatus::None => "—".to_string(),
            BeadsTaskStatus::NotStarted { status_name, .. }
            | BeadsTaskStatus::InProgress { status_name, .. }
            | BeadsTaskStatus::Completed { status_name, .. } => {
                truncate_with_ellipsis(status_name, 10)
            }
            BeadsTaskStatus::Error { .. } => "Error".to_string(),
        }
    }

    pub fn status_name_full(&self) -> Option<&str> {
        match self {
            BeadsTaskStatus::None => None,
            BeadsTaskStatus::NotStarted { status_name, .. }
            | BeadsTaskStatus::InProgress { status_name, .. }
            | BeadsTaskStatus::Completed { status_name, .. } => Some(status_name.as_str()),
            BeadsTaskStatus::Error { .. } => None,
        }
    }

    pub fn id(&self) -> Option<&str> {
        match self {
            BeadsTaskStatus::None => None,
            BeadsTaskStatus::NotStarted { id, .. }
            | BeadsTaskStatus::InProgress { id, .. }
            | BeadsTaskStatus::Completed { id, .. }
            | BeadsTaskStatus::Error { id, .. } => Some(id),
        }
    }

    pub fn identifier(&self) -> Option<&str> {
        match self {
            BeadsTaskStatus::None => None,
            BeadsTaskStatus::NotStarted { identifier, .. }
            | BeadsTaskStatus::InProgress { identifier, .. }
            | BeadsTaskStatus::Completed { identifier, .. } => Some(identifier),
            BeadsTaskStatus::Error { .. } => None,
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            BeadsTaskStatus::None => None,
            BeadsTaskStatus::NotStarted { name, .. }
            | BeadsTaskStatus::InProgress { name, .. }
            | BeadsTaskStatus::Completed { name, .. } => Some(name),
            BeadsTaskStatus::Error { message, .. } => Some(message),
        }
    }

    pub fn url(&self) -> Option<&str> {
        match self {
            BeadsTaskStatus::NotStarted { url, .. }
            | BeadsTaskStatus::InProgress { url, .. }
            | BeadsTaskStatus::Completed { url, .. } => Some(url),
            _ => None,
        }
    }

    pub fn is_linked(&self) -> bool {
        !matches!(self, BeadsTaskStatus::None)
    }
}

/// Get the Beads API base URL
///
/// Can be overridden via BEADS_API_URL environment variable
pub fn api_base_url() -> String {
    std::env::var("BEADS_API_URL").unwrap_or_else(|_| "https://api.beads.xyz".to_string())
}

/// Parse a Beads issue ID from various input formats
///
/// Supports:
/// - Full URL: https://beads.xyz/workspace/team/issue/BD-abc123
/// - Short ID: bd-abc123
/// - Just the hash: abc123
pub fn parse_beads_issue_id(input: &str) -> String {
    let trimmed = input.trim();

    // Handle full URL format
    if trimmed.contains("beads.xyz") || trimmed.contains("beads.app") {
        let url = trimmed.trim_end_matches('/');
        if let Some(pos) = url.find("/issue/") {
            let after_issue = &url[pos + 7..];
            let issue_id = after_issue
                .split('/')
                .next()
                .unwrap_or(after_issue)
                .split('?')
                .next()
                .unwrap_or(after_issue);
            return issue_id.to_string();
        }
    }

    // Handle short ID format (bd-xxx)
    let lower = trimmed.to_lowercase();
    if lower.starts_with("bd-") {
        return lower;
    }

    // Return as-is for hash-only format
    trimmed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_beads_issue_id_with_url() {
        let url = "https://beads.xyz/myworkspace/myteam/issue/bd-abc123";
        assert_eq!(parse_beads_issue_id(url), "bd-abc123");
    }

    #[test]
    fn test_parse_beads_issue_id_with_query() {
        let url = "https://beads.xyz/workspace/team/issue/bd-xyz789?view=board";
        assert_eq!(parse_beads_issue_id(url), "bd-xyz789");
    }

    #[test]
    fn test_parse_beads_issue_id_short_format() {
        assert_eq!(parse_beads_issue_id("BD-ABC123"), "bd-abc123");
        assert_eq!(parse_beads_issue_id("bd-xyz789"), "bd-xyz789");
    }

    #[test]
    fn test_parse_beads_issue_id_hash_only() {
        assert_eq!(parse_beads_issue_id("abc123"), "abc123");
    }

    #[test]
    fn test_beads_task_status_format_short() {
        let status = BeadsTaskStatus::NotStarted {
            id: "123".to_string(),
            identifier: "bd-abc".to_string(),
            name: "Implement feature XYZ".to_string(),
            status_name: "Todo".to_string(),
            url: "https://beads.xyz/issue/bd-abc".to_string(),
        };
        assert_eq!(status.format_short(), "Implement feature XYZ");

        let long_name_status = BeadsTaskStatus::InProgress {
            id: "123".to_string(),
            identifier: "bd-abc".to_string(),
            name: "This is a very long task name that should be truncated".to_string(),
            status_name: "In Progress".to_string(),
            url: "https://beads.xyz/issue/bd-abc".to_string(),
        };
        assert!(long_name_status.format_short().len() <= 27); // 24 + "..."
    }

    #[test]
    fn test_beads_task_status_format_status_name() {
        let status = BeadsTaskStatus::InProgress {
            id: "123".to_string(),
            identifier: "bd-abc".to_string(),
            name: "Task".to_string(),
            status_name: "In Progress".to_string(),
            url: "https://beads.xyz/issue/bd-abc".to_string(),
        };
        assert_eq!(status.format_status_name(), "In Progre…");
    }

    #[test]
    fn test_beads_task_status_is_linked() {
        assert!(!BeadsTaskStatus::None.is_linked());

        let status = BeadsTaskStatus::NotStarted {
            id: "123".to_string(),
            identifier: "bd-abc".to_string(),
            name: "Task".to_string(),
            status_name: "Todo".to_string(),
            url: "https://beads.xyz".to_string(),
        };
        assert!(status.is_linked());
    }

    #[test]
    fn test_beads_task_status_accessors() {
        let status = BeadsTaskStatus::NotStarted {
            id: "issue-123".to_string(),
            identifier: "bd-abc".to_string(),
            name: "My Task".to_string(),
            status_name: "Todo".to_string(),
            url: "https://beads.xyz/issue/bd-abc".to_string(),
        };

        assert_eq!(status.id(), Some("issue-123"));
        assert_eq!(status.identifier(), Some("bd-abc"));
        assert_eq!(status.name(), Some("My Task"));
        assert_eq!(status.url(), Some("https://beads.xyz/issue/bd-abc"));
        assert_eq!(status.status_name_full(), Some("Todo"));
    }

    #[test]
    fn test_beads_task_status_completed_with_url() {
        let status = BeadsTaskStatus::Completed {
            id: "issue-456".to_string(),
            identifier: "bd-def".to_string(),
            name: "Completed Task".to_string(),
            status_name: "Done".to_string(),
            url: "https://beads.xyz/issue/bd-def".to_string(),
        };

        assert_eq!(status.id(), Some("issue-456"));
        assert_eq!(status.identifier(), Some("bd-def"));
        assert_eq!(status.url(), Some("https://beads.xyz/issue/bd-def"));
    }

    #[test]
    fn test_beads_config_defaults() {
        let config = BeadsConfig::default();
        assert_eq!(config.refresh_secs, 0);
        assert_eq!(config.cache_ttl_secs, 0);

        assert_eq!(BeadsConfig::default_refresh_secs(), 120);
        assert_eq!(BeadsConfig::default_cache_ttl_secs(), 60);
    }

    #[test]
    fn test_beads_workspace() {
        let workspace = BeadsWorkspace {
            id: "ws-123".to_string(),
            name: "My Workspace".to_string(),
            key: "myworkspace".to_string(),
        };
        assert_eq!(workspace.id, "ws-123");
        assert_eq!(workspace.name, "My Workspace");
        assert_eq!(workspace.key, "myworkspace");
    }

    #[test]
    fn test_beads_team() {
        let team = BeadsTeam {
            id: "team-456".to_string(),
            name: "Backend Team".to_string(),
            key: "backend".to_string(),
        };
        assert_eq!(team.id, "team-456");
        assert_eq!(team.name, "Backend Team");
        assert_eq!(team.key, "backend");
    }
}
