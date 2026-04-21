# Agent Management Capability

**Capability Category**: Agent Lifecycle and Status Detection

**Source of Truth**: Reverse-engineered from source code analysis

---

## Core Capabilities

### 1. Agent Lifecycle Management

#### Capability Description
The AgentManager provides complete lifecycle management for AI coding agents, including creation, deletion, attachment, and output capture.

#### Requirements (SHALL/MUST)
- **MUST** create a dedicated tmux session named `grove-<uuid>` for each agent.
- **MUST** initialize a Git worktree for branch isolation on agent creation.
- **MUST** cleanly tear down on deletion (tmux kill, worktree removal).
- **MUST** recreate tmux session if missing on attach.
- **MUST** provide status detection via capture and detector.

#### GIVEN-WHEN-THEN Scenarios

**Scenario A: Create a new agent with isolation**
- GIVEN a repo path and a worktree base, and an AiAgent type (ClaudeCode/OpenCode/Codex/Gemini)
- WHEN `AgentManager::create_agent(name, branch, ai_agent, worktree_symlinks)` is invoked
- THEN a new git worktree is created for the branch, a tmux session `grove-<uuid>` is created, and a corresponding Agent is returned with id, name, branch, worktree_path, and tmux_session populated.

**Scenario B: Delete an existing agent**
- GIVEN an Agent with a live tmux session and a worktree
- WHEN `AgentManager::delete_agent(agent)` is invoked
- THEN the tmux session is killed and the worktree is removed if present.

**Scenario C: Attach to an agent (session missing)**
- GIVEN an Agent whose tmux session does not exist
- WHEN `AgentManager::attach_to_agent(agent, ai_agent)` is invoked
- THEN a new tmux session is created for the agent and attached.

**Scenario D: Capture agent output**
- GIVEN an Agent with a tmux session
- WHEN `AgentManager::capture_output(agent, lines)` is invoked
- THEN the captured output string is returned.

---

### 2. Status Detection

#### Capability Description
The detector module analyzes tmux pane output to determine agent status (Running, AwaitingInput, Completed, Error, etc.) across different AI provider types.

#### Requirements (SHALL/MUST)
- **MUST** classify foreground process to determine running vs. idle vs. awaiting input vs. error.
- **MUST** expose a single entry to detect status for a given output + foreground + agent type.
- **MUST** support per-agent-type logic (ClaudeCode, Opencode, Codex, Gemini) with their own edge-case patterns.
- **MUST** provide a StatusReason wrapping (status, reason, pattern, timestamp) for UI tooling.

#### GIVEN-WHEN-THEN Scenarios

**Scenario E: Detecting waiting-for-input in Claude Code**
- GIVEN output streams from a Claude Code tmux pane containing a question/prompt
- WHEN `detect_status_for_agent(output, ForegroundProcess::ClaudeRunning, AiAgent::ClaudeCode)` is called
- THEN the function returns StatusDetection with status AwaitingInput and reason "Found question/permission prompt".

**Scenario F: Detecting running state for Claude Code with a spinner**
- GIVEN output including spinner characters in the last 3 lines
- WHEN `detect_status` is invoked with foreground ClaudeRunning
- THEN returns Running with reason about spinner and pattern "SPINNER_CHARS".

**Scenario G: OpenCode detection of "permission required" as AwaitingInput**
- GIVEN output includes "permission required" across full output
- WHEN `detect_status_for_agent(output, ForegroundProcess::OpencodeRunning, AiAgent::Opencode)` is called
- THEN returns AwaitingInput with reason and pattern "permission_required".

**Scenario H: Codex shows "Working" indicator as Running**
- GIVEN output contains the Codex working indicator pattern (● Working …)
- WHEN `detect_status_for_agent(output, ForegroundProcess::CodexRunning, AiAgent::Codex)` is called
- THEN returns Running with pattern CODEX_WORKING_PATTERN.

---

### 3. Agent Data Structures

#### Capability Description
The Agent struct models per-agent state including identity, output buffer, activity history, and PM task status.

#### Requirements (SHALL/MUST)
- **MUST** model per-agent identity with unique id, branch, worktree_path, tmux_session.
- **MUST** track and expose status via AgentStatus enum with symbols and labels.
- **MUST** accumulate per-tick activity and expose sparkline data.
- **MUST** provide a safe patch/update of output with trimming.

#### Code References
- Agent struct: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/agent/model.rs` lines 248-336
- AgentStatus enum: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/agent/model.rs` lines 176-193
- output_buffer handling: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/agent/model.rs` lines 384-397
- activity_history tracking: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/agent/model.rs` lines 350-358

---

## Supported AI Providers

1. **Claude Code** - Uses `~/.claude/history.jsonl` for session discovery
2. **Opencode** - Uses SQLite DB at `opencode db path` for session lookup
3. **Codex** - Uses `~/.codex/state` for session discovery
4. **Gemini CLI** - Uses `~/.gemini/projects.json` for project mapping
5. **Pi-Session** - Custom RPC bridge for agent coordination

### Session Resume Patterns
| Provider | Resume Command | Session Discovery |
|----------|---------------|------------------|
| Claude Code | `claude --resume <session_id>` | `~/.claude/history.jsonl` |
| Opencode | `opencode -s <session_id>` | Opencode DB |
| Codex | `codex resume <session_id>` | `~/.codex/state` |
| Gemini | `gemini --resume <session_id>` | `gemini --list-sessions` |
| Pi-Session | N/A (RPC bridge) | RPC channel |