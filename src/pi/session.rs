use anyhow::Result;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::agent::Agent;
use crate::app::action::Action;

use super::GitOperation;
use super::PiAgent;
use super::RpcMessage;

#[allow(dead_code)]
pub struct PiSessionManager {
    agents: Vec<PiAgent>,
    rpc_tx: mpsc::Sender<RpcMessage>,
    #[allow(dead_code)] rpc_rx: mpsc::Receiver<RpcMessage>,
    #[allow(dead_code)] output_rx: mpsc::Receiver<String>,
}

impl PiSessionManager {
    pub fn new() -> Self {
        let (rpc_tx, rpc_rx) = mpsc::channel(100);
        let (_output_tx, output_rx) = mpsc::channel(100);
        
        Self {
            agents: Vec::new(),
            rpc_tx,
            rpc_rx,
            output_rx,
        }
    }

    pub fn add_agent(&mut self, agent: Agent) -> Uuid {
        let (_agent_tx, agent_rx) = mpsc::channel(100);
        let (output_tx, _output_rx) = mpsc::channel(100);
        
        let pi_agent = PiAgent::new(
            agent,
            self.rpc_tx.clone(),
            agent_rx,
            output_tx,
        );
        
        let id = pi_agent.agent.id;
        self.agents.push(pi_agent);
        
        id
    }

    pub async fn process_messages(&mut self) -> Result<Vec<Action>> {
        let mut all_actions = Vec::new();
        
        for agent in &mut self.agents {
            let actions = agent.process_rpc().await?;
            all_actions.extend(actions);
        }
        
        Ok(all_actions)
    }

    pub fn forward_to_pi(&self, action: &Action) -> Result<()> {
        match action {
            Action::AttachToAgent { id } => {
                let msg = RpcMessage::AttachToAgent { id: *id };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::DetachFromAgent => {
                let msg = RpcMessage::Detach;
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::UpdateAgentStatus { id, status, .. } => {
                let msg = RpcMessage::StatusUpdate { id: *id, status: status.clone() };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::UpdateAgentOutput { id, output } => {
                let msg = RpcMessage::Output { id: *id, output: output.clone() };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::FetchRemote { id } => {
                let msg = RpcMessage::GitOperation {
                    id: *id,
                    operation: GitOperation::FetchRemote { id: *id },
                };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::MergeMain { id } => {
                let msg = RpcMessage::GitOperation {
                    id: *id,
                    operation: GitOperation::MergeMain { id: *id },
                };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::PushBranch { id } => {
                let msg = RpcMessage::GitOperation {
                    id: *id,
                    operation: GitOperation::PushBranch { id: *id },
                };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::UpdateGitStatus { id, status } => {
                let msg = RpcMessage::GitOperation {
                    id: *id,
                    operation: GitOperation::UpdateGitStatus { id: *id, status: status.clone() },
                };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::OpenInEditor { id } => {
                let msg = RpcMessage::ExecuteTool {
                    tool: "editor".to_string(),
                    args: vec![id.to_string()],
                };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::CopyWorktreePath { id } => {
                let msg = RpcMessage::ExecuteTool {
                    tool: "file_ops".to_string(),
                    args: vec![id.to_string()],
                };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::AttachToDevServer { agent_id } => {
                let msg = RpcMessage::ExecuteTool {
                    tool: "terminal".to_string(),
                    args: vec![agent_id.to_string()],
                };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::PiStartSession { id, branch } => {
                let msg = RpcMessage::CreateAgent {
                    name: id.to_string(),
                    branch: branch.clone(),
                };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::PiStopSession { id: _ } => {
                let msg = RpcMessage::Detach;
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::PiSendMessage { id: _, message } => {
                let msg = RpcMessage::ExecuteTool {
                    tool: "send".to_string(),
                    args: vec![message.clone()],
                };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            Action::PiReceiveOutput { id, output } => {
                let msg = RpcMessage::Output { id: *id, output: output.clone() };
                let _ = self.rpc_tx.blocking_send(msg);
            }
            _ => { /* Non-forwardable actions */ }
        }
        
        Ok(())
    }
}

impl Default for PiSessionManager {
    fn default() -> Self {
        Self::new()
    }
}