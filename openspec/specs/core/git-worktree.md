# Git Worktree Management Capability

**Capability Category**: Infrastructure - Git Operations

**Source of Truth**: Reverse-engineered from source code analysis

---

## Core Capabilities

### 1. Worktree Creation

#### Requirements (SHALL/MUST)
- **MUST** open the repository before operations.
- **MUST** ensure the base worktrees directory exists (create_dir_all if not present).
- **MUST** derive the worktree name from the branch by replacing '/' with '-'.
- **MUST** return existing path if worktree already exists.
- **MUST** resolve or create branch reference refs/heads/{branch}.
- **MUST** create the actual worktree via git2::Repository::worktree().
- **MUST** map git2 errors into anyhow context.

#### GIVEN-WHEN-THEN Scenarios

**Scenario A: Create a new worktree for a branch**
- GIVEN a repo at path `/abs/repo` and a base worktree dir `/abs/worktrees`
- WHEN calling `worktree.create("feature/awesome")`
- THEN a new worktree is created under `/abs/worktrees/feature-awesome` and the path string is returned.

**Scenario B: Re-create or reuse existing worktree**
- GIVEN an existing worktree at `/abs/worktrees/feature-awesome`
- WHEN calling `worktree.create("feature/awesome")`
- THEN the existing path is returned without duplicating worktree.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/git/worktree.rs` lines 18-92

---

### 2. Worktree Removal

#### Requirements (SHALL/MUST)
- **MUST** prune the worktree in Git.
- **MUST** remove the worktree directory if present.

#### GIVEN-WHEN-THEN Scenarios

**Scenario C: Remove a worktree**
- GIVEN a worktree path `/abs/worktrees/feature-awesome`
- WHEN calling `remove("/abs/worktrees/feature-awesome")`
- THEN the worktree is pruned and the directory is removed if present.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/git/worktree.rs` lines 94-118

---

### 3. Symlink Creation

#### Requirements (SHALL/MUST)
- **MUST** check if target is a broken symlink.
- **MUST** remove broken symlink if present.
- **MUST** create missing parent directories.
- **MUST** create OS-specific symlinks (UNIX vs Windows).

#### GIVEN-WHEN-THEN Scenarios

**Scenario D: Symlink files into a worktree**
- GIVEN a worktree path and a list of files relative to repo
- WHEN calling `create_symlinks(worktree_path, &files)`
- THEN symlinks are created in the worktree pointing to the repo files.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/git/worktree.rs` lines 136-204

---

### 4. Worktree Listing

#### Requirements (SHALL/MUST)
- **MUST** list all worktrees in the repository.
- **MUST** return worktree names as Vec<String>.

#### GIVEN-WHEN-THEN Scenarios

**Scenario E: List worktrees**
- GIVEN a repository with several worktrees
- WHEN calling `list()`
- THEN a Vec<String> of worktree names is returned.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/git/worktree.rs` lines 119-126

---

### 5. Session Storage

#### Requirements (SHALL/MUST)
- **MUST** provide SessionData structure containing repo_path, agents, selected_index.
- **MUST** compute deterministic session filename using hash of repo path.
- **MUST** store sessions in ~/.grove/sessions/.
- **MUST** handle load/save/delete operations.

#### GIVEN-WHEN-THEN Scenarios

**Scenario F: Save a session**
- GIVEN a SessionStorage bound to a repo path and a SessionData instance
- WHEN `save_session(storage, repo_path, agents, selected_index)` is called
- THEN the session.json file with a prettified JSON payload is written.

**Scenario G: Load a session (existing)**
- GIVEN a session file exists at the computed path
- WHEN `load_session(storage)` is invoked
- THEN a SessionData instance is returned wrapped in Ok(Some(...)).

**Scenario H: Load a session (non-existing)**
- GIVEN no session file exists
- WHEN `load_session(storage)` is invoked
- THEN Ok(None) is returned.

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/storage/session.rs` lines 1-109

---

## Git Status Model

### Requirements (SHALL/MUST)
- **MUST** define GitSyncStatus with fields: ahead, behind, divergence_from_main, is_clean, is_synced.
- **MUST** provide format_short helper returning compact string.

#### GIVEN-WHEN-THEN Scenarios

**Scenario I: Display short status**
- GIVEN a GitSyncStatus with ahead=2, behind=1, divergence_from_main=0
- WHEN format_short() is called
- THEN it returns "↑2 ↓1 main+0".

#### Code References
- `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/git/status.rs` lines 3-24