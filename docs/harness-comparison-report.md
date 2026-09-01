# Báo Cáo So Sánh Core Agent Processing: xVora vs Pi

**Ngày:** 2026-09-01  
**Trọng tâm:** Agent loop, tool execution, reasoning/thinking, compaction — "bộ não" xử lý coding

---

## 1. Agent Turn Loop — Cách Xử Lý Một Turn Coding

### xVora Turn Flow

```
User Prompt
    │
    ▼
┌─────────────────────────────────────────────┐
│ 1. ChatStateActor::build_conversation_request │
│    - Strip dangling tool results             │
│    - Estimate tokens (estimate_messages)     │
│    - Check auto-compact threshold            │
│    - Inject memory reminder (if enabled)     │
│    - Handle image budget eviction            │
└─────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────┐
│ 2. AgenticSampler::sample()                 │
│    - Send to xAI Responses API / Chat Comps  │
│    - Stream response với thinking blocks     │
│    - Track per-model usage                   │
│    - reasoning_effort from SamplingConfig    │
└─────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────┐
│ 3. Tool Execution Loop                      │
│    for each tool_call:                      │
│      ├─ before_tool hook                    │
│      ├─ ToolBridge::execute(tool_call)      │
│      │   ├─ BashTool → spawn process        │
│      │   ├─ ReadFileTool → read file        │
│      │   ├─ SearchReplaceTool → edit        │
│      │   ├─ TaskTool → spawn subagent       │
│      │   └─ ...50+ tools                    │
│      ├─ after_tool hook                     │
│      ├─ Push ToolResult to conversation     │
│      └─ Check timeout/retry per-tool        │
└─────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────┐
│ 4. Compaction Check                         │
│    - total_tokens > threshold% ?            │
│    - Two-pass: pass1 (speculative) → pass2   │
│    - Repair dangling tool calls             │
└─────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────┐
│ 5. Turn End                                 │
│    - flush harness trace                    │
│    - record token usage                     │
│    - memory flush turn (optional)           │
│    - update notification meta               │
└─────────────────────────────────────────────┘
```

**Đặc điểm nổi bật:**
- **Actor pattern**: `ChatStateActor` chạy tokio task riêng, xử lý tuần tự qua `mpsc`
- **Strict append**: `StrictAppendAck` đảm bảo persistence đồng bộ
- **Image budget**: `image_budget.rs` — eviction khi body quá lớn
- **Two-pass compaction**: speculative summary giảm latency

---

### Pi Turn Flow

```
User Prompt
    │
    ▼
┌─────────────────────────────────────────────┐
│ 1. AgentHarness.prompt() / skill()          │
│    - Validate lane not busy                 │
│    - Create OperationStartedRecord          │
│    - before_run hook                        │
└─────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────┐
│ 2. Agentic Loop (Reducer)                   │ ← reducer.ts
│    state = { configuration, messages, ... } │
│    while not done:                          │
│      ├─ peekAction() → ActionInfo           │
│      ├─ executeAction():                    │
│      │   ├─ stream_assistant               │
│      │   ├─ execute_tool                   │
│      │   ├─ try_finish_run                 │
│      │   └─ hook                           │
│      └─ checkCompaction()                   │
└─────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────┐
│ 3. Tool Execution                           │
│    - Sequential hoặc parallel               │
│    - before_tool / after_tool hooks         │
│    - Result persisted as Entry              │
│    - replay policy: "never" | "safe"        │
└─────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────┐
│ 4. Compaction Check                         │
│    - contextTokens > window - reserve       │
│    - findCutPoint()                         │
│    - generateSummary()                      │
└─────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────┐
│ 5. Run End                                  │
│    - commit_follow_up queue                 │
│    - record usage                           │
│    - after_run_end hook                     │
└─────────────────────────────────────────────┘
```

**Đặc điểm nổi bật:**
- **Reducer state machine**: `ActionInfo` enum điều khiển flow
- **Durable operations**: Record-based recovery
- **Lane navigation**: `navigateTree()`, multi-branch

---

## 2. Thinking / Reasoning Layer — Tư Duy Model

### Pi: ThinkingLevel System

