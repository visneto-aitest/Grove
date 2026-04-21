# Grove Development Roadmap

This document outlines the strategic direction and planned features for Grove, a terminal UI for managing isolated AI coding agents.

## Current Status: v0.2.0 (Stable Core)
- ✅ Multi-agent management with isolated git worktrees
- ✅ Support for Claude Code, Opencode, Codex, and Gemini CLI
- ✅ Real-time monitoring and status detection
- ✅ Basic Project Management integrations (Asana, Notion, ClickUp, Airtable, Linear)
- ✅ Git Provider integrations (GitHub, GitLab, Codeberg)
- ✅ Dev Server management per agent
- ✅ Session persistence and basic settings UI

---

## Phase 1: UX Refinement & Reliability (Short-term)
*Goal: Polish the existing experience and ensure rock-solid stability.*

- [ ] **Complete Settings System**: Fully implement the planned settings architecture from `docs/settings-system-plan.md`.
  - [ ] Migration of remaining hardcoded values to TOML configuration.
  - [ ] Enhanced validation for configuration changes.
- [ ] **Keybind Cleanup**: Execute the `docs/keybind-cleanup-plan.md` to resolve conflicts and improve discoverability.
  - [ ] Interactive keybind help overlay.
  - [ ] Support for multi-key sequences.
- [ ] **Improved Status Detection**: Refine regex patterns and process monitoring for more accurate "Awaiting Input" and "Error" states.
  - [ ] Per-agent-type diagnostic logs in the UI.
- [ ] **Tutorial Enhancement**: Expand the interactive tutorial to cover advanced features like PM task linking and dev server management.
- [ ] **Robust Error Handling**: Replace broad `anyhow` usage with domain-specific error types for better recovery and user feedback.

## Phase 2: Customization & Intelligence (Medium-term)
*Goal: Give users more control over how agents behave and interact.*

- [ ] **Customizable Prompts**: Implement the `docs/customizable-prompts-plan.md`.
  - [ ] Project-level prompt templates for agent creation.
  - [ ] Support for context injection (e.g., current branch, linked task description).
- [ ] **Advanced File Browser**: Enhance the built-in file browser with basic editing capabilities and search.
- [ ] **Smart Resource Management**: Automatically pause/suspend agents when system resources are low or after periods of inactivity.
- [ ] **Integration Setup Wizards**: Polish the setup experience for all PM and Git providers with guided OAuth/token flows.
- [ ] **Global Status Dashboard**: A "birds-eye view" of all active projects and agent health across multiple repositories.

## Phase 3: Advanced Integrations & Bi-directional Sync
*Goal: Deepen the connection between coding activity and project management.*

- [ ] **Bi-directional PM Sync**: Automatically update task status in Asana/Linear when an agent completes a PR or fails a test.
- [ ] **Enhanced Git Provider Support**:
  - [ ] Live CI/CD pipeline monitoring within the Grove UI.
  - [ ] Direct PR/MR comment viewing and replying.
- [ ] **Pi-Agent Evolution**: Fully stabilize the RPC bridge for advanced agent-to-Grove communication.
- [ ] **Dev Server Port Forwarding**: Integrated support for exposing agent dev servers via tunnels (e.g., ngrok, cloudflare).
- [ ] **Timeline View**: A historical view of agent activity, commits, and status changes per task.

## Phase 4: Collaboration & Scale (Long-term)
*Goal: Support teams and large-scale deployments.*

- [ ] **Team Workspaces**: Shared configuration and agent templates for engineering teams.
- [ ] **Remote Agent Support**: Ability to manage agents running on remote servers or in containers.
- [ ] **Usage Analytics**: Track token usage, time-to-completion, and success rates across different AI providers.
- [ ] **Plugin System**: Allow users to create custom agent detectors, UI widgets, and PM integrations.
- [ ] **Mobile Companion**: A read-only mobile app (or web dashboard) for monitoring agent progress on the go.

---

## Technical Debt & Infrastructure
- [ ] **Test Coverage**: Increase unit and integration testing for core `AgentManager` and `AppState` logic.
- [ ] **Performance Benchmarking**: Optimize UI rendering for sessions with 50+ agents.
- [ ] **Documentation**: Complete the `openspec` documentation and create a contributor guide.
- [ ] **CI/CD Improvements**: Streamline the release process for multiple architectures.

---

*Note: This roadmap is a living document and will be updated as project priorities evolve.*
