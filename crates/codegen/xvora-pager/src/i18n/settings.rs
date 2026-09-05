//! Settings labels, descriptions, categories, enum displays, chrome.
use super::{Locale, locale};

pub fn category(en_label: &str) -> &'static str {
    let en = match en_label {
        "Appearance" => "Appearance",
        "Mouse" => "Mouse",
        "Editor & Input" => "Editor & Input",
        "Agent & Approval" => "Agent & Approval",
        "Privacy" => "Privacy",
        "Models" => "Models",
        "Session" => "Session",
        "Advanced" => "Advanced",
        _ => "Advanced", // unknown sections fall back
    };
    if locale() != Locale::Vi {
        return en;
    }
    match en_label {
        "Appearance" => "Giao diện",
        "Mouse" => "Chuột",
        "Editor & Input" => "Trình soạn & nhập",
        "Agent & Approval" => "Agent & duyệt",
        "Privacy" => "Riêng tư",
        "Models" => "Mô hình",
        "Session" => "Phiên",
        "Advanced" => "Nâng cao",
        _ => en,
    }
}

pub fn setting_label(key: &str, fallback: &'static str) -> &'static str {
    match (locale(), key) {
        (Locale::Vi, "compact_mode") => "Chế độ gọn",
        (Locale::En, "compact_mode") => "Compact mode",
        (Locale::Vi, "screen_mode") => "Chế độ màn hình mặc định",
        (Locale::En, "screen_mode") => "Default screen mode",
        (Locale::Vi, "show_timestamps") => "Hiện dấu thời gian",
        (Locale::En, "show_timestamps") => "Show timestamps",
        (Locale::Vi, "show_timeline") => "Thanh timeline",
        (Locale::En, "show_timeline") => "Timeline sidebar",
        (Locale::Vi, "simple_mode") => "Tắt vim ở ô nhập",
        (Locale::En, "simple_mode") => "Disable vim input mode",
        (Locale::Vi, "vim_mode") => "Điều hướng scrollback kiểu vim",
        (Locale::En, "vim_mode") => "Vim scrollback navigation",
        (Locale::Vi, "theme") => "Giao diện",
        (Locale::En, "theme") => "Theme",
        (Locale::Vi, "auto_dark_theme") => "Theme tối tự động",
        (Locale::En, "auto_dark_theme") => "Auto dark theme",
        (Locale::Vi, "auto_light_theme") => "Theme sáng tự động",
        (Locale::En, "auto_light_theme") => "Auto light theme",
        (Locale::Vi, "render_mermaid") => "Vẽ sơ đồ Mermaid",
        (Locale::En, "render_mermaid") => "Render Mermaid diagrams",
        (Locale::Vi, "permission_mode") => "Chế độ quyền",
        (Locale::En, "permission_mode") => "Permission mode",
        (Locale::Vi, "remember_tool_approvals") => "Ghi nhớ duyệt tool",
        (Locale::En, "remember_tool_approvals") => "Remember tool approvals",
        (Locale::Vi, "multiline_mode") => "Nhiều dòng",
        (Locale::En, "multiline_mode") => "Multiline",
        (Locale::Vi, "default_model") => "Mô hình mặc định",
        (Locale::En, "default_model") => "Default model",
        (Locale::Vi, "max_thoughts_width") => "Độ rộng suy nghĩ tối đa",
        (Locale::En, "max_thoughts_width") => "Max thoughts width",
        (Locale::Vi, "show_thinking_blocks") => "Hiện khối suy nghĩ",
        (Locale::En, "show_thinking_blocks") => "Show thinking blocks",
        (Locale::Vi, "prompt_suggestions") => "Gợi ý prompt",
        (Locale::En, "prompt_suggestions") => "Prompt suggestions",
        (Locale::Vi, "respect_manual_folds") => "Giữ thu gọn thủ công",
        (Locale::En, "respect_manual_folds") => "Respect manual folds",
        (Locale::Vi, "group_tool_verbs") => "Gộp lời gọi tool",
        (Locale::En, "group_tool_verbs") => "Group tool calls",
        (Locale::Vi, "collapsed_edit_blocks") => "Khối edit thu gọn",
        (Locale::En, "collapsed_edit_blocks") => "Collapsed edit blocks",
        (Locale::Vi, "display_refresh_auto_cadence") => "Khớp tần số làm tươi màn hình",
        (Locale::En, "display_refresh_auto_cadence") => "Match display refresh rate",
        (Locale::Vi, "scroll_speed") => "Tốc độ cuộn",
        (Locale::En, "scroll_speed") => "Scroll speed",
        (Locale::Vi, "scroll_mode") => "Kiểu cuộn",
        (Locale::En, "scroll_mode") => "Scroll input",
        (Locale::Vi, "scroll_lines") => "Số dòng mỗi lần cuộn",
        (Locale::En, "scroll_lines") => "Scroll lines",
        (Locale::Vi, "invert_scroll") => "Đảo chiều cuộn",
        (Locale::En, "invert_scroll") => "Invert scroll",
        (Locale::Vi, "keep_text_selection") => "Chọn văn bản",
        (Locale::En, "keep_text_selection") => "Text selection",
        (Locale::Vi, "coding_data_sharing") => "Chia sẻ dữ liệu coding",
        (Locale::En, "coding_data_sharing") => "Coding data sharing",
        (Locale::Vi, "default_selected_permission") => "Quyền chọn mặc định",
        (Locale::En, "default_selected_permission") => "Default selected permission",
        (Locale::Vi, "ask_user_question_timeout") => "Timeout Ask-Question",
        (Locale::En, "ask_user_question_timeout") => "Ask-Question timeout",
        (Locale::Vi, "plan_mode") => "Chế độ kế hoạch",
        (Locale::En, "plan_mode") => "Plan mode",
        (Locale::Vi, "show_tips") => "Hiện mẹo",
        (Locale::En, "show_tips") => "Show tips",
        (Locale::Vi, "contextual_hints") => "Hiện gợi ý ngữ cảnh",
        (Locale::En, "contextual_hints") => "Show contextual hints",
        (Locale::Vi, "auto_update") => "Tự cập nhật",
        (Locale::En, "auto_update") => "Auto-update",
        (Locale::Vi, "hunk_tracker_mode") => "Theo dõi hunk",
        (Locale::En, "hunk_tracker_mode") => "Hunk tracker",
        (Locale::Vi, "voice_capture_mode") => "Thu âm giọng",
        (Locale::En, "voice_capture_mode") => "Voice capture",
        (Locale::Vi, "voice_stt_language") => "Ngôn ngữ giọng nói",
        (Locale::En, "voice_stt_language") => "Voice language",
        (Locale::Vi, "fork_secondary_model") => "Mô hình phụ khi fork",
        (Locale::En, "fork_secondary_model") => "Fork secondary model",
        (Locale::Vi, "hint_undo") => "Hoàn tác",
        (Locale::En, "hint_undo") => "Undo",
        (Locale::Vi, "hint_plan_mode") => "Chế độ kế hoạch",
        (Locale::En, "hint_plan_mode") => "Plan mode",
        (Locale::Vi, "hint_image_input") => "Nhập ảnh",
        (Locale::En, "hint_image_input") => "Image input",
        (Locale::Vi, "hint_send_now") => "Gửi ngay",
        (Locale::En, "hint_send_now") => "Send now",
        (Locale::Vi, "hint_small_screen") => "Màn hình nhỏ",
        (Locale::En, "hint_small_screen") => "Small screen",
        (Locale::Vi, "hint_word_select") => "Chọn từ",
        (Locale::En, "hint_word_select") => "Word select",
        _ => fallback,
    }
}

pub fn setting_description(key: &str, fallback: &'static str) -> &'static str {
    match (locale(), key) {
        (Locale::Vi, "compact_mode") => {
            "Giảm khoảng trống quanh tin nhắn để hiển thị nhiều nội dung hơn. Tự bật khi terminal ≤ 20 hàng."
        }
        (Locale::En, "compact_mode") => {
            "Reduce padding around messages for more content density. Auto-enabled while the terminal is 20 rows or shorter."
        }
        (Locale::Vi, "screen_mode") => {
            "Cách xVora mở lần sau: Toàn màn hình (mặc định) hoặc Tối giản. Ghi [ui] screen_mode vào config.toml. Cần khởi động lại. Trong phiên dùng /minimal hoặc /fullscreen."
        }
        (Locale::En, "screen_mode") => {
            "How xVora opens next time: Fullscreen (default when unset) or Minimal. Writes [ui] screen_mode in config.toml. Restart required. Switch this session only with /minimal or /fullscreen."
        }
        (Locale::Vi, "show_timestamps") => "Hiện giờ cạnh tin nhắn người dùng và phản hồi agent.",
        (Locale::En, "show_timestamps") => {
            "Show clock time next to user messages and agent responses."
        }
        (Locale::Vi, "show_timeline") => {
            "Thanh tick theo lượt thay thanh cuộn: hover xem trước, click nhảy tới."
        }
        (Locale::En, "show_timeline") => {
            "Per-turn tick rail in place of the scrollbar: hover previews a turn, click jumps to it."
        }
        (Locale::Vi, "simple_mode") => "Ô nhập kiểu readline thay vì phím vim. Thử nghiệm.",
        (Locale::En, "simple_mode") => {
            "Use plain readline-style input instead of vim keys in the prompt. Experimental."
        }
        (Locale::Vi, "vim_mode") => {
            "Bật phím vim (h/j/k/l, gg/G, /) để điều hướng scrollback. Không ảnh hưởng ô nhập."
        }
        (Locale::En, "vim_mode") => {
            "Enable vim keys (h/j/k/l, gg/G, /) for navigating the scrollback. Does not affect the input prompt."
        }
        (Locale::Vi, "theme") => "Bảng màu giao diện pager.",
        (Locale::En, "theme") => "Color theme for the pager UI.",
        (Locale::Vi, "auto_dark_theme") => "Theme khi hệ thống ở chế độ tối (chỉ khi theme=auto).",
        (Locale::En, "auto_dark_theme") => {
            "Theme to use when the system is in dark mode (only with theme=auto)."
        }
        (Locale::Vi, "auto_light_theme") => {
            "Theme khi hệ thống ở chế độ sáng (chỉ khi theme=auto)."
        }
        (Locale::En, "auto_light_theme") => {
            "Theme to use when the system is in light mode (only with theme=auto)."
        }
        (Locale::Vi, "render_mermaid") => {
            "Cách hiện khối mermaid: auto/on thêm hàng bấm để mở sơ đồ; off hiện mã nguồn."
        }
        (Locale::En, "render_mermaid") => {
            "How mermaid code blocks are shown: auto/on add a clickable row to open the rendered diagram; off shows the raw source."
        }
        (Locale::Vi, "permission_mode") => {
            "Default = hành vi mặc định agent; Ask = hỏi mỗi tool; Auto = LLM duyệt tool an toàn; Always approve = tự duyệt tất cả."
        }
        (Locale::En, "permission_mode") => {
            "Default uses the agent's built-in behavior; Ask prompts for each tool action; Auto uses an LLM classifier for risky tools; Always approve grants all permissions automatically."
        }
        (Locale::Vi, "remember_tool_approvals") => {
            "Hiện tùy chọn Always allow trong hộp quyền. Áp dụng ask/auto; Always-approve vẫn bỏ qua. Cần khởi động lại."
        }
        (Locale::En, "remember_tool_approvals") => {
            "Show Always allow options in permission prompts so you can stop being re-asked about a specific command or tool. Applies in ask and auto; Always-approve still skips all prompts. Restart required."
        }
        (Locale::Vi, "multiline_mode") => {
            "Khi bật, Enter xuống dòng và Shift+Enter gửi. Reset mỗi phiên."
        }
        (Locale::En, "multiline_mode") => {
            "When on, Enter inserts a newline and Shift+Enter sends. Resets each session."
        }
        (Locale::Vi, "default_model") => {
            "Mô hình cho phiên mới. Đổi cũng chuyển phiên hiện tại. Chọn (no override) để xóa."
        }
        (Locale::En, "default_model") => {
            "Model used for new sessions. Changing this also switches the active session. Pick (no override) to clear."
        }
        (Locale::Vi, "max_thoughts_width") => {
            "Độ rộng panel suy nghĩ của agent (40-500, mặc định 120)."
        }
        (Locale::En, "max_thoughts_width") => {
            "Column width budget for the agent's thoughts panel (40-500, default 120)."
        }
        (Locale::Vi, "show_thinking_blocks") => "Hiện khối suy nghĩ/reasoning khi streaming.",
        (Locale::En, "show_thinking_blocks") => {
            "Show agent thinking/reasoning blocks in the scrollback while streaming."
        }
        (Locale::Vi, "prompt_suggestions") => {
            "Sau mỗi lượt, gợi ý prompt tiếp theo dạng ghost text (Tab để nhận). Gọi model nhỏ mỗi lượt."
        }
        (Locale::En, "prompt_suggestions") => {
            "After each turn, predict your likely next prompt and show it as ghost text in the input (Tab to accept). Uses a small model call per turn."
        }
        (Locale::Vi, "group_tool_verbs") => {
            "Gộp các tool read/search/list liên tiếp và subagent thành một hàng tóm tắt."
        }
        (Locale::En, "group_tool_verbs") => {
            "Fold consecutive read/search/list tool calls and subagent rows into one summary row; finished thoughts fold into the group too."
        }
        (Locale::Vi, "respect_manual_folds") => {
            "Giữ khối đã thu gọn thủ công khi streaming; dừng auto-scroll khi mở rộng. Thử nghiệm."
        }
        (Locale::En, "respect_manual_folds") => {
            "Keep manually folded blocks as-is while streaming and stop auto-scroll when expanding a block. Experimental."
        }
        (Locale::Vi, "collapsed_edit_blocks") => {
            "Hiện edit dạng +N/-M một dòng và gộp edit cùng file; mở rộng để xem diff."
        }
        (Locale::En, "collapsed_edit_blocks") => {
            "Show edits as one-line +N/-M diffstat summaries and merge back-to-back edits to the same file into one block; expand a row to see the diffs."
        }
        (Locale::Vi, "display_refresh_auto_cadence") => {
            "Màn hình tần số cao: TUI stream/cuộn nhanh hơn. Tắt giữ ~60 Hz. Cần khởi động lại."
        }
        (Locale::En, "display_refresh_auto_cadence") => {
            "On high-refresh displays, the TUI will stream/scroll faster to match the display. Off keeps the classic ~60 Hz cadence. Restart required."
        }
        (Locale::Vi, "scroll_speed") => {
            "Hệ số tốc độ cuộn chuột/trackpad (1-100). Cao hơn = nhanh hơn."
        }
        (Locale::En, "scroll_speed") => {
            "Mouse-wheel and trackpad scroll speed multiplier (1-100). Higher = faster."
        }
        (Locale::Vi, "scroll_mode") => "Ép kiểu cuộn wheel hoặc trackpad khi tự nhận sai thiết bị.",
        (Locale::En, "scroll_mode") => {
            "Force wheel or trackpad scroll behavior when auto-detection misreads your device."
        }
        (Locale::Vi, "scroll_lines") => {
            "Số dòng mỗi lần cuộn (1-10). Chưa đặt thì dùng profile terminal."
        }
        (Locale::En, "scroll_lines") => {
            "Lines per scroll tick for both wheel and trackpad (1-10). Until set, each terminal's own profile applies."
        }
        (Locale::Vi, "invert_scroll") => "Đảo chiều cuộn dọc (natural scrolling).",
        (Locale::En, "invert_scroll") => "Reverse vertical scroll direction (natural scrolling).",
        (Locale::Vi, "keep_text_selection") => {
            "Thời gian giữ vùng chọn trong app và double-click làm gì (thu gọn vs chọn & copy từ)."
        }
        (Locale::En, "keep_text_selection") => {
            "How long in-app selection stays on screen and what double-click does (fold vs. select & copy a word)."
        }
        (Locale::Vi, "coding_data_sharing") => {
            "Cho phép SpaceXAI lưu và huấn luyện trên dữ liệu phiên coding hay không."
        }
        (Locale::En, "coding_data_sharing") => {
            "Controls whether SpaceXAI may retain and train on coding session data."
        }
        (Locale::Vi, "default_selected_permission") => "Hàng con trỏ chọn sẵn trên hộp hỏi quyền.",
        (Locale::En, "default_selected_permission") => {
            "Which row the cursor preselects on permission prompts."
        }
        (Locale::Vi, "ask_user_question_timeout") => {
            "Khi bật, tool ask_user_question hết hạn sau một khoảng thay vì chờ mãi."
        }
        (Locale::En, "ask_user_question_timeout") => {
            "When on, the ask_user_question tool will time out after a set period instead of blocking forever."
        }
        (Locale::Vi, "plan_mode") => {
            "Khi bật, agent tóm tắt kế hoạch trước khi chạy tool hoặc sửa file."
        }
        (Locale::En, "plan_mode") => {
            "When on, the agent summarises a plan before running tools or making edits."
        }
        (Locale::Vi, "show_tips") => "Hiện banner mẹo ngày khi khởi động. Cần khởi động lại.",
        (Locale::En, "show_tips") => "Show the tip-of-the-day banner on startup. Restart required.",
        (Locale::Vi, "contextual_hints") => "Hiện gợi ý phím tắt theo ngữ cảnh; bật/tắt từng cái.",
        (Locale::En, "contextual_hints") => {
            "Show brief, in-context keyboard hints as you work; toggle each one individually."
        }
        (Locale::Vi, "auto_update") => {
            "Tự tải và cài cập nhật pager khi khởi động. Cần khởi động lại."
        }
        (Locale::En, "auto_update") => {
            "Automatically download and install pager updates on startup. Restart required."
        }
        (Locale::Vi, "hunk_tracker_mode") => {
            "File change nào agent theo dõi dạng hunk. Off tắt hẳn (kể cả LOC). Cần khởi động lại."
        }
        (Locale::En, "hunk_tracker_mode") => {
            "Which file changes the agent tracks as hunks. Off disables tracking (and LOC stats) entirely. Restart required."
        }
        (Locale::Vi, "voice_capture_mode") => {
            "Cách phím giọng (Ctrl+Space / F8) hoạt động: Bật/tắt hoặc Giữ để nói."
        }
        (Locale::En, "voice_capture_mode") => {
            "How the voice chord (Ctrl+Space / F8) behaves: Toggle or Hold to talk."
        }
        (Locale::Vi, "voice_stt_language") => {
            "Ngôn ngữ STT cho dictation. Mặc định tiếng Anh; System dùng locale máy khi hỗ trợ."
        }
        (Locale::En, "voice_stt_language") => {
            "Speech-to-text language for voice dictation. English by default; System uses your locale when supported."
        }
        (Locale::Vi, "fork_secondary_model") => {
            "Mô hình agent phụ khi fork. Chọn (no override) để xóa."
        }
        (Locale::En, "fork_secondary_model") => {
            "Model used for the secondary agent when forking. Pick (no override) to clear."
        }
        _ => fallback,
    }
}

pub fn enum_display(canonical: &str, fallback: &'static str) -> &'static str {
    match (locale(), canonical) {
        (Locale::Vi, "auto") => "Tự động",
        (Locale::En, "auto") => "Auto",
        (Locale::Vi, "groknight") => "xVora Night",
        (Locale::En, "groknight") => "xVora Night",
        (Locale::Vi, "grokday") => "xVora Day",
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
        (Locale::Vi, "Toggle") => "Bật/tắt",
        (Locale::En, "Toggle") => "Toggle",
        (Locale::Vi, "Hold to talk") => "Giữ để nói",
        (Locale::En, "Hold to talk") => "Hold to talk",
        (Locale::Vi, "System") => "Hệ thống",
        (Locale::En, "System") => "System",
        (Locale::Vi, "(no override)") => "(không ghi đè)",
        (Locale::En, "(no override)") => "(no override)",
        (Locale::Vi, "Flash after copy") => "Nháy sau copy",
        (Locale::En, "Flash after copy") => "Flash after copy",
        (Locale::Vi, "Hold until dismissed") => "Giữ đến khi bỏ",
        (Locale::En, "Hold until dismissed") => "Hold until dismissed",
        (Locale::Vi, "Word select (terminal-like)") => "Chọn từ (kiểu terminal)",
        (Locale::En, "Word select (terminal-like)") => "Word select (terminal-like)",
        (Locale::Vi, "Agent only") => "Chỉ agent",
        (Locale::En, "Agent only") => "Agent only",
        (Locale::Vi, "All dirty") => "Mọi file dirty",
        (Locale::En, "All dirty") => "All dirty",
        (Locale::Vi, "Auto-detect") => "Tự nhận",
        (Locale::En, "Auto-detect") => "Auto-detect",
        (Locale::Vi, "Mouse wheel") => "Bánh xe chuột",
        (Locale::En, "Mouse wheel") => "Mouse wheel",
        (Locale::Vi, "Trackpad") => "Trackpad",
        (Locale::En, "Trackpad") => "Trackpad",
        _ => fallback,
    }
}

pub fn chrome(key: &str) -> &'static str {
    match (locale(), key) {
        (Locale::Vi, "settings.title") => "Cài đặt",
        (Locale::En, "settings.title") => "Settings",
        (Locale::Vi, "settings.tip.long") => {
            "Mẹo · Hỏi xVora: \"đổi theme sang grokday\" hoặc \"compact mode làm gì?\""
        }
        (Locale::En, "settings.tip.long") => {
            "Tip · Ask xVora: \"change theme to grokday\" or \"what does compact mode do?\""
        }
        (Locale::Vi, "settings.tip.short") => "Mẹo · Hỏi xVora để đổi một cài đặt",
        (Locale::En, "settings.tip.short") => "Tip · Ask xVora to change a setting",
        (Locale::Vi, "settings.value.on") => "bật",
        (Locale::En, "settings.value.on") => "on",
        (Locale::Vi, "settings.value.off") => "tắt",
        (Locale::En, "settings.value.off") => "off",
        (Locale::Vi, "settings.pill.restart") => "· khởi động lại",
        (Locale::En, "settings.pill.restart") => "· restart",
        (Locale::Vi, "settings.preview") => "xem trước",
        (Locale::En, "settings.preview") => "preview",
        (Locale::Vi, "settings.no_matches") => "Không khớp",
        (Locale::En, "settings.no_matches") => "No matches for",
        (Locale::Vi, "settings.no_override") => "(không ghi đè)",
        (Locale::En, "settings.no_override") => "(no override)",
        (Locale::Vi, "shortcuts.press_again") => "nhấn lại để",
        (Locale::En, "shortcuts.press_again") => "press again to",
        (Locale::Vi, "shortcuts.help.title") => "Phím tắt",
        (Locale::En, "shortcuts.help.title") => "Keyboard Shortcuts",
        (Locale::Vi, "shortcuts.cat.essentials") => "Thiết yếu",
        (Locale::En, "shortcuts.cat.essentials") => "Essentials",
        (Locale::Vi, "shortcuts.cat.input") => "Nhập liệu",
        (Locale::En, "shortcuts.cat.input") => "Input",
        (Locale::Vi, "shortcuts.cat.conversation_nav") => "Điều hướng hội thoại",
        (Locale::En, "shortcuts.cat.conversation_nav") => "Conversation Navigation",
        (Locale::Vi, "shortcuts.cat.conversation_action") => "Thao tác hội thoại",
        (Locale::En, "shortcuts.cat.conversation_action") => "Conversation Actions",
        (Locale::Vi, "shortcuts.cat.panels") => "Panel",
        (Locale::En, "shortcuts.cat.panels") => "Panels",
        (Locale::Vi, "shortcuts.cat.session") => "Phiên",
        (Locale::En, "shortcuts.cat.session") => "Session",
        (Locale::Vi, "shortcuts.cat.dashboard") => "Dashboard",
        (Locale::En, "shortcuts.cat.dashboard") => "Dashboard",
        _ => "",
    }
}
