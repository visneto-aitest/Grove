# Grove + Airtable Integration Guide

This guide explains how to configure Grove to sync with Airtable for project management.

## Overview

Grove can integrate with Airtable to:

- Display tasks from an Airtable table as project management items
- Sync task status changes (Not Started → In Progress → Done)
- Track task URLs and link them to agents

## Prerequisites

- An Airtable account with at least one base
- A personal access token (PAT) from Airtable

## Configuration Steps

### Step 1: Get Your Airtable Personal Access Token

1. Go to [Airtable Developer Hub](https://airtable.com/create/tokens)
2. Click "Create token"
3. Set a name (e.g., "Grove Integration")
4. Select scopes: `data.records:read`, `data.records:write`, `schema.bases:read`
5. Select the bases you want to access
6. Copy the generated token

### Step 2: Set the Environment Variable

Add the token to your shell profile:

```bash
# For Zsh (most common on macOS)
echo 'export AIRTABLE_TOKEN="pat_your_token_here"' >> ~/.zshrc
source ~/.zshrc



# For Bash
echo 'export AIRTABLE_TOKEN="pat_your_token_here"' >> ~/.bashrc
source ~/.bashrc
```

### Step 3: Configure Repository Settings

Create or update `.grove/project.toml` in your repository:

```toml
[project_mgmt]
provider = "airtable"

[project_mgmt.airtable]
base_id = "appXXXXXXXXXXXXXX"  # Your Airtable base ID
table_name = "Tasks"           # Name of the table with tasks
status_field_name = "Status"  # Name of the single-select status field
```

#### Finding Your Base ID

Your base ID starts with `app` and is 17 characters. You can find it:

- In the Airtable URL: `https://airtable.com/appXXXXXXXXXXXXXX/...`
- Or using Grove's setup wizard (press `P` in the main view)

#### Finding Your Table and Field Names

- Table name: The name shown in your base's tabs (e.g., "Tasks", "Projects")
- Status field: A single-select field containing your workflow stages (e.g., "Not Started", "In Progress", "Done")

### Step 4: Optional Advanced Settings

```toml
[project_mgmt.airtable]
# Override default status matching
in_progress_option = "In Progress"   # Custom status for "In Progress"
done_option = "Done"                  # Custom status for "Done"
```

## Using the Setup Wizard

Grove provides a guided setup wizard. Press `P` from the main view to access it:

1. **Step 1: Token** - Verify your AIRTABLE_TOKEN is set
2. **Step 2: Base & Table** - Select your base and table from dropdowns
3. **Step 3: Advanced** - Optionally customize status field names and mappings

## How It Works

### Task Status Mapping

Grove automatically maps status changes:

| Grove Action        | Airtable Status Search                          |
| ------------------- | ----------------------------------------------- |
| Move to In Progress | "in progress", "doing", "active"                |
| Move to Done        | "done", "complete", "closed", "resolved"        |
| Move to Not Started | "not started", "todo", "backlog", "new", "open" |

If your Airtable uses different names, use the advanced settings to override.

### Task URLs

When you link a task to an agent, Grove parses the Airtable record URL:

- Full URL: `https://airtable.com/appXXX/tblYYY/recZZZ?blocks=hide`
- Short ID: `recZZZ`

Both formats work for linking and status updates.

## Troubleshooting

### "Airtable not configured" message

Ensure all four requirements are met:

1. `AIRTABLE_TOKEN` environment variable is set
2. `base_id` is configured in `.grove/project.toml`
3. `table_name` is configured
4. Status field exists in your table

### Authentication errors

- Verify your token hasn't expired
- Check that your token has the required scopes (`data.records:read`, `data.records:write`, `schema.bases:read`)
- Ensure you have access to the specified base

### Task status not updating

- Verify the status field is a "Single select" field type
- Check that the status value you're trying to set exists in your options
- Use the advanced settings to specify exact status values

### Cache issues

Grove caches Airtable data for 60 seconds by default. To force a refresh:

- Wait for the cache to expire (automatic)
- Or restart Grove

To change cache duration, add to global config `~/.grove/config.toml`:

```toml
[airtable]
cache_ttl_secs = 30
refresh_secs = 60
```

## Example Airtable Setups

### Minimal Setup (Basic Workflow)

A simple base with just the required fields:

| Name   | Status      |
| ------ | ----------- |
| Task 1 | Not Started |
| Task 2 | In Progress |
| Task 3 | Done        |

**Requirements:**
- **Name** field (single line text)
- **Status** field (single select)

---

### Complete Setup (Full Feature Set)

This example includes all fields Grove supports for the richest experience:

**Table: Tasks**

| Name | Status | Description | Assignee | Due Date | Parent Task | Priority | Tags |
|------|--------|-------------|----------|---------|------------|----------|------|
| Fix login bug | In Progress | Users cannot log in with SSO | John | 2024-03-15 | | High | bug,urgent |
| Add dark mode | Not Started | Add theme toggle to settings | Jane | 2024-03-20 | | Medium | feature,ui |
| API rate limiting | Done | Implement rate limiting | John | 2024-03-01 | | High | backend |
| User dashboard | Not Started | Create analytics dashboard | | 2024-04-01 | Add dark mode | Low | feature |
| Database migration | In Progress | Migrate to PostgreSQL | | 2024-03-25 | | High | backend |

**Field Definitions:**

| Field Name | Field Type | Description |
|-----------|------------|-------------|
| Name | Single line text | Task title (required) |
| Status | Single select | Workflow status (required) - values: Not Started, In Progress, Done |
| Description | Long text | Detailed task description |
| Assignee | Collaborator | User assigned to the task |
| Due Date | Date | Task due date |
| Parent Task | Link to another record | Self-referential link for subtasks |
| Priority | Single select | Priority level - values: Low, Medium, High |
| Tags | Multiple select | Labels - values: bug, feature, ui, backend, urgent |

**Status Field Options:**

```
Not Started
In Progress
Done
```

**Priority Field Options:**

```
Low
Medium
High
```

**Tags Field Options:**

```
bug
feature
ui
backend
urgent
```

---

### SubtAsk/Parent Structure

Grove supports hierarchical tasks via a self-referential link:

1. Create a field named "Parent Task" (link to "Tasks" table)
2. Tasks with a parent are marked as subtasks
3. Tasks with children are parents

**Example:**

| Name | Status | Parent Task |
|------|--------|-------------|
| Parent Task A | In Progress | |
| Subtask 1 | Done | Parent Task A |
| Subtask 2 | In Progress | Parent Task A |

Grove will display subtasks with a visual indicator and allow moving the parent task to update all children.

---

### Multi-Base Setup

For larger organizations with multiple bases:

**Base: Engineering**

- Table: Sprint Tasks
- Table: Bugs
- Table: Tech Debt

**Base: Product**

- Table: Feature Requests
- Table: Research

Each base can be configured separately in `.grove/project.toml`:

```toml
[project_mgmt]
provider = "airtable"

[project_mgmt.airtable]
base_id = "appXXXXXXXXXXXXXX"
table_name = "Sprint Tasks"
status_field_name = "Status"
in_progress_option = "In Progress"
done_option = "Done"
```

---

### Custom Status Workflow

If your team uses different status names:

**Single Select Options:**

```
To Do
In Progress
Review
Done
```

Configure Grove to match:

```toml
[project_mgmt.airtable]
status_field_name = "Status"
in_progress_option = "In Progress"
done_option = "Done"
```

Grove maps actions:
- Move to In Progress → "In Progress"
- Move to Done → "Done"
- Move to Not Started → "To Do" (falls back to first option)

## Key Bindings

Once configured, use these keybinds in Grove:

| Key | Action                          |
| --- | ------------------------------- |
| `P` | Open PM setup wizard            |
| `l` | Link current agent to a task    |
| `u` | Unlink task from agent          |
| `i` | Move linked task to In Progress |
| `x` | Move linked task to Done        |

## Architecture Reference

The Airtable integration consists of:

- `src/core/projects/airtable/client.rs` - API client with caching
- `src/core/projects/airtable/types.rs` - Data structures
- `src/app/config.rs` - Configuration structs (`AirtableConfig`, `RepoAirtableConfig`)

### API Endpoints Used

- `GET /v0/meta/bases` - List accessible bases
- `GET /v0/meta/bases/{base_id}/tables` - List tables in a base
- `GET /v0/{base_id}/{table_name}` - List records (with pagination)
- `PATCH /v0/{base_id}/{table_name}` - Update record status
