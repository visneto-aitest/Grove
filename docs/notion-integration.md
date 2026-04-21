# Grove + Notion Integration Guide

This guide explains how to configure Grove to sync with Notion.so for project management.

## Overview

Grove can integrate with Notion to:
- Display tasks from a Notion database as project management items
- Sync task status changes via Notion's Status property
- Support parent-child task relationships via relation fields
- Track task URLs and link them to agents

## Prerequisites

- A Notion account with access to at least one database
- An integration token (Internal or Public integration)

## Configuration Steps

### Step 1: Create a Notion Integration

1. Go to [Notion My Integrations](https://www.notion.so/my-integrations)
2. Click "New integration" or "Create new integration"
3. Set a name (e.g., "Grove Integration")
4. Copy the generated "Internal Integration Token"

### Step 2: Share Your Database with the Integration

1. Open your Notion database in Notion
2. Click the `...` menu (top right) → "Connections" → "Add connections"
3. Select your Grove integration and click "Confirm"

### Step 3: Set the Environment Variable

Add the token to your shell profile:

```bash
# For Zsh (most common on macOS)
echo 'export NOTION_TOKEN="secret_your_token_here"' >> ~/.zshrc
source ~/.zshrc

# For Bash
echo 'export NOTION_TOKEN="secret_your_token_here"' >> ~/.bashrc
source ~/.bashrc
```

### Step 4: Configure Repository Settings

Create or update `.grove/project.toml` in your repository:

```toml
[project_mgmt]
provider = "notion"

[project_mgmt.notion]
database_id = "your_database_id"           # Notion database ID
status_property_name = "Status"            # Name of the Status property
```

#### Finding Your Database ID

Your database ID is a 32-character UUID. You can find it:
- In the Notion URL: `https://notion.so/{workspace}/{database_id}?v=...`
- The ID appears after the workspace name and before the `?v=` query param

Example:
```
https://notion.so/team-12345/My-Database-abcdef01?...
                           ^^^^^^^^^^^^^^^^
                           Database ID
```

You can also use Grove's setup wizard (press `P`) to select from your accessible databases.

### Step 5: Optional Advanced Settings

```toml
[project_mgmt.notion]
# Override default status matching
in_progress_option = "In Progress"   # Custom status for "In Progress"
done_option = "Done"              # Custom status for "Done"
# Parent-child task relationships
# (uses "Tasks" relation property by default)
```

## Using the Setup Wizard

Grove provides a guided setup wizard. Press `P` from the main view to access it:

1. **Step 1: Token** - Verify your NOTION_TOKEN is set
2. **Step 2: Database** - Select your database from dropdown
3. **Step 3: Advanced** - Optionally customize status property name and mappings

## How It Works

### Task Status Mapping

Grove uses Notion's built-in Status property. The Status property has three groups:
- **Not Started** (gray): "Not started", "To Do"
- **In Progress** (blue): "In progress", "In Review"
- **Done** (green): "Done", "Complete"

Grove automatically maps status changes based on keywords:

| Grove Action | Notion Status Search |
|--------------|-------------------|
| Move to In Progress | "in progress", "in review", "doing", "active" |
| Move to Done | "done", "complete", "closed" |
| Move to Not Started | "not started", "to do", "todo", "backlog", "new", "open" |

### Parent-Child Task Relationships

Grove supports hierarchical tasks via Notion relation properties:

1. Create a relation property named "Tasks" in your database
2. Link child pages to parent pages
3. Grove will fetch both parent and child tasks, sorting children after their parents

### Task URLs

When you link a task to an agent, Grove parses the Notion page URL:
- Full URL: `https://notion.so/page/32-char-uuid?v=...`
- Short ID: The 32-character page ID

Both formats work for linking and status updates.

## Database Requirements

Your Notion database must have:

| Property | Type | Required |
|----------|------|----------|
| Title | Title | Yes |
| Status | Status | Yes |

Optional properties for parent-child relationships:

| Property | Type | Purpose |
|----------|------|---------|
| Tasks | Relation | Link child tasks to parents |

Example database structure:

| Title | Status | Tasks |
|-------|-------|-------|
| Task 1 | Not Started | |
| Task 2 | In Progress | Task 1 |
| Task 3 | Done | Task 1 |

## Troubleshooting

### "Notion not configured" message

Ensure all three requirements are met:
1. `NOTION_TOKEN` environment variable is set
2. `database_id` is configured in `.grove/project.toml`
3. Database is shared with your integration (see Step 2)

### Authentication errors

- Verify your token hasn't been revoked
- Check that the integration has access to the database (share the database again if needed)
- Ensure the integration is an Internal integration with database access

### Task status not updating

- Verify the Status field is the built-in Status property type (not a single-select)
- Check that the status value you're trying to set exists in your Status options
- Use the advanced settings to specify exact status values

### Database not visible in setup wizard

1. Verify the database is accessible to your integration
2. Go to the database → `...` menu → Connections → Add connections
3. Select your integration and confirm

### Cache issues

Grove caches Notion data for 60 seconds by default. To force a refresh:
- Wait for the cache to expire (automatic)
- Or restart Grove

To change cache duration, add to global config `~/.grove/config.toml`:

```toml
[notion]
cache_ttl_secs = 30
refresh_secs = 60
```

## Key Bindings

Once configured, use these keybinds in Grove:

| Key | Action |
|-----|--------|
| `P` | Open PM setup wizard |
| `l` | Link current agent to a task |
| `u` | Unlink task from agent |
| `i` | Move linked task to In Progress |
| `x` | Move linked task to Done |

## Architecture Reference

The Notion integration consists of:

- `src/core/projects/notion/client.rs` - API client with caching, status management
- `src/core/projects/notion/types.rs` - Data structures (NotionPageData, NotionTaskStatus)
- `src/app/config.rs` - Configuration structs (`NotionConfig`, `RepoNotionConfig`)

### API Endpoints Used

- `POST /v1/databases/{id}/query` - Query database pages
- `GET /v1/databases/{id}` - Get database schema (status options)
- `GET /v1/pages/{id}` - Get individual page
- `PATCH /v1/pages/{id}` - Update page status
- `POST /v1/search` - Search for databases (setup wizard)
- `PATCH /v1/blocks/{id}/children/append` - Append blocks to page

### Notion API Version

Grove uses Notion API version `2022-06-28`.