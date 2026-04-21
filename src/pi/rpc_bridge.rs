use anyhow::{Context, Result};
use uuid::Uuid;

use super::types::{PiConfig, PiMessage};

pub struct RpcBridge {
    sessions: std::collections::HashMap<Uuid, PiSessionHandle>,
    config: PiConfig,
}

pub struct PiSessionHandle {
    pub id: Uuid,
    pub session_path: std::path::PathBuf,
    is_alive: bool,
}

impl RpcBridge {
    pub fn new(config: PiConfig) -> Self {
        Self {
            sessions: std::collections::HashMap::new(),
            config,
        }
    }

    pub async fn spawn_session(&mut self, agent_id: Uuid, branch: &str) -> Result<()> {
        let session_path = self.get_session_path(agent_id);
        
        if let Some(parent) = session_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create session directory")?;
        }

        let mut cmd = std::process::Command::new("pi");
        cmd.arg("--rpc")
            .arg("--session")
            .arg(agent_id.to_string())
            .arg("--branch")
            .arg(branch);

        self.config.apply_env(&mut cmd);

        let child = cmd.spawn()
            .context("Failed to spawn pi process")?;

        let handle = PiSessionHandle {
            id: agent_id,
            session_path,
            is_alive: true,
        };

        self.sessions.insert(agent_id, handle);
        
        std::mem::forget(child);
        Ok(())
    }

    pub fn send_message(&self, agent_id: &Uuid, message: &PiMessage) -> Result<()> {
        if !self.sessions.contains_key(agent_id) {
            anyhow::bail!("Session not found");
        }

        let json = serde_json::to_string(message)?;
        
        if let Some(handle) = self.sessions.get(agent_id) {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .open(&handle.session_path)
                .context("Failed to open session file")?;
            
            use std::io::Write;
            writeln!(file, "{}", json)?;
        }
        
        Ok(())
    }

    pub fn receive_message(&self, agent_id: &Uuid) -> Result<Option<PiMessage>> {
        if !self.sessions.contains_key(agent_id) {
            return Ok(None);
        }

        if let Some(handle) = self.sessions.get(agent_id) {
            let content = std::fs::read_to_string(&handle.session_path)
                .context("Failed to read session file")?;
            
            let lines: Vec<&str> = content.lines().collect();
            if let Some(last_line) = lines.last() {
                if !last_line.is_empty() {
                    let message: PiMessage = serde_json::from_str(last_line)?;
                    return Ok(Some(message));
                }
            }
        }
        
        Ok(None)
    }

    pub async fn close_session(&mut self, agent_id: &Uuid) -> Result<()> {
        if let Some(_handle) = self.sessions.remove(agent_id) {
            // Process cleanup would happen here
        }
        Ok(())
    }

    fn get_session_path(&self, agent_id: Uuid) -> std::path::PathBuf {
        let session_dir = self.config.session_file.clone()
            .or_else(|| std::env::var("PI_SESSION_DIR").ok())
            .unwrap_or_else(|| "~/.pi/agent/sessions".to_string());

        let session_dir = shellexpand::tilde(&session_dir).into_owned();
        std::path::PathBuf::from(session_dir).join(format!("{}.jsonl", agent_id))
    }
}

impl Drop for RpcBridge {
    fn drop(&mut self) {
        for (_, handle) in self.sessions.iter_mut() {
            handle.is_alive = false;
        }
    }
}

impl PiConfig {
    pub fn apply_env(&self, cmd: &mut std::process::Command) {
        if let Some(ref api_key) = self.api_key {
            match self.provider.as_str() {
                "anthropic" => { cmd.env("ANTHROPIC_API_KEY", api_key); }
                "openai" => { cmd.env("OPENAI_API_KEY", api_key); }
                "google" => { cmd.env("GOOGLE_API_KEY", api_key); }
                _ => {}
            };
        }
        
        if let Some(ref model) = self.model {
            cmd.env("PI_MODEL", model);
        }
        
        cmd.env("PI_PROVIDER", &self.provider);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = PiConfig::default();
        assert_eq!(config.provider, "anthropic");
    }

    #[test]
    fn test_config_custom() {
        let config = PiConfig {
            provider: "openai".to_string(),
            model: Some("gpt-4".to_string()),
            api_key: Some("sk-test".to_string()),
            session_file: None,
        };
        assert_eq!(config.provider, "openai");
        assert_eq!(config.model.as_ref().unwrap(), "gpt-4");
    }
}