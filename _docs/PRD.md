# Product Requirements Document (PRD): Grove

**Version:** 1.0  
**Date:** March 1, 2026  
**Status:** Draft

---

## 1. Document Overview

This Product Requirements Document (PRD) describes Grove, a Terminal User Interface (TUI) application for managing multiple AI coding agents with git worktree isolation. Grove provides a unified interface to create, monitor, and manage AI agent workflows while maintaining complete isolation between different tasks through git worktrees.

This PRD is derived from comprehensive analysis of the source code, including the main application logic, configuration management, agent detection and management, git operations, UI components, and integration modules.

---

## 2. Objective

### 2.1 Product Vision

Grove aims to be the central hub for managing multiple AI coding agents (such as Claude Code, Opencode, Codex, and Gemini) in isolated development environments. The product enables developers to:

- Run multiple AI agents simultaneously on different branches
- Monitor agent status and output in real-time
- Integrate with popular project management tools
- Integrate with git hosting platforms (GitLab, GitHub, Codeberg)
- Manage development servers per agent
- Automate workflows through custom hooks

### 2.2 Primary Goal

The primary goal of Grove is to solve the problem of managing multiple concurrent AI agent sessions while maintaining clean, isolated development environments. By leveraging git worktrees, Grove ensures that each agent works on its own branch without interfering with other work.

---

## 3. Scope

### 3.1 In Scope

**Core Functionality:**
- Multi-agent TUI interface for terminal-based interaction
- Git worktree creation and management for agent isolation
- tmux session management for agent processes
- Real-time agent status detection and monitoring
- Agent output capture and display
- Session persistence across application restarts

**AI Agent Support:**
- Claude Code
- Opencode
- Codex
- Gemini CLI

**Integrations:**
- Git providers: GitLab, GitHub, Codeberg
- Project management: Asana, Notion, ClickUp, Airtable, Linear
- Dev server management per agent

**User Experience:**
- Interactive TUI with keyboard navigation
- Customizable keybindings
- Setup wizards for first-time users
- Settings modal for configuration
- Help overlay with keyboard shortcuts

### 3.2 Out of Scope

- Web-based or GUI interface (TUI only)
- Native mobile applications
- Direct integration with IDEs (VS Code, JetBrains)
- Cloud-based agent hosting
- Built-in AI model training or fine-tuning
- Collaborative features (multi-user sessions)

---

## 4. User Personas and Use Cases

### 4.1 User Personas

#### Persona 1: Solo Developer
**Name:** Alex  
**Role:** Independent software developer  
**Needs:**
- Manage multiple AI-assisted tasks simultaneously
- Keep main branch clean while experimenting
- Quick access to agent sessions
- Simple setup without complex configuration

**Goals:**
- Create isolated worktrees for each feature/bugfix
- Monitor agent progress at a glance
- Easily attach to agent sessions when needed

**Pain Points:**
- Manually managing tmux sessions and worktrees
- Losing track of which agent is working on what
- Difficulty resuming previous sessions after restart

#### Persona 2: Technical Team Lead
**Name:** Jordan  
**Role:** Engineering team lead managing multiple contributors  
**Needs:**
- Overview of all ongoing AI-assisted work
- Integration with team's project management tool
- Visibility into merge request status
- Ability to assign tasks to AI agents

**Goals:**
- Coordinate multiple AI agents across the team
- Track progress against project management system
- Ensure code is properly merged and reviewed

**Pain Points:**
- Lack of visibility into AI agent progress
- Manual synchronization between PM tools and git
- No standardized way to delegate work to AI agents

#### Persona 3: DevOps Engineer
**Name:** Casey  
**Role:** DevOps engineer automating workflows  
**Needs:**
- Programmatic control over agent lifecycle
- Automation hooks for task assignment
- CI/CD integration capabilities

**Goals:**
- Automate repetitive development tasks
- Trigger AI agents based on external events
- Maintain audit trail of agent activities

**Pain Points:**
- Limited automation capabilities
- No headless mode for batch operations
- Manual intervention required for routine tasks

### 4.2 Use Cases

#### Use Case 1: Creating a New Agent
**Actor:** Solo Developer (Alex)  
**Goal:** Create isolated environment for new feature work

**Preconditions:**
- User has Grove installed
- User is in a git repository
- tmux is installed

