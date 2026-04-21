# Beads Integration Technical Documentation

## Overview

This document describes the integration between Grove and [Beads](https://github.com/gastownhall/beads), a distributed graph issue tracker for AI agents powered by Dolt.

## What is Beads?

Beads is a memory upgrade for coding agents that provides:
- **Distributed graph issue tracking** with dependency-aware task relationships
- **Version-controlled SQL database** powered by Dolt (cell-level merge, native branching)
- **Hash-based IDs** (`bd-a1b2`) to prevent merge collisions in multi-agent workflows
- **Hierarchical task support** (epics → tasks → sub-tasks)
- **Messaging system** with threading and ephemeral lifecycle

## Integration Architecture

### Module Structure

```
src/core/projects/beads/
├── mod.rs        # Module exports and BeadsClient definition
├── types.rs      # Data structures (BeadsWorkspace, BeadsTeam, BeadsConfig)
└── client.rs    # OptionalBeadsClient wrapper
```

### Core Components

#### 1. Data Types (`types.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BeadsWorkspace {
    pub id: String,
    pub name: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BeadsTeam {
    pub id: String,
    pub name: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeadsConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub workspace_id: Option<String>,
    #[serde(default)]
    pub team_id: Option<String>,
}
```

#### 2. BeadsClient (`mod.rs`)

The `BeadsClient` provides:
- **Bearer token authentication** via the Beads API
- **Workspace fetching** - `GET https://api.beads.xyz/api/v1/workspaces`
- **Team/database fetching** - `GET https://api.beads.xyz/api/v1/teams`
- **In-memory caching** with mutex-protected workspace and database caches

```rust
pub struct BeadsClient {
    client: Client,
    cached_workspaces: Mutex<Option<Vec<(String, String, String)>>>, // (id, name, key)
    cached_databases: Mutex<Option<Vec<(String, String, String)>>>, // (id, name, key)
}

impl BeadsClient {
    pub fn new(token: &str) -> Result<Self>
    pub async fn fetch_workspaces(&self) -> Result<Vec<(String, String, String)>>
    pub async fn fetch_databases(&self) -> Result<Vec<(String, String, String)>>
    pub async fn get_cached_workspaces(&self) -> Option<Vec<(String, String, String)>>
    pub async fn get_cached_databases(&self) -> Option<Vec<(String, String, String)>>
    pub async fn set_cached_workspaces(&self, data: Vec<(String, String, String)>)
    pub async fn set_cached_databases(&self, data: Vec<(String, String, String)>)
    pub async fn invalidate_cache(&self)
}
```

#### 3. OptionalBeadsClient (`client.rs`)

A lazy-loading wrapper that only creates the client when a token is provided:

```rust
pub struct OptionalBeadsClient {
    client: RwLock<Option<BeadsClient>>,
}

impl OptionalBeadsClient {
    pub fn new(token: Option<&str>) -> Option<Self>
    pub async fn get_workspaces(&self) -> Result<Vec<(String, String, String)>>
    pub async fn get_databases(&self) -> Result<Vec<(String, String, String)>>
}
```

## API Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `https://api.beads.xyz/api/v1/workspaces` | GET | List user's workspaces |
| `https://api.beads.xyz/api/v1/teams` | GET | List teams (databases) |

## Configuration

Grove uses a two-level configuration system for project management integrations. Beads follows the same pattern as other integrations like Linear and Asana.

### Global Config (`~/.grove/config.toml`)

Global settings apply to all projects. Currently no global beads-specific settings are required.

```toml
[global]
ai_agent = "claude-code"
log_level = "info"
```

### Project Config (`.grove/project.toml`)

Project-specific settings go in the `.grove/` directory in your project.

```toml
[project_management]
provider = "beads"

[beads]
workspace_id = "ws-xxxxx"
team_id = "team-xxxxx"
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `BEADS_TOKEN` | API token for authentication (required) |

Get your API token from the Beads dashboard at https://beads.xyz after creating an account.

### Configuration Fields

| Field | Type | Description |
|-------|------|-------------|
| `provider` | string | Set to `"beads"` to use Beads as the project management backend |
| `workspace_id` | string | The workspace ID to use (optional - can be selected in UI) |
| `team_id` | string | The team/database ID to use (optional - can be selected in UI) |

### Understanding Workspaces vs Teams

In Beads cloud service (api.beads.xyz):

- **Workspace** - A user account container. When you sign up at beads.xyz, you get a personal workspace. Organizations can have shared workspaces.
- **Team** - A database containing issues for a specific project. Each team maps to a Dolt database.

```
Workspace (e.g., "Acme Corp")
├── Team: "backend-project" (database: backend_issues)
├── Team: "frontend-project" (database: frontend_issues)
└── Team: "mobile-app" (database: mobile_issues)
```

The `workspace_id` and `team_id` fields in Grove config map to these concepts:
- `workspace_id`: Your workspace (account or organization)
- `team_id`: The specific project database you want to track issues from

**Alternative: Local Beads (CLI)**

If you prefer local-first issue tracking without the cloud service, use the `bd` CLI directly instead of the cloud API. Grove can integrate with local Dolt databases in each project's `.beads/` directory - this would require additional integration work.

### Example Complete Configuration

```toml
# .grove/project.toml in your project

[git]
provider = "gitlab"
branch_prefix = "feature/"
main_branch = "main"

[project_management]
provider = "beads"

[beads]
workspace_id = "ws-abc123"
team_id = "team-xyz789"
```

## Setup Steps

1. **Create a Beads account** at https://beads.xyz
2. **Get your API token** from the Beads dashboard
3. **Set the environment variable** in your shell:
   ```bash
   export BEADS_TOKEN=your_token_here
   ```
4. **Initialize your project** with beads:
   ```bash
   cd your-project
   mkdir -p .grove
   ```
5. **Create `.grove/project.toml`** with the config above
6. **Restart Grove** to pick up the configuration

## Usage Patterns

### Initializing the Client

```rust
use crate::core::projects::beads::client::OptionalBeadsClient;

let token = std::env::var("BEADS_TOKEN").ok();
let client = OptionalBeadsClient::new(token.as_deref());
```

### Fetching Workspaces

```rust
let workspaces = client.get_workspaces().await?;
for (id, name, key) in workspaces {
    println!("Workspace: {} ({}) - {}", name, key, id);
}
```

### Fetching Teams/Databases

```rust
let databases = client.get_databases().await?;
for (id, name, key) in databases {
    println!("Team: {} ({}) - {}", name, key, id);
}
```

## Current Implementation Status

The Beads integration is **partially implemented**:

- ✅ Module structure created with types and client
- ✅ API client fetches workspaces and teams
- ✅ Bearer token authentication
- ✅ In-memory caching with invalidation
- ❌ Not added to `Config` struct in `src/app/config.rs`
- ❌ Not added to `RepoProjectMgmtConfig` in `src/app/config.rs`
- ❌ Not registered in `src/core/projects/mod.rs`
- ❌ No UI components for viewing/selecting beads workspaces
- ❌ No action handlers for beads operations

### What's Needed to Complete Integration

To fully integrate Beads into Grove, you need to add the following components:

#### 1. Global Config (`src/app/config.rs`)

In the `Config` struct around line 430, add:

```rust
#[serde(default)]
pub beads: BeadsConfig,
```

Add the `BeadsConfig` struct (similar to `LinearConfig`):

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeadsConfig {
    #[serde(default)]
    pub refresh_secs: u64,
    #[serde(default)]
    pub cache_ttl_secs: u64,
}

impl Default for BeadsConfig {
    fn default() -> Self {
        Self {
            refresh_secs: 120,
            cache_ttl_secs: 60,
        }
    }
}
```

#### 2. Repo Config (`src/app/config.rs`)

In `RepoProjectMgmtConfig` around line 1225, add:

```rust
#[serde(default)]
pub beads: RepoBeadsConfig,
```

Add `RepoBeadsConfig` struct:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RepoBeadsConfig {
    pub workspace_id: Option<String>,
    pub team_id: Option<String>,
}
```

Update `default()` to include beads:

```rust
let default = Config::default();
repo_config.project_management.beads = RepoBeadsConfig::default();
```

#### 3. Register Module (`src/core/projects/mod.rs`)

Add:

```rust
pub mod beads;
```

#### 4. Add to ProjectClients (`src/core/projects/statuses.rs`)

Import and add to struct:

```rust
use crate::core::projects::beads::OptionalBeadsClient;

pub struct ProjectClients {
    // ... existing fields
    pub beads: Arc<OptionalBeadsClient>,
}
```

Add match arm in `fetch_status_options`:

```rust
ProjectMgmtTaskStatus::Beads(beads_status) => {
    fetch_beads_status_options(beads_status, &clients.beads).await
}
```

#### 5. Add Task Status Enum (`src/agent/mod.rs` or wherever ProjectMgmtTaskStatus is defined)

Add `Beads` variant to `ProjectMgmtTaskStatus`:

```rust
pub enum ProjectMgmtTaskStatus {
    // ... existing
    Beads(BeadsTaskStatus),
}
```

#### 6. Wire Client Initialization

Update wherever `ProjectClients` are initialized (likely in app state setup) to include beads:

```rust
let beads_token = std::env::var("BEADS_TOKEN").ok();
let beads = Arc::new(OptionalBeadsClient::new(beads_token.as_deref()));
```

#### 7. UI Integration

- Add beads to the project picker UI
- Add workspace/team selector dropdown
- Display tasks from Beads API

#### 8. Add Provider Variant

In `ProjectMgmtProvider` enum, add:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ProjectMgmtProvider {
    // ...
    #[default]
    Beads,
}
```

---

## Files to Modify

| File | Changes |
|------|--------|
| `src/app/config.rs` | Add `BeadsConfig`, `RepoBeadsConfig`, wires in structs |
| `src/core/projects/mod.rs` | Add `pub mod beads;` |
| `src/core/projects/statuses.rs` | Add `beads` field to `ProjectClients`, fetch function |
| `src/agent/mod.rs` | Add `Beads` variant to `ProjectMgmtTaskStatus` |
| Source file for provider enum | Add `Beads` to `ProjectMgmtProvider` |
| UI components | Add beads project picker, workspace selector |

## Future Enhancements

1. **Register beads in project module** - Add to `src/core/projects/mod.rs`
2. **UI integration** - Add workspace/team selector to the project picker
3. **Task synchronization** - Sync tasks between Grove and Beads
4. **Full CRUD operations** - Create, update, close tasks via Beads API
5. **Dependency graph display** - Show task relationships in the TUI

## References

- [Beads GitHub](https://github.com/gastownhall/beads)
- [Beads Documentation](https://gastownhall.github.io/beads/)
- [Beads CLI Commands](https://github.com/gastownhall/beads/blob/main/docs/COMMANDS.md)