# Source File Reference Guide

This document provides a comprehensive reference to all source files in the Grove codebase.

## File Count Summary

| Module | Files | Description |
|--------|-------|-------------|
| `app/` | 5 | State management, configuration, actions |
| `agent/` | 4 | Agent model, detection, management |
| `git/` | 5 | Git operations, worktrees |
| `ui/` | 31 | Terminal UI components |
| `tmux/` | 2 | Tmux session management |
| `devserver/` | 3 | Development server management |
| `core/` | 30+ | External integrations |
| `storage/` | 2 | Session persistence |
| `automation/` | 2 | Automation hooks |
| AI Agents | 8 | Claude Code, Opencode, Codex, Gemini |
| Other | 6 | CI, cache, common utilities |

---

## Application Core (`src/app/`)

### `src/app/mod.rs` (19 lines)
**Purpose**: Module exports for application core

**Public API**:
```rust
pub mod action;
pub mod config;
pub mod state;
pub mod task_list;

pub use action::{Action, InputMode};
pub use config::{AiAgent, Config, GitProvider, ProjectMgmtProvider, ...};
pub use state::{AppState, PreviewTab, Toast, ToastLevel, ...};
pub use task_list::TaskListItem;
```

---

### `src/app/action.rs` (448 lines)
**Purpose**: Defines all user action types as an enum

**Key Types**:
- `Action` enum: 100+ variants covering all user interactions
- `InputMode` enum: Different input modes (NewAgent, SetNote, etc.)

**Action Categories**:
- Navigation: `SelectNext`, `SelectPrevious`, `SelectFirst`, `SelectLast`
- Agent Management: `CreateAgent`, `DeleteAgent`, `AttachToAgent`
- Git Operations: `MergeMain`, `PushBranch`, `FetchRemote`
- PM Integration: `AssignAsanaTask`, `CycleTaskStatus`
- UI: `ToggleHelp`, `ToggleSettings`, `EnterInputMode`

---

### `src/app/config.rs` (1446 lines)
**Purpose**: Configuration structures and serialization

**Key Structures**:
- `Config`: Global configuration (1446 lines of code)
- `RepoConfig`: Repository-specific configuration
- `AiAgent`: ClaudeCode, Opencode, Codex, Gemini
- `GitProvider`: GitLab, GitHub, Codeberg
- `ProjectMgmtProvider`: Asana, Notion, ClickUp, Airtable, Linear
- `Keybinds`: Customizable keyboard shortcuts

**Config Methods**:
```rust
impl Config {
    pub fn load() -> Result<Self>
    pub fn save(&self) -> Result<()>
    pub fn config_dir() -> Result<PathBuf>
    pub fn config_path() -> Result<PathBuf>
    pub fn gitlab_token() -> Option<String>
    // ... token getters
}
```

---

### `src/app/state.rs`
**Purpose**: Application state management

**Key Types**:
- `AppState`: Central state container
- `PreviewTab`: Output, Logs, GitDiff, Tasks, DevServer
- `Toast`: Notification system
- `SettingsState`: Settings modal state
- Various modal states: `GlobalSetupState`, `ProjectSetupState`, `PmSetupState`, `GitSetupState`

---

### `src/app/task_list.rs`
**Purpose**: Task list types for PM integration

---

## Agent Module (`src/agent/`)

### `src/agent/mod.rs` (10 lines)
**Purpose**: Module exports for agent management

---

### `src/agent/model.rs` (368 lines)
**Purpose**: Agent data model and status types

**Key Types**:
```rust
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub branch: String,
    pub worktree_path: String,
    pub tmux_session: String,
    pub status: AgentStatus,
    pub git_status: Option<GitSyncStatus>,
    pub mr_status: MergeRequestStatus,
    pub pr_status: PullRequestStatus,
    pub pm_task_status: ProjectMgmtTaskStatus,
    pub activity_history: VecDeque<bool>,
    pub checklist_progress: Option<(u32, u32)>,
    pub continue_session: bool,
    pub ai_session_id: Option<String>,
}

pub enum AgentStatus {
    Running,
    AwaitingInput,
    Completed,
    Idle,
    Error(String),
    Stopped,
}

pub enum ProjectMgmtTaskStatus {
    None,
    Asana(AsanaTaskStatus),
    Notion(NotionTaskStatus),
    ClickUp(ClickUpTaskStatus),
    Airtable(AirtableTaskStatus),
    Linear(LinearTaskStatus),
}
```