**Flow:**
1. User runs `grove` in repository directory
2. Grove displays agent list (possibly empty)
3. User presses `n` to create new agent
4. User enters agent name and branch name
5. Optionally selects task from project management system
6. Grove creates git worktree
7. Grove creates tmux session
8. Grove launches AI agent in worktree
9. Agent appears in list as "Running"

**Postconditions:**
- New agent visible in agent list
- Worktree created at configured location
- tmux session running with AI agent process

#### Use Case 2: Attaching to Agent Session
**Actor:** Solo Developer (Alex)  
**Goal:** Interact directly with AI agent

**Preconditions:**
- At least one agent exists and is running

**Flow:**
1. User selects agent from list
2. User presses `Enter` to attach
3. Grove saves current TUI state
4. Grove exits TUI mode
5. tmux attaches to agent's session
6. User interacts with AI agent
7. User presses tmux detach key (Ctrl-b d)
8. TUI mode restores

**Postconditions:**
- User returned to Grove TUI
- Agent status updated based on session state

#### Use Case 3: Merging Main Branch
**Actor:** Solo Developer (Alex)  
**Goal:** Update agent branch with latest from main

**Preconditions:**
- Agent exists and is on a feature branch
- User has configured main branch name

**Flow:**
1. User selects agent from list
2. User presses `m` to merge
3. Confirmation dialog appears
4. User confirms merge
5. Grove sends merge command to tmux session
6. AI agent performs merge
7. Status updates in UI

**Postconditions:**
- Agent branch contains latest from main branch
- Any merge conflicts handled by AI agent

#### Use Case 4: Assigning Project Management Task
**Actor:** Technical Team Lead (Jordan)  
**Goal:** Link agent to project management task

**Preconditions:**
- Project management integration configured
- Task exists in PM system

**Flow:**
1. User selects agent
2. User presses `a` to assign task
3. Task browser modal opens
4. User searches/browses tasks
5. User selects task
6. Grove assigns task to agent in PM system
7. Optional: automation hook triggers

**Postconditions:**
- Agent linked to task in PM system
- Task status visible in agent details
- Automation actions executed (if configured)

#### Use Case 5: Starting Dev Server
**Actor:** Solo Developer (Alex)  
**Goal:** Start development server for agent's worktree

**Preconditions:**
- Dev server command configured in project
- User selects agent

**Flow:**
1. User presses `Ctrl+s` to start dev server
2. Warning modal appears (if enabled)
3. User confirms
4. Grove starts configured dev command
5. Logs stream to devserver view
6. User can attach to server session

**Postconditions:**
- Dev server running in agent's worktree
- Server logs visible in preview panel

---

## 5. Functional Requirements

### 5.1 Agent Management

| ID | Requirement | Input | Expected Output | Constraint | Inferred/Assumed |
|----|-------------|-------|-----------------|------------|------------------|
| AG-001 | Create Agent | Agent name, branch name, optional task | New agent with worktree and tmux session | Branch name must be valid git ref | Inferred |
| AG-002 | Delete Agent | Agent ID | Removed agent, cleaned up worktree and tmux session | Cannot undo | Inferred |
| AG-003 | List Agents | None | Display all agents with status | Sorted by creation time | Inferred |
| AG-004 | Select Agent | Agent index/ID | Highlight selected agent | Navigation via keys | Inferred |
| AG-005 | Attach to Agent | Agent ID | tmux attach to agent session | Requires tmux | Inferred |
| AG-006 | Detect Agent Status | Agent output | Status: Running, AwaitingInput, Completed, Idle, Error, Stopped | Based on output parsing | Inferred |
| AG-007 | Persist Sessions | Agent state | Save/restore agents across restarts | Stored in JSON | Inferred |
| AG-008 | Resume Sessions | None | Auto-attach to existing tmux sessions | tmux must be running | Inferred |

### 5.2 Git Operations

| ID | Requirement | Input | Expected Output | Constraint | Inferred/Assumed |
|----|-------------|-------|-----------------|------------|------------------|
| GIT-001 | Create Worktree | Branch name | Git worktree at configured location | Requires git 2.5+ | Inferred |
| GIT-002 | Remove Worktree | Worktree path | Deleted worktree directory | Must be safe to delete | Inferred |
| GIT-003 | Create Symlinks | File list | Symlinks from main repo to worktree | Files must exist | Inferred |
| GIT-004 | Fetch Remote | None | Updated remote tracking branches | Network required | Inferred |
| GIT-005 | Merge Main | Agent ID | Merge main branch into agent branch | May have conflicts | Inferred |
| GIT-006 | Push Branch | Agent ID | Push branch to remote | Remote configured | Inferred |
| GIT-007 | Get Sync Status | None | Ahead/behind status for each agent | | Inferred |