```typescript
// packages/agent/src/types.ts
export type ThinkingLevel = "off" | "minimal" | "low" | "medium" | "high" | "xhigh" | "max";

// packages/agent/src/agent.ts (line 450)
reasoning: this._state.thinkingLevel === "off"
    ? undefined
    : this._state.thinkingLevel,

// packages/agent/src/agent-loop.ts (line 183)
// Thinking level có thể đổi giữa các turns
reasoning: nextTurnSnapshot.thinkingLevel === "off"
    ? undefined
    : nextTurnSnapshot.thinkingLevel,
```

**Đặc điểm:**
- 7 levels: `off`, `minimal`, `low`, `medium`, `high`, `xhigh`, `max`
- Truyền vào API request như `reasoning: { effort: "high" }`
- Có thể thay đổi dynamic giữa các turns
- Stored trong session entry: `ThinkingLevelEntry`

---

### xVora: ReasoningEffort System

```rust
// crates/codegen/xvora-sampling-types/src/types.rs (line 750)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ReasoningEffort {
    None,      // equivalent to "off"
    Minimal,   // equivalent to "minimal"
    Low,       // equivalent to "low"
    #[default]
    Medium,    // equivalent to "medium"
    High,      // equivalent to "high"
    Xhigh,     // equivalent to "xhigh"
    Max,       // equivalent to "max"
}

// Mapping sang xAI Responses API
impl ReasoningEffort {
    pub fn to_responses_api(self) -> crate::rs::ReasoningEffort {
        match self {
            Self::None => crate::rs::ReasoningEffort::None,
            Self::Minimal => crate::rs::ReasoningEffort::Minimal,
            // ...
        }
    }
}
```

**Đặc điểm:**
- 7 levels tương đương Pi
- Mapping sang xAI Responses API format
- `Thinking` blocks được handle trong `conversation/messages.rs`:
  ```rust
  // Auto-pair thinking.type = "adaptive"
  // thinking.display = "summarized" cho 4.7+ models
  ```
- Cache breakpoint跳过 `Thinking` blocks

---

### So Sánh Thinking System

| Aspect | xVora | Pi |
|---|---|---|
| **Type name** | `ReasoningEffort` | `ThinkingLevel` |
| **Values** | `None, Minimal, Low, Medium, High, Xhigh, Max` | `off, minimal, low, medium, high, xhigh, max` |
| **API mapping** | `to_responses_api()` | Direct string passthrough |
| **Dynamic change** | Không rõ (cần check SamplingConfig) | Có, qua `prepareNextTurn()` |
| **Session storage** | `ThinkingLevelEntry` | `thinking_level_change` entry |
| **Compaction aware** | Cần implement | ✓ Có trong `compaction.ts` |

**Khuyến nghị:** xVora nên thêm dynamic thinking level switch giữa turns như Pi.

---

## 3. Prompt Assembly — Cách Dựng System Prompt

### xVora Prompt Layers

```rust
// crates/codegen/xvora-agent/src/prompt/context.rs
pub struct PromptContext {
    audience: PromptAudience,           // Primary vs Subagent
    prompt_mode: PromptMode,            // Extend vs Full
    agents_md_files: Vec<AgentConfigFile>,
    persona_summaries: Vec<String>,
    memory_enabled: bool,
    memory_global_path: Option<String>,
    memory_workspace_path: Option<String>,
    role_instructions: Option<String>,
    persona_instructions: Option<String>,
    os_name: Option<String>,
    shell_path: Option<String>,
    working_directory: Option<String>,
    current_date: Option<String>,
    system_prompt_label: String,        // "Grok" default
}
```

**Rendering flow:**
```
Base Template (prompt.md)
    │
    ├── Layer 1: Tool conventions (rendered from ToolBridge)
    ├── Layer 2: AGENTS.md sections (if agents_md=true)
    ├── Layer 3: Persona blocks (if persona_instructions set)
    ├── Layer 4: Memory section (if memory_enabled)
    ├── Layer 5: User info block (OS, shell, cwd, date)
    └── Layer 6: Prompt body (from agent definition)
```

**Template modes:**
- `Extend`: Body appended to base template
- `Full`: Body replaces entire prompt
- `Codex`: Special template for Codex compatibility

---

### Pi Prompt Layers

