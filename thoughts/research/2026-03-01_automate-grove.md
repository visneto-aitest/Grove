---
date: 2026-03-01T18:30:00Z
git_commit: bf30270f2637077c4a6fcd304da3c650d4e8dee1
branch: researcher
repository: github.com/ziim/grove
topic: "How to improve Grove to run automatically on its own"
tags: [research, codebase, automation, autonomous, headless]
last_updated: 2026-03-01T18:30:00Z
---

## Ticket Synopsis

Research how to improve Grove so it can run automatically on its own (autonomous/headless mode). Currently, Grove is an interactive TUI that requires user input. This research explores how to add automation capabilities.

## Summary

Grove currently requires user interaction to function. To make it run automatically, there are several approaches:

1. **Add headless/autonomous CLI mode** - Run with command-line arguments instead of TUI
2. **Improve automation hooks** - Currently only Asana automation exists
3. **Add background daemon mode** - Run as a background service
4. **Webhook/trigger support** - React to external events

## Current State

### What Grove Currently Does

- **Interactive TUI**: Full terminal UI requiring user input
- **Basic automation**: Asana task automation (move task on assign/push/delete)
- **Manual triggers**: User must press keys to trigger actions
- **No headless mode**: Cannot run without terminal

### Current Automation (Limited)

From `src/automation/executor.rs`:
- **on_task_assign**: Move task to section when assigned
- **on_push**: Move task when code is pushed
- **on_delete**: Move task when agent is deleted

### Current CLI Arguments

From `src/main.rs:138-144`:
```rust
let repo_path = std::env::args().nth(1).unwrap_or_else(|| {
    std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
});
```

Only accepts repository path - no other CLI options.

## Proposed Improvements

### 1. Headless/Autonomous Mode

**Concept**: Run Grove with CLI arguments to perform actions without TUI

**Proposed CLI**:
```bash
# Create agent and start working
grove --agent "feature-login" --branch "feature/auth" --task "12345"

# Run in autonomous mode
grove --auto --agent "feature-x" --prompt "Implement login"

# Monitor mode (no interaction)
grove --monitor

# One-shot command
grove --create-agent my-agent feature-branch
grove --attach my-agent
grove --push my-agent
grove --merge my-agent
```

**Implementation locations**:
- `src/main.rs` - Add CLI argument parsing with clap or structopt
- New module for headless operations

### 2. Enhanced Automation System

**Current state** (`src/app/config.rs:236-247`):
```rust
pub struct AutomationConfig {
    pub on_task_assign: Option<String>,
    pub on_push: Option<String>,
    pub on_delete: Option<String>,
    pub on_task_assign_subtask: Option<String>,
    pub on_delete_subtask: Option<String>,
}
```

**Improvements**:
- Add more triggers: on_agent_complete, on_mr_created, on_error
- Add action types beyond Asana: run script, send notification, trigger CI
- Add conditions: only on certain branch patterns, time-based triggers

### 3. Background Daemon Mode

**Concept**: Run Grove as a background service

**Proposed**:
```bash
# Start daemon
grove daemon start

# Daemon watches for:
# - New Asana tasks in "To Do"
# - New branches matching pattern
# - Automatically creates agents
grove daemon watch --provider asana --project 12345

# Status
grove daemon status
grove daemon logs
```

**Implementation**:
- Add daemon command module
- Use tokio for background polling
- Store daemon state in config

### 4. Webhook/Trigger System

**Concept**: React to external events

**Proposed webhooks**:
- GitHub/GitLab webhook receiver
- Asana webhook for task updates
- Timer-based triggers (cron-like)

**Implementation**:
- Add HTTP server for webhooks (using actix-web or axum)
- Webhook handlers in `src/automation/`
- Secure webhook verification

### 5. Scheduled Jobs

**Concept**: Run tasks on schedule

**Proposed**:
```toml
[automation.schedule]
daily_morning = "0 9 * * *"
weekly_merge = "0 10 * * 1"

[automation.schedule.daily_morning]
action = "create_agent"
branch = "daily/$(date +%Y%m%d)"
prompt = "Review PRs and fix critical issues"
```

## Architecture Changes Needed

### Phase 1: CLI Arguments (Low Effort)

Add command-line argument support:

```rust
// src/main.rs additions
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "grove")]
#[command(about = "AI Agent Worktree Manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Run in autonomous mode (no TUI)
    #[arg(long)]
    auto: bool,
    
    /// Repository path
    #[arg(default_value = ".")]
    repo: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new agent
    Create {
        name: String,
        branch: String,
        /// Task ID to link
        #[arg(long)]
        task: Option<String>,
    },
    /// Attach to agent session
    Attach { name: String },
    /// Push agent changes
    Push { name: String },
    /// Merge main into agent branch
    Merge { name: String },
    /// Delete an agent
    Delete { name: String },
    /// Start daemon mode
    Daemon {
        /// Watch for new tasks
        #[arg(long)]
        watch: bool,
    },
}
```

### Phase 2: Headless Operations (Medium Effort)

Extract business logic from main.rs:
- Create `src/operations/` module for headless operations
- Reuse agent management, git operations, PM integration
- Add error handling for non-interactive mode

### Phase 3: Daemon Mode (High Effort)

- Background service with tokio
- Polling system for PM providers
- Event queue for automation triggers

### Phase 4: Webhooks (High Effort)

- HTTP server (consider adding `axum` to Cargo.toml)
- Webhook handlers
- Security (verification, rate limiting)

## Code References

- `src/main.rs:138-144` - Current CLI argument parsing
- `src/automation/executor.rs` - Current automation (62 lines)
- `src/app/config.rs:236-247` - AutomationConfig struct
- `src/app/state.rs:966-967` - Automation state in AppState

## Implementation Roadmap

### Step 1: Add CLI Framework
- Add `clap` or `structopt` to Cargo.toml
- Define CLI struct with commands
- Add `--help` output

### Step 2: Implement Headless Commands
- `grove create <name> <branch>`
- `grove push <name>`
- `grove merge <name>`
- Test with existing integration tests

### Step 3: Add `--auto` Mode
- Non-interactive agent creation
- Prompt-based task execution
- Exit on completion

### Step 4: Improve Automation
- Expand AutomationConfig
- Add more trigger types
- Add script execution

### Step 5: Daemon Mode
- Background service
- Watch task lists
- Auto-create agents

## Examples of Similar Tools

| Tool | Autonomous Mode |
|------|----------------|
| Claude Code | `--print` for non-interactive |
| OpenCode | `--auto` mode |
| Kilo CLI | `kilo run --auto "message"` |
| Smithery.ai | CLI + server mode |

## Risks and Considerations

1. **Session management**: How to handle tmux in headless mode?
2. **Output handling**: Where to send agent output?
3. **Error handling**: How to report errors without TUI?
4. **Rate limiting**: PM providers have API limits
5. **Security**: Webhook authentication

## Historical Context

No prior research on automation. This is a new feature exploration.

## Related Research

- Previous research on Kilo CLI integration shows autonomous mode pattern
- Current automation limited to Asana task movement

## Open Questions

1. **Use case clarity**: What specific automation scenarios are most valuable?
2. **Daemon vs cron**: Should automation be daemon-based or trigger-based?
3. **TUI coexistence**: Should headless mode share code with TUI?

## Follow-up Research

- Research specific CLI argument patterns from similar tools
- Investigate webhook implementation options
- Study daemon patterns in Rust