### 5.3 Git Provider Integration

| ID | Requirement | Input | Expected Output | Constraint | Inferred/Assumed |
|----|-------------|-------|-----------------|------------|------------------|
| GP-001 | GitLab MR Status | Project ID, branch | Merge request status, pipeline status | GitLab token required | Inferred |
| GP-002 | GitHub PR Status | Owner, repo, branch | Pull request status | GitHub token required | Inferred |
| GP-003 | Codeberg PR Status | Owner, repo, branch | Pull request status | Codeberg token required | Inferred |
| GP-004 | Create MR/PR | Agent ID | Created merge/pull request | Must have push access | Assumed |
| GP-005 | Detect MR URL | Agent output | Parsed MR/PR URL from output | | Inferred |

### 5.4 Project Management Integration

| ID | Requirement | Input | Expected Output | Constraint | Inferred/Assumed |
|----|-------------|-------|-----------------|------------|------------------|
| PM-001 | Asana Integration | Project GID | Task list, status options | API token required | Inferred |
| PM-002 | Notion Integration | Database ID | Task list, status options | API token required | Inferred |
| PM-003 | ClickUp Integration | List ID | Task list, status options | API token required | Inferred |
| PM-004 | Airtable Integration | Base ID, table | Task list, status options | API token required | Inferred |
| PM-005 | Linear Integration | Team ID | Task list, status options | API token required | Inferred |
| PM-006 | Assign Task | Agent ID, task URL/GID | Task assigned to agent | PM system must support | Inferred |
| PM-007 | Cycle Task Status | Agent ID | Task status changes | Valid status required | Inferred |

### 5.5 Dev Server Management

| ID | Requirement | Input | Expected Output | Constraint | Inferred/Assumed |
|----|-------------|-------|-----------------|------------|------------------|
| DS-001 | Start Dev Server | Agent ID | Running dev server process | Command must be configured | Inferred |
| DS-002 | Stop Dev Server | Agent ID | Terminated dev server process | | Inferred |
| DS-003 | View Server Logs | Agent ID | Streaming log output | | Inferred |
| DS-004 | Restart Dev Server | Agent ID | Restarted dev server process | | Assumed |

### 5.6 User Interface

| ID | Requirement | Input | Expected Output | Constraint | Inferred/Assumed |
|----|-------------|-------|-----------------|------------|------------------|
| UI-001 | Agent List View | None | List of agents with status columns | | Inferred |
| UI-002 | Preview Panel | Agent selection | Output logs, diff, tasks, dev server | | Inferred |
| UI-003 | Settings Modal | None | Configurable settings interface | Press Shift+S | Inferred |
| UI-004 | Help Overlay | None | Keyboard shortcuts reference | Press ? | Inferred |
| UI-005 | Setup Wizard | First launch | Guided configuration flow | | Inferred |
| UI-006 | Task Browser | None | Project management task selection | | Inferred |
| UI-007 | Toast Notifications | Events | Non-blocking notifications | | Inferred |
| UI-008 | Status Bar | None | Quick actions and info display | | Inferred |

### 5.7 Automation

| ID | Requirement | Input | Expected Output | Constraint | Inferred/Assumed |
|----|-------------|-------|-----------------|------------|------------------|
| AUTO-001 | On Task Assign | Task ID, automation config | Execute configured action | | Inferred |
| AUTO-002 | On Push | Agent ID, automation config | Execute configured action | | Inferred |
| AUTO-003 | On Delete | Agent ID, automation config | Execute configured action | | Inferred |
| AUTO-004 | Custom Hooks | Script path | Execute shell script | | Assumed |

---

## 6. Non-Functional Requirements

### 6.1 Performance

| Requirement | Description | Target | Inferred/Assumed |
|------------|-------------|--------|------------------|
| Startup Time | Time from launch to usable UI | < 2 seconds | Inferred |
| Frame Rate | UI refresh rate | 30 FPS (configurable) | Inferred |
| Tick Rate | Input processing interval | 250ms (configurable) | Inferred |
| Memory Usage | Application memory footprint | < 100 MB | Assumed |
| Agent Polling | Status detection interval | 500ms (configurable) | Inferred |

### 6.2 Scalability

