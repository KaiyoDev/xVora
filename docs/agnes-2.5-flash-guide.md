# Hướng Dẫn Sử Dụng Model Agnes-2.5-Flash

**Tham chiếu:** https://www.agnes-ai.com/en/docs/agnes-25-flash

---

## Tổng Quan

| Thuộc tính | Giá trị |
|---|---|
| **Model ID** | `agnes-2.5-flash` |
| **Context Window** | 512K tokens |
| **Max Output** | 65.5K tokens |
| **Base URL** | `https://apihub.agnes-ai.com/v1` |
| **Compatible Formats** | OpenAI Chat Completions, Anthropic Messages, OpenAI Responses API |
| **Pricing** | Miễn phí (ở thời điểm hiện tại) |

### Thay thế từ phiên bản cũ
- kế thừa API từ `agnes-2.0-flash`
- Chỉ đổi `model` name là đủ

---

## Authentication

**OpenAI format:**
```
Authorization: Bearer YOUR_API_KEY
```

**Anthropic format:**
```
x-api-key: YOUR_API_KEY
anthropic-version: 2023-06-01
```

---

## 1. Enable Thinking Mode — Điểm Khác Biệt Lớn

Agnes-2.5-Flash **không** dùng `reasoning_effort` như xAI/models khác. Thay vào đó dùng `chat_template_kwargs` hoặc `thinking` field riêng.

### Format OpenAI-compatible

```json
{
  "model": "agnes-2.5-flash",
  "messages": [{"role": "user", "content": "Giải phương trình bậc 2"}],
  "chat_template_kwargs": {
    "enable_thinking": true
  }
}
```

### Format Anthropic-compatible

```json
{
  "model": "agnes-2.5-flash",
  "messages": [{"role": "user", "content": "Giải phương trình bậc 2"}],
  "thinking": {
    "type": "enabled",
    "budget_tokens": 2048
  }
}
```

### Budget Tokens Recommender

| Kịch bản | `budget_tokens` |
|---|---|
| Chat đơn giản | 512 |
| Coding task vừa | 1024 |
| Debugging phức tạp | 2048 (mặc định) |
| Refactoring large codebase | 4096+ |
| Multi-step agent workflow | 8192 |

> `budget_tokens` = số token tối đa model được phép dùng cho phần thinking. Không giới hạn output token.

---

## 2. Image Input — Cách Đưa Ảnh Vào Prompt

Agnes-2.5-Flash nhận ảnh qua **public URL** trong `content` array.

### curl example

```bash
curl https://apihub.agnes-ai.com/v1/chat/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "agnes-2.5-flash",
    "messages": [{
      "role": "user",
      "content": [
        {"type": "text", "text": "Describe this UI screenshot and identify any UX issues."},
        {"type": "image_url", "image_url": {"url": "https://example.com/screenshot.png"}}
      ]
    }]
  }'
```

### Response với thinking enabled

```json
{
  "model": "agnes-2.5-flash",
  "messages": [{
    "role": "user",
    "content": [
      {"type": "text", "text": "Tóm tắt hình ảnh này và đề xuất cải thiện"}
    ]
  }],
  "chat_template_kwargs": {"enable_thinking": true}
}
```

Response sẽ chứa cả `thinking` block + `content` block.

---

## 3. Tool Calling

Same schema như OpenAI tool calling chuẩn.

```bash
curl https://apihub.agnes-ai.com/v1/chat/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "agnes-2.5-flash",
    "messages": [{"role": "user", "content": "Thời tiết Hà Nội hôm nay?"}],
    "tools": [{
      "type": "function",
      "function": {
        "name": "get_weather",
        "description": "Lấy thời tiết theo thành phố",
        "parameters": {
          "type": "object",
          "properties": {
            "city": {"type": "string", "description": "Tên thành phố"}
          },
          "required": ["city"]
        }
      }
    }],
    "tool_choice": "auto"
  }'
```

---

## 4. Streaming Response

```bash
curl https://apihub.agnes-ai.com/v1/chat/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "agnes-2.5-flash",
    "messages": [{"role": "user", "content": "Giải thích reactive programming"}],
    "stream": true,
    "chat_template_kwargs": {"enable_thinking": true}
  }'
```

