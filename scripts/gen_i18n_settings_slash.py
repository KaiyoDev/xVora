#!/usr/bin/env python3
"""Generate settings.rs and slash.rs i18n modules."""
from pathlib import Path

ROOT = Path("crates/codegen/xvora-pager/src/i18n")


def esc(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


# key -> (en, vi)
SETTINGS_LABELS = {
    "compact_mode": ("Compact mode", "Chế độ gọn"),
    "screen_mode": ("Default screen mode", "Chế độ màn hình mặc định"),
    "show_timestamps": ("Show timestamps", "Hiện dấu thời gian"),
    "show_timeline": ("Timeline sidebar", "Thanh timeline"),
    "simple_mode": ("Disable vim input mode", "Tắt vim ở ô nhập"),
    "vim_mode": ("Vim scrollback navigation", "Điều hướng scrollback kiểu vim"),
    "theme": ("Theme", "Giao diện"),
    "auto_dark_theme": ("Auto dark theme", "Theme tối tự động"),
    "auto_light_theme": ("Auto light theme", "Theme sáng tự động"),
    "render_mermaid": ("Render Mermaid diagrams", "Vẽ sơ đồ Mermaid"),
    "permission_mode": ("Permission mode", "Chế độ quyền"),
    "remember_tool_approvals": ("Remember tool approvals", "Ghi nhớ duyệt tool"),
    "multiline_mode": ("Multiline", "Nhiều dòng"),
    "default_model": ("Default model", "Mô hình mặc định"),
    "max_thoughts_width": ("Max thoughts width", "Độ rộng suy nghĩ tối đa"),
    "show_thinking_blocks": ("Show thinking blocks", "Hiện khối suy nghĩ"),
    "prompt_suggestions": ("Prompt suggestions", "Gợi ý prompt"),
    "respect_manual_folds": ("Respect manual folds", "Giữ thu gọn thủ công"),
    "group_tool_verbs": ("Group tool calls", "Gộp lời gọi tool"),
    "collapsed_edit_blocks": ("Collapsed edit blocks", "Khối edit thu gọn"),
    "display_refresh_auto_cadence": ("Match display refresh rate", "Khớp tần số làm tươi màn hình"),
    "scroll_speed": ("Scroll speed", "Tốc độ cuộn"),
    "scroll_mode": ("Scroll input", "Kiểu cuộn"),
    "scroll_lines": ("Scroll lines", "Số dòng mỗi lần cuộn"),
    "invert_scroll": ("Invert scroll", "Đảo chiều cuộn"),
    "keep_text_selection": ("Text selection", "Chọn văn bản"),
    "coding_data_sharing": ("Coding data sharing", "Chia sẻ dữ liệu coding"),
    "default_selected_permission": ("Default selected permission", "Quyền chọn mặc định"),
    "ask_user_question_timeout": ("Ask-Question timeout", "Timeout Ask-Question"),
    "plan_mode": ("Plan mode", "Chế độ kế hoạch"),
    "show_tips": ("Show tips", "Hiện mẹo"),
    "contextual_hints": ("Show contextual hints", "Hiện gợi ý ngữ cảnh"),
    "auto_update": ("Auto-update", "Tự cập nhật"),
    "hunk_tracker_mode": ("Hunk tracker", "Theo dõi hunk"),
    "voice_capture_mode": ("Voice capture", "Thu âm giọng"),
    "voice_stt_language": ("Voice language", "Ngôn ngữ giọng nói"),
    "fork_secondary_model": ("Fork secondary model", "Mô hình phụ khi fork"),
    "hint_undo": ("Undo", "Hoàn tác"),
    "hint_plan_mode": ("Plan mode", "Chế độ kế hoạch"),
    "hint_image_input": ("Image input", "Nhập ảnh"),
    "hint_send_now": ("Send now", "Gửi ngay"),
    "hint_small_screen": ("Small screen", "Màn hình nhỏ"),
    "hint_word_select": ("Word select", "Chọn từ"),
}

SETTINGS_DESCS = {
    "compact_mode": (
        "Reduce padding around messages for more content density. Auto-enabled while the terminal is 20 rows or shorter.",
        "Giảm khoảng trống quanh tin nhắn để hiển thị nhiều nội dung hơn. Tự bật khi terminal ≤ 20 hàng.",
    ),
    "screen_mode": (
        "How xVora opens next time: Fullscreen (default when unset) or Minimal. Writes [ui] screen_mode in config.toml. Restart required. Switch this session only with /minimal or /fullscreen.",
        "Cách xVora mở lần sau: Toàn màn hình (mặc định) hoặc Tối giản. Ghi [ui] screen_mode vào config.toml. Cần khởi động lại. Trong phiên dùng /minimal hoặc /fullscreen.",
    ),
    "show_timestamps": (
        "Show clock time next to user messages and agent responses.",
        "Hiện giờ cạnh tin nhắn người dùng và phản hồi agent.",
    ),
    "show_timeline": (
        "Per-turn tick rail in place of the scrollbar: hover previews a turn, click jumps to it.",
        "Thanh tick theo lượt thay thanh cuộn: hover xem trước, click nhảy tới.",
    ),
    "simple_mode": (
        "Use plain readline-style input instead of vim keys in the prompt. Experimental.",
        "Ô nhập kiểu readline thay vì phím vim. Thử nghiệm.",
    ),
    "vim_mode": (
        "Enable vim keys (h/j/k/l, gg/G, /) for navigating the scrollback. Does not affect the input prompt.",
        "Bật phím vim (h/j/k/l, gg/G, /) để điều hướng scrollback. Không ảnh hưởng ô nhập.",
    ),
    "theme": ("Color theme for the pager UI.", "Bảng màu giao diện pager."),
    "auto_dark_theme": (
        "Theme to use when the system is in dark mode (only with theme=auto).",
        "Theme khi hệ thống ở chế độ tối (chỉ khi theme=auto).",
    ),
    "auto_light_theme": (
        "Theme to use when the system is in light mode (only with theme=auto).",
        "Theme khi hệ thống ở chế độ sáng (chỉ khi theme=auto).",
    ),
    "render_mermaid": (
        "How mermaid code blocks are shown: auto/on add a clickable row to open the rendered diagram; off shows the raw source.",
        "Cách hiện khối mermaid: auto/on thêm hàng bấm để mở sơ đồ; off hiện mã nguồn.",
    ),
    "permission_mode": (
        "Default uses the agent's built-in behavior; Ask prompts for each tool action; Auto uses an LLM classifier for risky tools; Always approve grants all permissions automatically.",
        "Default = hành vi mặc định agent; Ask = hỏi mỗi tool; Auto = LLM duyệt tool an toàn; Always approve = tự duyệt tất cả.",
    ),
    "remember_tool_approvals": (
        "Show Always allow options in permission prompts so you can stop being re-asked about a specific command or tool. Applies in ask and auto; Always-approve still skips all prompts. Restart required.",
        "Hiện tùy chọn Always allow trong hộp quyền. Áp dụng ask/auto; Always-approve vẫn bỏ qua. Cần khởi động lại.",
    ),
    "multiline_mode": (
        "When on, Enter inserts a newline and Shift+Enter sends. Resets each session.",
        "Khi bật, Enter xuống dòng và Shift+Enter gửi. Reset mỗi phiên.",
    ),
    "default_model": (
        "Model used for new sessions. Changing this also switches the active session. Pick (no override) to clear.",
        "Mô hình cho phiên mới. Đổi cũng chuyển phiên hiện tại. Chọn (no override) để xóa.",
    ),
    "max_thoughts_width": (
        "Column width budget for the agent's thoughts panel (40-500, default 120).",
        "Độ rộng panel suy nghĩ của agent (40-500, mặc định 120).",
    ),
    "show_thinking_blocks": (
        "Show agent thinking/reasoning blocks in the scrollback while streaming.",
        "Hiện khối suy nghĩ/reasoning khi streaming.",
    ),
    "prompt_suggestions": (
        "After each turn, predict your likely next prompt and show it as ghost text in the input (Tab to accept). Uses a small model call per turn.",
        "Sau mỗi lượt, gợi ý prompt tiếp theo dạng ghost text (Tab để nhận). Gọi model nhỏ mỗi lượt.",
    ),
    "permission_mode_short": ("", ""),  # placeholder skip
    "group_tool_verbs": (
        "Fold consecutive read/search/list tool calls and subagent rows into one summary row; finished thoughts fold into the group too.",
        "Gộp các tool read/search/list liên tiếp và subagent thành một hàng tóm tắt.",
    ),
    "respect_manual_folds": (
        "Keep manually folded blocks as-is while streaming and stop auto-scroll when expanding a block. Experimental.",
        "Giữ khối đã thu gọn thủ công khi streaming; dừng auto-scroll khi mở rộng. Thử nghiệm.",
    ),
    "collapsed_edit_blocks": (
        "Show edits as one-line +N/-M diffstat summaries and merge back-to-back edits to the same file into one block; expand a row to see the diffs.",
        "Hiện edit dạng +N/-M một dòng và gộp edit cùng file; mở rộng để xem diff.",
    ),
    "display_refresh_auto_cadence": (
        "On high-refresh displays, the TUI will stream/scroll faster to match the display. Off keeps the classic ~60 Hz cadence. Restart required.",
        "Màn hình tần số cao: TUI stream/cuộn nhanh hơn. Tắt giữ ~60 Hz. Cần khởi động lại.",
    ),
    "scroll_speed": (
        "Mouse-wheel and trackpad scroll speed multiplier (1-100). Higher = faster.",
        "Hệ số tốc độ cuộn chuột/trackpad (1-100). Cao hơn = nhanh hơn.",
    ),
    "scroll_mode": (
        "Force wheel or trackpad scroll behavior when auto-detection misreads your device.",
        "Ép kiểu cuộn wheel hoặc trackpad khi tự nhận sai thiết bị.",
    ),
    "scroll_lines": (
        "Lines per scroll tick for both wheel and trackpad (1-10). Until set, each terminal's own profile applies.",
        "Số dòng mỗi lần cuộn (1-10). Chưa đặt thì dùng profile terminal.",
    ),
    "invert_scroll": (
        "Reverse vertical scroll direction (natural scrolling).",
        "Đảo chiều cuộn dọc (natural scrolling).",
    ),
    "keep_text_selection": (
        "How long in-app selection stays on screen and what double-click does (fold vs. select & copy a word).",
        "Thời gian giữ vùng chọn trong app và double-click làm gì (thu gọn vs chọn & copy từ).",
    ),
    "coding_data_sharing": (
        "Controls whether SpaceXAI may retain and train on coding session data.",
        "Cho phép SpaceXAI lưu và huấn luyện trên dữ liệu phiên coding hay không.",
    ),
    "default_selected_permission": (
        "Which row the cursor preselects on permission prompts.",
        "Hàng con trỏ chọn sẵn trên hộp hỏi quyền.",
    ),
    "ask_user_question_timeout": (
        "When on, the ask_user_question tool will time out after a set period instead of blocking forever.",
        "Khi bật, tool ask_user_question hết hạn sau một khoảng thay vì chờ mãi.",
    ),
    "plan_mode": (
        "When on, the agent summarises a plan before running tools or making edits.",
        "Khi bật, agent tóm tắt kế hoạch trước khi chạy tool hoặc sửa file.",
    ),
    "show_tips": (
        "Show the tip-of-the-day banner on startup. Restart required.",
        "Hiện banner mẹo ngày khi khởi động. Cần khởi động lại.",
    ),
    "contextual_hints": (
        "Show brief, in-context keyboard hints as you work; toggle each one individually.",
        "Hiện gợi ý phím tắt theo ngữ cảnh; bật/tắt từng cái.",
    ),
    "auto_update": (
        "Automatically download and install pager updates on startup. Restart required.",
        "Tự tải và cài cập nhật pager khi khởi động. Cần khởi động lại.",
    ),
    "hunk_tracker_mode": (
        "Which file changes the agent tracks as hunks. Off disables tracking (and LOC stats) entirely. Restart required.",
        "File change nào agent theo dõi dạng hunk. Off tắt hẳn (kể cả LOC). Cần khởi động lại.",
    ),
    "voice_capture_mode": (
        "How the voice chord (Ctrl+Space / F8) behaves: Toggle or Hold to talk.",
        "Cách phím giọng (Ctrl+Space / F8) hoạt động: Bật/tắt hoặc Giữ để nói.",
    ),
    "voice_stt_language": (
        "Speech-to-text language for voice dictation. English by default; System uses your locale when supported.",
        "Ngôn ngữ STT cho dictation. Mặc định tiếng Anh; System dùng locale máy khi hỗ trợ.",
    ),
    "fork_secondary_model": (
        "Model used for the secondary agent when forking. Pick (no override) to clear.",
        "Mô hình agent phụ khi fork. Chọn (no override) để xóa.",
    ),
}

CATEGORIES = {
    "Appearance": ("Appearance", "Giao diện"),
    "Mouse": ("Mouse", "Chuột"),
    "Editor & Input": ("Editor & Input", "Trình soạn & nhập"),
    "Agent & Approval": ("Agent & Approval", "Agent & duyệt"),
    "Privacy": ("Privacy", "Riêng tư"),
    "Models": ("Models", "Mô hình"),
    "Session": ("Session", "Phiên"),
    "Advanced": ("Advanced", "Nâng cao"),
}

ENUM_DISPLAY = {
    "auto": ("Auto", "Tự động"),
    "groknight": ("xVora Night", "xVora Night"),
    "grokday": ("xVora Day", "xVora Day"),
    "tokyonight": ("Tokyo Night", "Tokyo Night"),
    "rosepine-moon": ("Rose Pine Moon", "Rose Pine Moon"),
    "oscura-midnight": ("Oscura Midnight", "Oscura Midnight"),
    "default": ("Default", "Mặc định"),
    "ask": ("Ask", "Hỏi"),
    "always-approve": ("Always approve", "Luôn duyệt"),
    "opt-in": ("Opt in", "Cho phép"),
    "opt-out": ("Opt out", "Từ chối"),
    "fullscreen": ("Fullscreen", "Toàn màn hình"),
    "minimal": ("Minimal", "Tối giản"),
    "on": ("On", "Bật"),
    "off": ("Off", "Tắt"),
    "Toggle": ("Toggle", "Bật/tắt"),
    "Hold to talk": ("Hold to talk", "Giữ để nói"),
    "System": ("System", "Hệ thống"),
    "(no override)": ("(no override)", "(không ghi đè)"),
    "Flash after copy": ("Flash after copy", "Nháy sau copy"),
    "Hold until dismissed": ("Hold until dismissed", "Giữ đến khi bỏ"),
    "Word select (terminal-like)": ("Word select (terminal-like)", "Chọn từ (kiểu terminal)"),
    "Agent only": ("Agent only", "Chỉ agent"),
    "All dirty": ("All dirty", "Mọi file dirty"),
    "Auto-detect": ("Auto-detect", "Tự nhận"),
    "Mouse wheel": ("Mouse wheel", "Bánh xe chuột"),
    "Trackpad": ("Trackpad", "Trackpad"),
}

CHROME = {
    "settings.title": ("Settings", "Cài đặt"),
    "settings.tip.long": (
        'Tip · Ask xVora: "change theme to grokday" or "what does compact mode do?"',
        'Mẹo · Hỏi xVora: "đổi theme sang grokday" hoặc "compact mode làm gì?"',
    ),
    "settings.tip.short": (
        "Tip · Ask xVora to change a setting",
        "Mẹo · Hỏi xVora để đổi một cài đặt",
    ),
    "settings.value.on": ("on", "bật"),
    "settings.value.off": ("off", "tắt"),
    "settings.pill.restart": ("· restart", "· khởi động lại"),
    "settings.preview": ("preview", "xem trước"),
    "settings.no_matches": ("No matches for", "Không khớp"),
    "settings.no_override": ("(no override)", "(không ghi đè)"),
    "shortcuts.press_again": ("press again to", "nhấn lại để"),
    "shortcuts.help.title": ("Keyboard Shortcuts", "Phím tắt"),
    "shortcuts.cat.essentials": ("Essentials", "Thiết yếu"),
    "shortcuts.cat.input": ("Input", "Nhập liệu"),
    "shortcuts.cat.conversation_nav": ("Conversation Navigation", "Điều hướng hội thoại"),
    "shortcuts.cat.conversation_action": ("Conversation Actions", "Thao tác hội thoại"),
    "shortcuts.cat.panels": ("Panels", "Panel"),
    "shortcuts.cat.session": ("Session", "Phiên"),
    "shortcuts.cat.dashboard": ("Dashboard", "Dashboard"),
}

SLASH = {
    "quit": ("Quit the application", "Thoát ứng dụng"),
    "help": ("Browse commands and keyboard shortcuts", "Duyệt lệnh và phím tắt"),
    "docs": ("Open How-to Guides or online Build docs", "Mở hướng dẫn hoặc docs online"),
    "home": ("Return to the welcome screen", "Về màn hình chào"),
    "new": ("Start a new session", "Bắt đầu phiên mới"),
    "fork": ("Branch the current session into a peer agent", "Nhánh phiên hiện tại thành agent song song"),
    "compact": ("Compact conversation history", "Nén lịch sử hội thoại"),
    "copy": ("Copy last response to clipboard (/copy N for Nth-latest)", "Sao chép phản hồi gần nhất (/copy N)"),
    "find": ("Search the conversation scrollback", "Tìm trong hội thoại"),
    "history": ("Search prompt history", "Tìm lịch sử prompt"),
    "export": ("Export the current conversation to a file or clipboard", "Xuất hội thoại ra file hoặc clipboard"),
    "transcript": ("View the full conversation transcript in your pager ($PAGER)", "Xem toàn bộ transcript trong pager"),
    "expand": ("Re-print the last collapsed block, fully expanded (minimal mode)", "In lại khối thu gọn gần nhất (minimal)"),
    "context": ("View context usage", "Xem mức dùng context"),
    "minimal": ("Reopen this session in minimal mode — switch back with /fullscreen", "Mở lại phiên ở chế độ tối giản — /fullscreen để về"),
    "fullscreen": ("Reopen this session in fullscreen mode — switch back with /minimal", "Mở lại phiên toàn màn hình — /minimal để về"),
    "model": ("Switch the active model", "Đổi mô hình đang dùng"),
    "effort": ("Set reasoning effort for the current model", "Đặt mức reasoning cho mô hình"),
    "always-approve": ("Toggle always-approve mode (skip all permission prompts)", "Bật/tắt luôn duyệt (bỏ qua hỏi quyền)"),
    "auto": ("Toggle auto mode (classifier approves safe tools)", "Bật/tắt auto (LLM duyệt tool an toàn)"),
    "multiline": ("Toggle multiline input mode (swap Enter and Shift+Enter)", "Bật/tắt nhập nhiều dòng (đổi Enter / Shift+Enter)"),
    "compact-mode": ("Toggle compact UI (less padding, more content)", "Bật/tắt UI gọn (ít padding, nhiều nội dung)"),
    "vim-mode": ("Toggle vim-style scrollback keybindings", "Bật/tắt phím vim cho scrollback"),
    "hooks": ("View hooks", "Xem hooks"),
    "plugins": ("View plugins", "Xem plugins"),
    "marketplace": ("View marketplace", "Xem marketplace"),
    "skills": ("View skills", "Xem skills"),
    "share": ("Share this session via URL", "Chia sẻ phiên qua URL"),
    "session-info": ("Show session info", "Hiện thông tin phiên"),
    "rename": ("Rename the current session", "Đổi tên phiên hiện tại"),
    "dashboard": ("Open the Agent Dashboard — overview of every running session", "Mở Agent Dashboard — tổng quan mọi phiên"),
    "cd": ("Change the working directory for new agents", "Đổi thư mục làm việc cho agent mới"),
    "theme": ("Switch the color theme", "Đổi bảng màu"),
    "feedback": ("Send feedback about the current session", "Gửi góp ý về phiên hiện tại"),
    "announcements": ("Show or hide announcements", "Hiện hoặc ẩn thông báo"),
    "remember": ("Save a memory note", "Lưu ghi chú memory"),
    "plan": ("Enter plan mode", "Vào chế độ kế hoạch"),
    "view-plan": ("View the current plan", "Xem kế hoạch hiện tại"),
    "resume": ("Resume a previous session", "Tiếp tục phiên trước"),
    "mcps": ("Show MCP server status", "Hiện trạng thái MCP"),
    "btw": ("Ask a side question without interrupting", "Hỏi phụ mà không ngắt lượt"),
    "recap": ("Summarize the session so far", "Tóm tắt phiên đến nay"),
    "terminal-setup": ("Check terminal, color, and clipboard setup", "Kiểm tra terminal, màu, clipboard"),
    "voice": ("Dictation (Ctrl+Space/F8; Esc/Enter to stop)", "Dictation (Ctrl+Space/F8; Esc/Enter dừng)"),
    "loop": ("Run a prompt on a recurring interval", "Chạy prompt theo chu kỳ"),
    "imagine": ("Generate an image from a text description", "Tạo ảnh từ mô tả"),
    "imagine-video": ("Generate a video from a text description", "Tạo video từ mô tả"),
    "timestamps": ("Toggle message timestamps on/off", "Bật/tắt dấu thời gian tin nhắn"),
    "timeline": ("Toggle the timeline sidebar", "Bật/tắt thanh timeline"),
    "toggle-mouse-reporting": ("Toggle terminal mouse reporting (native click-drag copy/paste)", "Bật/tắt mouse reporting terminal"),
    "settings": ("Open the settings modal", "Mở modal cài đặt"),
    "privacy": ("Show or toggle privacy & data retention status", "Xem/đổi trạng thái riêng tư & lưu dữ liệu"),
    "rewind": ("Rewind to a previous turn", "Hoàn tác về lượt trước"),
    "jump": ("Jump to a turn in the conversation", "Nhảy tới một lượt"),
    "login": ("Log in or re-authenticate with your account", "Đăng nhập hoặc xác thực lại"),
    "logout": ("Log out and return to the login screen", "Đăng xuất và về màn đăng nhập"),
    "import-claude": ("Open the Claude settings import modal", "Mở modal nhập cài đặt Claude"),
    "usage": ("View credit usage or manage billing", "Xem usage credit hoặc billing"),
    "queue": ("List the prompts queued behind the running turn", "Liệt kê prompt đang xếp hàng"),
    "tasks": ("List background tasks, subagents, and scheduled tasks", "Liệt kê tác vụ nền, subagent, lịch"),
    "release-notes": ("View release notes for the current version", "Xem ghi chú phát hành phiên bản hiện tại"),
    "config-agents": ("Manage agent definitions", "Quản lý định nghĩa agent"),
    "personas": ("Manage personas (create, edit, delete)", "Quản lý persona (tạo, sửa, xóa)"),
    "debug": ("Toggle debug overlays", "Bật/tắt overlay debug"),
}


def write_settings():
    lines = [
        "//! Settings labels, descriptions, categories, enum displays, chrome.",
        "use super::{Locale, locale};",
        "",
        "pub fn category(en_label: &str) -> &'static str {",
        "    let en = match en_label {",
    ]
    for en, (e, v) in CATEGORIES.items():
        lines.append(f'        "{esc(en)}" => "{esc(e)}",')
    lines += [
        '        _ => "Advanced", // unknown sections fall back',
        "    };",
        "    if locale() != Locale::Vi {",
        "        return en;",
        "    }",
        "    match en_label {",
    ]
    for en, (e, v) in CATEGORIES.items():
        lines.append(f'        "{esc(en)}" => "{esc(v)}",')
    lines += [
        "        _ => en,",
        "    }",
        "}",
        "",
        "pub fn setting_label(key: &str, fallback: &'static str) -> &'static str {",
        "    match (locale(), key) {",
    ]
    for k, (e, v) in SETTINGS_LABELS.items():
        lines.append(f'        (Locale::Vi, "{k}") => "{esc(v)}",')
        lines.append(f'        (Locale::En, "{k}") => "{esc(e)}",')
    lines += [
        "        _ => fallback,",
        "    }",
        "}",
        "",
        "pub fn setting_description(key: &str, fallback: &'static str) -> &'static str {",
        "    match (locale(), key) {",
    ]
    for k, (e, v) in SETTINGS_DESCS.items():
        if not e:
            continue
        lines.append(f'        (Locale::Vi, "{k}") => "{esc(v)}",')
        lines.append(f'        (Locale::En, "{k}") => "{esc(e)}",')
    lines += [
        "        _ => fallback,",
        "    }",
        "}",
        "",
        "pub fn enum_display(canonical: &str, fallback: &'static str) -> &'static str {",
        "    match (locale(), canonical) {",
    ]
    for k, (e, v) in ENUM_DISPLAY.items():
        lines.append(f'        (Locale::Vi, "{esc(k)}") => "{esc(v)}",')
        lines.append(f'        (Locale::En, "{esc(k)}") => "{esc(e)}",')
    lines += [
        "        _ => fallback,",
        "    }",
        "}",
        "",
        "pub fn chrome(key: &str) -> &'static str {",
        "    match (locale(), key) {",
    ]
    for k, (e, v) in CHROME.items():
        lines.append(f'        (Locale::Vi, "{esc(k)}") => "{esc(v)}",')
        lines.append(f'        (Locale::En, "{esc(k)}") => "{esc(e)}",')
    lines += [
        '        _ => "",',
        "    }",
        "}",
        "",
    ]
    (ROOT / "settings.rs").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("settings.rs", len(lines))


def write_slash():
    lines = [
        "//! Slash command dropdown descriptions by command name.",
        "use super::{Locale, locale};",
        "",
        "pub fn description(name: &str, fallback: &'static str) -> &'static str {",
        "    match (locale(), name) {",
    ]
    for k, (e, v) in SLASH.items():
        lines.append(f'        (Locale::Vi, "{esc(k)}") => "{esc(v)}",')
        lines.append(f'        (Locale::En, "{esc(k)}") => "{esc(e)}",')
    # fullscreen alias full
    lines.append(f'        (Locale::Vi, "full") => "{esc(SLASH["fullscreen"][1])}",')
    lines.append(f'        (Locale::En, "full") => "{esc(SLASH["fullscreen"][0])}",')
    lines.append(f'        (Locale::Vi, "welcome") => "{esc(SLASH["home"][1])}",')
    lines.append(f'        (Locale::En, "welcome") => "{esc(SLASH["home"][0])}",')
    lines.append(f'        (Locale::Vi, "changelog") => "{esc(SLASH["release-notes"][1])}",')
    lines.append(f'        (Locale::En, "changelog") => "{esc(SLASH["release-notes"][0])}",')
    lines += [
        "        _ => fallback,",
        "    }",
        "}",
        "",
    ]
    (ROOT / "slash.rs").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("slash.rs", len(lines))


if __name__ == "__main__":
    write_settings()
    write_slash()
