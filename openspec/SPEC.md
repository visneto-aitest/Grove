# Grove OpenSpec Documentation

This directory contains the formal OpenSpec Source of Truth document for Grove, reverse-engineered from the source code.

## Document Structure

```
openspec/
├── SPEC.md                           # This file
├── specs/
│   ├── core/
│   │   ├── system-overview.md        # System description, stack, constraints
│   │   ├── agent-management.md    # Agent lifecycle, status detection
│   │   ├── git-worktree.md      # Git worktree operations
│   │   ├── configuration.md    # Config system
│   │   └── tmux-session.md    # tmux session management
│   ├── integration/
│   │   ├── git-providers.md       # GitHub, GitLab, Codeberg
│   │   ├── project-management.md # Asana, Notion, ClickUp, Airtable, Linear
│   │   ├── pi-agent.md           # Pi-Coding agent RPC bridge
│   │   └── gemini-agent.md       # Google Gemini CLI integration
│   └── ui/
│       └── terminal-ui.md       # TUI components and rendering
```

## How to Use This Document

### For Development
- Reference specific requirements when implementing features
- Use GIVEN-WHEN-THEN scenarios as test cases
- Match code against SHALL/MUST requirements

### For Documentation
- Use scenarios as acceptance criteria examples
- Reference code anchors for exact implementation details

### For Testing
- Use scenarios to create test cases
- Reference code paths for test setup

## Key Sections

| Section | Description |
|---------|-------------|
| System Overview | Technical stack, dependencies, constraints |
| Agent Management | Lifecycle, status detection, session resume |
| Git Worktree | Worktree ops, storage, status |
| Git Providers | GitHub, GitLab, Codeberg, CI |
| Project Management | PM tool integrations |
| AI Agent Integrations | Pi-Coding bridge, Gemini CLI |
| Terminal UI | TUI rendering, settings, wizards |

## Code Anchors

All spec documents reference absolute file paths in the source repository:
- Base: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/`

## Version

- **Source**: Grove v0.2.0
- **Generated**: 2026-04-15
- **Method**: Reverse-engineered from source code analysis