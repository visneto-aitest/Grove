# Grove – Gemini CLI Agent Integration

**Requirement Document** – `docs/gemini_agent_requirement.md`

---

## 1. Overview

The **Pi-Coding** subsystem in Grove provides bidirectional integration between Grove's action-based system and external AI coding agents. This document defines the requirements for extending this integration to support **Google Gemini CLI** as a first-class agent alongside existing adapters (Claude Code, Opencode, Codex, etc.).

Gemini CLI is Google's official command-line interface to its Gemini family of large language models. It provides:

- **Interactive REPL mode** (`gemini -i`) for conversational coding assistance
- **Task mode** (`gemini -t`) for autonomous multi-step operations
- **Session persistence** via `~/.gemini/projects.json` and `gemini --list-sessions`
- **Tool ecosystem** including filesystem operations, shell execution, web search/fetch
- **JSON-RPC 2.0 API** via `--experimental-acp` flag for programmatic control
- **MCP (Model Context Protocol) server support** for extensible tool integration

Our goal is to enable Grove agents to:

1. **Spawn and manage Gemini CLI sessions** as managed agents within Grove's worktree-based workflow
2. **Execute Gemini commands** through the existing Pi RPC bridge and tool registry
3. **Stream model output bidirectionally** between Gemini CLI, Grove UI, and remote pi-session agents
4. **Support session resumption** across Grove restarts via existing session persistence mechanisms
5. **Integrate with existing automation** (task assignment, push prompts, git operations)

---

## 2. Goals & Success Criteria

| Goal | Success Indicator |
|------|-------------------|
| **Gemini session lifecycle** | Users can create, attach, detach, and terminate Gemini sessions from Grove UI |
| **Session auto-discovery** | Grove detects existing Gemini sessions via `gemini --list-sessions` and offers resume |
| **Command execution** | Gemini CLI commands (`query`, `resume`, `tool` invocations) map to Grove Actions |
| **Bidirectional streaming** | Model output streams to UI buffer and forwards to pi-session via RPC |
| **Worktree integration** | Each Gemini session operates within its own Git worktree, isolated per branch/task |
| **Configuration parity** | Gemini respects Grove's `ai_agent` config setting and repository-specific prompts |
| **Backward compatibility** | Existing Pi-agent RPC protocol unchanged; Gemini is additive only |
| **Security compliance** | User confirmation required for filesystem/shell operations (Gemini tool safety) |
| **Observability** | Gemini operations logged with `gemini-*` prefix; errors surface as toasts |
| **Test coverage** | Unit/integration tests for session mgmt, command execution, error paths (≥80%) |

---

## 3. Architecture Overview

### 3.1. System Context

```
┌─────────────────────────────────────────────────────────────┐
│                        Grove Core                           │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │   AppState  │  │ Action Queue │  │  Config (gemini) │   │
│  └──────┬──────┘  └──────┬───────┘  └────────┬─────────┘   │
│         └─────────────────┼───────────────────┘             │
│                           │                                 │
│                    action_tx (async)                        │
└───────────────────────────┼─────────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            │               │               │
            ▼               ▼               ▼
┌──────────────────┐ ┌──────────┐ ┌──────────────────┐
│ PiSessionManager │ │ PiAgent  │ │  Gemini Bridge   │
│ - Pi agents      │ │ - RPC    │ │ - CLI wrapper    │
│ - Gemini agents  │ │ - State  │ │ - Subprocess     │
└────────┬─────────┘ └────┬─────┘ └────────┬─────────┘
         │                │                │
         └────────────────┼────────────────┘
                          │
                          ▼
                ┌──────────────────┐
                │   Gemini CLI    │
                │  (subprocess)   │
                │  --experimental │
                │     -acp        │
                └────────┬────────┘
                         │
              ┌──────────┴──────────┐
              │   ~/.gemini/        │
              │  projects.json      │
              │  (session state)    │
              └─────────────────────┘
```

