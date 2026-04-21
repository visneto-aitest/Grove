# Terminal UI Capability

**Capability Category**: User Interface - Terminal TUI

**Source of Truth**: Reverse-engineered from source code analysis

---

## Core Capabilities

### 1. UI Layout and Conditional Rendering

#### Requirements (SHALL/MUST)
- **MUST** provide UI config flags for show_banner, show_preview, show_metrics, show_logs.
- **MUST** use config to conditionally render UI sections.

#### GIVEN-WHEN-THEN Scenarios

**Scenario A: All panels visible**
- GIVEN AppState.config.ui with show_banner=true, show_preview=true, show_metrics=true, show_logs=true
- WHEN AppWidget::render(frame) is called
- THEN all sections (banner, agent list, preview, metrics, logs, footer) are rendered.

**Scenario B: Minimal UI**
- GIVEN only show_banner=true, all others false
- WHEN render is invoked
- THEN only the banner and footer render.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/ui/app.rs` lines 71-75, 96-128

---

### 2. Application State Model

#### Requirements (SHALL/MUST)
- **MUST** define AppState as central state container.
- **MUST** track agents map, agent_order, selected_index, config, running flag.
- **MUST** maintain UI state: show_help, input_mode, input_buffer.
- **MUST** maintain metrics buffers: cpu_history, memory_history.
- **MUST** support preview content and git diff content.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/app/state.rs` lines 1251-1299

---

### 3. Action System

#### Requirements (SHALL/MUST)
- **MUST** define Action enum with all capability variants.
- **MUST** support navigation actions: SelectNext/Previous/First/Last.
- **MUST** support agent lifecycle: CreateAgent, DeleteAgent, AttachToAgent.
- **MUST** support Git operations: MergeMain, PushBranch, FetchRemote.
- **MUST** support PM tasks: AssignProjectTask, UpdateProjectTaskStatus.
- **MUST** support Dev Server: StartDevServer, StopDevServer.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/app/action.rs` lines 9-457

---

### 4. Preview Content System

#### Requirements (SHALL/MUST)
- **MUST** define PreviewTab enum with variants: Preview, GitDiff, DevServer.
- **MUST** track preview_content and gitdiff_content.
- **MUST** render preview pane with agent context.

#### GIVEN-WHEN-THEN Scenarios

**Scenario C: Show preview content**
- GIVEN state.preview_content = Some(text)
- WHEN AppWidget renders the preview pane
- THEN OutputViewWidget renders the text.

**Scenario D: Show git diff**
- GIVEN gitdiff_content = Some(diff)
- WHEN DevPreview tab is showing
- THEN DiffViewWidget renders the diff content.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/app/state.rs` lines 31-36
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/ui/app.rs` lines 503-612

---

### 5. System Metrics

#### Requirements (SHALL/MUST)
- **MUST** track cpu_history and memory_history.
- **MUST** record system metrics at tick intervals.
- **MUST** render metrics in UI.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/app/state.rs` lines 1267-1280

---

### 6. Settings/Config Management

#### Requirements (SHALL/MUST)
- **MUST** define SettingsTab enum with tabs: General, Git, ProjectMgmt, DevServer, Automation, Keybinds, Appearance.
- **MUST** define SettingsField enum with all configurable fields.
- **MUST** support navigation (next/prev) and reset to defaults.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/app/state.rs` lines 54-201, 955-1159

---

### 7. Wizard/Setup Flows

#### Requirements (SHALL/MUST)
- **MUST** support GlobalSetupState for first-launch.
- **MUST** support ProjectSetupState for project config.
- **MUST** support TutorialState for onboarding.
- **MUST** render wizard widgets conditionally.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/app/state.rs` lines 1404-1430

---

### 8. Toast Notifications

#### Requirements (SHALL/MUST)
- **MUST** define Toast struct with message and level (Info, Warning, Error).
- **MUST** show ephemeral notifications.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/app/state.rs` lines 1530-1536

---

### 9. Log System

#### Requirements (SHALL/MUST)
- **MUST** track LogEntry struct with timestamp and message.
- **MUST** maintain logs Vec in AppState.
- **MUST** provide log helpers: log_info, log_error, log_debug.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/app/state.rs` lines 1507-1651

---

## Keyboard Shortcuts (Default Bindings)

| Key | Action |
|-----|--------|
| j/k | Navigate down/up |
| n | Create new agent |
| Enter | Attach to agent |
| d | Delete agent |
| c | Copy worktree cd command |
| m | Merge main |
| p | Push branch |
| o | Open MR/PR in browser |
| S | Toggle settings |
| / | Toggle diff view |
| ? | Toggle help |