```typescript
// packages/agent/src/harness/system-prompt.ts
// packages/coding-agent/src/core/system-prompt.ts

function buildSystemPrompt(options: {
    skills: Skill[],
    tools: HarnessTool[],
    cwd: string,
    // ...
}): string {
    let prompt = DEFAULT_SYSTEM_PROMPT;

    // Skills as XML blocks
    for (const skill of visibleSkills) {
        prompt += formatSkillInvocation(skill);
    }

    // Tool descriptions
    prompt += renderToolDescriptions(options.tools);

    // Context
    prompt += `<cwd>${options.cwd}</cwd>`;

    return prompt;
}
```

---

### So Sánh Prompt System

| Aspect | xVora | Pi |
|---|---|---|
| **Structure** | Multi-layer template | Flat concatenation |
| **AGENTS.md** | ✓ Auto-discover | ✗ Không có |
| **Personas** | ✓ `persona_instructions` | ✗ Không có |
| **Memory injection** | ✓ Auto-add `<memory>` section | ✗ Không có |
| **User info** | ✓ OS, shell, cwd, date | ✓ CWD only |
| **Template modes** | Extend, Full, Codex | Extend, Full |
| **Subagent prompt** | ✓ Compact template | ✗ Same as primary |

**Khuyến nghị:** Pi nên thêm AGENTS.md và persona system từ xVora.

---

## 4. Tool Execution — Cách Chạy Tools

### xVora Tool Architecture

```
crates/codegen/xvora-tools/src/
├── bridge.rs          ← ToolBridge: unified dispatch
├── registry/mod.rs    ← ToolRegistry: lookup, validation
├── implementations/   ← 256+ tool implementations
│   ├── grok_build/    ← Bash, ReadFile, SearchReplace, Grep...
│   ├── codex/         ← Codex-compatible tools
│   ├── opencode/      ← OpenCode-style tools
│   ├── search_tool/   ← agentic search
│   ├── memory/        ← memory_search, memory_get
│   └── computer/      ← computer use tools
├── mcp_elicitation/   ← MCP server auto-discovery
└── notification/      ← Progress notifications
```

**Tool presets:**
```rust
// crates/codegen/xvora-agent/src/config.rs
fn native_toolset_presets() -> Vec<(&'static str, ToolServerConfig)> {
    vec![
        ("grok-build", workspace_grok_build_toolset()),
        ("grok-build-concise", grok_build_concise_toolset()),
        ("grok-build-plan", grok_build_plan_toolset()),
        ("codex", codex_toolset()),
        ("explore", explore_toolset()),
        ("plan", plan_toolset()),
        ("grok-computer", grok_computer_toolset()),
        ("orchestrator", orchestrator_toolset()),
    ]
}
```

**Tool retry config:**
```rust
pub struct ToolRetryConfig {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}
```

---

### Pi Tool Architecture

```
packages/agent/src/harness/tools/
├── index.ts      ← re-export
├── bash.ts       ← BashTool: exec command
├── edit.ts       ← EditTool: search/replace với diff
├── write.ts      ← WriteTool: full file write
├── read.ts       ← ReadTool: read file + image
├── tool-context.ts ← ExecutionEnv context
└── image.ts      ← Image tool
```

**Tool execution signature:**
```typescript
tool.execute(
    toolCallId: string,
    params: Static<TParameters>,
    signal: AbortSignal | undefined,
    onUpdate: AgentToolUpdateCallback<TDetails> | undefined,
    context: TContext,
): Promise<AgentToolResult<TDetails>>;
```

**Replay policy:**
```typescript
type HarnessTool = AgentTool & {
    replay?: "never" | "safe";
};
```

---

### So Sánh Tool System

| Aspect | xVora | Pi |
|---|---|---|
| **Tool count** | 256+ implementations | ~9 core tools |
| **MCP support** | ✓ Auto-elicitation | ✓ Via ExecutionEnv |
| **Retry config** | Per-tool `ToolRetryConfig` | Global `RetryPolicy` |
| **Replay policy** | Không có | `"never" \| "safe"` |
| **Progress** | Notification handle | `onUpdate` callback |
| **Subagent tools** | TaskTool, wait, kill | Không có |
| **Web tools** | WebSearch, WebFetch | Không có |
| **Image tools** | ImageGen, ImageToVideo | Có trong read tool |

---

## 5. Subagent / Task Coordination

### xVora Subagent System

