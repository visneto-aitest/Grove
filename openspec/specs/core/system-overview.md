# Grove System Overview

**Capability Category**: Core System Architecture

**Source of Truth**: Reverse-engineered from source code analysis

---

## System Description

Grove is a Terminal User Interface (TUI) application for managing multiple AI coding agents simultaneously. Each agent runs in an isolated git worktree with its own branch, allowing parallel development workflows.

## Technical Stack

- **Language**: Rust (edition 2021)
- **Async Runtime**: Tokio
- **TUI Framework**: Ratatui
- **Git Operations**: git2-rs
- **HTTP Client**: reqwest with rustls-tls
- **Data Formats**: TOML, JSON, SQLite (bundled)

## Supported AI Agents

1. **Claude Code** (`claude`)
2. **Opencode** (`opencode`)
3. **Codex** (`codex`)
4. **Gemini CLI** (`gemini`)
5. **Pi-Session** (custom RPC bridge)

## Supported Integrations

### Git Providers
- GitLab
- GitHub
- Codeberg (with Forgejo Actions / Woodpecker CI)

### Project Management
- Asana
- Notion
- ClickUp
- Airtable
- Linear

---

## Core Components

```
grove/
├── src/
│   ├── main.rs          # Entry point, event loop
│   ├── lib.rs           # Module exports
│   ├── agent/           # Agent model, status detection
│   ├── app/            # State, config, actions
│   ├── automation/      # Automation actions
│   ├── cache/         # Cache management
│   ├── ci/            # CI status integration
│   ├── claude_code/    # Claude Code CLI
│   ├── codex/          # OpenAI Codex
│   ├── core/           # Core integrations
│   ├── devserver/      # Dev server management
│   ├── gemini/         # Google Gemini CLI
│   ├── git/            # Git worktree ops
│   ├── opencode/       # Opencode CLI
│   ├── pi/             # Pi-session bridge
│   ├── storage/       # Session persistence
│   ├── tmux/           # tmux session mgmt
│   └── ui/             # TUI components
```

---

## Technical Constraints

1. **MUST** have tmux installed and in PATH
2. **MUST** run from within a git repository
3. **MUST** use git version 2.5+ with worktree support
4. **SHOULD** have at least one AI agent CLI installed

## Dependencies

### Required Runtime Versions
- Rust 2021 edition
- tokio 1.x
- git2 0.19+

### Optional Integrations (environment variables)
- `GITLAB_TOKEN` - GitLab API
- `GITHUB_TOKEN` - GitHub API
- `CODEBERG_TOKEN` - Codeberg API
- `WOODPECKER_TOKEN` - Woodpecker CI
- `ASANA_TOKEN` - Asana API
- `NOTION_TOKEN` - Notion API
- `CLICKUP_TOKEN` - ClickUp API
- `AIRTABLE_TOKEN` - Airtable API
- `LINEAR_TOKEN` - Linear API