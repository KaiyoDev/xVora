# Báo Cáo So Sánh Core Harness: xVora vs Pi

**Ngày:** 2026-09-01  
**xVora:** `D:\Kaiyo\Project\xVora` (Rust, ~80 crates)  
**Pi:** `D:\Kaiyo\Project\pi` (TypeScript/Node.js, monorepo 1499 nodes)

---

## 1. Tổng Quan Kiến Trúc

| | xVora | Pi |
|---|---|---|
| **Ngôn ngữ** | Rust | TypeScript |
| **Runtime** | Tokio async | Node.js event loop |
| **Kiến trúc** | Actor-based (ChatStateActor + mpsc channels) | Class-based orchestrator (AgentHarness) |
| **Session store** | SQLite + memory | JSONL (file-based) |
| **Số crates/packages** | ~80 crates | ~20 packages |

---

## 2. So Sánh Từng Thành Phần Core

### 2.1 Agent Core & Lifecycle

| Thành phần | xVora | Pi | Ghi chú |
|---|---|---|---|
| **Agent definition** | `AgentDefinition` từ `.md` YAML frontmatter | `AgentHarnessOptions` struct | xVora có preset system (grok-build, codex, explore...) |
| **Lifecycle hooks** | `agent-lifecycle` crate: `LocalExtensionRegistry`, `TurnLifecycleContributor`, `SessionLifecycleContributor` | Hooks interface: `before_run`, `after_tool`, `before_compaction`... | Cả 2 đều có hook system, xVora phân tách local/send rõ hơn |
| **System prompt** | `PromptContext.render()`, `TemplateOverride` | `buildSystemPrompt()`, `buildCodingAgentHarnessSystemPrompt()` | Tương đương |

### 2.2 Tool System

| Thành phần | xVora | Pi | Ghi chú |
|---|---|---|---|
| **Tool registry** | `ToolBridge` + `ToolServerConfig` | `HarnessTool[]` array | xVora có MCP elicitation, tool taxonomy |
| **Built-in tools** | Bash, ReadFile, SearchReplace, Grep, ListDir, TodoWrite, TaskTool, WebSearch, ImageGen, Lsp... | Bash, Read, Edit, Write (diff-based) | xVora phong phú hơn nhiều (256+ tool files) |
| **Tool execution** | `xvora-tools/src/implementations/` | `tools/bash.ts`, `tools/edit.ts`, `tools/write.ts` | Pi dùng diff-based edit, xVora dùng SearchReplace |
| **MCP support** | `xvora-mcp` crate | Có trong `ExecutionEnv` abstraction | xVora có MCP server config inheritance |

### 2.3 Session & State Management

| Thành phần | xVora | Pi | Ghi chú |
|---|---|---|---|
| **State container** | `ChatStateActor` (tokio actor) | `Session<TMetadata>` class | Cả 2 bất đồng bộ, xVora có message passing qua mpsc |
| **Message types** | `ConversationItem` enum | `AgentMessage` union type | Tương đương |
| **Persistence** | `ChatPersistence` trait + SQLite | JSONL file storage | Pi có `JsonlSessionRepo`, `memory.ts` for in-memory |
| **Command/query pattern** | `ChatStateCommand` + `ChatStateHandle` | `SessionTree` interface | xVora dùng actor pattern rõ ràng hơn |

### 2.4 Compaction System

| Thành phần | xVora | Pi | Ghi chú |
|---|---|---|---|
| **Policy** | `CompactionPolicy`: threshold %, compact_model, wall_clock_budget, two_pass | `CompactionSettings`: enabled, reserveTokens, keepRecentTokens | xVora có two-pass speculative compaction |
| **Utils** | `xvora-chat-state/compaction_utils.rs` | `compaction/compaction.ts` + `utils.ts` | Cả 2 có token estimation, cut-point finding |
| **Summarization prompt** | Có trong `xvora-compaction-transcript` | `SUMMARIZATION_PROMPT` hardcoded | Tương đương |
| **Branch summary** | `xvora-compaction` crate | `branch-summarization.ts` | Cả 2 đều hỗ trợ |

### 2.5 Skills System

| Thành phần | xVora | Pi | Ghi chú |
|---|---|---|---|
| **Discovery** | `xvora-agent/src/prompt/skills.rs` | `skills.ts` - recursive SKILL.md loader | Cùng pattern: `.md` files với YAML frontmatter |
| **Format** | XML `<skill>` blocks trong system prompt | XML `<skill name>` blocks | Giống hệt |
| **Validation** | Name validation, description validation | Same validation logic | Pi có `SkillDiagnostic` tracking |

### 2.6 Telemetry & Observability

| Thành phần | xVora | Pi | Ghi chú |
|---|---|---|---|
| **Schema** | Tự định nghĩa trong code | `AI_TELEMETRY_SCHEMA` + `HARNESS_TELEMETRY_SCHEMA` constants | Pi có typed span system mạnh hơn |
| **Spans** | `pi.harness.run`, `pi.harness.turn`, `pi.harness.tool`... | Event tracing | Pi có span hierarchy rõ ràng (parents) |
| **Metrics** | Token usage, cost tracking | `Usage` type với cache_read/write, reasoning tokens | Tương đương |

