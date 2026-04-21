Feature: Render Agent List

  When agent list panel renders. Displays list of agents with columns: Name, Branch, Status, Output preview. Selected agent highlighted. Shows empty state 'No agents' when list empty.

  Scenario: Render Agent List - Happy Path
    Given the preconditions are met
    When the user performs the action
    Then the expected outcome should occur

  # TODO: Add more scenarios based on requirements