```rust
// crates/codegen/xvora-agent/src/
// Subagent types: GeneralPurpose, Explore, Plan

// Orchestrator mode (agent definition)
const ORCHESTRATOR_PROMPT_BODY: &str = "
## Orchestrator Mode
You are a technical lead orchestrating a team of senior-engineer subagents.
Your job is to think, plan, coordinate, and review.
Their job is to explore, implement, and execute.

### ALWAYS delegate to subagents:
- ALL file modifications
- ALL builds, tests, and verification
- Deep codebase exploration
- Multi-step implementation
";

// Spawning logic (conceptual)
pub async fn handle_subagent_request(&self, request: SubagentRequest) {
    let agent_type = match request.agent_name.as_str() {
        "general-purpose" => BuiltinAgentName::GeneralPurpose,
        "explore" => BuiltinAgentName::Explore,
        "plan" => BuiltinAgentName::Plan,
        _ => return Err(...),
    };

    let subagent_def = self.build_subagent_definition(agent_type, request);
    let session = SessionActor::spawn(subagent_def).await;
    TaskOutputTool::register(session)
}
```

**Isolation modes:**
```rust
pub enum IsolationMode {
    None,
    Worktree,  // git worktree isolation
}
```

**Task management tools:**
- `spawn_subagent` — create subagent
- `wait_commands_or_subagents` — wait for completion
- `kill_command_or_subagent` — terminate
- `get_command_or_subagent_output` — get result

---

### Pi Subagent System

```typescript
// Pi KHÔNG có built-in subagent spawning
// Thay vào đó dùng:
// - Queue system: steer / followUp / nextRun
// - Lane navigation: navigateTree()
// - Skills: formatSkillInvocation()
```

**Queue modes:**
```typescript
type QueueMode = "one-at-a-time" | "parallel";

interface AgentHarnessOptions {
    steeringMode?: QueueMode;
    followUpMode?: QueueMode;
    toolExecution?: "sequential" | "parallel";
    drive?: "automatic" | "manual";
}
```

---

### So Sánh Subagent System

| Aspect | xVora | Pi |
|---|---|---|
| **Built-in spawning** | ✓ | ✗ |
| **Orchestrator mode** | ✓ | ✗ |
| **Isolation** | Worktree | None |
| **Task management** | Full suite | None |
| **Queue system** | Có | ✓ steer/followUp/nextRun |
| **Lane navigation** | ✗ | ✓ multi-branch |
| **Parallel execution** | ✓ | configurable |

---

## 6. Compaction — Cách Nén Context

### xVora Compaction

```rust
// crates/codegen/xvora-agent/src/compaction.rs
pub struct CompactionPolicy {
    auto_compact_threshold_percent: u32,  // default 85%
    compact_model: Option<String>,        // model cho summary
    memory_flush_enabled: bool,           // flush memory trước compact
    wall_clock_budget_secs: u64,          // default 300s
    two_pass_enabled: bool,               // speculative summary
}

// crates/codegen/xvora-chat-state/src/compaction_utils.rs
// Strip tool results trước summarization
pub(crate) fn strip_tool_messages_for_conversation_item(
    conversation: Vec<ConversationItem>,
) -> Vec<ConversationItem> {
    // Filter out ToolResult, BackendToolCall
    // Flatten assistant tool_calls into text
}
```

**Two-pass flow:**
```
Pass 1 (background, speculative):
  Khi usage đạt 75% → summmarize early history
  Lưu vào temporary buffer

Pass 2 (triggered):
  Khi usage đạt 85% → merge pass1 + recent tail
  Tạo CompactionEntry
```

---

### Pi Compaction

```typescript
// packages/agent/src/harness/compaction/compaction.ts
export interface CompactionSettings {
    enabled: boolean;
    reserveTokens: number;       // default 16384
    keepRecentTokens: number;    // default 20000
}

export function findCutPoint(
    entries: Entry[],
    startIndex: number,
    endIndex: number,
    keepRecentTokens: number,
): CutPointResult {
    // 1. Find valid cut points
    const cutPoints = findValidCutPoints(entries, startIndex, endIndex);

    // 2. Accumulate tokens from end
    let accumulatedTokens = 0;
    for (let i = endIndex - 1; i >= startIndex; i--) {
        accumulatedTokens += estimateTokens(entries[i].message);
        if (accumulatedTokens >= keepRecentTokens) {
            cutIndex = findNearestCutPoint(i);
            break;
        }
    }

    // 3. Handle split turns
    const isUserMessage = cutEntry.type === "message" && cutEntry.message.role === "user";
    const turnStartIndex = isUserMessage ? -1 : findTurnStartIndex(...);
    return { firstKeptEntryIndex, turnStartIndex, isSplitTurn };
}
```

