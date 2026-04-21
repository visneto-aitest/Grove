use crate::app::action::Action;
use uuid::Uuid;

pub enum ToolMapping {
    Action(Action),
    Unknown,
}

pub struct ToolRegistry;

impl ToolRegistry {
    pub fn map_tool(tool: &str, args: &[String]) -> ToolMapping {
        match tool {
            "git" => {
                if let Some(arg) = args.first() {
                    match arg.as_str() {
                        "status" => ToolMapping::Action(Action::RefreshSelected),
                        "diff" => ToolMapping::Action(Action::ToggleDiffView),
                        _ => ToolMapping::Action(Action::RefreshSelected),
                    }
                } else {
                    ToolMapping::Action(Action::RefreshSelected)
                }
            }
            "editor" => {
                if let Some(id_str) = args.first() {
                    if let Ok(id) = Uuid::parse_str(id_str) {
                        ToolMapping::Action(Action::OpenInEditor { id })
                    } else {
                        ToolMapping::Action(Action::RefreshSelected)
                    }
                } else {
                    ToolMapping::Action(Action::RefreshSelected)
                }
            }
            "file_ops" => {
                if let Some(id_str) = args.first() {
                    if let Ok(id) = Uuid::parse_str(id_str) {
                        ToolMapping::Action(Action::CopyWorktreePath { id })
                    } else {
                        ToolMapping::Action(Action::RefreshSelected)
                    }
                } else {
                    ToolMapping::Action(Action::RefreshSelected)
                }
            }
            "terminal" => {
                if let Some(id_str) = args.first() {
                    if let Ok(id) = Uuid::parse_str(id_str) {
                        ToolMapping::Action(Action::AttachToDevServer { agent_id: id })
                    } else {
                        ToolMapping::Action(Action::RefreshSelected)
                    }
                } else {
                    ToolMapping::Action(Action::RefreshSelected)
                }
            }
            "debug" => ToolMapping::Action(Action::ToggleStatusDebug),
            "test" => ToolMapping::Action(Action::RefreshSelected),
            _ => ToolMapping::Unknown,
        }
    }

    pub fn get_available_tools() -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "git".to_string(),
                description: "Git operations (status, diff, add, commit)".to_string(),
                parameters: vec!["command".to_string(), "args".to_string()],
            },
            ToolDefinition {
                name: "editor".to_string(),
                description: "Open file in editor".to_string(),
                parameters: vec!["file_path".to_string()],
            },
            ToolDefinition {
                name: "file_ops".to_string(),
                description: "File operations (copy path, create)".to_string(),
                parameters: vec!["operation".to_string(), "path".to_string()],
            },
            ToolDefinition {
                name: "terminal".to_string(),
                description: "Attach to terminal/development server".to_string(),
                parameters: vec!["agent_id".to_string()],
            },
            ToolDefinition {
                name: "debug".to_string(),
                description: "Toggle debug status view".to_string(),
                parameters: vec![],
            },
        ]
    }
}

pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_tool_mapping_status() {
        assert!(matches!(
            ToolRegistry::map_tool("git", &["status".to_string()]),
            ToolMapping::Action(Action::RefreshSelected)
        ));
    }

    #[test]
    fn test_git_tool_mapping_diff() {
        assert!(matches!(
            ToolRegistry::map_tool("git", &["diff".to_string()]),
            ToolMapping::Action(Action::ToggleDiffView)
        ));
    }

    #[test]
    fn test_editor_tool_mapping() {
        let uuid = Uuid::new_v4();
        assert!(matches!(
            ToolRegistry::map_tool("editor", &[uuid.to_string()]),
            ToolMapping::Action(Action::OpenInEditor { .. })
        ));
    }

    #[test]
    fn test_terminal_tool_mapping() {
        let uuid = Uuid::new_v4();
        assert!(matches!(
            ToolRegistry::map_tool("terminal", &[uuid.to_string()]),
            ToolMapping::Action(Action::AttachToDevServer { .. })
        ));
    }

    #[test]
    fn test_unknown_tool_returns_unknown() {
        assert!(matches!(
            ToolRegistry::map_tool("unknown_tool", &[]),
            ToolMapping::Unknown
        ));
    }

    #[test]
    fn test_get_available_tools() {
        let tools = ToolRegistry::get_available_tools();
        assert_eq!(tools.len(), 5);
        assert!(tools.iter().any(|t| t.name == "git"));
        assert!(tools.iter().any(|t| t.name == "editor"));
    }
}