---

### `src/agent/detector.rs`
**Purpose**: Real-time agent status detection from terminal output

**Key Functions**:
```rust
pub fn detect_status(output: &str) -> AgentStatus
pub fn detect_status_with_process(output: &str, foreground: Option<ForegroundProcess>) -> AgentStatus
pub fn detect_mr_url(output: &str) -> Option<String>
pub fn detect_checklist_progress(output: &str) -> Option<(u32, u32)>
```

---

### `src/agent/manager.rs`
**Purpose**: Agent lifecycle management

**Key Functions**:
- Create worktrees
- Create tmux sessions
- Attach to agents
- Cleanup resources

---

## Git Module (`src/git/`)

### `src/git/mod.rs` (9 lines)
**Purpose**: Module exports for git operations

---

### `src/git/worktree.rs`
**Purpose**: Git worktree management

**Key Functions**:
```rust
pub struct Worktree { ... }

impl Worktree {
    pub fn new(repo_path: &str, worktree_base: PathBuf) -> Self
    pub fn create(&self, name: &str, branch: &str) -> Result<PathBuf>
    pub fn delete(&self, path: &Path) -> Result<()>
    pub fn create_symlinks(&self, worktree_path: &Path, symlinks: &[String]) -> Result<()>
}
```

---

### `src/git/status.rs`
**Purpose**: Git status tracking

**Key Types**:
```rust
pub enum GitSyncStatus {
    UpToDate,
    Ahead(u32),
    Behind(u32),
    Diverged { ahead: u32, behind: u32 },
}
```

---

### `src/git/sync.rs`
**Purpose**: Git sync operations (fetch, merge, push)

---

### `src/git/remote.rs`
**Purpose**: Remote URL parsing and validation

---

## UI Module (`src/ui/`)

### `src/ui/mod.rs` (16 lines)
**Purpose**: Module exports for UI components

---

### `src/ui/app.rs`
**Purpose**: Main application widget and layout

**Key Types**:
```rust
pub struct AppWidget<'a> {
    state: &'a AppState,
    devserver_info: Option<DevServerRenderInfo>,
}

impl<'a> AppWidget<'a> {
    pub fn new(state: &'a AppState) -> Self
    pub fn with_devserver(mut self, info: Option<DevServerRenderInfo>) -> Self
    pub fn with_devserver_statuses(self, statuses: HashMap<Uuid, DevServerStatus>) -> Self
    pub fn render(self, frame: &mut Frame, area: Rect)
}
```

---

### `src/ui/appearance.rs`
**Purpose**: Colors, icons, and visual theming

**Key Constants**:
- `COLOR_PALETTE`: Available terminal colors
- `ICON_PALETTE`: Status icons
- `ICON_PRESETS`: Preset icon combinations

---

### `src/ui/helpers.rs`
**Purpose**: UI utility functions

**Key Functions**:
```rust
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect
pub fn render_field_line(f: &mut Frame, area: Rect, opts: FieldLineOptions)
pub fn token_status(status: &str, width: usize) -> String
pub fn token_status_line(status: &str, width: usize) -> Vec<Span>
```

---

### `src/ui/components/`

