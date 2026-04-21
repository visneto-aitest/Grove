# Project Management Integration Capability

**Capability Category**: External Integration - Task Tracking

**Source of Truth**: Reverse-engineered from source code analysis

---

## Supported Providers

### 1. Asana Integration

#### Requirements (SHALL/MUST)
- **MUST** support task assignment via task URL or GID
- **MUST** fetch task status (status string)
- **MUST** provide caching with configurable TTL

#### Environment Variable
- `ASANA_TOKEN`

#### Code References
- Client: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/asana/client.rs`
- Types: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/asana/types.rs`

---

### 2. Notion Integration

#### Requirements (SHALL/MUST)
- **MUST** support database-backed task tracking
- **MUST** fetch page status via status property
- **MUST** parse page IDs from URLs

#### Environment Variable
- `NOTION_TOKEN`

#### Configuration
- `database_id` - Notion database ID
- `status_property_name` - Property name for status

#### Code References
- Client: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/notion/client.rs`
- Types: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/notion/types.rs`

---

### 3. ClickUp Integration

#### Requirements (SHALL/MUST)
- **MUST** support list-based task management
- **MUST** fetch task status
- **MUST** parse task IDs from URLs

#### Environment Variable
- `CLICKUP_TOKEN`

#### Configuration
- `list_id` - ClickUp list ID

#### Code References
- Client: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/clickup/client.rs`
- Types: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/clickup/types.rs`

---

### 4. Airtable Integration

#### Requirements (SHALL/MUST)
- **MUST** support base/table structure
- **MUST** fetch record status via field
- **MUST** parse record IDs

#### Environment Variable
- `AIRTABLE_TOKEN`

#### Configuration
- `base_id` - Airtable base ID
- `table_name` - Table name
- `status_field_name` - Field for status

#### Code References
- Client: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/airtable/client.rs`
- Types: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/airtable/types.rs`

---

### 5. Linear Integration

#### Requirements (SHALL/MUST)
- **MUST** support team-based issue tracking
- **MUST** fetch issue status
- **MUST** parse issue IDs from URLs

#### Environment Variable
- `LINEAR_TOKEN`

#### Configuration
- `team_id` - Linear team ID

#### Code References
- Client: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/linear/client.rs`
- Types: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/linear/types.rs`

---

## Common Abstractions

### Requirements (SHALL/MUST)
- **MUST** provide `OptionalClient` wrapper for graceful "not configured" handling
- **MUST** provide `fetch_status_options` to get available status values
- **MUST** provide HTTP helpers (http_get, http_post, http_put)

#### Code References
- Helpers: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/helpers.rs`
- Statuses: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/projects/statuses.rs`

---

## Task Status Mapping

Each provider maps to a common `ProjectMgmtTaskStatus` enum with variants:
- Linked (with provider-specific ID and status)
- None