Response stream sẽ emit `thinking` chunk trước, rồi `content` chunk.

---

## 5. So Sánh Với Các Model Khác

| Feature | Agnes-2.5-Flash | xAI (deepseek-v4) | Claude |
|---|---|---|---|
| **Thinking param** | `chat_template_kwargs.enable_thinking` / `thinking.type` | `reasoning_effort` enum | `thinking.budget_tokens` |
| **Thinking control** | Bật/tắt hoặc budget | 7 levels (None→Max) | Budget tokens |
| **Image input** | `image_url` public URL | `image_url` public URL | `image_url` public URL |
| **Context window** | 512K | 200K | 200K |
| **Max output** | 65.5K | 32K | 32K |
| **Anthropic format** | ✓ | ✗ | ✓ |
| **OpenAI format** | ✓ | ✓ | ✗ |

### Điểm Khác Biệt Chính

**1. Thinking model:**
- xAI: `reasoning_effort: "medium"` — discrete level
- Agnes: `enable_thinking: true` — toggle đơn giản, hoặc `budget_tokens` — granular control
- Claude: `thinking.budget_tokens: 2048` — tương tự Agnes

**2. API兼容:**
- Agnes support cả 3 formats: OpenAI Chat, OpenAI Responses, Anthropic Messages
- Giúp switch model dễ dàng mà không đổi code (chỉ đổi `model` string)

---

## 6. Tích Hợp Vào xVora

Nếu muốn dùng Agnes-2.5-Flash làm sampling model trong xVora:

```rust
// crates/codegen/xvora-sampling-types/src/types.rs
pub struct SamplingConfig {
    pub model_id: String,                    // "agnes-2.5-flash"
    pub reasoning_effort: Option<ReasoningEffort>,  // KHÔNG dùng (Agnes không support)
    // Thêm:
    pub agnes_thinking: Option<AgnesThinkingConfig>,
}

pub struct AgnesThinkingConfig {
    pub enable: bool,
    pub budget_tokens: u32,  // default 2048
}
```

```rust
// Khi build request:
fn build_request(&self, config: &SamplingConfig) -> Request {
    if config.model_id == "agnes-2.5-flash" {
        // Anthropic format
        Request {
            model: "agnes-2.5-flash".to_string(),
            messages: self.messages.clone(),
            extra_body: json!({
                "thinking": {
                    "type": "enabled",
                    "budget_tokens": config.agnes_thinking
                        .as_ref()
                        .map(|t| t.budget_tokens)
                        .unwrap_or(2048)
                }
            }),
            ..Default::default()
        }
    } else {
        // Default xAI format
        // ...
    }
}
```

---

## 7. Error Handling

| Status Code | Meaning | Action |
|---|---|---|
| `401` | Invalid API key | Check auth header |
| `400` | Bad request (invalid param) | Check model/thinking format |
| `429` | Rate limited | Exponential backoff |
| `500` | Server error | Retry once |
| `503` | Model overloaded | Wait and retry |

### Thinking-specific Errors

```json
{
  "error": {
    "type": "invalid_request_error",
    "message": "thinking.budget_tokens must be between 256 and 32768"
  }
}
```

---

## 8. Quick Reference Card

```bash
# Basic chat with thinking
curl https://apihub.agnes-ai.com/v1/chat/completions \
  -H "Authorization: Bearer $AGNES_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "agnes-2.5-flash",
    "messages": [{"role": "user", "content": "Hello"}],
    "chat_template_kwargs": {"enable_thinking": true}
  }'

# With image
curl https://apihub.agnes-ai.com/v1/chat/completions \
  -H "Authorization: Bearer $AGNES_API_KEY" \
  -d '{
    "model": "agnes-2.5-flash",
    "messages": [{
      "role": "user",
      "content": [
        {"type": "text", "text": "What is in this image?"},
        {"type": "image_url", "image_url": {"url": "https://imgur.com/abc123.png"}}
      ]
    }],
    "thinking": {"type": "enabled", "budget_tokens": 1024}
  }'
```

---

## Nguồn Tham Khảo

- Tài liệu chính thức: https://www.agnes-ai.com/en/docs/agnes-25-flash
- API Hub: https://apihub.agnes-ai.com
- Pricing: Miễn phí (thời điểm 2026-09)
