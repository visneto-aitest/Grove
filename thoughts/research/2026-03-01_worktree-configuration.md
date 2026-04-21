---
date: 2026-03-01T18:25:00Z
git_commit: bf30270f2637077c4a6fcd304da3c650d4e8dee1
branch: researcher
repository: github.com/ziim/grove
topic: "How to configure different CLI tools to use different git worktrees"
tags: [research, codebase, git, worktree, configuration]
last_updated: 2026-03-01T18:25:00Z
---

## Ticket Synopsis

How to configure different CLI tools to use different git worktrees in the same folder. This question is about understanding Grove's worktree isolation mechanism and how to customize worktree locations.

## Summary

In Grove, **each AI agent automatically gets its own isolated git worktree**. The worktree location is configured globally per-project, not per-agent or per-CLI-tool. Here's how it works:

1. **One worktree per agent** - When you create an agent with a branch, Grove creates a dedicated git worktree
2. **Shared worktree base** - All agents in a project share the same worktree directory (configured as `.worktrees/` or `~/.grove/worktrees/`)
3. **Worktrees are named by branch** - Worktree path is `{worktree_base}/{branch-name-with-dashes}`
4. **Symlinks for shared files** - Optional symlinks for `node_modules`, `.env`, etc.

**Current limitation**: Grove does NOT support different worktree locations for different AI agents. The worktree location is a global setting per repository.

## Detailed Findings

### 1. How Grove Manages Worktrees

**Worktree Creation Flow** (`src/git/worktree.rs:18-92`):
```rust
pub fn create(&self, branch: &str) -> Result<String> {
    // 1. Create worktree base directory if needed
    // 2. Convert branch name to worktree name (replace / with -)
    // 3. Create git worktree at {worktree_base}/{worktree_name}
    // 4. Return the worktree path
}
```

Each agent gets:
- A unique git worktree at `{worktree_base}/{branch-name}`
- The branch is created if it doesn't exist
- The agent's tmux session runs in that worktree directory

### 2. Worktree Location Configuration

Grove supports two worktree locations (`src/app/config.rs:191-215`):

| Location | Path | Description |
|----------|------|-------------|
| `project` | `{repo}/.worktrees/` | Worktrees stored alongside the repo |
| `home` | `~/.grove/worktrees/{repo-hash}/` | Worktrees stored in home directory |

**Configuration** (`~/.grove/config.toml`):
```toml
[global]
worktree_location = "project"  # or "home"
```

**Via UI**: Press `Shift+S` → Navigate to "Worktree Location"

### 3. Worktree Structure

When you create an agent named "feature-branch", Grove creates:

```
# If worktree_location = "project"
/path/to/repo/
├── .worktrees/
│   └── feature-branch/    # The worktree
│       ├── .git           # Worktree marker (not a full repo)
│       └── [project files]
│
# If worktree_location = "home"
~/.grove/worktrees/
└── abc123def456/          # Hashed repo identifier
    └── feature-branch/
        ├── .git
        └── [project files]
```

### 4. Per-Agent Worktree

Each agent in Grove has its own worktree:

| Agent | Branch | Worktree Path |
|-------|--------|---------------|
| Agent 1 | feature/auth | `{base}/feature-auth` |
| Agent 2 | bugfix/login | `{base}/bugfix-login` |
| Agent 3 | main | N/A (uses main repo) |

### 5. Symlinks for Shared Files

Grove can create symlinks to share certain files across worktrees (`src/git/worktree.rs:136-204`):

**Configuration** (`.grove/project.toml`):
```toml
[dev_server]
worktree_symlinks = ["node_modules", ".env", ".venv"]
```

This is useful for:
- Large dependencies (node_modules, .venv)
- Environment files (.env)
- Any git-ignored files you want to share

### 6. How Agents Use Worktrees

When Grove creates an agent:
1. Creates worktree: `git worktree add {path} {branch}`
2. Creates symlinks if configured
3. Starts tmux session in worktree directory
4. Launches AI CLI in that tmux session
5. The AI agent works in the isolated worktree

The AI CLI (Claude Code, OpenCode, etc.) automatically detects the worktree as its working directory.

## Configuration Reference

### Global Config (`~/.grove/config.toml`)

```toml
[global]
worktree_location = "project"  # Options: project, home
ai_agent = "claude-code"      # Which AI CLI to use
```

### Per-Project Config (`.grove/project.toml`)

```toml
[git]
branch_prefix = "feature/"
main_branch = "main"

[dev_server]
worktree_symlinks = ["node_modules", ".env", ".venv"]
command = "npm run dev"
port = 3000
```

### Worktree Base Path Resolution

From `src/app/config.rs:1122-1133`:

```rust
pub fn worktree_base_path(&self, repo_path: &str) -> PathBuf {
    match self.global.worktree_location {
        WorktreeLocation::Project => {
            PathBuf::from(repo_path).join(".worktrees")
        }
        WorktreeLocation::Home => {
            let repo_hash = Self::repo_hash(repo_path);
            Self::config_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("worktrees")
                .join(repo_hash)
        }
    }
}
```

## Code References

- `src/git/worktree.rs` - Worktree creation, deletion, symlinks (205 lines)
- `src/app/config.rs:191-215` - WorktreeLocation enum
- `src/app/config.rs:1122-1133` - worktree_base_path()
- `src/agent/model.rs:117-118` - Agent.worktree_path field
- `src/main.rs:319-328` - Worktree creation in agent creation flow

## Architecture Insights

### Design Decision: Global Worktree Location

Grove uses a global worktree location because:
1. **Simplicity** - Single configuration for the entire project
2. **Organization** - All agent worktrees in one place
3. **Cleanup** - Easy to find and remove all worktrees

### Limitation: No Per-Agent Worktree Location

Currently, you cannot:
- Have Agent A use `project` location while Agent B uses `home`
- Specify a custom worktree path for a specific agent
- Have different worktree bases for different AI CLIs

This would require significant changes to the architecture.

### Why Worktrees?

From `src/git/worktree.rs`:
- **Isolation**: Each agent works on its own branch
- **No pollution**: Main repo stays clean
- **Parallel work**: Multiple agents can work simultaneously
- **Easy cleanup**: Remove worktree when agent is done

## Historical Context

The worktree system was designed from the beginning of Grove to provide isolated development environments for each AI agent. This follows the git worktree best practices for parallel development.

## Related Research

- `thoughts/research/2026-03-01_kilo-ai-integration.md` - Kilo AI integration (uses same worktree system)
- `thoughts/research/2026-03-01_codex-configuration.md` - Codex configuration

## Open Questions

1. **Per-agent worktree location**: Is there a use case for different worktree locations per agent?
2. **Custom worktree paths**: Would supporting custom worktree paths be useful?
3. **Worktree migration**: Should there be a way to migrate worktrees between locations?

## Follow-up Research

- Investigate if per-agent worktree location would be valuable
- Look into worktree cleanup policies
- Consider worktree migration tools
