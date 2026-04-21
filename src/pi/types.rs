use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent::AgentStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiConfig {
    pub provider: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
    pub session_file: Option<String>,
}

impl Default for PiConfig {
    fn default() -> Self {
        Self {
            provider: "anthropic".to_string(),
            model: None,
            api_key: None,
            session_file: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotResponse {
    pub id: Uuid,
    pub name: String,
    pub branch: String,
    pub worktree_path: String,
    pub status: AgentStatus,
    pub output: Vec<String>,
    pub tmux_session: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiMessage {
    pub jsonrpc: String,
    pub id: Option<i32>,
    pub method: Option<String>,
    pub params: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    pub error: Option<JsonRpcError>,
}

impl PiMessage {
    pub fn new_execute(prompt: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            method: Some("execute".to_string()),
            params: Some(serde_json::json!({ "prompt": prompt })),
            result: None,
            error: None,
        }
    }

    pub fn new_tool(tool: &str, args: Vec<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            method: Some("tool".to_string()),
            params: Some(serde_json::json!({ "name": tool, "args": args })),
            result: None,
            error: None,
        }
    }

    pub fn new_snapshot() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            method: Some("snapshot".to_string()),
            params: None,
            result: None,
            error: None,
        }
    }

    pub fn new_exit() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            method: Some("exit".to_string()),
            params: None,
            result: None,
            error: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_execute_request() {
        let msg = PiMessage::new_execute("test prompt");
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"method\":\"execute\""));
        assert!(json.contains("test prompt"));
    }

    #[test]
    fn test_deserialize_tool_response() {
        let json = r#"{"jsonrpc":"2.0","id":1,"result":{"output":"ok"}}"#;
        let msg: PiMessage = serde_json::from_str(json).unwrap();
        assert!(msg.result.is_some());
    }

    #[test]
    fn test_deserialize_error() {
        let json = r#"{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid Request"}}"#;
        let msg: PiMessage = serde_json::from_str(json).unwrap();
        assert!(msg.error.is_some());
    }
}
