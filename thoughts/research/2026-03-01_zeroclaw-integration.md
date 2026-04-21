---
date: 2026-03-01T18:35:00Z
git_commit: bf30270f2637077c4a6fcd304da3c650d4e8dee1
branch: researcher
repository: github.com/ziim/grove
topic: "How to extend Grove with ZeroClaw support"
tags: [research, codebase, ai-agent, zeroclaw, integration]
last_updated: 2026-03-01T18:35:00Z
---

## Ticket Synopsis

Research how to develop Grove to be extensible with ZeroClaw - a lightweight, daemon-based AI agent framework written in Rust. ZeroClaw is fundamentally different from existing agents (Claude Code, OpenCode, Codex, Gemini) as it runs as a background service rather than a CLI tool.

## Summary

ZeroClaw is a Rust-based AI assistant framework that runs as a daemon (`zeroclaw gateway`) with CLI interaction (`zeroclaw chat`). It differs fundamentally from existing Grove agents:

| Aspect | Claude/OpenCode/Codex/Gemini | ZeroClaw |
|--------|------------------------------|----------|
| Architecture | CLI tool, runs in terminal | Daemon + CLI client |
| Startup | Start on agent creation | Pre-started gateway |
| Session | Per-directory session | Managed by daemon |
| Memory | Varies (100MB-1GB) | <5MB (ultra-lightweight) |
| Language | TypeScript/Node.js | 100% Rust |

**Integration Approach**: Unlike existing agents that run directly in tmux, ZeroClaw requires:
1. Managing the ZeroClaw daemon lifecycle
2. Interacting via `zeroclaw chat` CLI
3. Adapting status detection for ZeroClaw's output format
4. Handling its unique session management

## Detailed Findings

### 1. What is ZeroClaw?

From research at https://github.com/zeroclaw-labs/zeroclaw:

- **Type**: Runtime operating system for agentic workflows
- **Language**: 100% Rust
- **Memory**: <5MB RAM (vs 100MB-1GB for others)
- **Binary Size**: ~8.8MB
- **Architecture**: Trait-driven, fully swappable components

**Key Commands**:
```bash
zeroclaw gateway    # Start daemon (serves web UI at http://127.0.0.1:3000/)
zeroclaw chat "message"  # Send message to agent
zeroclaw chat         # Interactive chat mode
```

**Features**:
- 22+ AI providers (OpenAI, Claude, Ollama, Groq, Mistral, etc.)
- Multiple channel integrations (Telegram, Discord, Slack, WhatsApp)
- Secure by design (pairing auth, sandboxing, workspace scoping)
- Daemon-based with instant cold starts (<10ms)

### 2. Current Agent Pattern

From `src/agent/manager.rs:25-56`, existing agents follow this pattern:

```rust
pub fn create_agent(
    &self,
    name: &str,
    branch: &str,
    ai_agent: &AiAgent,
    worktree_symlinks: &[String],
) -> Result<Agent> {
    // 1. Create worktree
    let worktree_path = worktree.create(branch)?;
    
    // 2. Create tmux session
    let session = TmuxSession::new(&agent.tmux_session);
    session.create(&worktree_path, ai_agent.command())?;
    
    // 3. Agent runs in tmux, user attaches to interact
}
```

Each agent module provides:
- `find_session_by_directory(worktree_path)` - Find existing session
- `build_resume_command(cmd, session_id)` - Build resume command

### 3. ZeroClaw Integration Challenges

**Challenge 1: Daemon-based Architecture**

Unlike CLI tools, ZeroClaw runs as a background daemon:
```bash
# Start gateway (runs in background)
zeroclaw gateway

# Interact via CLI
zeroclaw chat "Hello"
```

Grove would need to:
- Check if `zeroclaw gateway` is running
- Start daemon if not running
- Manage daemon lifecycle per-project or globally

**Challenge 2: Session Management**

ZeroClaw manages sessions internally (unlike file-based sessions):
- Sessions are stored in ZeroClaw's internal state
- No SQLite file to query like OpenCode/Codex
- Session persistence handled by daemon

**Challenge 3: Terminal Interaction**

Current agents run interactively in tmux - users can attach and type directly. ZeroClaw:
- Requires `zeroclaw chat` for each interaction
- No direct terminal interaction mode
- Output captured from CLI response

**Challenge 4: Status Detection**

Current agents output to terminal - ZeroClaw responds via CLI:
- Need to parse `zeroclaw chat` output
- Different status indicators
- May need to add specific flags for structured output

### 4. Proposed Integration Approach

#### Option A: CLI Wrapper Mode (Recommended)

Wrap ZeroClaw in a pseudo-terminal approach:

1. **Daemon Management**:
   - Check for running `zeroclaw gateway`
   - Start on first ZeroClaw agent creation
   - Keep running until Grove exits or manual stop