| File | Purpose |
|------|---------|
| `agent_list.rs` | Agent list rendering with selection |
| `status_bar.rs` | Bottom status bar with shortcuts |
| `help_overlay.rs` | Keyboard shortcuts overlay (`?`) |
| `settings_modal.rs` | Settings editor modal |
| `task_list_modal.rs` | PM task browser modal |
| `diff_view.rs` | Git diff viewer |
| `output_view.rs` | Agent output display |
| `devserver_view.rs` | Dev server logs viewer |
| `modal.rs` | Base modal component |
| `toast.rs` | Toast notifications |
| `loading_overlay.rs` | Loading indicator |
| `file_browser.rs` | File/directory browser |
| `column_selector.rs` | Column visibility selector |
| `status_dropdown.rs` | Status dropdown component |
| `setup_dialog.rs` | Generic setup dialog |
| `global_setup.rs` | First-launch setup wizard |
| `project_setup.rs` | Project configuration wizard |
| `pm_setup_modal.rs` | PM provider setup |
| `git_setup_modal.rs` | Git provider setup |
| `devserver_warning.rs` | Dev server start warning |
| `task_reassignment_warning.rs` | Task reassignment warning |
| `status_debug_overlay.rs` | Debug status overlay |
| `pm_status_debug_overlay.rs` | Debug PM status overlay |
| `system_metrics.rs` | CPU/memory metrics display |
| `tutorial_wizard.rs` | Tutorial overlay |

---

## Tmux Module (`src/tmux/`)

### `src/tmux/mod.rs` (3 lines)
**Purpose**: Module exports for tmux

---

### `src/tmux/session.rs`
**Purpose**: Tmux session management

**Key Functions**:
```rust
pub struct TmuxSession {
    name: String,
}

impl TmuxSession {
    pub fn new(name: &str) -> Self
    pub fn create(&self, worktree_path: &str, command: &[&str]) -> Result<()>
    pub fn exists(&self) -> bool
    pub fn attach(&self) -> Result<()>
    pub fn send_keys(&self, keys: &str) -> Result<()>
    pub fn kill(&self) -> Result<()>
}

pub fn is_tmux_available() -> bool
pub fn list_grove_sessions() -> Vec<String>
```

---

## DevServer Module (`src/devserver/`)

### `src/devserver/mod.rs` (7 lines)
**Purpose**: Module exports for dev server

---

### `src/devserver/manager.rs`
**Purpose**: Manages multiple dev servers

**Key Types**:
```rust
pub struct DevServerManager {
    servers: HashMap<Uuid, DevServer>,
    action_tx: mpsc::UnboundedSender<Action>,
}

impl DevServerManager {
    pub fn new(tx: mpsc::UnboundedSender<Action>) -> Self
    pub fn get(&self, id: Uuid) -> Option<&DevServer>
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut DevServer>
    pub fn start(&mut self, id: Uuid, config: DevServerConfig, ...) -> Result<()>
    pub fn stop(&mut self, id: Uuid) -> Result<()>
    pub fn all_statuses(&self) -> HashMap<Uuid, DevServerStatus>
}
```

---

### `src/devserver/process.rs`
**Purpose**: Individual dev server process management

**Key Types**:
```rust
pub enum DevServerStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Error(String),
}

pub struct DevServer {
    agent_id: Uuid,
    command: String,
    working_dir: PathBuf,
    status: DevServerStatus,
    logs: Vec<String>,
    child: Option<Child>,
}

impl DevServer {
    pub fn start(&mut self, agent_name: &str) -> Result<()>
    pub fn stop(&mut self) -> Result<()>
    pub fn append_log(&mut self, line: String)
    pub fn status(&self) -> &DevServerStatus
    pub fn logs(&self) -> &[String]
}
```

---

## Core Integrations (`src/core/`)

### `src/core/mod.rs` (3 lines)
**Purpose**: Module exports for core

---

### `src/core/common/`
- `mod.rs`: Common utilities
- `string_utils.rs`: String manipulation utilities

---

### `src/core/git_providers/`

| Provider | Files | Purpose |
|----------|-------|---------|
| `gitlab/` | 4 | GitLab API (MRs, projects) |
| `github/` | 4 | GitHub API (PRs) |
| `codeberg/` | 6 | Codeberg API (PRs, Woodpecker CI, Forgejo) |
| `helpers.rs` | Shared API utilities |

---

### `src/core/projects/`

| Provider | Files | Purpose |
|----------|-------|---------|
| `asana/` | 3 | Asana tasks |
| `notion/` | 3 | Notion databases |
| `clickup/` | 3 | ClickUp tasks |
| `airtable/` | 3 | Airtable records |
| `linear/` | 3 | Linear issues |
| `helpers.rs` | Shared utilities |
| `statuses.rs` | Common status types |

