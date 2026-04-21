# Gemini CLI Agent Integration Capability

**Capability Category**: External Integration - AI Coding Agent (Google Gemini)

**Source of Truth**: 
- Requirement doc: `docs/gemini_agent_requirement.md`
- Implementation: `src/gemini/*.rs`, `src/pi/` (Gemini extensions)

---

## 1. Overview

### Project Type
Feature Extension for Grove TUI - First-class integration with Google Gemini CLI

### Core Functionality
Integrate Google Gemini CLI (https://github.com/google-gemini/gemini-cli) into Grove as a managed AI coding agent, leveraging Grove's existing worktree isolation, session persistence, and Pi-Coding agent infrastructure. Enables developers to use Gemini's REPL mode, task mode, and tool ecosystem within Grove's multi-agent management workflow.

### Target Users
- Developers using Gemini CLI who want worktree-based project isolation
- Teams combining Gemini with Claude Code, Opencode, or Codex in a unified TUI
- Users requiring Gemini's web search, filesystem tools, and MCP server capabilities
- Developers needing session persistence and cross-restart resume for Gemini

---

## 2. Architecture

### Module Structure (Current vs Planned)

```
src/gemini/
├── mod.rs              # ✅ IMPLEMENTED - Public exports, module interface
├── session.rs          # ✅ IMPLEMENTED - Session discovery, resume command builder
└── execute.rs          # ❌ PLANNED - Subprocess execution bridge (Pi extension)

src/pi/
├── mod.rs              # ✅ IMPLEMENTED - PiAgent, PiSessionManager
├── conversion.rs       # ✅ IMPLEMENTED - action_to_rpc, rpc_to_action
├── tool_registry.rs    # ✅ IMPLEMENTED - Tool mappings (needs Gemini extension)
├── types.rs            # ✅ IMPLEMENTED - Base RPC types (needs Gemini variants)
└── session.rs          # ✅ IMPLEMENTED - Session lifecycle

src/agent/
├── detector.rs         # ✅ IMPLEMENTED - Gemini status detection
└── manager.rs          # ✅ IMPLEMENTED - Agent lifecycle (supports Gemini via AiAgent enum)
```

### System Context

```
Grove TUI Application
    ├── Grove Core (Rust)
    │   ├── Agent Manager (supports AiAgent::Gemini)
    │   ├── Git Worktree System
    │   ├── Pi-Session Manager (extended for Gemini)
    │   └── UI Renderer
    └── Gemini Integration Layer (extends Pi)
        ├── Session Discovery (projects.json)
        ├── Execute Bridge (subprocess wrapper)
        ├── Tool Mapper (Gemini-specific tools)
        └── RPC Message Types (Gemini extensions)

Google Gemini CLI (External Node.js process)
    ├── ~/.gemini/projects.json (session persistence)
    ├── --list-sessions (session discovery)
    ├── --resume <session-id> (session restore)
    ├── -i (interactive REPL mode)
    ├── -t (task mode)
    └── --experimental-acp (JSON-RPC 2.0 API)
```

### Sequence Diagram: Gemini Session Lifecycle

```
Developer → Grove: CreateAgent { name: "gemini-test", branch: "feature/ai", ai_agent: Gemini }
Grove → AgentManager: create_agent(name, branch, Gemini)
AgentManager → Worktree: create(branch)
Worktree → AgentManager: worktree_path
AgentManager → TmuxSession: create(worktree_path, "gemini")
TmuxSession → gemini CLI: spawn process

Developer → Grove: AttachToAgent { id }
Grove → Gemini Session: find_session_by_directory(worktree_path)
gemini CLI → stdout: Available sessions...
Grove → stdout: parse session list
Grove → gemini CLI: gemini --resume <session_id> (or fresh gemini)
Developer ↔ gemini CLI: Interactive REPL session

Developer → Grove: DetachFromAgent
Grove → TmuxSession: detach

Developer → Grove: (restart Grove later)
Grove → Gemini Session: find_session_by_directory(worktree_path)
Grove → Agent: Resume with session_id from projects.json
```

### Sequence Diagram: Gemini Tool Execution via Pi Bridge

```
pi-agent → PiBridge: {"jsonrpc":"2.0","method":"tool","params":{"name":"gemini","args":["query","Explain this code"]}}
PiBridge → ToolRegistry: map_tool("gemini", ["query", "Explain this code"])
ToolRegistry → PiBridge: Action::ExecuteGeminiQuery { prompt: "Explain this code" }
PiBridge → Grove Core: action_tx.send(ExecuteGeminiQuery)

Grove Core → Gemini Bridge: execute_query(agent_id, "Explain this code")
Gemini Bridge → gemini CLI: gemini query "Explain this code"
gemini CLI → LLM Provider: API call with prompt + context
gemini CLI → Gemini Bridge: streaming output chunks
Gemini Bridge → Grove Core: Action::AppendGeminiOutput { chunk }
Grove Core → UI: Update output buffer

Gemini Bridge → PiBridge: GeminiOutput completion
PiBridge → pi-agent: {"jsonrpc":"2.0","result":{"output":"Full explanation..."}}
```

---

## 3. Functional Requirements

### FR1: Gemini Session Discovery

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR1.1 | Discover sessions via `gemini --list-sessions` | MUST | ✅ IMPLEMENTED | `src/gemini/session.rs` |
| FR1.2 | Parse session list output with regex | MUST | ✅ IMPLEMENTED | `parse_session_list()` line 85 |
| FR1.3 | Map worktree paths to session IDs via `projects.json` | MUST | ✅ IMPLEMENTED | `find_session_by_directory()` |
| FR1.4 | Support multiple sessions per project (select latest) | MUST | ✅ IMPLEMENTED | Index comparison logic |
| FR1.5 | Cache session IDs in Agent's `ai_session_id` field | SHOULD | ✅ PARTIAL | Field exists, auto-resume implemented |

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/gemini/session.rs` lines 1-120
- Session discovery: `find_session_by_directory()` line 20
- Resume command builder: `build_resume_command()` line 107

### FR2: Session Lifecycle Management

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR2.1 | Spawn new Gemini session on agent creation | MUST | ✅ IMPLEMENTED | TmuxSession::create with "gemini" command |
| FR2.2 | Resume existing session via `--resume <id>` | MUST | ✅ IMPLEMENTED | `build_resume_command()` in main.rs |
| FR2.3 | Graceful session termination on agent delete | MUST | ✅ PARTIAL | tmux kill handles this |
| FR2.4 | Persist session association in Agent config | SHOULD | ✅ IMPLEMENTED | `ai_session_id` field |
| FR2.5 | Auto-resume on Grove restart | SHOULD | ✅ IMPLEMENTED | main.rs auto-continue logic |

#### GIVEN-WHEN-THEN Scenarios

**Scenario A: Fresh Gemini session creation**
- GIVEN no existing Gemini session for a worktree
- WHEN `AgentManager::create_agent()` is called with `AiAgent::Gemini`
- THEN a new tmux session spawns `gemini` (not `--resume`), and a new session is created in `~/.gemini/projects.json`

**Scenario B: Session resumption on attach**
- GIVEN a cached `ai_session_id` for an agent OR `gemini --list-sessions` returns a session for the worktree
- WHEN `AgentManager::attach_to_agent()` is invoked
- THEN the command becomes `gemini --resume <session_id>` and context is restored

**Scenario C: Cross-restart session recovery**
- GIVEN a Gemini agent with `continue_session: true` and a valid `ai_session_id`
- WHEN Grove restarts and loads session storage
- THEN `gemini --resume <session_id>` is executed automatically

### FR3: Gemini-Specific Status Detection

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR3.1 | Detect "Answer questions" panel as AwaitingInput | MUST | ✅ IMPLEMENTED | `detect_status_gemini()` line detector.rs |
| FR3.2 | Detect spinner patterns (dots, braille) as Running | MUST | ✅ IMPLEMENTED | Dot/braille spinner detection |
| FR3.3 | Detect keyboard hints as AwaitingInput | MUST | ✅ IMPLEMENTED | "Keyboard" pattern matching |
| FR3.4 | Detect permission prompts as AwaitingInput | MUST | ✅ IMPLEMENTED | Permission/confirmation patterns |
| FR3.5 | Handle "Press Esc to cancel" as Running | MUST | ✅ IMPLEMENTED | Esc-cancel pattern |
| FR3.6 | Detect completion markers (✓, Idle at prompt) | MUST | ✅ IMPLEMENTED | Completion and idle patterns |

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/agent/detector.rs` lines 500-700
- Gemini detection: `detect_status_gemini()` function
- Pattern matching: Answer questions, spinners, keyboard hints, permission prompts

#### GIVEN-WHEN-THEN Scenarios

**Scenario D: Detecting question panel**
- GIVEN tmux output contains "Answer questions" with numbered items or input field
- WHEN `detect_status_gemini(output, GeminiRunning)` is called
- THEN returns `StatusDetection { status: AwaitingInput, reason: "Found question/answer panel" }`

**Scenario E: Detecting braille spinner as running**
- GIVEN tmux output contains braille characters (⠋, ⠙, ⠹, ⠸, ⠼, ⠴, ⠦, ⠧, ⠇, ⠏) in last 3 lines
- WHEN `detect_status_gemini(output, GeminiRunning)` is called
- THEN returns `Running` with reason "Gemini braille spinner detected"

### FR4: Pi-Coding Agent Integration (Bridge)

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR4.1 | Register "gemini" tool in ToolRegistry | MUST | ❌ PLANNED | Add to `map_tool()` |
| FR4.2 | Map `gemini query` → ExecuteGeminiQuery action | MUST | ❌ PLANNED | ToolRegistry extension |
| FR4.3 | Map `gemini resume` → ResumeGeminiSession action | MUST | ❌ PLANNED | ToolRegistry extension |
| FR4.4 | Add GeminiCommand RPC message variant | MUST | ❌ PLANNED | `src/pi/types.rs` extension |
| FR4.5 | Add GeminiOutput streaming RPC variant | MUST | ❌ PLANNED | `src/pi/types.rs` extension |
| FR4.6 | Implement bidirectional streaming | SHOULD | ❌ PLANNED | Async stream handling |
| FR4.7 | Support tool approval flow for Gemini | SHOULD | ❌ PLANNED | Security confirmation |

#### Tool Mapping Matrix (Planned)

| Pi Tool Call | Grove Action | Gemini CLI Equivalent | Status |
|-------------|--------------|----------------------|--------|
| `gemini query <prompt>` | `ExecuteGeminiQuery` | `gemini query <prompt>` | ❌ PLANNED |
| `gemini resume [id]` | `ResumeGeminiSession` | `gemini --resume <id>` | ❌ PLANNED |
| `gemini list-sessions` | `RefreshGeminiSessions` | `gemini --list-sessions` | ❌ PLANNED |
| `gemini debug` | `ToggleStatusDebug` | Debug session info | ❌ PLANNED |
| `gemini tool <name>` | `ExecuteGeminiTool` | Tool via CLI | ❌ PLANNED |

### FR5: Configuration Management

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR5.1 | Add `GeminiConfig` struct to config | MUST | ❌ PLANNED | `src/app/config.rs` |
| FR5.2 | Support `gemini_cmd_path` override | SHOULD | ❌ PLANNED | Binary path configuration |
| FR5.3 | Configure `max_consecutive_turns` | SHOULD | ❌ PLANNED | Multi-turn conversation limit |
| FR5.4 | Configure security confirmations | MUST | ❌ PLANNED | `confirm_writes`, `confirm_shell` |
| FR5.5 | Support `--experimental-acp` flag | SHOULD | ❌ PLANNED | JSON-RPC mode toggle |

#### Configuration Schema (Planned)

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeminiConfig {
    pub enabled: bool,
    pub binary_path: Option<String>,        // Path to gemini binary
    pub use_json_rpc: bool,                 // --experimental-acp flag
    pub confirm_writes: bool,               // Confirm file writes
    pub confirm_shell: bool,                // Confirm shell commands
    pub confirm_fetch: bool,                // Confirm web fetch
    pub max_consecutive_turns: u32,         // Auto-continue limit
    pub command_timeout_secs: u64,          // Subprocess timeout
    pub auto_approve_tools: Vec<String>,    // Whitelist tools
}
```

---

## 4. Implementation Analysis

### 4.1. Existing Gemini Session Code

#### Session Discovery Implementation

**Location**: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/gemini/session.rs`

**Requirements (SHALL/MUST)**
- **MUST** read `~/.gemini/projects.json` to map worktree paths to project names
- **MUST** execute `gemini --list-sessions` in the worktree directory
- **MUST** parse session list with regex pattern `r"^\s*(\d+)\.\s+.+?\s+\[([0-9a-f-]+)\]\s*$"`
- **MUST** return the session with highest index (most recent)
- **SHALL** handle missing projects.json gracefully (return None)

#### Code Reference
```rust
pub fn find_session_by_directory(worktree_path: &str) -> Result<Option<String>> {
    let projects_path = get_projects_json_path();
    // ... read projects.json ...
    let output = Command::new("gemini")
        .args(["--list-sessions"])
        .current_dir(worktree_path)
        .output()?;
    // ... parse with regex ...
    Ok(latest_session)
}
```

#### Unit Tests (Implemented)
```rust
#[test]
fn test_parse_session_list() {
    let output = r#"Available sessions for this project (1):
  1. Generate 200 lorem ipsum paragraphs... [8cfa2711-514a-4197-ac0e-df46c9fee46f]"#;
    let result = parse_session_list(output).unwrap();
    assert_eq!(result.unwrap(), "8cfa2711-514a-4197-ac0e-df46c9fee46f");
}

#[test]
fn test_parse_session_list_multiple() {
    // Returns highest index (most recent)
    let output = r#"Available sessions for this project (2):
  1. First session (2 hours ago) [aaa111]
  2. Second session (Just now) [bbb222]"#;
    let result = parse_session_list(output).unwrap();
    assert_eq!(result.unwrap(), "bbb222");
}
```

### 4.2. Resume Command Builder

**Requirements (SHALL/MUST)**
- **MUST** build command `gemini --resume <session_id>` when session_id provided
- **MUST** build command `gemini` (fresh session) when no session_id
- **SHALL** handle empty/whitespace session_id as None

#### Code Reference
```rust
pub fn build_resume_command(base_cmd: &str, session_id: Option<&str>) -> String {
    match session_id {
        Some(id) => format!("{} --resume {}", base_cmd, id),
        None => base_cmd.to_string(),
    }
}
```

### 4.3. Agent Detector - Gemini Patterns

**Location**: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/agent/detector.rs`

**Pattern Detection Matrix**

| Pattern | Detection Method | Status Result | Reason |
|---------|-----------------|---------------|--------|
| "Answer questions" + numbered items | Regex + context | AwaitingInput | Question panel detected |
| Braille spinner (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏) | Char class check | Running | Braille spinner detected |
| Dot spinner (...) | String check | Running | Dot spinner detected |
| "Keyboard" hints | Substring | AwaitingInput | Keyboard guidance shown |
| "Permission required" | Substring | AwaitingInput | Tool approval needed |
| "Press Esc to cancel" | Substring | Running | Cancellation available |
| "✓" completion markers | Char check | Completed | Task completion |
| Idle prompt (no patterns) | Default | Idle | No activity detected |

---

## 5. Integration Points

### 5.1. Agent Manager Integration

Gemini is already integrated as an `AiAgent` variant:

```rust
// src/app/config.rs
pub enum AiAgent {
    ClaudeCode,
    Opencode,
    Codex,
    Gemini,  // ✅ EXISTS
}

impl AiAgent {
    pub fn command(&self) -> &str {
        match self {
            AiAgent::Gemini => "gemini",  // ✅ EXISTS
            // ...
        }
    }
}
```

### 5.2. Main.rs Auto-Continue Logic

**Location**: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/main.rs` lines 300-400

Gemini session resumption is already integrated in the auto-continue startup sequence:

```rust
AiAgent::Gemini => {
    let session_id = ai_session_id
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .or_else(|| {
            grove::gemini::find_session_by_directory(&worktree_path)
                .ok()
                .flatten()
        });
    grove::gemini::build_resume_command(
        ai_agent.command(),
        session_id.as_deref()
    )
}
```

### 5.3. Detector Integration

Gemini detection is already wired into the status detection system:

```rust
// src/agent/detector.rs
pub fn detect_status_for_agent(
    output: &str,
    foreground: ForegroundProcess,
    agent_type: AiAgent,
) -> StatusDetection {
    match agent_type {
        AiAgent::Gemini => detect_status_gemini(output, foreground),
        // ... other agents
    }
}
```

---

## 6. Implementation Phases

### Phase 1: Pi-Coding Bridge Extension (Foundation)

| Task ID | Task | Dependencies | Priority | Status |
|--------|------|-------------|----------|--------|
| T1.1 | Add `GeminiConfig` to `AppConfig` | None | MUST | ❌ NOT STARTED |
| T1.2 | Extend `ToolRegistry` with `gemini` tool | T1.1 | MUST | ❌ NOT STARTED |
| T1.3 | Add `GeminiCommand`, `GeminiOutput` RPC variants | T1.2 | MUST | ❌ NOT STARTED |
| T1.4 | Add Gemini actions to `Action` enum | T1.3 | MUST | ❌ NOT STARTED |
| T1.5 | Implement `gemini_operation_to_action()` in `PiAgent` | T1.4 | MUST | ❌ NOT STARTED |

### Phase 2: Execute Bridge (Core)

| Task ID | Task | Dependencies | Priority | Status |
|--------|------|-------------|----------|--------|
| T2.1 | Create `GeminiBridge` subprocess manager | T1.5 | MUST | ❌ NOT STARTED |
| T2.2 | Implement `execute_gemini_query()` with streaming | T2.1 | MUST | ❌ NOT STARTED |
| T2.3 | Implement `execute_gemini_resume()` | T2.1 | MUST | ❌ NOT STARTED |
| T2.4 | Implement `list_gemini_sessions()` | T2.1 | SHOULD | ❌ NOT STARTED |
| T2.5 | Add timeout and error handling | T2.1 | MUST | ❌ NOT STARTED |

### Phase 3: UI & Security (Polish)

| Task ID | Task | Dependencies | Priority | Status |
|--------|------|-------------|----------|--------|
| T3.1 | Add Gemini icon to agent list | T2.3 | SHOULD | ❌ NOT STARTED |
| T3.2 | Implement tool approval modal | T2.3 | MUST | ❌ NOT STARTED |
| T3.3 | Add Gemini settings to setup wizard | T1.1 | SHOULD | ❌ NOT STARTED |
| T3.4 | Update help overlay with Gemini commands | T3.1 | SHOULD | ❌ NOT STARTED |

### Phase 4: Testing & Documentation

| Task ID | Task | Dependencies | Priority | Status |
|--------|------|-------------|----------|--------|
| T4.1 | Unit tests for `GeminiBridge` | T2.5 | MUST | ❌ NOT STARTED |
| T4.2 | Integration tests with mock gemini | T4.1 | MUST | ❌ NOT STARTED |
| T4.3 | E2E tests with real Gemini CLI | T4.2 | SHOULD | ❌ NOT STARTED |
| T4.4 | Update OpenSpec documentation | All | MUST | ❌ NOT STARTED |

---

## 7. Non-Functional Requirements

### NFR1: Performance

| ID | Requirement | Target | Status |
|----|-------------|--------|--------|
| NFR1.1 | Session discovery time | < 1 second | ✅ VERIFIED |
| NFR1.2 | Query response streaming | < 100ms first chunk | ❌ NOT TESTED |
| NFR1.3 | Resume command latency | < 500ms | ✅ VERIFIED |
| NFR1.4 | Memory per Gemini session | < 150MB (Node.js overhead) | ⚠️ MONITOR |

### NFR2: Reliability

| ID | Requirement | Status |
|----|-------------|--------|
| NFR2.1 | Graceful handling of missing gemini binary | ✅ IMPLEMENTED (error toast) |
| NFR2.2 | Recovery from gemini process crash | ❌ NOT IMPLEMENTED |
| NFR2.3 | Session persistence across Grove restarts | ✅ IMPLEMENTED |

### NFR3: Security

| ID | Requirement | Status |
|----|-------------|--------|
| NFR3.1 | User confirmation for file writes | ❌ NOT IMPLEMENTED (Gemini handles internally) |
| NFR3.2 | User confirmation for shell commands | ❌ NOT IMPLEMENTED (Gemini handles internally) |
| NFR3.3 | Grove acts as pass-through, doesn't bypass Gemini's confirmations | ✅ CURRENT BEHAVIOR |

---

## 8. Acceptance Criteria

### AC1: Session Discovery
- [ ] Grove detects existing Gemini sessions on startup
- [ ] Session IDs correctly parsed from `gemini --list-sessions`
- [ ] Latest session (highest index) selected when multiple exist

### AC2: Session Lifecycle
- [ ] Fresh Gemini agent creates new session
- [ ] Existing session resumed with `--resume <id>`
- [ ] Session persists across Grove restart
- [ ] Tmux detach/reattach works correctly

### AC3: Status Detection
- [ ] "Answer questions" detected as AwaitingInput
- [ ] Spinners detected as Running
- [ ] Completion markers detected correctly

### AC4: Pi-Coding Integration (Future)
- [ ] `gemini query` tool call maps to action
- [ ] Query output streams to UI
- [ ] Output forwarded to pi-session

### AC5: Configuration
- [ ] Gemini config section in settings
- [ ] Binary path override works
- [ ] Confirmation toggles respected

---

## 9. Error Codes

| Code | Description | Handling |
|------|-------------|----------|
| E100 | Gemini binary not found | Toast: "Install with: npm install -g @google/gemini-cli" |
| E101 | Session discovery failed | Log warning, proceed with fresh session |
| E102 | Invalid session ID format | Log error, ignore cached ID |
| E103 | Resume failed (session expired) | Create new session, notify user |
| E104 | Gemini process spawn failed | Retry once, then error state |
| E105 | Query timeout | Kill process, partial output preserved |

---

## 10. Out of Scope (Future Work)

- **JSON-RPC 2.0 full implementation** (`--experimental-acp`) - currently uses text mode
- **MCP server integration** - custom tool servers for Gemini
- **Multi-model support** - switching between Gemini models via CLI
- **Plan mode integration** - Grove-managed `gemini -t` task mode
- **Web search result caching** - beyond Gemini's internal caching
- **IDE companion integration** - VS Code extension coordination

---

## 11. References

| Document | Purpose |
|----------|---------|
| `docs/gemini_agent_requirement.md` | Full feature requirements |
| `docs/pi_agent_requirement.md` | Parent Pi-Coding requirements |
| `src/gemini/session.rs` | Session discovery implementation |
| `src/agent/detector.rs` | Status detection patterns |
| `src/app/config.rs` | Configuration structures |
| [Gemini CLI Docs](https://google-gemini.github.io/gemini-cli/docs/) | External CLI documentation |

---

**Version**: 0.1.0  
**Status**: Requirements Complete, Implementation Partial  
**Last Updated**: 2026-04-16