### 3.2. Key Components to Extend

| Component | File | Responsibility |
|-----------|------|----------------|
| **Gemini Session Module** | `src/gemini/session.rs` | Extend existing session discovery and resume commands |
| **Pi Tool Registry** | `src/pi/tool_registry.rs` | Register "gemini" tool with mappings for query, resume, list-sessions |
| **PiAgent** | `src/pi/mod.rs` | Add `gemini_operation_to_action()` handler for Gemini-specific RPC messages |
| **RPC Message Types** | `src/pi/types.rs` | Add `GeminiCommand`, `GeminiOutput`, `GeminiSessionEvent` enum variants |
| **Action Enum** | `src/app/action.rs` | New actions: `ExecuteGeminiQuery`, `ResumeGeminiSession`, `GeminiToolRequest` |
| **Conversion Layer** | `src/pi/conversion.rs` | Map Gemini CLI output to Grove Actions; forward Grove actions to Gemini subprocess |
| **Agent Detector** | `src/agent/detector.rs` | Detect Gemini CLI foreground processes and status (awaiting input, running, etc.) |
| **Config** | `src/app/config.rs` | Gemini-specific settings: security prompts, auto-approve, max turns |

---

## 4. Detailed Requirements

### 4.1. Session Management

#### 4.1.1. Session Discovery

Grove must detect existing Gemini sessions via the existing `src/gemini/session.rs` infrastructure:

- **Discovery method**: Execute `gemini --list-sessions` in worktree directory
- **Session identification**: Parse output for session IDs (UUID format: `8cfa2711-514a-4197-ac0e-df46c9fee46f`)
- **Mapping**: Map session IDs to Grove Agent IDs via `projects.json` path matching
- **UI presentation**: Display discovered sessions with timestamp and last activity

```rust
// Existing function (extend for Pi-agent context)
pub fn find_session_by_directory(worktree_path: &str) -> Result<Option<String>>;
```

#### 4.1.2. Session Creation

When creating a new agent configured for Gemini (`AiAgent::Gemini`):

1. Create Git worktree via existing `Worktree::create(branch)`
2. Execute `gemini new-session <name>` or `gemini` with initial prompt
3. Parse response for session ID
4. Associate session ID with Agent's `ai_session_id` field
5. Spawn subprocess with `--experimental-acp` if JSON-RPC mode enabled
6. Register with PiSessionManager for RPC bridging

#### 4.1.3. Session Resume

On agent attachment (`Action::AttachToAgent`):

- If `ai_session_id` exists, execute `gemini --resume <id>`
- If no session exists, start fresh `gemini` process
- Re-establish RPC bridge connection
- Restore output buffer from session storage

### 4.2. Tool Registry Integration

Gemini CLI must be registered as an executable tool within the Pi-agent ecosystem:

```rust
// src/pi/tool_registry.rs
impl ToolRegistry {
    pub fn map_tool(tool: &str, args: &[String]) -> ToolMapping {
        match tool {
            "gemini" => Self::map_gemini_tool(args),
            // existing tools...
        }
    }
    
    fn map_gemini_tool(args: &[String]) -> ToolMapping {
        match args.get(0).map(|s| s.as_str()) {
            Some("query") => ToolMapping::Action(Action::ExecuteGeminiQuery { 
                prompt: args[1..].join(" ") 
            }),
            Some("resume") => ToolMapping::Action(Action::ResumeGeminiSession { 
                session_id: args.get(1).cloned() 
            }),
            Some("list-sessions") => ToolMapping::Action(Action::RefreshGeminiSessions),
            Some("debug") => ToolMapping::Action(Action::ToggleStatusDebug),
            _ => ToolMapping::Action(Action::ExecuteGeminiRaw { 
                args: args.to_vec() 
            }),
        }
    }
}
```

**Supported Gemini tool commands:**