2. **Session Handling**:
   - ZeroClaw sessions are workspace-based
   - Use worktree path as workspace identifier
   - Track active sessions in Agent state

3. **Interaction Flow**:
   ```
   User types in tmux → Grove captures → `zeroclaw chat "input"` → Capture output → Display
   ```

4. **Implementation**:
```rust
// src/zeroclaw/mod.rs
pub mod session;

pub fn find_session_by_directory(worktree_path: &str) -> Result<Option<String>> {
    // ZeroClaw doesn't have session files
    // Return None - sessions handled by daemon
    Ok(None)
}

pub fn build_resume_command(base_cmd: &str, session_id: Option<&str>) -> String {
    // For ZeroClaw, base_cmd is "zeroclaw chat"
    // We don't need session_id - daemon handles it
    format!("{} --workdir {}", base_cmd, worktree_path)
}

pub fn is_daemon_running() -> bool {
    // Check if zeroclaw gateway is running
    std::process::Command::new("pgrep")
        .args(["-f", "zeroclaw gateway"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn start_daemon() -> Result<()> {
    // Start zeroclaw gateway in background
    std::process::Command::new("zeroclaw")
        .arg("gateway")
        .spawn()?;
    Ok(())
}
```

#### Option B: Web API Mode

Use ZeroClaw's web API instead of CLI:
- ZeroClaw runs HTTP server at http://127.0.0.1:3000/
- Send POST requests to interact
- More complex but more feature-rich

#### Option C: Hybrid Mode

Use tmux with a wrapper script:
```bash
# In tmux session, run:
while true; do
    read -r input
    zeroclaw chat "$input"
done
```

### 5. Files to Modify

| File | Changes |
|------|---------|
| `src/app/config.rs` | Add `ZeroClaw` to `AiAgent` enum |
| `src/zeroclaw/mod.rs` | New module |
| `src/zeroclaw/session.rs` | Session handling |
| `src/lib.rs` | Export zeroclaw module |
| `src/agent/detector.rs` | Add ZeroClaw status detection |
| `src/agent/manager.rs` | Handle daemon lifecycle |
| `src/main.rs` | Add ZeroClaw-specific handling |
| UI components | Add ZeroClaw to dropdowns |

### 6. Configuration

```toml
[global]
ai_agent = "zeroclaw"  # New option

[zeroclaw]
# ZeroClaw-specific settings
provider = "claude"     # Default provider
model = "claude-sonnet-4-20250514"
```

### 7. User Experience

**Creating a ZeroClaw agent**:
1. User selects "ZeroClaw" as AI agent
2. Grove checks if `zeroclaw gateway` is running
3. If not, starts daemon in background
4. Creates worktree (same as other agents)
5. Agent is ready - but interaction is via CLI, not tmux attach

**Using ZeroClaw agent**:
- Unlike other agents, user doesn't attach to tmux
- Instead, Grove provides input field or uses key commands
- Grove sends input to `zeroclaw chat` and displays response
- Output shown in preview pane

## Code References

- `src/app/config.rs:7-72` - AiAgent enum and methods
- `src/agent/manager.rs:25-161` - Agent creation/management
- `src/opencode/session.rs` - Similar session handling (for comparison)
- `src/main.rs:347-420` - Agent-specific session handling
- `src/agent/detector.rs:939-1042` - Status detection patterns

## Architecture Insights

### Why This is Different

| Aspect | CLI Agents | ZeroClaw |
|--------|-----------|----------|
| Process | Direct execution | Daemon + client |
| Session | File-based tracking | Internal daemon state |
| Interaction | tmux attach | CLI/API |
| Output | Terminal capture | CLI response |

### Extensibility Benefits

Adding ZeroClaw support demonstrates Grove's extensibility:
1. **Plugin-like architecture** - Easy to add new agents
2. **Daemon support** - Future agents could be service-based
3. **Multiple interfaces** - CLI, API, tmux all supported

## Historical Context

This is a new integration exploration. Previous work:
- Claude Code, OpenCode, Codex, Gemini integrations already exist
- Kilo CLI integration researched (similar to OpenCode)
- Automation capabilities being explored

## Related Research

- `thoughts/research/2026-03-01_automate-grove.md` - Automation improvements
- `thoughts/research/2026-03-01_kilo-ai-integration.md` - Kilo integration (similar agent)

## Open Questions

1. **Daemon lifecycle**: Should ZeroClaw daemon be per-project or global?
2. **Interaction model**: CLI input field vs key commands vs tmux hybrid?
3. **Session persistence**: How to track ZeroClaw sessions across restarts?
4. **Output streaming**: ZeroClaw supports streaming - display in real-time?
5. **Provider config**: How to configure ZeroClaw providers in Grove UI?

## Follow-up Research

- Test ZeroClaw installation and daemon behavior
- Design interaction model (CLI vs API vs hybrid)
- Investigate ZeroClaw's session management
- Prototype ZeroClaw agent creation flow