### 2.7 Memory System

| Thành phần | xVora | Pi | Ghi chú |
|---|---|---|---|
| **Implementation** | `xvora-memory` crate: embedding, chunking, MMR index, search | Không có built-in memory | xVora có vector search với embeddings |
| **Scopes** | User/Project/Local | Không có | xVora hỗ trợ multi-scope |

### 2.8 Hook System

| Thành phần | xVora | Pi | Ghi chú |
|---|---|---|---|
| **Runner** | Command + HTTP runners | Function callbacks | xVora cóTrust system cho hooks |
| **Events** | `event.rs` with matchers | `Hooks.on()` pattern | Tương đương |

---

## 3. Công Nghệ Cần Tích Hợp Từ Pi Vào xVora

### 3.1 Priority Cao (Critical Gaps)

| STT | Công nghệ | Mô tả | Mức độ ưu tiên |
|---|---|---|---|
| 1 | **TypeBox schema validation** | Pi dùng `typebox` cho tool parameter validation - xVora cần cơ chế tương tự | Cao |
| 2 | **Typed telemetry spans** | Pi có `TelemetrySchemaDefinition` typed system - xVora nên có typed span API | Cao |
| 3 | **File-system abstraction** | Pi có `FileSystem` + `Shell` traits - xVora nên abstract hóa để testable | Trung |
| 4 | **Diff-based edit tool** | Pi dùng edit-diff thay vì search-replace - UX tốt hơn cho model | Thấp |
| 5 | **Result type pattern** | Pi có `Result<T, E>` monad - xVora dùng `-> Result<T, Error>` truyền thống | Đã có tương đương |

### 3.2 Priority Trung Bình

| STT | Công nghệ | Mô tả | Mức độ ưu tiên |
|---|---|---|---|
| 6 | **Deferred execution** | Pi có `DeferredHandle` cho async tool results - xVora cần thêm | Trung |
| 7 | **Queue modes** | Pi có `steeringMode`/`followUpMode` (one-at-a-time vs batch) - xVora chưa có | Trung |
| 8 | **Lane-based session tree** | Pi có multiple lanes, branch navigation - xVora chỉ có single conversation | Thấp |
| 9 | **Prompt templates** | Pi có `prompt-templates.ts` với argument formatting - xVora đã có `TemplateOverride` | Đã có |

### 3.3 Công Nghệ Pi Có Nhưng xVora Không Có

| Công nghệ | Pi | xVora | Đề xuất |
|---|---|---|---|
| **Memory search/get tools** | ✓ | ✗ (có crate nhưng chưa expose as tools) | Tích hợp vào toolset |
| **Image-to-video tools** | ✓ | ✗ | Thêm nếu cần |
| **Web fetch tool** | ✓ | Có `web_fetch` nhưng không có wrapper | Chuẩn hóa |
| **Scheduler tools** | ✓ | ✗ | Thêm nếu cần background tasks |
| **Monitor tool** | ✓ | ✗ | Thêm nếu cần long-running ops |

---

## 4. Kiến Trúc Nên Giữ Nguyên Từ xVora

1. **Actor pattern** (`ChatStateActor`) - hiệu suất cao, không lock contention
2. **ToolBridge abstraction** - decouple tool registry từ agent core
3. **Two-pass compaction** - speculative summary là optimization quan trọng
4. **Plugin system** - `xvora-agent/plugins/` đã hoàn chỉnh
5. **Hook trust system** - bảo mật quan trọng cho command execution
6. **MCP inheritance** - config inheritance pattern hay
7. **Worktree isolation** - subagent isolation mode

---

## 5. Khuyến Nghị Tích Hợp

### Ưu tiên ngay:
1. **Import typed telemetry** từ Pi - xVora đang missing span hierarchy
2. **Thêm FileSystem/Shell abstractions** - giúp testing dễ hơn
3. **Chuẩn hóa Result types** - Pi có `ok()`/`err()` helpers rất hữu ích

### Nên cân nhắc:
1. **Deferred execution pattern** - cho tool results không ready ngay
2. **Queue modes** - một-at-a-time vs parallel execution
3. **Lane-based sessions** - nếu cần multi-branch conversations

### Không nhất thiết:
1. Diff-based edit (xVora's SearchReplace đã OK)
2. Pi's exact telemetry schema (xVora có tracing)
3. Lane navigation (xVora's single conversation đủ cho大部分 use cases)

---

## 6. Số Liệu So Sánh

| Metric | xVora | Pi |
|---|---|---|
| **Tổng nodes/elements** | ~80 crates | ~1499 TS nodes |
| **Tool implementations** | 256+ files | ~9 tool files |
| **Session persistence** | SQLite | JSONL |
| **Compaction styles** | Single + Two-pass | Single + Branch |
| **Hook runners** | Command + HTTP | Function only |
| **Memory backends** | Embedding + MMR | None |

---

*Kết luận: xVora có kiến trúc core vững chắc hơn (actor pattern, two-pass compaction, plugin system). Pi có điểm mạnh về typed telemetry và abstraction layers. Nên kết hợp cả 2.*
