Feature: Create Agent
  As a developer
  I want to create a new AI coding agent
  So that I can work on a specific task in an isolated environment

  # ==========================================
  # API SCENARIOS (API Request & Response)
  # ==========================================

  Scenario: [API] Create Agent Success
    Given a git repository is initialized
    When the client sends POST request to "/api/agents" with:
      """
      {
        "branchName": "feature/new-agent",
        "worktreePath": "worktrees/new-agent"
      }
      """
    Then the response status should be 201
    And the response body should contain "id"
    And the response body should contain "branchName"
    And the response body should contain "worktreePath"
    And the response body should contain "tmuxSession"
    And the response body should contain "status"

  Scenario: [API] Create Agent Failure (Branch already exists)
    Given a git repository is initialized
    And the branch "feature/existing" already exists
    When the client sends POST request to "/api/agents" with:
      """
      {
        "branchName": "feature/existing",
        "worktreePath": "worktrees/existing"
      }
      """
    Then the response status should be 400
    And the response body should contain "error"
    And the response error should be "Branch already exists"