| Tool Command | Grove Action | Description |
|-------------|--------------|-------------|
| `gemini query <prompt>` | `ExecuteGeminiQuery` | One-shot query to model |
| `gemini resume <id>` | `ResumeGeminiSession` | Resume existing session |
| `gemini list-sessions` | `RefreshGeminiSessions` | List available sessions |
| `gemini --list-sessions` | `RefreshGeminiSessions` | CLI-compatible variant |
| `gemini debug` | `ToggleStatusDebug` | Debug session state |

### 4.3. RPC Protocol Extensions

Extend the RPC message protocol to support Gemini-specific operations:

```rust
// src/pi/types.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RpcMessage {
    // Existing variants...
    
    /// Execute Gemini CLI command
    GeminiCommand {
        id: Uuid,
        command: String,       // "query", "resume", "tool"
        args: Vec<String>,
        worktree_path: String,
    },
    
    /// Streaming output from Gemini model
    GeminiOutput {
        id: Uuid,
        chunk: String,
        is_complete: bool,
    },
    
    /// Gemini tool request (from model to user)
    GeminiToolRequest {
        id: Uuid,
        tool_name: String,     // "read_file", "run_shell_command", etc.
        tool_args: serde_json::Value,
        request_id: String,      // For correlation
    },
    
    /// User response to tool request
    GeminiToolResponse {
        id: Uuid,
        request_id: String,
        approved: bool,
        result: Option<String>,
    },
    
    /// Session state events
    GeminiSessionEvent {
        id: Uuid,
        event: SessionEvent,   // Created, Resumed, Paused, Error
        details: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionEvent {
    Created,
    Resumed,
    Paused,
    WaitingForInput,
    ToolApprovalRequired,
    Completed,
    Error,
}
```

### 4.4. Action Definitions

New actions to add to `src/app/action.rs`:

```rust
pub enum Action {
    // Existing actions...
    
    // Gemini-specific actions
    ExecuteGeminiQuery {
        id: Uuid,
        prompt: String,
        context: Option<String>, // Previous conversation context
    },
    
    ResumeGeminiSession {
        id: Uuid,
        session_id: Option<String>, // If None, use cached id from Agent
    },
    
    RefreshGeminiSessions {
        worktree_path: String,
    },
    
    GeminiSessionsLoaded {
        sessions: Vec<GeminiSessionInfo>,
    },
    
    ExecuteGeminiRaw {
        id: Uuid,
        args: Vec<String>,
    },
    
    // Streaming output
    AppendGeminiOutput {
        id: Uuid,
        chunk: String,
    },
    
    // Tool approval flow
    RequestGeminiToolApproval {
        id: Uuid,
        tool_name: String,
        tool_args: String,
        request_id: String,
    },
    
    ApproveGeminiTool {
        request_id: String,
    },
    
    DenyGeminiTool {
        request_id: String,
    },
    
    // Session lifecycle
    CreateGeminiAgent {
        name: String,
        branch: String,
        initial_prompt: Option<String>,
    },
    
    GeminiSessionCreated {
        id: Uuid,
        session_id: String,
    },
    
    GeminiSessionError {
        id: Uuid,
        error: String,
    },
}

#[derive(Debug, Clone)]
pub struct GeminiSessionInfo {
    pub session_id: String,
    pub project_name: String,
    pub last_activity: String,
    pub worktree_path: Option<String>,
}
```

### 4.5. Bidirectional Streaming

Implement real-time streaming of Gemini model output:

**Grove → Pi-session:**
- Spawn async task to read Gemini CLI stdout via `BufReader`
- Parse output lines (handling both text and JSON-RPC modes)
- Emit `AppendGeminiOutput` actions for each chunk
- Forward chunks via RPC bridge to remote pi-session agents

**Pi-session → Grove:**
- Pi-agent can send `GeminiCommand` RPC messages
- Convert to subprocess input via stdin
- Support multi-turn conversation state

**UI Integration:**
- Gemini output appears in agent's output buffer
- Support markdown rendering (Gemini outputs markdown)
- Syntax highlighting for code blocks
- Streaming animation indicators