**Compaction prompt template:**
```markdown
## Goal
[What is the user trying to accomplish?]

## Constraints & Preferences
- [Any constraints]

## Progress
### Done
- [x] [Completed tasks]

### In Progress
- [ ] [Current work]

### Blocked
- [Issues preventing progress]

## Key Decisions
- **[Decision]**: [Brief rationale]

## Next Steps
1. [Ordered list]

## Critical Context
- [File paths, function names, error messages]
```

---

### So Sánh Compaction

| Aspect | xVora | Pi |
|---|---|---|
| **Trigger** | Threshold % (85%) | Token budget |
| **Two-pass** | ✓ Speculative | ✗ Single-pass |
| **Model override** | ✓ `compact_model` | ✗ Uses current model |
| **Memory flush** | ✓ Optional | ✗ Không có |
| **Wall clock** | ✓ Budget limit | ✗ Không có |
| **Split turn** | ✓ Handle | ✓ Handle |
| **File ops tracking** | ✗ | ✓ readFiles, modifiedFiles |
| **Branch summary** | ✓ | ✓ |

---

## 7. Hook System — Lifecycle Events

### xVora Hooks

```rust
// crates/codegen/agent-lifecycle/src/

pub trait TurnLifecycleContributor: Send + Sync {
    fn on_turn_start(&self, _input: &TurnStartInput) { }
    fn on_turn_done(&self, _input: &TurnDoneInput) { }
    fn on_turn_abort(&self, _input: &TurnAbortInput) { }
    fn on_turn_error(&self, _input: &TurnErrorInput) { }
}

pub trait SessionLifecycleContributor: Send + Sync {
    fn on_session_idle(&self, _input: &SessionIdleInput) { }
}

pub trait CommandContributor: Send + Sync {
    fn handle_command(&self, _input: &CommandInvocation) -> CommandAction {
        CommandAction::Ignore
    }
}
```

**Runner types:**
- `LocalExtensionRegistry`: hooks chạy same process
- `ExtensionRegistry`: hooks có thể spawn HTTP/commands
- Trust system cho command execution

---

### Pi Hooks

```typescript
// packages/agent/src/harness/agent-harness.ts
export type HookName =
    | "before_run"
    | "before_resume"
    | "before_run_end"
    | "transform_context"
    | "before_request"
    | "before_payload"
    | "after_response"
    | "before_tool"
    | "after_tool"
    | "before_compaction"
    | "before_navigation";

export interface Hooks {
    on(name: HookName, handler: (event: unknown) => unknown | Promise<unknown>): () => void;
}
```

**Telemetry spans:**
```typescript
"pi.harness.hook": {
    startAttributes: {
        "pi.hook.name": HookName,
        "pi.hook.registration_id": string,
    },
    endAttributes: {
        "pi.hook.outcome": "completed" | "skipped" | "blocked" | "failed",
    },
}
```

---

## 8. Session & State Management

### xVora

```rust
// crates/codegen/chat-state/src/actor/mod.rs
pub struct ChatStateActor {
    state: ChatState,
    persistence: Box<dyn ChatPersistence>,  // SQLite
    cmd_rx: mpsc::UnboundedReceiver<ChatStateCommand>,
    event_tx: mpsc::UnboundedSender<ChatStateEvent>,
    cancellation_token: CancellationToken,
}

// Commands (mutation + query)
enum ChatStateCommand {
    PushUserMessage { item },
    PushAssistantResponse { item },
    PushToolResult { item },
    BuildConversationRequest { ... },
    CheckAutoCompactNeeded { threshold_percent },
    // ... 30+ commands
}
```

---

### Pi

