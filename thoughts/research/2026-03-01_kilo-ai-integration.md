---
date: 2026-03-01T18:15:00Z
git_commit: bf30270f2637077c4a6fcd304da3c650d4e8dee1
branch: researcher
repository: github.com/ziim/grove
topic: "Supporting kilo.ai CLI in Grove"
tags: [research, codebase, ai-agent, integration, kilo]
last_updated: 2026-03-01T18:15:00Z
---

## Ticket Synopsis

Research how to support the kilo.ai CLI tool (https://kilo.ai/cli) in Grove. Kilo is an open-source AI coding agent (fork of OpenCode) that provides a terminal-based interface for orchestrating AI agents. The task involves understanding the existing AI agent integration patterns and determining what changes are needed to add kilo as a supported agent type.

## Summary

Supporting kilo.ai CLI in Grove requires modifications across multiple files following the existing patterns for AI agent integration. The implementation involves:

1. Adding `Kilo` variant to `AiAgent` enum in `src/app/config.rs` (lines 7-15)
2. Creating a new `kilo` module with `session.rs` following the pattern of other agents
3. Updating status detection in `src/agent/detector.rs`
4. Adding UI elements for the new agent type

Kilo CLI is essentially a fork of OpenCode, so the implementation will closely mirror the OpenCode integration with minor adjustments for session management.

## Detailed Findings

### 1. AiAgent Enum Configuration

The `AiAgent` enum in `src/app/config.rs:7-15` defines all supported AI agents. Each agent variant requires implementing several methods:
- `display_name()` - Human-readable name
- `all()` - List of all agents
- `command()` - CLI command name
- `push_command()` - Push command (if applicable)
- `push_prompt()` - Push prompt message
- `process_names()` - Process names to detect for status

### 2. Session Module Pattern

Each AI agent has its own module with `session.rs`. Required functions:

| Function | Purpose |
|----------|---------|
| `find_session_by_directory(worktree_path)` | Find existing session by worktree directory |
| `build_resume_command(base_cmd, session_id)` | Build command to resume a session |

**Existing implementations for comparison:**

| Agent | Session Storage | Resume Command |
|-------|-----------------|----------------|
| Claude Code | `~/.claude/history.jsonl` | `claude --resume <id>` |
| OpenCode | SQLite via `opencode db path` | `opencode -s <id>` |
| Codex | SQLite at `~/.codex/state` | `codex resume <id>` |
| Gemini | `~/.gemini/projects.json` + CLI | `gemini --resume <id>` |

### 3. Kilo Session Storage Location

Based on research of kilo.ai documentation:
- **Config location**: `~/.config/kilo/` (or `~/.config/opencode/` for backwards compatibility)
- **Sessions**: Likely stored in SQLite database similar to OpenCode
- **CLI binary**: `kilo` (installed via `npm install -g @kilocode/cli`)

The CLI commands for kilo:
```bash
kilo                    # Start TUI
kilo --continue         # Resume last session
kilo -c                # Resume session (short flag)
```

### 4. Status Detection Requirements

In `src/agent/detector.rs`, the `ForegroundProcess` enum and `detect_checklist_progress` function need updates:
- Add `KiloRunning` to `ForegroundProcess` enum
- Add `AiAgent::Kilo` to `process_names()` method
- Add `AiAgent::Kilo` case to `detect_checklist_progress` function

### 5. Main.rs Integration Points

The `main.rs` file has several switch statements handling each `AiAgent` variant:
- Lines 336-415: Auto-continue agents - need to add `AiAgent::Kilo` case
- Lines 6513-6768: Settings UI handling
- Lines 7283-7295: Push command handling

### 6. UI Components

Key UI files that need updates:
- `src/ui/components/settings_modal.rs` - Add Kilo to AI agent dropdown
- `src/ui/components/global_setup.rs` - Add Kilo to setup wizard
- `src/ui/components/setup_dialog.rs` - Add Kilo to agent selection

## Implementation Roadmap

### Step 1: Add Kilo to AiAgent enum
**File**: `src/app/config.rs`

Add `Kilo` to enum and implement methods:
- `display_name()`: "Kilo"
- `command()`: "kilo"
- `push_command()`: None (verify from docs)
- `push_prompt()`: Same as OpenCode
- `process_names()`: `&["node", "kilo", "npx"]`

### Step 2: Create kilo module
**Files**: `src/kilo/mod.rs` and `src/kilo/session.rs`

Follow OpenCode pattern - likely very similar implementation since Kilo is a fork.

### Step 3: Update lib.rs exports
**File**: `src/lib.rs`

Add kilo module and re-export session functions.

### Step 4: Update detector.rs
**File**: `src/agent/detector.rs`

1. Add `KiloRunning` to `ForegroundProcess` enum
2. Update `from_command_for_agent` method
3. Update `is_agent_running` method
4. Update `detect_checklist_progress` function

### Step 5: Update main.rs
Add Kilo case in multiple switch statements throughout the file.

### Step 6: Update UI components
Add Kilo to dropdown lists and setup wizards.

## Code References

- `src/app/config.rs:7-72` - AiAgent enum and implementation
- `src/claude_code/session.rs` - Claude Code session pattern
- `src/opencode/session.rs` - OpenCode session pattern (most similar to Kilo)
- `src/codex/session.rs` - Codex session pattern
- `src/gemini/session.rs` - Gemini session pattern
- `src/agent/detector.rs:46-103` - ForegroundProcess enum
- `src/agent/detector.rs:283-287` - Checklist detection
- `src/main.rs:336-415` - Agent auto-continue logic
- `src/lib.rs:1-16` - Module exports

## Architecture Insights

### Pattern Consistency
All AI agents in Grove follow the same integration pattern:
1. Config enum variant
2. Session module with detection and command building
3. Status detection updates
4. UI component updates

This makes adding new agents straightforward but requires updates in multiple places.

### Kilo Specific Notes
- Kilo is a fork of OpenCode (confirmed from docs)
- Session management should be nearly identical to OpenCode
- Main difference is database location (`~/.config/kilo/` vs `~/.opencode/`)
- Kilo uses `--continue` flag (not `-c`)

### Key Differences from OpenCode

| Aspect | OpenCode | Kilo |
|--------|----------|------|
| CLI name | `opencode` | `kilo` |
| Config dir | `~/.opencode/` | `~/.config/kilo/` |
| Resume flag | `-s <id>` | `--continue <id>` |
| Database | SQLite | SQLite |

## Historical Context

No existing research or thoughts found for kilo integration. This is a new feature request.

## Related Research

No related research documents exist at this time.

## Open Questions

1. **Database Schema**: Need to verify Kilo's SQLite database schema for session storage
2. **Push Support**: Does Kilo support push via CLI commands like Claude's `/push`?
3. **Config Command**: Does Kilo have `kilo db path` command like OpenCode?
4. **Status Patterns**: Need to identify status detection patterns specific to Kilo
5. **Testing**: Need to verify actual behavior with installed Kilo CLI

## Follow-up Research

- Test Kilo CLI installation and verify session storage location
- Examine Kilo source code for exact database schema
- Identify any Kilo-specific status output patterns
- Verify push functionality support
