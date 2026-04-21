# Configuration Management Capability

**Capability Category**: Application Configuration

**Source of Truth**: Reverse-engineered from source code analysis

---

## Configuration Architecture

### Two-Level Configuration

Grove uses a two-level configuration system:

1. **Global Config** (~/.grove/config.toml) - User preferences
2. **Project Config** (.grove/project.toml) - Project-specific settings

---

## Global Config

### File Location
- `~/.grove/config.toml`

### Sections

#### Global Section
```toml
[global]
ai_agent = "claude-code"  # claude-code, opencode, codex, gemini
log_level = "info"       # debug, info, warn, error
worktree_location = "project"  # project or home
```

#### UI Section
```toml
[ui]
frame_rate = 30
tick_rate_ms = 250
output_buffer_lines = 5000
show_banner = true
show_preview = true
show_metrics = true
show_logs = false
```

#### Performance Section
```toml
[performance]
agent_poll_ms = 500
git_refresh_secs = 30
gitlab_refresh_secs = 30
github_refresh_secs = 30
codeberg_refresh_secs = 30
```

#### Provider Cache TTL
```toml
[asana]
refresh_secs = 30
cache_ttl_secs = 300

[notion]
refresh_secs = 30
cache_ttl_secs = 300

[clickup]
refresh_secs = 30
cache_ttl_secs = 300

[airtable]
refresh_secs = 30
cache_ttl_secs = 300

[linear]
refresh_secs = 30
cache_ttl_secs = 300
```

---

## Project Config

### File Location
- `<repo>/.grove/project.toml`

### Sections

#### Git Section
```toml
[git]
provider = "gitlab"           # gitlab, github, codeberg
branch_prefix = "feature/"
main_branch = "main"
# GitLab-specific
[git.gitlab]
base_url = "https://gitlab.com"
project_id = ""

# GitHub-specific
[git.github]
owner = ""
repo = ""

# Codeberg-specific
[git.codeberg]
base_url = "https://codeberg.org"
owner = ""
repo = ""
ci_provider = "forgejo-actions"  # forgejo-actions, woodpecker
```

#### Dev Server Section
```toml
[dev_server]
command = "npm run dev"
port = 3000
auto_start = false
worktree_symlinks = []  # Files to symlink from main repo
```

#### Project Management Section
```toml
[project_mgmt]
provider = "asana"  # asana, notion, clickup, airtable, linear

# Asana-specific
[project_mgmt.asana]
project_gid = ""

# Notion-specific
[project_mgmt.notion]
database_id = ""
status_property_name = "Status"

# ClickUp-specific
[project_mgmt.clickup]
list_id = ""

# Airtable-specific
[project_mgmt.airtable]
base_id = ""
table_name = ""
status_field_name = "Status"

# Linear-specific
[project_mgmt.linear]
team_id = ""
```

---

## Configuration Loading

### Requirements (SHALL/MUST)
- **MUST** load global config from ~/.grove/config.toml
- **MUST** create config directory if not exists
- **MUST** merge with defaults if file missing
- **MUST** load project config from .grove/project.toml
- **MUST** auto-detect provider from git remotes

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/app/config.rs` lines 1-1503

---

## Keybind Configuration

### Requirements (SHALL/MUST)
- **MUST** define keybinds with modifiers and key
- **MUST** support customization per action
- **MUST** provide defaults

#### Default Keybinds

| Action | Default Keybind |
|--------|----------------|
| nav_down | "j" |
| nav_up | "k" |
| nav_first | "g" |
| nav_last | "G" |
| new_agent | "n" |
| delete_agent | "d" |
| attach | "Enter" |
| quit | "q" |
| refresh_task_list | "r" |
| toggle_help | "?" |
| toggle_settings | "S" |
| toggle_diff_view | "/" |