# DeepSeek Harness vs xVora — So Sánh Core Harness

**Ngày:** 2026-09-01  
**Nguồn:** `D:\Kaiyo\Project\deepseek-harness`

---

## Mục Lục

1. [Tổng Quan Kiến Trúc](#1-tổng-quan-kiến-trúc)
2. [Agent Loop — Turn & Step Lifecycle](#2-agent-loop—turn--step-lifecycle)
3. [Tool Execution — Parallel Dispatch](#3-tool-execution--parallel-dispatch)
4. [Reasoning Effort System](#4-reasoning-effort-system)
5. [Content Block Types (Thinking Support)](#5-content-block-types-thinking-support)
6. [Compaction System](#6-compaction-system)
7. [Session Event Log](#7-session-event-log)
8. [System Prompt Assembly](#8-system-prompt-assembly)
9. [Workspace Instructions (AGENTS.md)](#9-workspace-instructions-agentsmd)
10. [Image Support](#10-image-support)
11. [Hooks / Waterfall System](#11-hooks--waterfall-system)
12. [So Sánh Tổng Thể](#12-so-sánh-tổng-thể)
13. [Khuyến Nghị Tích Hợp Cho xVora](#13-khuyến-nghị-tích-hợp-cho-xvora)

---

## 1. Tổng Quan Kiến Trúc

### DeepSeek Harness

```
TypeScript/Node.js monorepo (pnpm workspaces)
├── packages/core/agent-loop/          ← Agent driver chính
├── packages/core/agent/               ← Agent interface, Inbox
├── packages/core/session/             ← Session event log types
├── packages/core/system-prompt/       ← Prompt assembly registry
├── packages/compaction/compaction/    ← Compaction engine interface
├── packages/compaction/compaction-basic/  ← Default summarizer
├── packages/llm/llm/                  ← LLM abstraction layer
├── packages/llm/llm-deepseek/         ← DeepSeek API adapter
├── packages/context/agent-instructions/ ← AGENTS.md loader
├── packages/session/session-persistence-jsonl/ ← JSONL persistence
├── packages/tools/                    ← Tool runtime (dsh-tools)
└── apps/cli/ / apps/web/              ← Frontend + CLI
```

**Runtime:** Node.js, single-threaded event loop  
**Dependency injection:** Cordis framework (`Context` + `Service`)  
**State:** Durable JSONL event log + in-memory surface projection

---

### xVora

```
Rust/Tokio workspace
├── crates/codegen/chat-state/         ← ChatStateActor (mpsc channel)
├── crates/codegen/agent-lifecycle/    ← Lifecycle hooks
├── crates/codegen/xvora-agent/        ← Agent builder, prompt assembly
├── crates/codegen/xvora-tools/        ← 256+ tool implementations
├── crates/codegen/xvora-sampling-types/ ← ReasoningEffort enum
└── crates/codegen/agent-lifecycle/    ← Compaction utilities
```

**Runtime:** Tokio async, actor pattern với mpsc channels  
**State:** SQLite persistence + in-memory ChatState

---

## 2. Agent Loop — Turn & Step Lifecycle

### DeepSeek Harness: ReactLoopAgent

```typescript
// packages/core/agent-loop/src/agent.ts

type Phase =
  | { kind: 'idle'; lastTurn: number }
  | { kind: 'maintenance'; abort: AbortController; lastTurn: number; wakeRequested: boolean }
  | { kind: 'running'; abort: AbortController; turn: number; step: number; wakeRequested: boolean }

class ReactLoopAgent implements Agent {
  inbox: Inbox          // dual queue: next-turn + next-step
  phase: Phase
  scope: Scope          // Cordis lifecycle scope

  // Input routing
  followup(msg) → inbox.next-turn  // standard new prompt
  steer(msg)    → inbox.next-step  // mid-turn intervention
  inject(msg)   → inbox.next-step  // system-injected context

  async run() {         // maintenance job
  async cancel() {      // AbortController
  async whenIdle() {}   // wait for completion
}
```

**Turn/Step Flow:**
```
kick()
  └─ while turn():
       ├─ turn/start event
       ├─ while step():
       │    ├─ preStep():
       │    │   ├─ inbox.claim(target)  // next-turn or next-step
       │    │   ├─ systemPrompt.assemble()  // render full prompt
       │    │   └─ dispatch.waterfall('agent/pre-step')  // plugins can modify
       │    ├─ step/start event
       │    ├─ LLM call (stream)
       │    │   ├─ assistant/chunk events (durable)
       │    │   └─ assembler.finish()
       │    ├─ assistant/message event
       │    ├─ if tool_calls: executeToolCalls()
       │    └─ step/end event
       │    └─ if inbox.nextStep empty → break
       │    └─ target = 'next-step' for next iteration
       ├─ turn/end event
       └─ inbox.hasPending? → start next turn
```

**Điểm khác biệt so với xVora:**
| Aspect | DeepSeek Harness | xVora |
|---|---|---|
| **Pattern** | Async iterator (`while true`) | Actor + mpsc channels |
| **Input routing** | 3 types: followup/steer/inject | Single `PushUserMessage` |
| **Turn boundaries** | Durable `turn/start`/`turn/end` events | Implicit (no boundary events) |
| **Step boundaries** | `step/start`/`step/end` events | No explicit step events |
| **Pre-step hook** | `agent/pre-step` waterfall (plugin-modifiable) | `TurnLifecycleContributor.on_turn_start` |
| **Abort handling** | `AbortController` per phase | `CancellationToken` |

---

## 3. Tool Execution — Parallel Dispatch

### DeepSeek Harness

```typescript
// packages/core/agent-loop/src/tool-calls.ts

async function executeToolCalls(
  ctx, turn, step, toolCalls: ToolCallBlock[], signal,
  acceptContext: (context: UserMessage) => void,
): Promise<{ concluded: boolean }>
```

**Algorithm:**
```
for each tool call in model order:
  ├─ mode = ctx.tools.executionMode(call)  // 'parallel' or 'exclusive'
  ├─ if parallel: form group = contiguous parallel calls
  ├─ if exclusive: group = [single call]
  ├─ runGroup(ctx, group, mode, signal, acceptContext):
  │   ├─ bounded parallel pool (maxParallelToolCalls from config)
  │   ├─ ordered commit: results appended model-order
  │   ├─ each result may add additionalContext → accepted into next-step inbox
  │   └─ concluded flag from any result.concludesTurn
  ├─ if mode changes mid-group: reclassify before starting next call
  └─ commit results in model order (not execution order)
```

**Key features:**
- **Ordered commit**: Results always appear in model's call order, even if executed in parallel
- **Barrier semantics**: When a later call is `exclusive`, it forces sequential execution
- **Abort safety**: Skipped calls get synthetic error results for valid replay
- **Additional context**: Tools can inject messages into `next-step` inbox mid-turn
- **Max parallel**: Configurable `maxParallelToolCalls`

---

### xVora

```rust
// crates/codegen/xvora-tools/src/implementations/grok_build/web_search/mod.rs
// Tool execution is handled by ToolBridge::execute()
```

- Sequential execution (one tool call at a time)
- No parallel dispatch group
- No `executionMode` differentiation
- Simpler but no concurrent tool calls

---

## 4. Reasoning Effort System

### DeepSeek Harness

```typescript
// packages/llm/llm/src/types.ts
export interface GenerateOptions {
  provider: string
  model: string
  reasoningEffort?: ReasoningEffortId  // provider-specific string ID
  // ...
}

// Adapter-level metadata
export interface LlmModelReasoningInfo {
  efforts: readonly LlmReasoningEffortInfo[]  // [{ id, name, description }]
  defaultEffort?: ReasoningEffortId
}
```

**Persistence across turns:**
```typescript
// agent.ts line 458-476
const persistedReasoningEffort = persistedConfig?.provider === route.provider
  && persistedConfig.model === route.model
  ? persistedConfig.reasoningEffort
  : undefined
const reasoningEffort = this.options.reasoningEffort ?? persistedReasoningEffort
```

- Per-model persistence: stays bound to the exact provider+model route
- Clears when model changes
- `requestProposal()` strips adapter defaults before plugin proposals

---

### xVora

```rust
// crates/codegen/xvora-sampling-types/src/types.rs
pub enum ReasoningEffort {
    None, Minimal, Low, Medium, High, Xhigh, Max
}
pub struct SamplingConfig {
    pub reasoning_effort: Option<ReasoningEffort>,
}
```

- Static per-config, no per-turn dynamic change
- Enum-based (strongly typed) vs DeepSeek's opaque string ID
- Same 7 levels as Pi

---

## 5. Content Block Types (Thinking Support)

### DeepSeek Harness

```typescript
// packages/llm/llm/src/types.ts
export interface ContentBlockMap {
  'text': TextBlock
  'reasoning': ReasoningBlock   // ← thinking/thinking content
  'image': ImageBlock
  'tool-call': ToolCallBlock
  'tool-result': ToolResultBlock
}

export interface ReasoningBlock {
  type: 'reasoning'
  text: string
}

// Stream chunks include reasoning deltas
export type StreamChunk =
  | { type: 'reasoning-delta'; index: number; text: string }
  | { type: 'text-delta'; index: number; text: string }
  | { type: 'tool-call-delta'; ... }
  | { type: 'usage'; usage: TokenUsage }  // includes reasoningTokens
```

**Token usage tracking:**
```typescript
export interface TokenUsage {
  inputTokens: number
  outputTokens: number
  reasoningTokens?: number  // separate from output
  cacheReadTokens?: number
  cacheWriteTokens?: number
}
```

**BlockAssembler** handles interleaved reasoning + text streams, producing clean separated blocks.

---

### xVora

- `Thinking` block type exists in conversation types but uses `ThinkingBlock` struct
- No `reasoningTokens` separate tracking in usage (only total tokens per model)
- Thinking blocks use `display: "summarized"` for UI optimization

---

## 6. Compaction System

### DeepSeek Harness

```
packages/compaction/compaction/          ← Interface
packages/compaction/compaction-basic/    ← Default implementation
packages/compaction/compaction-tool-result-pruner/ ← Pruner
```

**Interface:**
```typescript
abstract class CompactionEngine extends Service {
  // Automatic trigger (called by agent loop on pressure)
  abstract compactIfNeeded(agent, trigger, signal): Promise<CompactionResult | null>

  // Manual trigger (from /compact command)
  abstract compactNow(agent, signal, commandId?): Promise<CompactionResult | null>

  // Region-based (explicit start/end)
  abstract compactRegion(start, end, agent, signal?): Promise<CompactionResult>
}
```

**Trigger types:**
- `'pressure'` — automatic, based on token budget
- `'context-overflow'` — forced when context exceeds window

**Session events (log-only, not surface):**
```typescript
'turn/start' | 'turn/end'
'step/start' | 'step/end'
'assistant/chunk' | 'assistant/message'
'tool/call' | 'tool/result'
'request/header' | 'request/context'
'compaction/start' | 'compaction/summary' | 'compaction/end' | 'compaction/prune'
```

**Compaction flow:**
```
1. compaction/start (acquire durable lock)
2. Find range to compact (balanced tool pair check)
3. Reconstruct messages + system prompt + tools from log
4. Call LLM stream with COMPACTION_INSTRUCTION
5. compaction/summary (log result, shadowed range)
6. Append replacement user message with frame:
   - CHECKPOINT_PREAMBLE
   - <compacted-summary> ... </compacted-summary>
7. Surface replace (shadow range → single summary node)
8. compaction/end (release lock)
```

**Summarization instruction template** (from `compaction-basic/src/summarizer.ts`):
```markdown
You are now acting as a compaction engine...
Condense the conversation ABOVE into:
## Primary Request and Intent
## Key Technical Concepts
## Files and Code
## Errors and Fixes
## Pending Jobs
## Current Work
## Next Step
## Critical Context
Rules: preserve file paths, commands, error strings, function signatures...
If a <compacted-summary> already exists, merge, don't copy verbatim.
```

---

### xVora

- Two-pass speculative compaction
- Model-override for summary (`compact_model`)
- Memory flush before compaction
- Wall clock budget (300s default)
- Tool result stripping before summary

---

## 7. Session Event Log

### DeepSeek Harness

```typescript
// packages/core/session/src/types.ts
export interface SessionEventMap {
  'turn/start': { turn: number }
  'turn/end': { turn: number; reason: TurnEndReason }
  'step/start': { turn: number; step: number }
  'step/end': { turn: number; step: number }
  'user/message': UserMessage          // source distinguishes human/inject/skill
  'assistant/chunk': { turn, step, chunk: StreamChunk }
  'assistant/message': { turn, step, message, usage?, interrupted? }
  'tool/call': { turn, step, callId, name, arguments }
  'tool/result': { turn, step, message, error?, meta? }
  'request/header': { header: EpochHeader, reason, startsSeries? }
  'request/context': RequestContext
  'session/end-seed': Record<string, never>
  // Compaction events
  'compaction/start': { compactionId, sourceCommandId?, turn? }
  'compaction/summary': { compactionId, summary, shadowedRange, ... }
  'compaction/end': { compactionId, error? }
  'compaction/prune': { shadowedRange, shadowedSeqs, shadowedTokenCount }
}
```

**Surface events** (`surfaceOp: 'append' | { op: 'replace', start, end }`):
- `user/message`
- `assistant/message`
- `tool/result`

**Log format:** Append-only JSONL with monotonic `seq` numbers. `surfaceOp` marks which events appear in the derived conversation. Compaction uses `replace` to shadow a range with a summary node.

---

### xVora

- SQLite table-based persistence
- `ConversationItem` enum with explicit variants
- No granular step boundaries
- No durable event log for replay reconstruction

---

## 8. System Prompt Assembly

### DeepSeek Harness

```typescript
// packages/core/system-prompt/src/index.ts
class SystemPrompt extends Service {
  // Section orders (fixed positions)
  HARNESS_IDENTITY: -1000
  DEPLOYMENT_PERSONA: 0
  TOOL_BASH: 1000, TOOL_READ: 1100, TOOL_WRITE: 1200, ...
  TOOL_WEB_SEARCH: 2000, TOOL_WEB_FETCH: 2100
  TOOL_LSP: 2200, TOOL_SUBAGENT: 2800, ...
  STRUCTURED_OUTPUT: 9900

  // Dynamic contexts
  SANDBOX_POLICY: 110, APPROVAL_POLICY: 115, SUBAGENT_DELEGATION: 120

  // Registration API
  section({ name, order, text })           // static prompt section
  context({ name, order, text })           // dynamic runtime context
  tools(provider)                          // tool schema provider
  variable(name, provider)                 // {{variable}} interpolation
}
```

**Assembly process:**
```
1. Collect global + scoped sections (scoped shadows global)
2. Sort by order, deterministic by name for ties
3. Evaluate each text (static string or () => string)
4. Collect tool schemas from all providers
5. Validate toolOrder configuration
6. Run assembly waterfall (plugins can transform)
7. Render variables: {{name}} → resolved value
8. Join sections with blank lines
```

**Plugin integration via Cordis:**
- `system-prompt/assemble` waterfall: plugins can inspect/modify assembly
- `system-prompt/change` event: fired when registrations change

---

### xVora

- Fixed template layers (prompt.md + overrides)
- `PromptContext` struct with named fields
- `render_*` functions produce final prompt
- No plugin extensibility at assembly time
- No variable interpolation system

---

## 9. Workspace Instructions (AGENTS.md)

### DeepSeek Harness

```typescript
// packages/context/agent-instructions/src/index.ts
function apply(ctx, config: Config) { ... }
```

**Mechanism:**
1. On session start: find `AGENTS.md` at project root, load as baseline
2. On each `pre-step`: re-evaluate if files changed
3. On file touch (read/write/edit): queue re-composition
4. Context injected into `next-step` inbox between user message and system context
5. `baselineIdentity` detects project root changes
6. Version cache (`WeakMap<Session, Map<scope, state>>`) for incremental updates

**Config:**
```typescript
interface Config {
  projectRootMarkers: string[]     // e.g. ['.git', 'package.json']
  maxBytes: number                 // cap on instruction size
  maxSourceBytes: number           // per-file cap
  instructionFileCandidates: string[]  // e.g. ['AGENTS.md', '.claude/AGENTS.md']
  dshHome: string                  // harness home directory
}
```

---

### xVora

- `agents_md_files: Vec<AgentConfigFile>` in `PromptContext`
- Loaded once at agent creation
- No file-watcher based incremental updates
- No baseline identity detection

---

## 10. Image Support

### DeepSeek Harness

```typescript
export interface ImageBlock {
  type: 'image'
  attachment: ImageAttachmentRef  // immutable bytes + display metadata
}

// In LlmImageRequestPrice
export interface LlmImageRequestPrice {
  visualTokens: number    // provider-side image token cost
  text: string            // text substitution sent to model
}
```

**Architecture:**
- `dsh-attachment` package manages image storage
- Images stored as immutable references (not base64 inline)
- Provider adapters translate to wire format
- Visual token pricing tracked separately from text tokens
- Only user messages carry images (current production)

---

### xVora

- `ImageBudget` component with eviction logic
- Images embedded as base64 in conversation
- `image_budget.rs` — auto-eviction when body too large
- `ImageGenTool` for generation

---

## 11. Hooks / Waterfall System

### DeepSeek Harness

Cordis framework provides structured event hooks:

```typescript
// agent-loop events
'agent/status'          → { status: 'idle' | 'running' }
'agent/error'           → { turn, step, error }
'agent/pre-step'        → waterfall, returns PreStepDecision
'agent/request'         → waterfall, modifies LlmCallConfig
'agent/turn-stopping'   → serial, runs when turn is about to end

// system-prompt events
'system-prompt/assemble' → waterfall, returns PromptAssembly
'system-prompt/change'   → emit

// tool events
'tools/result'           → tool completion callback
```

**PreStepDecision:**
```typescript
type PreStepDecision =
  | { kind: 'enter'; messages: UserMessage[]; assembly: PromptAssembly }
  | { kind: 'reject' }  // block the step
```

Plugins can intercept at:
- **pre-step**: modify messages, inject context, reject the step
- **request**: modify provider/model/reasoningEffort
- **turn-stopping**: cleanup or notification before turn ends
- **system-prompt/assemble**: transform prompt sections

---

### xVora

```rust
pub trait TurnLifecycleContributor {
    fn on_turn_start(&self, input: &TurnStartInput) { }
    fn on_turn_done(&self, input: &TurnDoneInput) { }
    fn on_turn_abort(&self, input: &TurnAbortInput) { }
    fn on_turn_error(&self, input: &TurnErrorInput) { }
}
```

- Hook-based, simpler model
- No waterfall/interception pattern
- No ability to modify messages mid-turn

---

## 12. So Sánh Tổng Thể

| Aspect | DeepSeek Harness | xVora | Winner |
|---|---|---|---|
| **Lang** | TypeScript/Node.js | Rust/Tokio | Different tradeoffs |
| **Turn flow** | Async loop + durable events | Actor + mpsc | Harness (replayable) |
| **Step granularity** | `step/start/end` events | Not tracked | Harness |
| **Tool parallelism** | Bounded pool + ordered commit | Sequential | Harness |
| **Reasoning** | Per-model persist, adapter-driven | Static enum | Harness (more flexible) |
| **Content blocks** | 5 types incl. reasoning+image | 4 types | Harness |
| **Compaction** | Lock+checkpoint+durable summary | Two-pass speculative | Both各有优势 |
| **Session log** | Append-only JSONL, replayable | SQLite, point-in-time | Harness |
| **Prompt assembly** | Plugin waterfall + variable interp | Fixed templates | Harness |
| **AGENTS.md** | File watcher + incremental | Load once | Harness |
| **Image support** | Attachment ref + visual token pricing | Base64 + budget eviction | Harness |
| **Hook system** | Waterfall + serial + emit | Trait-based callbacks | Harness |
| **Tool count** | ~30 core + MCP | 256+ | xVora |
| **Subagent** | Built-in delegation depth | Orchestrator mode | Tie |
| **Memory** | Session telemetry + checkpoint | Embedding + MMR | xVora |
| **Cross-platform** | Node.js (cross-platform) | Cross-compiled Rust | Tie |

---

## 13. Khuyến Nghị Tích Hợp Cho xVora

### Priority Cao — Bắt Buộc

**1. Durable Step Boundaries**
```rust
// Thêm session event types
enum ChatStateEvent {
    StepStart { turn: u32, step: u32 },
    StepEnd { turn: u32, step: u32 },
    ToolCall { turn: u32, step: u32, call_id: String, name: String, arguments: String },
    ToolResult { turn: u32, step: u32, call_id: String, content: String, meta: Option<JsonValue> },
}
```
→ Enables replay, better debugging, precise compaction ranges

**2. Ordered Parallel Tool Dispatch**
```rust
enum ToolExecutionMode { Parallel, Exclusive }
async fn execute_tool_group(mode, calls, signal) -> Vec<ToolResult>
```
→ `maxParallelToolCalls` config, results committed in model order

**3. Reasoning Tokens Tracking**
```rust
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub reasoning_tokens: Option<u64>,  // NEW
    pub cache_read_tokens: Option<u64>, // NEW
    pub cache_write_tokens: Option<u64>,// NEW
}
```

**4. Content Block Type for Reasoning**
```rust
enum ContentBlock {
    Text { text: String },
    Reasoning { text: String },  // NEW
    Image { url: String },
    ToolCall { id: String, name: String, args: String },
    ToolResult { call_id: String, content: Vec<ContentBlock> },
}
```

### Priority Trung Bình

**5. System Prompt Plugin Waterfall**
```rust
pub trait PromptAssemblyPlugin: Send + Sync {
    fn on_assemble(&self, assembly: &mut PromptAssembly) -> Result<(), Error>;
}
// Register: registry.register_prompt_plugin(my_plugin);
```

**6. AGENTS.md Incremental Watcher**
```rust
pub struct AgentInstructionsWatcher {
    paths: Vec<PathBuf>,
    watcher: NotifyWatcher,
}
// On file change: re-render and inject into inbox
```

**7. Compaction Checkpoint Format**
```rust
// Standardized compaction template (like DeepSeek's structure)
pub const COMPACTION_TEMPLATE: &str = r#"
## Primary Request and Intent
- [...]
## Key Technical Concepts
- [...]
## Files and Code
- [...]
## Errors and Fixes
- [...]
## Pending Jobs
- [...]
## Current Work
- [...]
## Next Step
- [...]
"#;
```

**8. Prompt Variable Interpolation**
```rust
pub struct PromptVariables {
    vars: HashMap<String, String>,
}
impl PromptVariables {
    pub fn interpolate(&self, template: &str) -> String {
        // Replace {{name}} with resolved values
    }
}
```

### Priority Thấp

**9. Image Attachment Reference**
```rust
pub struct ImageAttachment {
    pub id: String,
    pub path: PathBuf,
    pub visual_tokens: u32,
}
// Store as ref, not base64 inline
```

**10. Request Header Persistence**
```rust
pub struct EpochHeader {
    pub config: LlmCallConfig,
    pub system: Option<String>,
    pub tools: Option<Vec<ToolSchema>>,
}
// Persist latest header for compaction context reconstruction
```

---

## 14. DeepSeek Harness — Điểm Mạnh Nhất

1. **Durable event log**: Every action is logged with seq numbers → full replay capability
2. **Ordered parallel tools**: Concurrent execution without losing model-order semantics
3. **Plugin waterfall**: Extensible prompt/tool/request pipeline
4. **AGENTS.md incremental**: File watch + baseline identity = correct context always
5. **Compaction checkpoint**: Structured template preserves engineering context
6. **Reasoning token accounting**: Separate `reasoningTokens` from output tokens
7. **Image as attachment**: Stored separately, visual token pricing

## 15. xVora — Điểm Mạnh Nhất

1. **256+ tool implementations**: Most mature tool ecosystem
2. **Actor isolation**: No shared mutable state, tokio-native concurrency
3. **Two-pass compaction**: Speculative summary reduces latency
4. **Memory system**: Embedding + MMR semantic search
5. **Subagent orchestration**: Built-in orchestrator mode + worktree isolation
6. **Tool preset system**: Easy mode switching (codex/grok-build/explore)
7. **SQLite persistence**: Queryable state, not just append log

---

*Tài liệu này so sánh kiến trúc core agent processing của hai harness. Chi tiết implementation xem source code:*
- *DeepSeek Harness: `D:\Kaiyo\Project\deepseek-harness\packages\core\agent-loop\src\agent.ts`*
- *xVora: `crates/codegen/chat-state/src/actor/mod.rs`, `crates/codegen/xvora-agent/src/agent.rs`*
