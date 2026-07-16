#!/usr/bin/env python3
"""Generate crates/codegen/xvora-pager/src/i18n/actions.rs"""
from pathlib import Path

actions = [
    ("SelectNext", "nav", "Select next entry", "điều hướng", "Chọn mục tiếp"),
    ("SelectPrev", "nav", "Select previous entry", "điều hướng", "Chọn mục trước"),
    ("NextTurn", "turn", "Next turn", "lượt", "Lượt tiếp"),
    ("PrevTurn", "turn", "Previous turn", "lượt", "Lượt trước"),
    ("NextResponse", "response", "Next response", "phản hồi", "Phản hồi tiếp"),
    ("PrevResponse", "response", "Previous response", "phản hồi", "Phản hồi trước"),
    ("GotoTop", "top/btm", "Go to top", "đầu/cuối", "Lên đầu"),
    ("GotoBottom", "bottom", "Go to bottom", "cuối", "Xuống cuối"),
    ("ScrollUp", "scroll up", "Scroll up one line", "cuộn lên", "Cuộn lên 1 dòng"),
    ("ScrollDown", "scroll down", "Scroll down one line", "cuộn xuống", "Cuộn xuống 1 dòng"),
    ("HalfPageUp", "half page up", "Scroll up half page", "nửa trang lên", "Cuộn nửa trang lên"),
    ("HalfPageDown", "half page down", "Scroll down half page", "nửa trang xuống", "Cuộn nửa trang xuống"),
    ("PageUp", "page up", "Scroll up one page", "trang lên", "Cuộn một trang lên"),
    ("PageDown", "page down", "Scroll down one page", "trang xuống", "Cuộn một trang xuống"),
    ("Collapse", "fold", "Collapse selected entry", "thu gọn", "Thu gọn mục chọn"),
    ("Expand", "fold", "Expand selected entry", "mở rộng", "Mở rộng mục chọn"),
    ("ToggleFold", "fold", "Expand / collapse", "thu/mở", "Mở rộng / thu gọn"),
    ("ToggleExpandAll", "all", "Expand all / collapse all", "tất cả", "Mở/thu tất cả"),
    ("ExpandAllThinking", "expand/collapse thinking", "Toggle all thinking blocks", "suy nghĩ", "Bật/tắt khối suy nghĩ"),
    ("ToggleRaw", "raw", "Toggle raw markdown", "thô", "Bật/tắt markdown thô"),
    ("CopyBlockContent", "copy", "Copy content", "sao chép", "Sao chép nội dung"),
    ("CopyBlockMeta", "copy cmd", "Copy command / path", "sao chép lệnh", "Sao chép lệnh / đường dẫn"),
    ("OpenBlockViewer", "view", "Open in viewer", "xem", "Mở trong trình xem"),
    ("OpenNextLink", "link", "Next link", "liên kết", "Liên kết tiếp"),
    ("OpenPrevLink", "link", "Previous link", "liên kết", "Liên kết trước"),
    ("Rewind", "rewind", "Rewind to selected turn", "hoàn tác", "Hoàn tác về lượt đã chọn"),
    ("KillBgTask", "kill", "Kill background task", "dừng", "Dừng tác vụ nền"),
    ("SendPrompt", "send", "Send", "gửi", "Gửi"),
    ("FocusPrompt", "prompt", "Focus prompt", "prompt", "Focus ô nhập"),
    ("FocusScrollback", "scrollback", "Focus scrollback", "cuộn", "Focus vùng cuộn"),
    ("CancelTurn", "cancel", "Cancel turn", "hủy", "Hủy lượt"),
    ("CycleMode", "mode", "Cycle mode (Normal / Plan / Always-approve)", "chế độ", "Đổi chế độ (Thường / Kế hoạch / Luôn duyệt)"),
    ("ToggleTodos", "todos", "Toggle todo pane", "todo", "Bật/tắt panel todo"),
    ("ToggleTasks", "tasks", "Toggle tasks pane", "tác vụ", "Bật/tắt panel tác vụ"),
    ("ToggleQueue", "queue", "Toggle prompt queue", "hàng đợi", "Bật/tắt hàng đợi prompt"),
    ("OpenSessions", "sessions", "Open sessions", "phiên", "Mở phiên"),
    ("OpenExtensions", "extensions", "Open extensions", "mở rộng", "Mở tiện ích mở rộng"),
    ("SendToBackground", "send to bg", "Send running task to background", "nền", "Gửi tác vụ đang chạy ra nền"),
    ("InterjectPrompt", "send now", "Send now while running (cancels the current turn)", "gửi ngay", "Gửi ngay khi đang chạy (hủy lượt hiện tại)"),
    ("EnableVoiceMode", "voice mode", "Start voice dictation (Ctrl+Space / F8)", "giọng nói", "Bắt đầu dictation (Ctrl+Space / F8)"),
    ("VoiceToggle", "mic", "Voice dictation (Ctrl+Space / F8)", "mic", "Dictation giọng nói (Ctrl+Space / F8)"),
    ("ToggleMultiline", "multiline", "Toggle multiline", "nhiều dòng", "Bật/tắt nhập nhiều dòng"),
    ("BashMode", "shell", "Shell mode (type ! on empty prompt)", "shell", "Chế độ shell (gõ ! khi prompt trống)"),
    ("ToggleYolo", "yolo", "Toggle always-approve", "luôn duyệt", "Bật/tắt luôn duyệt"),
    ("NewSession", "new", "New session", "mới", "Phiên mới"),
    ("Quit", "quit", "Quit", "thoát", "Thoát"),
    ("CommandPalette", "commands", "Command palette", "lệnh", "Bảng lệnh"),
    ("ShortcutsHelp", "shortcuts", "Keyboard shortcuts", "phím tắt", "Phím tắt"),
    ("ModelPicker", "model", "Pick model", "mô hình", "Chọn mô hình"),
    ("OpenSettings", "settings", "Open the settings modal", "cài đặt", "Mở modal cài đặt"),
    ("ToggleMouseCapture", "mouse reporting", "Toggle mouse reporting (native copy/paste)", "chuột", "Bật/tắt báo cáo chuột (copy/paste native)"),
    ("OpenDashboard", "dashboard", "Open the Agent Dashboard", "bảng điều khiển", "Mở Agent Dashboard"),
    ("DashboardSelectNext", "next", "Select next row", "tiếp", "Chọn hàng tiếp"),
    ("DashboardSelectPrev", "prev", "Select previous row", "trước", "Chọn hàng trước"),
    ("DashboardTogglePin", "pin", "Pin / unpin agent", "ghim", "Ghim / bỏ ghim agent"),
    ("DashboardBeginRename", "rename", "Rename agent", "đổi tên", "Đổi tên agent"),
    ("DashboardStop", "stop", "Stop / Close agent", "dừng", "Dừng / đóng agent"),
    ("DashboardCycleMode", "mode", "Cycle dispatch mode", "chế độ", "Đổi chế độ dispatch"),
    ("DashboardToggleGrouping", "group", "Toggle row grouping", "nhóm", "Bật/tắt nhóm hàng"),
    ("DashboardReorderUp", "reorder up", "Reorder agent up", "lên", "Sắp xếp agent lên"),
    ("DashboardReorderDown", "reorder down", "Reorder agent down", "xuống", "Sắp xếp agent xuống"),
    ("DashboardShortcutsHelp", "shortcuts", "Show shortcuts overlay", "phím tắt", "Hiện overlay phím tắt"),
    ("DashboardExit", "exit", "Close dashboard", "thoát", "Đóng dashboard"),
    ("DashboardToggleAutoApprove", "always-approve", "Toggle always-approve", "luôn duyệt", "Bật/tắt luôn duyệt"),
    ("DashboardOpenLocationPicker", "location", "Change working directory for new agents", "thư mục", "Đổi thư mục làm việc cho agent mới"),
    ("DashboardToggleWorktree", "worktree", "Toggle worktree mode for new agents", "worktree", "Bật/tắt worktree cho agent mới"),
    ("DashboardOverlayExit", "close overlay", "Back to dashboard", "đóng overlay", "Về dashboard"),
    ("DashboardOverlayPrev", "prev session", "Previous session", "phiên trước", "Phiên trước"),
    ("DashboardOverlayNext", "next session", "Next session", "phiên sau", "Phiên sau"),
    ("DashboardOverlayStop", "stop", "Stop agent, close session (back to dashboard)", "dừng", "Dừng agent, đóng phiên (về dashboard)"),
    ("NewSessionInWorktree", "new wt", "New session in worktree", "mới wt", "Phiên mới trong worktree"),
    ("ExitSession", "exit", "Exit session", "thoát phiên", "Thoát phiên"),
    ("NextModel", "model", "Next model", "mô hình", "Mô hình tiếp"),
    ("DumpInputLog", "debug", "Dump input log", "debug", "Dump input log"),
]


