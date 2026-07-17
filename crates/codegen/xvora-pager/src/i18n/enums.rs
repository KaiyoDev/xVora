//! Full enum choice display + description i18n.
use super::{Locale, locale};

pub fn display(canonical: &str, fallback: &'static str) -> &'static str {
    match (locale(), canonical) {
        (Locale::Vi, "auto") => "Tự động",
        (Locale::En, "auto") => "Auto",
        (Locale::Vi, "en") => "English",
        (Locale::En, "en") => "English",
        (Locale::Vi, "vi") => "Tiếng Việt",
        (Locale::En, "vi") => "Tiếng Việt",
        (Locale::Vi, "xvoranight") => "xVora Night",

        (Locale::Vi, "groknight") => "xVora Night",
        (Locale::En, "xvoranight") => "xVora Night",

        (Locale::En, "groknight") => "xVora Night",
        (Locale::Vi, "xvoraday") => "xVora Day",

        (Locale::Vi, "grokday") => "xVora Day",
        (Locale::En, "xvoraday") => "xVora Day",

        (Locale::En, "grokday") => "xVora Day",
        (Locale::Vi, "tokyonight") => "Tokyo Night",
        (Locale::En, "tokyonight") => "Tokyo Night",
        (Locale::Vi, "rosepine-moon") => "Rose Pine Moon",
        (Locale::En, "rosepine-moon") => "Rose Pine Moon",
        (Locale::Vi, "oscura-midnight") => "Oscura Midnight",
        (Locale::En, "oscura-midnight") => "Oscura Midnight",
        (Locale::Vi, "default") => "Mặc định",
        (Locale::En, "default") => "Default",
        (Locale::Vi, "ask") => "Hỏi",
        (Locale::En, "ask") => "Ask",
        (Locale::Vi, "always-approve") => "Luôn duyệt",
        (Locale::En, "always-approve") => "Always approve",
        (Locale::Vi, "opt-in") => "Cho phép",
        (Locale::En, "opt-in") => "Opt in",
        (Locale::Vi, "opt-out") => "Từ chối",
        (Locale::En, "opt-out") => "Opt out",
        (Locale::Vi, "fullscreen") => "Toàn màn hình",
        (Locale::En, "fullscreen") => "Fullscreen",
        (Locale::Vi, "minimal") => "Tối giản",
        (Locale::En, "minimal") => "Minimal",
        (Locale::Vi, "on") => "Bật",
        (Locale::En, "on") => "On",
        (Locale::Vi, "off") => "Tắt",
        (Locale::En, "off") => "Off",
        (Locale::Vi, "agent-only") => "Chỉ agent",
        (Locale::En, "agent-only") => "Agent only",
        (Locale::Vi, "all-dirty") => "Mọi file dirty",
        (Locale::En, "all-dirty") => "All dirty",
        (Locale::Vi, "wheel") => "Bánh xe chuột",
        (Locale::En, "wheel") => "Mouse wheel",
        (Locale::Vi, "trackpad") => "Trackpad",
        (Locale::En, "trackpad") => "Trackpad",
        (Locale::Vi, "auto-detect") => "Tự nhận",
        (Locale::En, "auto-detect") => "Auto-detect",
        (Locale::Vi, "flash") => "Nháy sau copy",
        (Locale::En, "flash") => "Flash after copy",
        (Locale::Vi, "hold") => "Giữ đến khi bỏ",
        (Locale::En, "hold") => "Hold until dismissed",
        (Locale::Vi, "word") => "Chọn từ (kiểu terminal)",
        (Locale::En, "word") => "Word select (terminal-like)",
        (Locale::Vi, "toggle") => "Bật/tắt",
        (Locale::En, "toggle") => "Toggle",
        (Locale::Vi, "hold-to-talk") => "Giữ để nói",
        (Locale::En, "hold-to-talk") => "Hold to talk",
        (Locale::Vi, "system") => "Hệ thống",
        (Locale::En, "system") => "System",
        _ => fallback,
    }
}

pub fn description(canonical: &str, fallback: &'static str) -> &'static str {
    // Disambiguate shared canonicals (auto/on/off) via English source text.
    if matches!(canonical, "auto" | "on" | "off") {
        return description_by_en_fallback(fallback);
    }
    match (locale(), canonical) {
        (Locale::Vi, "xvoranight") => "Tối trung tính, accent magenta.",

        (Locale::Vi, "groknight") => "Tối trung tính, accent magenta.",
        (Locale::En, "xvoranight") => "Neutral dark with magenta accent.",

        (Locale::En, "groknight") => "Neutral dark with magenta accent.",
        (Locale::Vi, "xvoraday") => "Theme sáng cho môi trường nhiều ánh sáng.",

        (Locale::Vi, "grokday") => "Theme sáng cho môi trường nhiều ánh sáng.",
        (Locale::En, "xvoraday") => "Light theme for bright environments.",

        (Locale::En, "grokday") => "Light theme for bright environments.",
        (Locale::Vi, "tokyonight") => "Tối xanh dương; cần truecolor.",
        (Locale::En, "tokyonight") => "Dark + blue-tinted; needs truecolor.",
        (Locale::Vi, "rosepine-moon") => "Tối dịu accent mauve; cần truecolor.",
        (Locale::En, "rosepine-moon") => "Muted dark with mauve accents; needs truecolor.",
        (Locale::Vi, "oscura-midnight") => "Tối sâu accent ấm; cần truecolor.",
        (Locale::En, "oscura-midnight") => "Deep dark with warm accents; needs truecolor.",
        (Locale::Vi, "default") => "Dùng hành vi quyền mặc định của agent (hiện tương đương Hỏi).",
        (Locale::En, "default") => "Use the agent's default permission behavior (currently equivalent to Ask).",
        (Locale::Vi, "ask") => "Hỏi quyền trước mỗi thao tác tool.",
        (Locale::En, "ask") => "Prompt for permission before tool actions.",
        (Locale::Vi, "always-approve") => "Tự duyệt mọi tool. Bỏ qua mọi hộp hỏi quyền.",
        (Locale::En, "always-approve") => "Auto-approve every tool action. Skips ALL permission prompts.",
        (Locale::Vi, "opt-in") => "Cho phép SpaceXAI lưu và dùng dữ liệu phiên coding để huấn luyện và cải thiện sản phẩm.",
        (Locale::En, "opt-in") => "Allow SpaceXAI to retain and use coding session data for training and product improvement.",
        (Locale::Vi, "opt-out") => "Không lưu dữ liệu phiên coding. Code không dùng để huấn luyện.",
        (Locale::En, "opt-out") => "Do not retain coding session data. Code requests will not be used for training.",
        (Locale::Vi, "fullscreen") => "Mở xVora ở TUI toàn màn hình chuẩn. Mặc định khi chưa đặt.",
        (Locale::En, "fullscreen") => "Open xVora in the standard fullscreen TUI. Default when unset.",
        (Locale::Vi, "minimal") => "Mở xVora ở chế độ tối giản (scrollback-native).",
        (Locale::En, "minimal") => "Open xVora in scrollback-native (minimal) mode.",
        (Locale::Vi, "agent-only") => "Chỉ theo dõi file agent sửa (mặc định).",
        (Locale::En, "agent-only") => "Track only files the agent edits (default).",
        (Locale::Vi, "all-dirty") => "Theo dõi mọi file git dirty, kể cả sửa ngoài.",
        (Locale::En, "all-dirty") => "Track every git-dirty file, including external edits.",
        (Locale::Vi, "wheel") => "Luôn coi cuộn là bánh xe (số dòng cố định mỗi tick).",
        (Locale::En, "wheel") => "Always treat scrolling as wheel notches (fixed lines per tick).",
        (Locale::Vi, "trackpad") => "Luôn coi cuộn là trackpad (tích lũy phân số).",
        (Locale::En, "trackpad") => "Always treat scrolling as a trackpad (fractional accumulation).",
        (Locale::Vi, "auto-detect") => "Tự nhận wheel vs trackpad theo timing gesture. Mặc định.",
        (Locale::En, "auto-detect") => "Detect wheel vs trackpad per gesture from event timing. Default.",
        (Locale::Vi, "flash") => "Nháy ngắn khi nhả chuột, rồi xóa. Double-click thu/mở. Mặc định.",
        (Locale::En, "flash") => "Brief highlight on mouse-up, then clear. Double-click toggles fold. Default.",
        (Locale::Vi, "hold") => "Giữ vùng chọn đến Esc, click, hoặc cuộn. Double-click thu/mở.",
        (Locale::En, "hold") => "Keep the selection visible until Esc, click, or scroll. Double-click toggles fold.",
        (Locale::Vi, "word") => "Double-click chọn & copy từ, triple-click cả dòng; giữ đến khi bỏ.",
        (Locale::En, "word") => "Double-click selects & copies a word, triple-click a line; selection stays until dismissed.",
        (Locale::Vi, "toggle") => "Ctrl+Space / F8 bắt đầu dictation; nhấn lại (hoặc Esc/Enter) để dừng.",
        (Locale::En, "toggle") => "Ctrl+Space / F8 starts dictation; press again (or Esc/Enter) to stop.",
        (Locale::Vi, "hold-to-talk") => "Giữ Ctrl+Space / F8 để ghi, thả để dừng. Cần terminal Kitty-protocol.",
        (Locale::En, "hold-to-talk") => "Hold Ctrl+Space / F8 to record, release to stop. Needs a Kitty-protocol terminal.",
        (Locale::Vi, "system") => "Dùng locale hệ thống nếu STT hỗ trợ; không thì tiếng Anh.",
        (Locale::En, "system") => "Use the system locale when it is a supported STT language; otherwise English.",
        _ => fallback,
    }
}

fn description_by_en_fallback(fallback: &'static str) -> &'static str {
    if locale() == Locale::En {
        return fallback;
    }
    // Vietnamese: pick by English source prefix / contains.
    if fallback.starts_with("Follow system") {
        return "Theo giao diện tối/sáng của hệ thống.";
    }
    if fallback.contains("LLM classifier") || fallback.contains("classifier approves") {
        return "LLM duyệt tool an toàn; hành động nguy hiểm vẫn có thể hỏi hoặc từ chối.";
    }
    if fallback.contains("clickable") || fallback.contains("Mermaid") || fallback.contains("mermaid")
    {
        if fallback.contains("raw") {
            return "Luôn hiện mã nguồn Mermaid dạng code block.";
        }
        if fallback.contains("always show the clickable") {
            return "Luôn hiện hàng bấm để mở/copy sơ đồ đã render.";
        }
        return "Hiện sơ đồ kèm hàng bấm để mở/copy ảnh render.";
    }
    if fallback.contains("summarises a plan") || fallback.contains("summarizes a plan") {
        return "Agent tóm tắt kế hoạch và xin duyệt trước khi chạy tool.";
    }
    if fallback.contains("runs tools and edits") {
        return "Agent chạy tool và sửa file trực tiếp (mặc định).";
    }
    if fallback.contains("Disable hunk") || fallback.contains("hunk tracking entirely") {
        return "Tắt theo dõi hunk (và LOC) hoàn toàn.";
    }
    fallback
}

