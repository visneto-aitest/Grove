//! Pi-session integration layer for Grove
// Provides RPC-to-action adapter and pi-agent implementation

pub mod types;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use uuid::Uuid;

#[allow(unused_imports)]
use crate::agent::{Agent, AgentStatus};
use crate::app::action::Action;
use crate::git::GitSyncStatus;

/// RPC message types from pi-session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RpcMessage {
    /// Request to create a new agent
    CreateAgent { name: String, branch: String },
    /// Request to attach to an existing agent
    AttachToAgent { id: Uuid },
    /// Request to detach from current agent
    Detach,
    /// Agent status update from pi
    StatusUpdate { id: Uuid, status: AgentStatus },
    /// Agent output from pi
    Output { id: Uuid, output: String },
    /// Git operations from pi
    GitOperation { id: Uuid, operation: GitOperation },
    /// Tool execution from pi
    ExecuteTool { tool: String, args: Vec<String> },
    /// Request agent state snapshot
    RequestSnapshot { id: Uuid },
    /// Response to snapshot request
    SnapshotResponse(SnapshotResponse),
}

/// Git operations that can be forwarded from pi to Grove
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GitOperation {
    FetchRemote { id: Uuid },
    MergeMain { id: Uuid },
    PushBranch { id: Uuid },
    UpdateGitStatus { id: Uuid, status: GitSyncStatus },
}

/// Tool types that pi can request to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolType {
    Git,
    Editor,
    FileOps,
    Terminal,
    Debug,
    Test,
}

/// Response from snapshot request
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

/// PiAgent wraps an Agent with RPC communication
pub struct PiAgent {
    agent: Agent,
    rpc_tx: mpsc::Sender<RpcMessage>,
    rpc_rx: mpsc::Receiver<RpcMessage>,
    /// Buffer for forwarding output to pi
    output_tx: mpsc::Sender<String>,
}

impl PiAgent {
    pub fn new(
        agent: Agent,
        rpc_tx: mpsc::Sender<RpcMessage>,
        rpc_rx: mpsc::Receiver<RpcMessage>,
        output_tx: mpsc::Sender<String>,
    ) -> Self {
        Self {
            agent,
            rpc_tx,
            rpc_rx,
            output_tx,
        }
    }

    /// Process incoming RPC messages and convert to Grove actions
    pub async fn process_rpc(&mut self) -> Result<Vec<Action>> {
        let mut actions = Vec::new();
        
        while let Ok(msg) = self.rpc_rx.try_recv() {
            match msg {
                RpcMessage::CreateAgent { name, branch } => {
                    actions.push(Action::CreateAgent {
                        name,
                        branch,
                        task: None,
                    });
                }
                RpcMessage::AttachToAgent { id } => {
                    actions.push(Action::AttachToAgent { id });
                }
                RpcMessage::Detach => {
                    actions.push(Action::DetachFromAgent);
                }
                RpcMessage::StatusUpdate { id, status } => {
                    actions.push(Action::UpdateAgentStatus {
                        id,
                        status,
                        status_reason: None,
                    });
                }
                RpcMessage::Output { id, output } => {
                    actions.push(Action::UpdateAgentOutput { id, output: output.clone() });
                    // Forward output to pi via output channel
                    let _ = self.output_tx.blocking_send(output);
                }
                RpcMessage::GitOperation { id, operation } => {
                    actions.extend(self.git_operation_to_action(id, operation).await?);
                }
                RpcMessage::ExecuteTool { tool, args } => {
                    actions.push(self.tool_to_action(tool, args)?);
                }
                RpcMessage::RequestSnapshot { id } => {
                    let snapshot = self.create_snapshot(id).await;
                    let _ = self.rpc_tx.blocking_send(RpcMessage::SnapshotResponse(snapshot));
                }
                RpcMessage::SnapshotResponse(_) => {
                    // Snapshot response - nothing to do, already handled
                }
            }
        }
        
        Ok(actions)
    }

    async fn git_operation_to_action(&self, id: Uuid, op: GitOperation) -> Result<Vec<Action>> {
        match op {
            GitOperation::FetchRemote { .. } => Ok(vec![Action::FetchRemote { id }]),
            GitOperation::MergeMain { .. } => Ok(vec![Action::MergeMain { id }]),
            GitOperation::PushBranch { .. } => Ok(vec![Action::PushBranch { id }]),
            GitOperation::UpdateGitStatus { id, status } => {
                Ok(vec![Action::UpdateGitStatus { id, status }])
            }
        }
    }

    fn tool_to_action(&self, tool: String, args: Vec<String>) -> Result<Action> {
        match tool.as_str() {
            "git" => {
                if let Some(arg) = args.first() {
                    match arg.as_str() {
                        "status" => Ok(Action::RefreshSelected),
                        "diff" => Ok(Action::ToggleDiffView),
                        _ => Ok(Action::RefreshSelected),
                    }
                } else {
                    Ok(Action::RefreshSelected)
                }
            }
            "editor" => {
                if let Some(id) = args.first().and_then(|s| Uuid::parse_str(s).ok()) {
                    Ok(Action::OpenInEditor { id })
                } else {
                    Ok(Action::RefreshSelected)
                }
            }
            "file_ops" => {
                if let Some(id) = args.first().and_then(|s| Uuid::parse_str(s).ok()) {
                    Ok(Action::CopyWorktreePath { id })
                } else {
                    Ok(Action::RefreshSelected)
                }
            }
            "terminal" => {
                if let Some(id) = args.first().and_then(|s| Uuid::parse_str(s).ok()) {
                    Ok(Action::AttachToDevServer { agent_id: id })
                } else {
                    Ok(Action::RefreshSelected)
                }
            }
            "debug" => Ok(Action::ToggleStatusDebug),
            "test" => Ok(Action::RefreshSelected),
            _ => Ok(Action::RefreshSelected),
        }
    }

    async fn create_snapshot(&self, _id: Uuid) -> SnapshotResponse {
        let agent = &self.agent;
        SnapshotResponse {
            id: agent.id,
            name: agent.name.clone(),
            branch: agent.branch.clone(),
            worktree_path: agent.worktree_path.clone(),
            status: agent.status.clone(),
            output: agent.output_buffer.clone(),
            tmux_session: agent.tmux_session.clone(),
        }
    }
}

pub mod session;
pub mod tool_registry;
pub mod rpc_bridge;

pub use session::PiSessionManager;
pub use tool_registry::{ToolRegistry, ToolMapping, ToolDefinition};
pub use rpc_bridge::RpcBridge;

pub mod conversion;
pub use conversion::{action_to_rpc, rpc_to_action};