def esc(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


lines = [
    "//! Action bar / shortcuts i18n (label + description by ActionId).",
    "use crate::actions::ActionId;",
    "use super::{Locale, locale};",
    "",
    "pub fn label(id: ActionId) -> &'static str {",
    "    match locale() {",
    "        Locale::Vi => label_vi(id),",
    "        Locale::En => label_en(id),",
    "    }",
    "}",
    "",
    "pub fn description(id: ActionId) -> &'static str {",
    "    match locale() {",
    "        Locale::Vi => desc_vi(id),",
    "        Locale::En => desc_en(id),",
    "    }",
    "}",
    "",
    "fn label_en(id: ActionId) -> &'static str {",
    "    match id {",
]
for a, en_l, en_d, vi_l, vi_d in actions:
    lines.append(f'        ActionId::{a} => "{esc(en_l)}",')
lines += [
    '        _ => "",',
    "    }",
    "}",
    "",
    "fn label_vi(id: ActionId) -> &'static str {",
    "    match id {",
]
for a, en_l, en_d, vi_l, vi_d in actions:
    lines.append(f'        ActionId::{a} => "{esc(vi_l)}",')
lines += [
    "        _ => label_en(id),",
    "    }",
    "}",
    "",
    "fn desc_en(id: ActionId) -> &'static str {",
    "    match id {",
]
for a, en_l, en_d, vi_l, vi_d in actions:
    lines.append(f'        ActionId::{a} => "{esc(en_d)}",')
lines += [
    '        _ => "",',
    "    }",
    "}",
    "",
    "fn desc_vi(id: ActionId) -> &'static str {",
    "    match id {",
]
for a, en_l, en_d, vi_l, vi_d in actions:
    lines.append(f'        ActionId::{a} => "{esc(vi_d)}",')
lines += [
    "        _ => desc_en(id),",
    "    }",
    "}",
    "",
]

out = Path("crates/codegen/xvora-pager/src/i18n/actions.rs")
out.write_text("\n".join(lines) + "\n", encoding="utf-8")
print(f"wrote {out} ({len(lines)} lines)")