| Requirement | Description | Inferred/Assumed |
|------------|-------------|------------------|
| Concurrent Agents | Maximum agents supported | 10-20 (practical limit) |
| Log Buffer | Maximum output lines retained | 5000 (configurable) |
| API Polling | Background refresh rates | Configurable per provider |

### 6.3 Security

| Requirement | Description | Inferred/Assumed |
|------------|-------------|------------------|
| Token Storage | API tokens stored | Environment variables only |
| Config Security | Sensitive data in config | None (tokens via env vars) |
| Worktree Safety | Prevent accidental deletion | Confirmation dialogs |

### 6.4 Maintainability

| Requirement | Description | Inferred/Assumed |
|------------|-------------|------------------|
| Modular Architecture | Separation of concerns | Agent, Git, UI, Core modules |
| Error Handling | Use of anyhow for errors | Consistent Result types |
| Testing | Unit tests for critical paths | Present in code |
| Logging | Structured logging via tracing | Configurable log levels |

### 6.5 Usability

| Requirement | Description | Inferred/Assumed |
|------------|-------------|------------------|
| Keyboard Navigation | All actions via keyboard | Primary interaction mode |
| Setup Wizards | Guided first-time setup | Global and project config |
| Help System | In-app keyboard shortcut reference | Press ? to view |
| Error Messages | Clear, actionable error messages | Context provided via anyhow |

---

## 7. Technical Specifications

### 7.1 Technology Stack

**Core Language:**
- Rust 2021 edition

**Key Dependencies:**

| Category | Library | Version | Purpose |
|----------|---------|---------|---------|
| UI | ratatui | 0.29 | Terminal UI rendering |
| Terminal | crossterm | 0.28 | Terminal events & input |
| Async | tokio | 1 | Concurrent operations |
| Git | git2 | 0.19 | Git worktree management |
| HTTP | reqwest | 0.12 | API integrations |
| Serialization | serde | 1 | Config & state |
| Database | rusqlite | 0.32 | Cache storage |
| Error Handling | anyhow | 1 | Error propagation |
| Logging | tracing | 0.1 | Debug logging |
| System Info | sysinfo | 0.31 | CPU/memory metrics |

### 7.2 Architecture

Grove follows a layered, modular architecture:

```
┌─────────────────────────────────────────┐
│           Main Event Loop               │
│         (src/main.rs)                   │
└─────────────┬───────────────────────────┘
              │
┌─────────────▼───────────────────────────┐
│           App State                     │
│       (src/app/state.rs)                │
│  - Agent state                          │
│  - UI state                            │
│  - Configuration                       │
└─────────────┬───────────────────────────┘
              │
    ┌─────────┼─────────┬──────────────┐
    ▼         ▼         ▼              │
┌───────┐ ┌──────┐ ┌──────┐    ┌──────────┐
│ Agent │ │ Git  │ │ Core │    │   UI     │
│Module │ │Module│ │Module│    │ Components│
└───────┘ └──────┘ └──────┘    └──────────┘
     │         │       │           │
     ▼         ▼       ▼           ▼
┌───────┐ ┌──────┐ ┌──────┐    ┌──────────┐
│ tmux  │ │ git2 │ │ HTTP │    │ ratatui  │
│Sessions│ │     │ │Client│    │           │
└───────┘ └──────┘ └──────┘    └──────────┘
```

### 7.3 Key Components

#### Agent Module (`src/agent/`)
- **model.rs**: Agent data structures, status types
- **detector.rs**: Real-time status detection from terminal output
- **manager.rs**: Agent lifecycle management

#### Git Module (`src/git/`)
- **worktree.rs**: Git worktree creation and management
- **status.rs**: Git sync status tracking
- **sync.rs**: Fetch, merge, push operations
- **remote.rs**: Remote URL handling

