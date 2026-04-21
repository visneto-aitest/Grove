# Git Provider Integration Capability

**Capability Category**: External Integration - Version Control

**Source of Truth**: Reverse-engineered from source code analysis

---

## Supported Providers

### 1. GitHub Integration

#### Core Capabilities
- Retrieve PR status for a specific branch
- Retrieve PRs for multiple branches
- Test connectivity

#### Requirements (SHALL/MUST)
- **MUST** return PullRequestStatus (Open, Draft, Merged, Closed, None) based on PR state.
- **MUST** fetch PRs from GitHub API endpoint.
- **MUST** map GitHub check runs to PipelineStatus.
- **MUST** provide OptionalGitHubClient for graceful "not configured" handling.

#### GIVEN-WHEN-THEN Scenarios

**Scenario A: Branch with open PR and passing checks**
- GIVEN a branch with an open PR that has passing checks
- WHEN `get_pr_for_branch("branchA")` is called
- THEN returns `PullRequestStatus::Open { number, url, pipeline: PipelineStatus }`

**Scenario B: Merged PR**
- GIVEN a branch with a merged PR
- WHEN `get_pr_for_branch("branchB")` is called
- THEN returns `PullRequestStatus::Merged { number }`

**Scenario C: Branch with no PRs**
- GIVEN a branch with no PRs
- WHEN `get_pr_for_branch("branchZ")` is called
- THEN returns `PullRequestStatus::None`

#### Code References
- Client: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/git_providers/github/client.rs`
- Types: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/git_providers/github/types.rs`

---

### 2. GitLab Integration

#### Core Capabilities
- Fetch MR (Merge Request) status for a branch
- Get MRs for multiple branches
- Test connectivity

#### Requirements (SHALL/MUST)
- **MUST** return MergeRequestStatus variants (Open, Merged, Conflicts, Approved, NeedsRebase, None).
- **MUST** perform two-step fetch: list MRs, then fetch detail for pipeline state.
- **MUST** support OptionalGitLabClient wrapper.

#### GIVEN-WHEN-THEN Scenarios

**Scenario D: Branch with open MR**
- GIVEN a branch with an open MR
- WHEN `get_mr_for_branch("branchA")` is called
- THEN returns `MergeRequestStatus::Open { iid, url, pipeline }`

**Scenario E: MR with conflicts**
- GIVEN an MR has conflicts
- WHEN `get_mr_for_branch("branchB")` is called
- THEN returns `MergeRequestStatus::Conflicts { iid, url, pipeline }`

**Scenario F: No MR found**
- GIVEN no MR found for branch
- WHEN `get_mr_for_branch("branchX")` is called
- THEN returns `MergeRequestStatus::None`

#### Code References
- Client: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/git_providers/gitlab/client.rs`
- Types: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/git_providers/gitlab/types.rs`

---

### 3. Codeberg Integration

#### Core Capabilities
- Fetch PR status for a branch
- Support Forgejo Actions CI
- Support Woodpecker CI
- Test connectivity

#### Requirements (SHALL/MUST)
- **MUST** return PullRequestStatus with CI pipeline.
- **MUST** support ForgejoActions and Woodpecker CI providers.
- **MUST** provide OptionalCodebergClient wrapper.

#### GIVEN-WHEN-THEN Scenarios

**Scenario G: Open PR with Forgejo CI**
- GIVEN a branch with an open PR on Codeberg with Forgejo CI
- WHEN `get_pr_for_branch("branchA")` is called
- THEN returns `PullRequestStatus::Open { number, url, pipeline }` with pipeline from Forgejo

**Scenario H: Draft PR with Woodpecker CI**
- GIVEN a PR is a draft with Woodpecker CI
- WHEN `get_pr_for_branch("branchDraft")` is called
- THEN returns `PullRequestStatus::Draft { number, url, pipeline }`

#### Code References
- Client: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/git_providers/codeberg/client.rs`
- Types: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/git_providers/codeberg/types.rs`
- Woodpecker: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/git_providers/codeberg/woodpecker.rs`
- Forgejo: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/git_providers/codeberg/forgejo_actions.rs`

---

## Common HTTP Client

### Requirements (SHALL/MUST)
- **MUST** create a Forge HTTP client with appropriate authentication (Bearer, PrivateToken, Token).
- **MUST** provide `test_forge_connection` to validate connectivity.
- **MUST** provide `forge_get` and `forge_get_with_query` utility functions.
- **MUST** provide `check_forge_response` to fail-fast on non-2xx responses.
- **MUST** provide `OptionalForgeClient` wrapper for graceful "not configured" handling.

#### Code References
- Helpers: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/core/git_providers/helpers.rs`

---

## CI Status Integration

### PipelineStatus Enum
- **MUST** define variants: None, Running, Pending, Success, Failed, Canceled, Skipped, Manual
- **MUST** provide mapping from GitLab status (`from_gitlab_status`)
- **MUST** provide mapping from Woodpecker status (`from_woodpecker_status`)
- **MUST** provide mapping from Forgejo status (`from_forgejo_status`)

#### Code References
- Types: `/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/ci/types.rs`

---

## Environment Variables

| Provider | Token Variable |
|----------|--------------|
| GitLab | `GITLAB_TOKEN` |
| GitHub | `GITHUB_TOKEN` |
| Codeberg | `CODEBERG_TOKEN` |
| Woodpecker | `WOODPECKER_TOKEN` |