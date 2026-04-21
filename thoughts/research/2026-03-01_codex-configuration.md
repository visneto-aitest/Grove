---
date: 2026-03-01T18:20:00Z
git_commit: bf30270f2637077c4a6fcd304da3c650d4e8dee1
branch: researcher
repository: github.com/ziim/grove
topic: "How to configure Codex in Grove"
tags: [research, codebase, ai-agent, codex, configuration]
last_updated: 2026-03-01T18:20:00Z
---

## Ticket Synopsis

How to configure the Codex AI CLI tool in Grove. The user wants to know how to set up and use Codex as the AI agent in Grove.

## Summary

Codex is already integrated into Grove as one of the supported AI agents. To configure Codex:

1. **Install Codex CLI** - Download from https://github.com/openai/codex and ensure `codex` is in your PATH
2. **Set ai_agent in config** - Add `ai_agent = "codex"` to your global config at `~/.grove/config.toml`
3. **Configure push prompts** (optional) - Set custom push prompts in repo config

## Detailed Findings

### 1. Codex Integration in Grove

Codex is fully supported as an AI agent in Grove. The integration includes:

- **Session management**: Located at `src/codex/session.rs`
- **Status detection**: Implemented in `src/agent/detector.rs` (lines 53-54, 76, 99, 287, 539, 800, 926, 939-1042)
- **Configuration**: Defined in `src/app/config.rs` (lines 13, 22, 31, 40, 49, 60, 69)

### 2. Installation Requirements

To use Codex with Grove:

1. **Install Codex CLI**:
   ```bash
   # Follow instructions at https://github.com/openai/codex
   # Ensure 'codex' is available in your PATH
   codex --version
   ```

2. **Verify Codex is in PATH**:
   ```bash
   which codex
   ```

### 3. Configuration Steps

#### Option A: Global Config (all projects)

Edit `~/.grove/config.toml`:

```toml
[global]
ai_agent = "codex"  # Set to codex
```

#### Option B: Per-Project Config

Create/edit `.grove/project.toml` in your repository:

```toml
[prompts]
# Optional: Custom push prompt for Codex
push_prompt_codex = "Please commit and push these changes"
```

#### Option C: Via Settings UI

1. Press `Shift+S` to open settings
2. Navigate to "AI Agent" field
3. Select "Codex"

### 4. Codex Session Management

Codex stores session data in a SQLite database:

- **Default location**: `~/.codex/state`
- **Custom location**: Set `CODEX_SQLITE_HOME` environment variable

Grove automatically:
- Detects existing Codex sessions for the worktree directory
- Resumes sessions with `codex resume <id>` or `codex resume --last`
- Tracks Codex process status (Running, Idle, etc.)

### 5. Codex-Specific Features

From `src/app/config.rs:60-70`:

```rust
pub fn push_prompt(&self) -> Option<&'static str> {
    match self {
        // ...
        AiAgent::Codex => Some("Please commit and push these changes"),
        // ...
    }
}

pub fn process_names(&self) -> &'static [&'static str] {
    match self {
        // ...
        AiAgent::Codex => &["codex"],
        // ...
    }
}
```

Codex:
- Supports push functionality (via prompt)
- Process detection: looks for `codex` process
- Session storage: SQLite database at `~/.codex/state`

### 6. Status Detection

Codex status detection is handled in `src/agent/detector.rs:939-1042`:

- **Working status**: Detects "Working (Xs • esc to interrupt)" pattern
- **Idle detection**: When Codex process is running but no activity
- **Shell detection**: Handles Codex → shell transitions

## Configuration Reference

### Global Config (`~/.grove/config.toml`)

```toml
[global]
ai_agent = "codex"           # Options: claude-code, opencode, codex, gemini
log_level = "info"          # debug, info, warn, error
worktree_location = "project" # project or home
editor = "code {path}"       # Editor command

[ui]
frame_rate = 30
tick_rate_ms = 250
output_buffer_lines = 5000
```

### Environment Variables

| Variable | Purpose |
|----------|---------|
| `CODEX_SQLITE_HOME` | Custom path to Codex state database |

### Codex Resume Commands

| Scenario | Command |
|----------|---------|
| Resume specific session | `codex resume <session-id>` |
| Resume last session | `codex resume --last` |
| New session | `codex` |

## Code References

- `src/codex/session.rs` - Session detection and command building
- `src/codex/mod.rs` - Module exports
- `src/app/config.rs:13` - Codex enum variant
- `src/app/config.rs:60-70` - Codex-specific config methods
- `src/agent/detector.rs:939-1042` - Codex status detection
- `src/lib.rs:7` - Codex module export
- `README.md:72` - Codex as supported AI CLI

## Architecture Insights

### How Agent Selection Works

1. User sets `ai_agent` in config
2. Grove spawns the specified CLI (`codex` in this case)
3. Session detection queries the agent's storage (SQLite for Codex)
4. Status detection monitors the running process
5. UI displays agent status in real-time

### Why Multiple Agents Supported

The architecture uses a plugin-like pattern:
- Each agent has its own `session.rs` module
- Common interface: `find_session_by_directory()`, `build_resume_command()`
- Status detection handles agent-specific output patterns

## Historical Context

Codex integration was added in a previous update (see CHANGELOG.md line 184: "Merge remote-tracking branch 'origin/main' into integrate-codex"). This follows the same pattern as other AI agents (Claude Code, OpenCode, Gemini).

## Related Research

- `thoughts/research/2026-03-01_kilo-ai-integration.md` - Research on adding Kilo (another OpenCode fork)

## Open Questions

1. **Codex CLI availability**: The main requirement is having Codex installed. The open-source version may have limited availability.
2. **API keys**: Does Codex require authentication? Check Codex documentation for any API key requirements.
3. **Feature parity**: Some features (like `/push` in Claude Code) may not be available in Codex.

## Follow-up Research

- Verify Codex CLI installation and configuration requirements
- Check if Codex supports the same features as other agents (push, status, etc.)
- Test actual Codex integration end-to-end