#### UI Module (`src/ui/`)
- **app.rs**: Main application widget
- **components/**: 25+ specialized UI components
- **appearance.rs**: Color and icon theming

#### Core Module (`src/core/`)
- **git_providers/**: GitLab, GitHub, Codeberg API clients
- **projects/**: Asana, Notion, ClickUp, Airtable, Linear clients

#### Other Modules
- **tmux/**: tmux session management
- **devserver/**: Development server process management
- **storage/**: Session persistence
- **automation/**: Automation hook execution
- **claude_code/**: Claude Code session handling
- **opencode/**: Opencode session handling
- **codex/**: Codex session handling
- **gemini/**: Gemini session handling

### 7.4 Data Flow

```
User Input → Action → State Update → Render
                │
                ▼
         Background Tasks
                │
                ▼
    ┌────────────┼────────────┐
    ▼            ▼            ▼
 Git Polls   PM Polls    Agent Polls
    │            │            │
    └────────────┴────────────┘
                │
                ▼
         Update State
```

### 7.5 Configuration System

Grove uses a two-level configuration:

**Global Config (~/.grove/config.toml):**
- AI agent preference
- Worktree location (project/home)
- UI settings
- Performance tuning
- Keybindings

**Project Config (.grove/project.toml):**
- Git provider settings
- Project management settings
- Dev server configuration
- Automation hooks
- Custom prompts

---

## 8. Risks and Assumptions

### 8.1 Risks

| Risk | Description | Mitigation |
|------|-------------|------------|
| tmux Dependency | Requires tmux installation | Check prerequisites on startup |
| Git Version | Requires git 2.5+ for worktrees | Version check on startup |
| API Rate Limits | PM/Git providers may throttle | Configurable polling intervals |
| Network Dependency | Online features require connectivity | Graceful offline handling |
| Process Management | Agent processes may become orphaned | Session recovery on startup |

### 8.2 Assumptions

| Assumption | Description |
|------------|-------------|
| Single User | Application designed for single-user local use |
| Unix-like Systems | Primary target is macOS/Linux (tmux requirement) |
| Terminal Environment | Requires terminal emulator with adequate features |
| Git Repository | Must be run from within a git repository |
| Network Access | Assumes connectivity for API integrations |
| Token Management | Users will provide API tokens via environment variables |

---

## 9. Dependencies

### 9.1 System Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| tmux | Any recent | Session management |
| git | 2.5+ | Worktree support |

### 9.2 External Services

| Service | Purpose | Authentication |
|---------|---------|----------------|
| GitLab | MR status, pipelines | GITLAB_TOKEN |
| GitHub | PR status | GITHUB_TOKEN |
| Codeberg | PR status, CI | CODEBERG_TOKEN, WOODPECKER_TOKEN |
| Asana | Task management | ASANA_TOKEN |
| Notion | Task management | NOTION_TOKEN |
| ClickUp | Task management | CLICKUP_TOKEN |
| Airtable | Task management | AIRTABLE_TOKEN |
| Linear | Task management | LINEAR_TOKEN |

### 9.3 Build Dependencies

- Rust 2021 edition
- Cargo (comes with Rust)

---

## 10. Timeline and Milestones

**Note:** This information is not available in the source code. The project appears to be actively maintained with version 0.2.0 currently released. Future development milestones would be determined by the project maintainers.

---

## 11. Appendix

### A. File Structure

```
src/
├── main.rs              # Entry point, event loop
├── lib.rs               # Module exports
├── version.rs           # Version info
├── app/                 # State & configuration
│   ├── action.rs        # Action definitions
│   ├── config.rs        # Configuration structures
│   ├── state.rs         # AppState
│   └── task_list.rs     # Task types
├── agent/               # Agent management
│   ├── model.rs         # Agent data model
│   ├── detector.rs      # Status detection
│   └── manager.rs       # Agent lifecycle
├── git/                 # Git operations
│   ├── worktree.rs      # Worktree management
│   ├── status.rs        # Git status
│   ├── sync.rs          # Sync operations
│   └── remote.rs        # Remote handling
├── ui/                  # Terminal UI
│   ├── app.rs           # Main widget
│   ├── appearance.rs    # Theming
│   └── components/       # UI components
├── tmux/                # Tmux integration
├── devserver/           # Dev server management
├── core/                # External integrations
│   ├── git_providers/  # GitLab, GitHub, Codeberg
│   └── projects/       # PM integrations
├── storage/             # Session persistence
├── automation/          # Automation hooks
└── [ai agent modules]  # Claude Code, Opencode, etc.
```

### B. Key Configuration Options

```toml
[global]
ai_agent = "claude-code"
worktree_location = "project"

[ui]
frame_rate = 30
tick_rate_ms = 250

[performance]
agent_poll_ms = 500

[keybinds]
nav_down = "Down"
new_agent = "n"
```

### C. Keyboard Shortcuts (Sample)

| Key | Action |
|-----|--------|
| n | New agent |
| d | Delete agent |
| Enter | Attach to agent |
| m | Merge main |
| p | Push branch |
| a | Assign task |
| ? | Help overlay |
| Shift+S | Settings |

---

**Document Prepared:** March 1, 2026  
**Source Code Version:** 0.2.0  
**Repository:** https://github.com/ziim/grove