### 4.6. Security & Confirmation

Gemini CLI can execute powerful tools (shell commands, file writes). Grove must implement confirmation gates:

| Tool Category | Requires Confirmation | Configuration |
|-------------|------------------------|---------------|
| `read_file`, `list_directory` | No (read-only) | Always allow |
| `write_file`, `replace` | Yes | `gemini_confirm_writes: bool` |
| `run_shell_command` | Yes | `gemini_confirm_shell: bool` |
| `web_fetch` | Yes (warnings for local addresses) | `gemini_confirm_fetch: bool` |
| Custom MCP tools | Configurable per tool | `gemini_tool_policy: HashMap` |

**Implementation:**
- Intercept `GeminiToolRequest` RPC messages
- Show modal toast: "Gemini requests to execute: `{tool}` with args `{args}`. Approve? (y/n/a)"
- On approve: send `GeminiToolResponse { approved: true }`
- On deny: send `GeminiToolResponse { approved: false }`
- Support "always allow for this session" (a key)

### 4.7. Configuration

Extend `src/app/config.rs` with Gemini-specific settings:

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeminiConfig {
    /// Enable Pi-agent Gemini integration
    pub enabled: bool,
    
    /// Path to gemini binary (default: "gemini" in PATH)
    pub binary_path: Option<String>,
    
    /// Use JSON-RPC mode (--experimental-acp)
    pub use_json_rpc: bool,
    
    /// Require confirmation for writes
    pub confirm_writes: bool,
    
    /// Require confirmation for shell commands
    pub confirm_shell: bool,
    
    /// Require confirmation for web fetch
    pub confirm_fetch: bool,
    
    /// Auto-approve known safe tools
    pub auto_approve_tools: Vec<String>,
    
    /// Maximum consecutive model turns before user input required
    pub max_consecutive_turns: u32,
    
    /// Timeout for commands (seconds)
    pub command_timeout_secs: u64,
    
    /// Default model (if supported by CLI)
    pub default_model: Option<String>,
}

