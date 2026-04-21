# Pi-Coding Agent Integration Capability

**Capability Category**: External Integration - AI Coding Agent

**Source of Truth**: 
- Requirement doc: `docs/pi_agent_requirement.md`
- Implementation: `src/pi/*.rs`

---

## 1. Overview

### Project Type
Feature Extension / Plugin Integration for Grove TUI

### Core Functionality
Integrate the pi-coding agent (https://github.com/badlogic/pi-mono/) into Grove to enable AI-powered coding assistance while leveraging Grove's existing agent management, git worktree isolation, and TUI capabilities.

### Target Users
- Developers who use Grove for multi-agent management
- Users who want ai-coding assistance via pi agent within Grove's TUI
- Teams using pi for coding with need for Grove's worktree isolation

---

## 2. Architecture

### Module Structure (Planned vs Implemented)

```
src/pi/
├── mod.rs              # ✅ IMPLEMENTED - PiAgent, PiSessionManager
├── conversion.rs       # ✅ IMPLEMENTED - action_to_rpc, rpc_to_action
├── types.rs          # ✅ IMPLEMENTED - PiConfig, PiMessage, JsonRpcError
├── rpc_bridge.rs     # ❌ NOT IMPLEMENTED - Process spawn, stdio
├── session.rs        # ❌ NOT IMPLEMENTED - Session lifecycle
└── tool_registry.rs  # ❌ NOT IMPLEMENTED - Tool registration
```

### System Context

```
Grove TUI Application
    ├── Grove Core (Rust)
    │   ├── Action Handler
    │   ├── Agent Manager
    │   ├── Git Integration
    │   └── UI Renderer
    └── Pi Integration Layer (Rust)
        ├── RpcBridge (NOT IMPLEMENTED)
        ├── ActionConverter (IMPLEMENTED)
        ├── ToolMapper (IMPLEMENTED)
        └── SessionManager (IMPLEMENTED)

pi-coding-agent (External CLI)
    └── ↔ JSON-RPC stdio communication
```

### Sequence Diagram: Tool Execution Flow

```
Developer → Grove Core: Action::CreateAgent { name, branch, enable_pi: true }
Grove Core → PiBridge: spawn_pi_session(agent_id)
PiBridge → pi-agent: Spawn pi process --rpc --session <id>

Developer → Grove Core: Action::SendToPi { agent_id, message }
Grove Core → PiBridge: send_message(agent_id, message)
PiBridge → pi-agent: {"jsonrpc":"2.0","method":"execute","params":{"prompt":"..."}}

pi-agent → LLM Provider: API call with prompt + tools
LLM Provider → pi-agent: Response with tool_call

pi-agent → PiBridge: {"jsonrpc":"2.0","method":"tool","params":{"name":"git","args":["status"]}}
PiBridge → Grove: Action::RefreshSelected
Grove → PiBridge: tool_result: "On branch main..."

PiBridge → pi-agent: {"jsonrpc":"2.0","id":1,"result":{"output":"On branch main..."}}
pi-agent → LLM Provider: Continue with result

pi-agent → PiBridge: final_output
PiBridge → Grove: Action::UpdateAgentOutput { output }
Grove → Developer: Display in output panel
```

---

## 3. Functional Requirements

### FR1: Pi Process Management

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR1.1 | Spawn pi process | MUST | ❌ NOT IMPLEMENTED | `pi --rpc --session <id>` |
| FR1.2 | Process lifecycle | MUST | ❌ NOT IMPLEMENTED | Handle start, running, termination |
| FR1.3 | Process health check | SHOULD | ❌ NOT IMPLEMENTED | Monitor and restart if crashed |
| FR1.4 | Environment setup | MUST | ❌ NOT IMPLEMENTED | Pass env vars (API keys, session path) |

### FR2: RPC Communication

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR2.1 | JSON-RPC serialization | MUST | ✅ IMPLEMENTED | `src/pi/types.rs` PiMessage |
| FR2.2 | Stdout parsing | MUST | ❌ NOT IMPLEMENTED | Parse from pi stdout |
| FR2.3 | Stdin sending | MUST | ❌ NOT IMPLEMENTED | Send via pi stdin |
| FR2.4 | Stream handling | MUST | ❌ NOT IMPLEMENTED | Handle SSE |
| FR2.5 | Error handling | MUST | ✅ PARTIAL | Error types defined |

### FR3: Action Conversion

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR3.1 | Action → RPC | MUST | ✅ IMPLEMENTED | `action_to_rpc()` conversion.rs |
| FR3.2 | RPC → Action | MUST | ✅ IMPLEMENTED | `rpc_to_action()` conversion.rs |
| FR3.3 | Bidirectional sync | MUST | ✅ IMPLEMENTED | PiSessionManager::forward_to_pi() |

### FR4: Tool Mapping

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR4.1 | git status → RefreshSelected | MUST | ✅ IMPLEMENTED | tool_to_action() line 164 |
| FR4.2 | git diff → ToggleDiffView | MUST | ✅ IMPLEMENTED | tool_to_action() line 165 |
| FR4.3 | editor → OpenInEditor | MUST | ✅ IMPLEMENTED | tool_to_action() line 172-178 |
| FR4.4 | terminal → AttachToDevServer | MUST | ✅ IMPLEMENTED | tool_to_action() line 186-191 |
| FR4.5 | file_ops → CopyWorktreePath | MUST | ✅ IMPLEMENTED | tool_to_action() line 179-184 |
| FR4.6 | Custom tool registry | SHOULD | ❌ NOT IMPLEMENTED | Allow custom tools |

### FR5: Session Management

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR5.1 | Session creation | MUST | ✅ PARTIAL | PiSessionManager::add_agent() |
| FR5.2 | Session persistence | MUST | ❌ NOT IMPLEMENTED | JSONL session files |
| FR5.3 | Session restoration | SHOULD | ❌ NOT IMPLEMENTED | Restore on Grove restart |
| FR5.4 | Session cleanup | MUST | ✅ PARTIAL | Agent deletion cleanup |

### FR6: Output Handling

| ID | Requirement | Priority | Status | Implementation |
|----|-------------|----------|--------|--------------|
| FR6.1 | Stream to UI | MUST | ✅ IMPLEMENTED | Output channel → output panel |
| FR6.2 | Progress display | SHOULD | ❌ NOT IMPLEMENTED | Show tool execution progress |
| FR6.3 | Error display | MUST | ✅ IMPLEMENTED | Display tool errors in UI |

---

## 4. Implementation Analysis

### 4.1 RPC Message Types (Implemented)

#### Requirements (SHALL/MUST)
- **MUST** define RpcMessage enum with variants for all operations
- **MUST** serialize/deserialize as JSON-RPC 2.0

#### Code References
- `/Volumes/.../Grove/src/pi/mod.rs` lines 18-40

#### GIVEN-WHEN-THEN Scenarios

**Scenario A: CreateAgent RPC**
- GIVEN a request to create agent with name "test", branch "feature/test"
- WHEN RpcMessage::CreateAgent is serialized
- THEN JSON contains `"type":"CreateAgent", "name":"test", "branch":"feature/test"`

**Scenario B: ExecuteTool RPC**
- GIVEN a tool execution request for "editor" with args ["uuid"]
- WHEN RpcMessage::ExecuteTool is received
- THEN converts to Action::OpenInEditor { id } via tool_to_action()

---

### 4.2 PiAgent Wrapper (Implemented)

#### Requirements (SHALL/MUST)
- **MUST** wrap Agent with RPC channels
- **MUST** process incoming RPC messages
- **MUST** convert to Grove actions
- **MUST** create snapshots on request

#### Code References
- `/Volumes/.../Grove/src/pi/mod.rs` lines 74-211

---

### 4.3 PiSessionManager (Implemented)

#### Requirements (SHALL/MUST)
- **MUST** manage multiple PiAgent instances
- **MUST** add agents and assign IDs
- **MUST** process all RPC messages
- **MUST** forward Grove actions to pi

#### Code References
- `/Volumes/.../Grove/src/pi/mod.rs` lines 213-337

---

### 4.4 Action Conversion (Implemented)

#### Code References
- `/Volumes/.../Grove/src/pi/conversion.rs` lines 1-135

#### Conversion Matrix

| Grove Action | RPC Message | Implementation |
|-------------|------------|--------------|
| CreateAgent { name, branch } | RpcMessage::CreateAgent | ✅ |
| AttachToAgent { id } | RpcMessage::AttachToAgent | ✅ |
| DetachFromAgent | RpcMessage::Detach | ✅ |
| UpdateAgentStatus { id, status } | RpcMessage::StatusUpdate | ✅ |
| UpdateAgentOutput { id, output } | RpcMessage::Output | ✅ |
| FetchRemote { id } | RpcMessage::GitOperation::FetchRemote | ✅ |
| MergeMain { id } | RpcMessage::GitOperation::MergeMain | ✅ |
| PushBranch { id } | RpcMessage::GitOperation::PushBranch | ✅ |
| OpenInEditor { id } | RpcMessage::ExecuteTool { tool: "editor" } | ✅ |
| CopyWorktreePath { id } | RpcMessage::ExecuteTool { tool: "file_ops" } | ✅ |
| AttachToDevServer { agent_id } | RpcMessage::ExecuteTool { tool: "terminal" } | ✅ |

---

### 4.5 PiMessage Types (Implemented)

#### Code References
- `/Volumes/.../Grove/src/pi/types.rs` lines 1-124

#### Unit Tests (Already Implemented)
```rust
#[test]
fn test_serialize_execute_request() {
    let msg = PiMessage::new_execute("test prompt");
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"method\":\"execute\""));
}

#[test]
fn test_deserialize_tool_response() {
    let json = r#"{"jsonrpc":"2.0","id":1,"result":{"output":"ok"}}"#;
    let msg: PiMessage = serde_json::from_str(json).unwrap();
    assert!(msg.result.is_some());
}
```

---

## 5. Implementation Phases (From Requirement Doc)

### Phase 1: Core Infrastructure

| Task ID | Task | Dependencies | Type | Status |
|--------|------|-------------|------|--------|
| T1.1 | Fix pi/conversion.rs imports | None | Bug fix | ✅ DONE |
| T1.2 | Create src/pi/types.rs | None | New file | ✅ DONE |
| T1.3 | Create src/pi/rpc_bridge.rs | T1.2 | New file | ❌ NOT IMPLEMENTED |
| T1.4 | Create src/pi/session.rs | T1.3 | New file | ❌ NOT IMPLEMENTED |
| T1.5 | Create src/pi/tool_registry.rs | T1.2 | New file | ❌ NOT IMPLEMENTED |

### Phase 2: Integration

| Task ID | Task | Dependencies | Type | Status |
|--------|------|-------------|------|--------|
| T2.1 | Add pi config to AppConfig | None | Extension | ❌ NOT IMPLEMENTED |
| T2.2 | Add PiStartSession to Action | None | Extension | ❌ NOT IMPLEMENTED |
| T2.3 | Integrate PiSessionManager in main.rs | T1.4, T2.1 | Integration | ✅ PARTIAL |
| T2.4 | Add Action → PiRPC forward | T1.3, T2.2 | Extension | ✅ DONE |

### Phase 3: Testing

| Task ID | Task | Dependencies | Type | Status |
|--------|------|-------------|------|--------|
| T3.1 | Add RPC serialization test | T1.2 | Test | ✅ DONE (in types.rs) |
| T3.2 | Add action conversion test | T1.3 | Test | ❌ NOT IMPLEMENTED |
| T3.3 | Add tool mapping test | T1.5 | Test | ❌ NOT IMPLEMENTED |
| T3.4 | Add session integration test | T1.4 | Test | ❌ NOT IMPLEMENTED |

### Phase 4: UI Polish

| Task ID | Task | Dependencies | Type | Status |
|--------|------|-------------|------|--------|
| T4.1 | Add pi indicator to status bar | T2.3 | UI | ❌ NOT IMPLEMENTED |
| T4.2 | Add pi settings to setup modal | T2.1 | UI | ❌ NOT IMPLEMENTED |
| T4.3 | Update help overlay | T4.1 | UI | ❌ NOT IMPLEMENTED |

---

## 6. Non-Functional Requirements

### NFR1: Performance

| ID | Requirement | Target | Status |
|----|-------------|--------|--------|
| NFR1.1 | Startup time | < 500ms for pi init | ❌ NOT TESTED |
| NFR1.2 | Message latency | < 100ms RPC round-trip | ❌ NOT TESTED |
| NFR1.3 | Memory usage | < 100MB per pi session | ❌ NOT TESTED |

### NFR2: Reliability

| ID | Requirement | Description | Status |
|----|-------------|-------------|--------|
| NFR2.1 | Graceful degradation | ✅ IMPLEMENTED |
| NFR2.2 | Process isolation | ❌ NOT IMPLEMENTED |
| NFR2.3 | Recovery | ❌ NOT IMPLEMENTED |

### NFR3: Security

| ID | Requirement | Description | Status |
|----|-------------|-------------|--------|
| NFR3.1 | API key handling | ❌ NOT IMPLEMENTED |
| NFR3.2 | Process sandboxing | ❌ NOT IMPLEMENTED |
| NFR3.3 | Input validation | ❌ NOT IMPLEMENTED |

---

## 7. Configuration

### Environment Variables
- `ANTHROPIC_API_KEY` - Anthropic API key
- `OPENAI_API_KEY` - OpenAI API key
- `GOOGLE_API_KEY` - Google API key
- `PI_SESSION_DIR` - Session directory (default: ~/.pi/agent/sessions/)

### Config Structure
```toml
[pi]
enabled = true
provider = "anthropic"
model = "claude-3-5-sonnet-20241022"
```

---

## 8. RPC Protocol Specification

### JSON-RPC 2.0 Message Format

```yaml
jsonrpc: "2.0"

methods:
  execute:
    description: Execute pi agent with prompt
    params:
      type: object
      properties:
        prompt:
          type: string
        session_file:
          type: string

  tool:
    description: Request tool execution
    params:
      type: object
      properties:
        name:
          type: string
        args:
          type: array
          items:
            type: string
    result:
      output: string

  snapshot:
    description: Request agent state snapshot
    result:
      type: object
      properties:
        name: string
        branch: string
        status: string
        output: array

  exit:
    description: Gracefully close session
```

---

## 9. Acceptance Criteria

### AC1: Pi Process Management
- [ ] Grove can spawn pi process with `--rpc` flag
- [ ] Process is cleaned up when agent is deleted
- [ ] Process errors are logged gracefully

### AC2: RPC Communication
- [ ] JSON-RPC messages correctly serialized/deserialized ✅
- [ ] Stdout parsed without blocking
- [ ] Stdin sends without blocking

### AC3: Action Conversion
- [ ] All actions convert to RPC correctly ✅
- [ ] All RPC tool calls convert to Grove actions ✅
- [ ] Bidirectional sync works ✅

### AC4: Tool Mapping
- [ ] git status → RefreshSelected ✅
- [ ] git diff → ToggleDiffView ✅
- [ ] editor <uuid> → OpenInEditor ✅
- [ ] terminal <uuid> → AttachToDevServer ✅
- [ ] file_ops <uuid> → CopyWorktreePath ✅

### AC5: Session Management
- [ ] Each Grove agent has pi session (partial) ✅
- [ ] Session persists to JSONL
- [ ] Session restores on restart

### AC6: Output Handling
- [ ] Pi output in Grove output panel ✅
- [ ] Tool execution shows progress
- [ ] Errors displayed in UI ✅

---

## 10. Test Coverage Goals

| Module | Target Coverage | Current |
|--------|--------------|---------|
| types.rs | 100% | ✅ 100% |
| rpc_bridge.rs | 80%+ | ❌ 0% |
| session.rs | 80%+ | ❌ 0% |
| conversion.rs | 90%+ | ✅ PARTIAL |
| tool_registry.rs | 90%+ | ❌ 0% |

---

## 11. Error Codes

| Code | Description | Handling |
|------|-------------|----------|
| E001 | Pi process not found | Show error, suggest npm install |
| E002 | Process spawn failed | Log, show error, retry |
| E003 | Invalid RPC response | Log, discard, continue |
| E004 | Tool execution failed | Return error to pi |
| E005 | Session restore failed | Create new session |

---

## 12. Out of Scope

- MCP server integration (future work)
- Custom pi extensions (future work)
- Multi-provider fallback (future work)
- WebUI integration (not applicable)