```typescript
// packages/agent/src/harness/session/session.ts
export class Session<TMetadata> implements SessionTree {
    private storage: SessionStorage<TMetadata>;  // JSONL

    async getLeafId(): Promise<string | null>
    async appendMessage(message: AgentMessage): Promise<string>
    async findEntries(query?: EntryQuery): Promise<Entry[]>
    async createLane(lane: string, at: string | null): Promise<void>
}

// Entries
type Entry =
    | MessageEntry
    | CompactionEntry
    | BranchSummaryEntry
    | ThinkingLevelEntry
    | ModelChangeEntry
    | ActiveToolsEntry
    | CustomEntry;
```

---

## 9. Điểm Mạnh/Yếu Tổng Thể

### xVora — Điểm Mạnh
1. **Actor isolation** — không lock contention, scalable
2. **Two-pass compaction** — speculative summary giảm latency
3. **Tool preset system** — dễ swap modes (codex/grok-build/explore)
4. **Subagent orchestration** — orchestrator mode, worktree isolation
5. **Memory system** — embedding + MMR semantic search
6. **Prompt layers** — AGENTS.md, personas, memory injection
7. **Image budget** — auto-eviction when body too large

### xVora — Điểm Yếu
1. **Không có lane/branch navigation** — single conversation path
2. **Thinking level dynamic change** — cần implement
3. **Hook system phức tạp** — nhiều traits cần quản lý
4. **Chưa có typed telemetry schema** — debugging khó hơn

### Pi — Điểm Mạnh
1. **Reducer pattern** — explicit state machine với ActionInfo
2. **Typed telemetry** — schema-driven spans, TypeBox
3. **Lane-based sessions** — multi-branch, navigateTree()
4. **Queue modes** — one-at-a-time vs parallel
5. **File mutation queue** — prevent concurrent writes
6. **Deferred execution** — DeferredHandle cho async tools
7. **Thinking level dynamic** — đổi giữa turns dễ dàng

### Pi — Điểm Yếu
1. **Không có subagent spawning** — phải tự implement
2. **JSONL storage** — không query được như SQLite
3. **Tool count ít** — chỉ ~9 built-in tools
4. **Không có memory system** — không có semantic search
5. **Single-threaded** — reducer chạy trên event loop

---

## 10. Khuyến Nghị Tích Hợp

### Từ Pi → xVora (Priority Cao)

1. **Dynamic Thinking Level**
   ```rust
   // Thêm vào SamplingConfig
   pub struct SamplingConfig {
       pub reasoning_effort: Option<ReasoningEffort>,
       // Thêm:
       pub thinking_level: ThinkingLevel,  // dynamic per-turn
   }
   ```

2. **Typed Telemetry Schema**
   ```rust
   // Thêm schema-driven span system
   const HARNESS_SPAN_SCHEMA: TelemetrySchema = /* Pi's schema */;
   ```

3. **File Mutation Queue**
   ```rust
   struct FileMutationQueue {
       pending: BTreeMap<PathBuf, Vec<WriteOp>>,
   }
   ```

### Từ xVora → Pi (Priority Cao)

1. **Subagent orchestration** — orchestrator mode
2. **Two-pass compaction** — speculative summary
3. **Tool preset system** — easy mode switching
4. **Memory system** — embedding + search
5. **AGENTS.md injection** — multi-file system prompt

---

## 11. Số Liệu So Sánh

| Metric | xVora | Pi |
|---|---|---|
| **Turn loop** | Actor pattern (tokio) | Reducer (single-threaded) |
| **Tools** | 256+ implementations | ~9 core tools |
| **Subagents** | Built-in, orchestrated | Not supported |
| **Compaction** | Two-pass + memory flush | Single-pass |
| **Thinking levels** | 7 (static per-config) | 7 (dynamic per-turn) |
| **Session store** | SQLite | JSONL |
| **Navigation** | Single path | Multi-lane, branches |
| **Memory** | Embedding + MMR | None |

---

## 12. Kết Luận

**xVora mạnh về:**
- Scalability (actor pattern)
- Tool richness (256+ tools)
- Subagent orchestration
- Compaction optimization (two-pass)
- Prompt assembly (layers, AGENTS.md)

**Pi mạnh về:**
- Observability (typed telemetry)
- Navigation (lanes, branches)
- Dynamic thinking control
- State machine clarity (reducer)
- Queue management (steer/followUp)

**Recommendation:** Giữ xVora's actor pattern + two-pass compaction + subagent system, thêm Pi's typed telemetry + dynamic thinking level + lane navigation.