// In RepoConfig or GlobalConfig:
pub gemini: Option<GeminiConfig>,
```

---

## 5. User-Facing Capabilities

### 5.1. Creating a Gemini Agent

**User flow:**
1. User selects "New Agent" → chooses "Gemini CLI" from AI agent dropdown
2. Grove creates worktree for branch
3. Grove spawns `gemini` process in worktree
4. UI shows agent with Gemini icon (♊ or similar)
5. Output streams to agent buffer

**Commands available:**
- `gemini query "What does this file do?"`
- `gemini help`
- `/tools` (in REPL mode)

### 5.2. Session Resumption

- On Grove restart, detects existing Gemini sessions via `gemini --list-sessions`
- Offers "Resume" button for sessions matching worktree paths
- Restores previous conversation context

### 5.3. Tool Execution from Grove

User can trigger Gemini tools directly:

```
> gemini tool web_search "Rust async patterns"
> gemini tool read_file src/main.rs
> gemini tool run_shell "cargo test"
```

Each triggers confirmation if required, then streams output.

### 5.4. Multi-Turn Conversations

- Support interactive REPL mode (`gemini -i`)
- User input forwarded via `PiSendMessage`
- Model responses streamed back
- State maintained across turns in session

---

## 6. Error Handling & Recovery

| Error Scenario | Handling | User Notification |
|---------------|----------|-------------------|
| Gemini binary not found | Log error; disable Gemini menu items | Toast: "Gemini CLI not installed. Run `npm install -g @google/gemini-cli`" |
| Session creation failed | Retry once; fallback to error state | Toast: "Failed to create Gemini session: {error}" |
| Command timeout | Kill subprocess; mark agent as error | Toast: "Gemini command timed out" |
| Model quota exceeded | Parse error; suggest retry with delay | Toast: "Gemini API quota exceeded. Retrying in 60s..." |
| Tool execution denied | Send rejection to model | Model receives "User denied execution" |
| Subprocess crashed | Attempt restart; preserve session ID | Toast: "Gemini session restarted after error" |
| JSON-RPC parse error | Fall back to text mode | Log warning; continue in text mode |

---

## 7. Testing Strategy

### 7.1. Unit Tests

- **Session parsing**: Verify `parse_session_list()` handles various `gemini --list-sessions` output formats
- **Tool mapping**: Verify `ToolRegistry::map_tool("gemini", args)` returns correct actions
- **RPC serialization**: Round-trip test for `GeminiCommand`, `GeminiOutput` messages
- **Config loading**: Verify `GeminiConfig` deserialization from TOML

### 7.2. Integration Tests

- **Mock Gemini subprocess**: Create mock that responds to stdin with predetermined output
- **Session lifecycle**: Create → query → output → detach → resume → terminate
- **Tool approval flow**: Verify modal appears; approval propagates; denial handled
- **Streaming**: Verify chunks emitted as actions within 100ms of receipt

### 7.3. End-to-End Tests

- **Real Gemini CLI** (if available in CI):
  - Create agent with "Hello world" prompt
  - Verify output appears in UI buffer
  - Verify session persisted to `projects.json`
  - Restart Grove, verify resume works

### 7.4. Regression Tests

- Existing Pi-agent tests (Claude Code, Opencode) continue to pass
- No changes to core action processing semantics
- Existing agents unaffected by Gemini additions

---

## 8. Open Questions & Decisions

| Question | Proposed Decision | Rationale |
|----------|-------------------|-----------|
| Support `--experimental-acp` JSON-RPC or text mode first? | Text mode first, JSON-RPC as opt-in | Text mode is stable; JSON-RPC is experimental |
| Should Gemini sessions share tmux sessions or run standalone? | Standalone subprocess (no tmux wrapper) | Gemini has its own session management via `projects.json` |
| How to handle Gemini's Plan Mode? | Expose as `gemini tool enter_plan_mode`; UI shows "planning" indicator | Plan mode is powerful for complex tasks |
| MCP server integration? | Support via `gemini_config.mcp_servers: Vec<PathBuf>` | Users can configure custom tool servers |
| Web search results caching? | Cache in agent's output buffer; no separate cache | Keeps implementation simple |
| Maximum output buffer size for long Gemini responses? | 10,000 lines, then truncate with "..." indicator | Prevents memory issues with infinite streams |

---

## 9. References

- [Gemini CLI Documentation](https://google-gemini.github.io/gemini-cli/docs/)
- [Gemini CLI SDK (Rust)](https://docs.rs/gemini-cli-sdk/latest/gemini_cli_sdk/)
- [Gemini CLI GitHub Repository](https://github.com/google-gemini/gemini-cli)
- [Pi Agent Integration](docs/pi_agent_requirement.md)
- [Grove Agent Detector](src/agent/detector.rs)
- [Existing Gemini Session Code](src/gemini/session.rs)

---

## 10. Implementation Checklist

- [ ] Extend `ToolRegistry` with `gemini` tool mappings
- [ ] Add `GeminiCommand`, `GeminiOutput` RPC variants to `RpcMessage`
- [ ] Add Gemini-specific actions to `Action` enum
- [ ] Implement `gemini_operation_to_action()` in `PiAgent`
- [ ] Create `GeminiBridge` module for subprocess management
- [ ] Extend `AgentDetector` for Gemini process detection
- [ ] Add `GeminiConfig` to configuration system
- [ ] Implement tool approval UI flow
- [ ] Add streaming output handler
- [ ] Write unit tests for session discovery and command execution
- [ ] Write integration tests with mock subprocess
- [ ] Update documentation and UI strings
- [ ] Add Gemini icon/styling to UI components

---

**Status:** Requirements complete, ready for implementation  
**Priority:** High  
**Target Release:** v0.9.0  
**Owner:** TBD