use super::GitOperation;
use crate::pi::RpcMessage;
use uuid::Uuid;

/// Convert Grove actions to pi-session RPC messages
pub fn action_to_rpc(action: &crate::app::action::Action) -> Option<RpcMessage> {
    match action {
        crate::app::action::Action::CreateAgent {
            name,
            branch,
            task: _,
        } => Some(RpcMessage::CreateAgent {
            name: name.clone(),
            branch: branch.clone(),
        }),
        crate::app::action::Action::AttachToAgent { id } => {
            Some(RpcMessage::AttachToAgent { id: *id })
        }
        crate::app::action::Action::DetachFromAgent => Some(RpcMessage::Detach),
        crate::app::action::Action::UpdateAgentStatus { id, status, .. } => {
            Some(RpcMessage::StatusUpdate {
                id: *id,
                status: status.clone(),
            })
        }
        crate::app::action::Action::UpdateAgentOutput { id, output } => Some(RpcMessage::Output {
            id: *id,
            output: output.clone(),
        }),
        crate::app::action::Action::FetchRemote { id } => Some(RpcMessage::GitOperation {
            id: *id,
            operation: GitOperation::FetchRemote { id: *id },
        }),
        crate::app::action::Action::MergeMain { id } => Some(RpcMessage::GitOperation {
            id: *id,
            operation: GitOperation::MergeMain { id: *id },
        }),
        crate::app::action::Action::PushBranch { id } => Some(RpcMessage::GitOperation {
            id: *id,
            operation: GitOperation::PushBranch { id: *id },
        }),
        crate::app::action::Action::UpdateGitStatus { id, status } => {
            Some(RpcMessage::GitOperation {
                id: *id,
                operation: GitOperation::UpdateGitStatus {
                    id: *id,
                    status: status.clone(),
                },
            })
        }
        crate::app::action::Action::OpenInEditor { id } => Some(RpcMessage::ExecuteTool {
            tool: "editor".to_string(),
            args: vec![id.to_string()],
        }),
        crate::app::action::Action::CopyWorktreePath { id } => Some(RpcMessage::ExecuteTool {
            tool: "file_ops".to_string(),
            args: vec![id.to_string()],
        }),
        crate::app::action::Action::AttachToDevServer { agent_id } => {
            Some(RpcMessage::ExecuteTool {
                tool: "terminal".to_string(),
                args: vec![agent_id.to_string()],
            })
        }
        crate::app::action::Action::PiStartSession { id, branch } => {
            Some(RpcMessage::CreateAgent {
                name: id.to_string(),
                branch: branch.clone(),
            })
        }
        crate::app::action::Action::PiStopSession { .. } => Some(RpcMessage::Detach),
        crate::app::action::Action::PiSendMessage { id: _, message } => {
            Some(RpcMessage::ExecuteTool {
                tool: "send".to_string(),
                args: vec![message.clone()],
            })
        }
        crate::app::action::Action::PiReceiveOutput { id, output } => Some(RpcMessage::Output {
            id: *id,
            output: output.clone(),
        }),
        _ => None,
    }
}

/// Convert pi-session RPC messages to Grove actions
pub fn rpc_to_action(message: RpcMessage) -> crate::app::action::Action {
    match message {
        RpcMessage::CreateAgent { name, branch } => crate::app::action::Action::CreateAgent {
            name,
            branch,
            task: None,
        },
        RpcMessage::AttachToAgent { id } => crate::app::action::Action::AttachToAgent { id },
        RpcMessage::Detach => crate::app::action::Action::DetachFromAgent,
        RpcMessage::StatusUpdate { id, status } => crate::app::action::Action::UpdateAgentStatus {
            id,
            status,
            status_reason: None,
        },
        RpcMessage::Output { id, output } => {
            crate::app::action::Action::UpdateAgentOutput { id, output }
        }
        RpcMessage::GitOperation { id, operation } => match operation {
            GitOperation::FetchRemote { .. } => crate::app::action::Action::FetchRemote { id },
            GitOperation::MergeMain { .. } => crate::app::action::Action::MergeMain { id },
            GitOperation::PushBranch { .. } => crate::app::action::Action::PushBranch { id },
            GitOperation::UpdateGitStatus { id, status } => {
                crate::app::action::Action::UpdateGitStatus { id, status }
            }
        },
        RpcMessage::ExecuteTool { tool, args } => match tool.as_str() {
            "editor" => {
                if let Some(id_str) = args.first() {
                    if let Ok(id) = Uuid::parse_str(id_str) {
                        crate::app::action::Action::OpenInEditor { id }
                    } else {
                        crate::app::action::Action::RefreshSelected
                    }
                } else {
                    crate::app::action::Action::RefreshSelected
                }
            }
            "file_ops" => {
                if let Some(id_str) = args.first() {
                    if let Ok(id) = Uuid::parse_str(id_str) {
                        crate::app::action::Action::CopyWorktreePath { id }
                    } else {
                        crate::app::action::Action::RefreshSelected
                    }
                } else {
                    crate::app::action::Action::RefreshSelected
                }
            }
            "terminal" => {
                if let Some(id_str) = args.first() {
                    if let Ok(id) = Uuid::parse_str(id_str) {
                        crate::app::action::Action::AttachToDevServer { agent_id: id }
                    } else {
                        crate::app::action::Action::RefreshSelected
                    }
                } else {
                    crate::app::action::Action::RefreshSelected
                }
            }
            "debug" => crate::app::action::Action::ToggleStatusDebug,
            "test" => crate::app::action::Action::RefreshSelected,
            _ => crate::app::action::Action::RefreshSelected,
        },
        RpcMessage::RequestSnapshot { .. } => crate::app::action::Action::RefreshSelected,
        RpcMessage::SnapshotResponse(_) => crate::app::action::Action::RefreshSelected,
    }
}