---

## Storage Module (`src/storage/`)

### `src/storage/mod.rs` (3 lines)
**Purpose**: Module exports for storage

---

### `src/storage/session.rs`
**Purpose**: Session persistence

**Key Functions**:
```rust
pub struct SessionStorage {
    path: PathBuf,
}

impl SessionStorage {
    pub fn new(repo_path: &str) -> Result<Self>
    pub fn load(&self) -> Result<Option<Session>>
    pub fn save(&self, agents: &[Agent], selected_index: usize) -> Result<()>
}
```

---

## Automation Module (`src/automation/`)

### `src/automation/mod.rs` (2 lines)
**Purpose**: Module exports for automation

---

### `src/automation/executor.rs`
**Purpose**: Script execution hooks

**Key Functions**:
```rust
pub async fn execute_automation(
    action_type: AutomationActionType,
    agent: &Agent,
    config: &AutomationConfig,
) -> Result<()>
```

---

## AI Agent Modules

### Claude Code (`src/claude_code/`)
- `mod.rs`: Module exports
- `session.rs`: Session detection and command building

### Opencode (`src/opencode/`)
- `mod.rs`: Module exports
- `session.rs`: Session detection and command building

### Codex (`src/codex/`)
- `mod.rs`: Module exports
- `session.rs`: Session detection and command building

### Gemini (`src/gemini/`)
- `mod.rs`: Module exports
- `session.rs`: Session detection and command building

---

## Other Modules

### `src/ci/mod.rs` & `types.rs`
**Purpose**: CI/CD integration types

---

### `src/cache/mod.rs`
**Purpose**: Caching utilities

---

### `src/version.rs`
**Purpose**: Version information (build-time generated)

---

### `src/main.rs` (1600+ lines)
**Purpose**: Application entry point

**Key Functions**:
```rust
fn matches_keybind(key: KeyEvent, keybind: &Keybind) -> bool
#[tokio::main]
async fn main() -> Result<()>
fn handle_key_event(key: KeyEvent, state: &AppState) -> Option<Action>
// Plus many action handler functions
```

---

## File Organization Summary

```
src/
├── main.rs                 # Entry point (~1600 lines)
├── lib.rs                 # Library root (16 lines)
├── version.rs             # Version info
│
├── app/                   # Core application
│   ├── action.rs          # Action definitions (~450 lines)
│   ├── config.rs          # Configuration (~1450 lines)
│   ├── state.rs           # State management
│   ├── task_list.rs       # Task types
│   └── mod.rs
│
├── agent/                 # Agent management
│   ├── model.rs           # Agent data model (~370 lines)
│   ├── detector.rs        # Status detection
│   ├── manager.rs         # Agent lifecycle
│   └── mod.rs
│
├── git/                   # Git operations
│   ├── worktree.rs        # Worktree management
│   ├── status.rs          # Git status
│   ├── sync.rs            # Fetch/merge/push
│   ├── remote.rs          # Remote handling
│   └── mod.rs
│
├── ui/                    # Terminal UI
│   ├── app.rs             # Main widget
│   ├── appearance.rs      # Theming
│   ├── helpers.rs         # Utilities
│   ├── mod.rs
│   └── components/        # 25+ components
│
├── tmux/                  # Tmux integration
│   ├── session.rs         # Session management
│   └── mod.rs
│
├── devserver/             # Dev server
│   ├── manager.rs         # Server coordinator
│   ├── process.rs         # Process management
│   └── mod.rs
│
├── core/                  # External integrations
│   ├── common/            # Utilities
│   ├── git_providers/    # GitLab, GitHub, Codeberg
│   └── projects/         # PM integrations
│
├── storage/               # Persistence
├── automation/           # Automation hooks
├── claude_code/          # Claude Code
├── opencode/             # Opencode
├── codex/                # Codex
├── gemini/               # Gemini
├── ci/                   # CI types
└── cache/                # Caching
```

---

*Generated for Grove v0.2.0*
