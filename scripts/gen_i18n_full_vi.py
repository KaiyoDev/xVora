#!/usr/bin/env python3
"""
Generate full Vietnamese i18n coverage for actions long_help, settings enums,
and expand action description maps. UTF-8 only.
"""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
I18N = ROOT / "crates/codegen/xvora-pager/src/i18n"
DEFAULTS = ROOT / "crates/codegen/xvora-pager/src/actions/defaults.rs"
DEFS = ROOT / "crates/codegen/xvora-pager/src/settings/defs.rs"


def esc(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")


# Manual VI translations for long_help (keyed by ActionId).
# English extracted from defaults.rs when regenerating.
LONG_HELP_VI: dict[str, str] = {
    "ToggleFold": (
        "Thu gọn hoặc mở rộng mục scrollback đang chọn để ẩn/hiện toàn bộ nội dung.\n"
        "Hữu ích khi lướt output tool hoặc reasoning dài.\n"
        "Liên quan: E thu/mở mọi mục, Ctrl+E bật/tắt mọi khối suy nghĩ."
    ),
    "ToggleExpandAll": (
        "Thu gọn hoặc mở rộng mọi mục scrollback cùng lúc, khác e chỉ tác động hàng đang chọn.\n"
        "Thu gọn transcript dài để quét tiêu đề, rồi mở lại tất cả.\n"
        "Khối suy nghĩ có phím riêng Ctrl+E."
    ),
    "ExpandAllThinking": (
        "Hiện hoặc ẩn mọi khối suy nghĩ (reasoning) của agent trên toàn transcript.\n"
        "Xem agent suy luận thế nào, hoặc ẩn để tập trung kết quả.\n"
        "Khác E — E thu/mở mọi loại entry."
    ),
    "ToggleRaw": (
        "Chuyển mục đang chọn giữa markdown đã render và mã nguồn thô.\n"
        "Dùng để copy markdown đúng, xem link, hoặc định dạng renderer ẩn.\n"
        "Nhấn lại để về bản đã render."
    ),
    "CopyBlockContent": (
        "Sao chép nội dung khối đang chọn: tin nhắn, output tool, hoặc code block.\n"
        "Chỉ có trên khối hỗ trợ copy.\n"
        "Chỉ cần lệnh/đường dẫn: dùng Y."
    ),
    "CopyBlockMeta": (
        "Chỉ copy định danh khối: dòng lệnh tool hoặc đường dẫn file, không phải body.\n"
        "Tiện chạy lại lệnh hoặc dán path.\n"
        "Dùng y thường để copy toàn bộ nội dung."
    ),
    "OpenBlockViewer": (
        "Mở khối đang chọn trong trình xem toàn màn hình, cuộn được.\n"
        "Tốt cho output tool dài, file lớn, hoặc code cần đọc riêng.\n"
        "Esc để về hội thoại."
    ),
    "Rewind": (
        "Hoàn tác hội thoại về lượt trước, khôi phục snapshot file lúc đó và bỏ thay đổi sau.\n"
        "Chọn lượt và phạm vi khôi phục (tất cả, chỉ hội thoại, hoặc chỉ file); lượt đang chạy có thể hủy trước.\n"
        "Phá hủy: các lượt sau bị xóa.\n"
        "Cũng vào được khi idle prompt trống bằng Esc Esc (trong 800ms), giống /rewind."
    ),
    "KillBgTask": (
        "Dừng tác vụ nền của khối task đang chọn (vd. lệnh shell chạy nền).\n"
        "Dùng để dừng process runaway hoặc không còn cần.\n"
        "Chỉ tác động task đang sống."
    ),
    "FocusScrollback": (
        "Chuyển focus từ prompt sang scrollback để điều hướng transcript.\n"
        "Tab hoạt động ở cả simple và vim scrollback.\n"
        "Esc dành cho clear/rewind (idle), không phải focus."
    ),
    "CancelTurn": (
        "Ngắt lượt agent hiện tại và dừng generate, giữ phiên mở.\n"
        "Ctrl+C hủy khi prompt trống; có nháp thì xóa prompt trước, lượt vẫn chạy.\n"
        "Dừng lượt, không thoát app; dùng phím thoát để thoát."
    ),
    "CycleMode": (
        "Xoay chế độ phiên: Thường → Kế hoạch → Luôn duyệt → Thường.\n"
        "Kế hoạch: agent lên plan trước, không ghi file; Luôn duyệt: chạy mọi tool không hỏi.\n"
        "Ctrl+O bật/tắt always-approve trực tiếp."
    ),
    "ToggleTodos": (
        "Hiện/ẩn panel todo: checklist việc agent đang làm.\n"
        "Theo dõi kế hoạch và phần còn lại khi lượt chạy.\n"
        "Panel bên; tắt để lấy lại chiều ngang."
    ),
    "ToggleTasks": (
        "Hiện/ẩn panel tác vụ nền và trạng thái.\n"
        "Theo dõi hoặc quay lại việc đã gửi nền bằng Ctrl+G.\n"
        "Panel bên; tắt để lấy lại chiều ngang."
    ),
    "ToggleQueue": (
        "Hiện/ẩn hàng đợi prompt.\n"
        "Xếp prompt follow-up trong khi lượt đang chạy; gửi tự động khi agent xong.\n"
        "VS Code macOS local: Ctrl+4 chính (Ctrl+; / Ctrl+' phụ). Còn lại Ctrl+; với Ctrl+' phụ."
    ),
    "OpenSessions": (
        "Mở trình duyệt phiên để resume hoặc chuyển hội thoại cũ.\n"
        "Chọn một phiên để gắn lại lịch sử đầy đủ.\n"
        "Khác Agent Dashboard (Ctrl+\\) quản lý nhiều agent sống."
    ),
    "OpenExtensions": (
        "Mở quản lý extension cho MCP và plugin: xem kết nối và tool thêm.\n"
        "Xác nhận integration đã load hoặc duyệt tool.\n"
        "Khác Settings (tùy chọn chung)."
    ),
    "SendToBackground": (
        "Tách lượt đang chạy để làm nền trong khi bạn đọc, xếp prompt, hoặc làm việc khác.\n"
        "Theo dõi/resume ở panel tasks (Ctrl+B).\n"
        "Chỉ có nghĩa khi lượt đang chạy."
    ),
    "InterjectPrompt": (
        "Gửi tin cho agent giữa lượt mà không hủy (interject), để chỉnh hướng hoặc thêm ngữ cảnh.\n"
        "Enter thường khi lượt chạy sẽ xếp follow-up; tổ hợp này gộp text composer vào lượt hiện tại.\n"
        "Composer trống: Enter (hoặc chord) force-gửi follow-up đầu hàng đợi.\n"
        "Dùng để chỉnh hướng mà không mất tiến độ lượt."
    ),
    "VoiceToggle": (
        "Thu mic dictation, gán Ctrl+Space (hoặc F8 — khi Ctrl+Space bị chiếm, vd. đổi IME macOS).\n"
        "Theo cài Voice capture: bật/tắt hoặc giữ-để-nói (cần terminal Kitty-protocol).\n"
        "/voice bật/tắt mọi nơi. Lời nói được chuyển thẳng vào prompt."
    ),
    "ToggleMultiline": (
        "Bật/tắt prompt nhiều dòng cố định để soạn tin dài.\n"
        "Xuống dòng bằng Shift+Enter hoặc Alt+Enter (hoặc \\ cuối dòng); Enter gửi.\n"
        "Ctrl+M bật multiline ở prompt; ngoài prompt thì mở model picker."
    ),
    "BashMode": (
        "Chạy lệnh shell trong chat: gõ ! đầu prompt trống, rồi lệnh.\n"
        "Output được ghi vào scrollback.\n"
        "Xóa ! để về prompt thường."
    ),
    "ToggleYolo": (
        "Bật/tắt always-approve (YOLO) cho phiên này.\n"
        "Khi bật, agent chạy mọi tool (sửa, shell, xóa) không hỏi từng bước.\n"
        "Cùng trạng thái Always-Approve của Shift+Tab; dùng cẩn thận."
    ),
    "NewSession": (
        "Bắt đầu phiên mới với scrollback và context trống.\n"
        "Cần xác nhận: nhấn hai lần (lần 1 chờ, lần 2 bắt đầu)\n"
        "để không vô tình bỏ hội thoại hiện tại."
    ),
    "Quit": (
        "Thoát app. Cần xác nhận: nhấn hai lần liên tiếp;\n"
        "nhấn một lần coi như phím lạc.\n"
        "Gán Ctrl+Q, alias Ctrl+D (Ctrl+D chính trong terminal VS Code)."
    ),
    "CommandPalette": (
        "Tìm mờ mọi action và slash command, chạy theo tên.\n"
        "Hữu ích khi quên phím tắt.\n"
        "Cũng mở bằng ? khi focus scrollback."
    ),
    "ShortcutsHelp": (
        "Mở bảng phím tắt này.\n"
        "Duyệt j/k, mở rộng help bằng e, Enter xem trang chi tiết.\n"
        "Gán Ctrl+. và Ctrl+X; thanh gợi ý phím terminal gửi ổn định."
    ),
    "ModelPicker": (
        "Mở chọn mô hình cho phiên; áp dụng các lượt sau.\n"
        "Gán Ctrl+M, nhưng khi focus prompt chord đó bật multiline.\n"
        "Vào từ scrollback hoặc command palette."
    ),
    "OpenDashboard": (
        "Mở Agent Dashboard: danh sách agent đang chạy/gần đây để theo dõi và chuyển.\n"
        "Hoạt động từ welcome và trong phiên.\n"
        "Dispatch, attach, dừng, nhóm, sắp xếp agent."
    ),
    "DashboardTogglePin": (
        "Ghim/bỏ ghim agent để luôn ở đầu danh sách bất kể sort/group.\n"
        "Giữ agent quan trọng trong tầm nhìn.\n"
        "Ghim lưu qua các lần mở dashboard."
    ),
    "DashboardStop": (
        "Dừng agent đang chọn và xóa hàng khỏi dashboard; ngắt lượt đang chạy trước.\n"
        "Dọn agent xong/không cần mà không attach.\n"
        "Trong overlay (Ctrl+X) có xác nhận trước khi dừng."
    ),
    "DashboardCycleMode": (
        "Xoay chế độ dispatch agent mới từ dashboard: Thường, Kế hoạch, Luôn duyệt.\n"
        "Kế hoạch: plan trước khi sửa file; Luôn duyệt: tool không hỏi.\n"
        "Giống chu kỳ Shift+Tab trong phiên, áp cho dispatch mới."
    ),
    "DashboardToggleGrouping": (
        "Chuyển dashboard giữa list phẳng và nhóm theo trạng thái (đang làm / idle).\n"
        "Nhóm nổi agent cần chú ý; list phẳng giữ thứ tự ổn định.\n"
        "Lựa chọn lưu qua phiên."
    ),
    "DashboardExit": (
        "Đóng dashboard và về chỗ trước đó.\n"
        "Esc theo tầng: đóng peek/filter trước, rồi mới thoát.\n"
        "Đổi phím action này để thoát thẳng."
    ),
    "DashboardToggleAutoApprove": (
        "Bật/tắt always-approve (YOLO) cho agent đang chọn ngay từ dashboard, không cần attach.\n"
        "Khi bật, agent chạy mọi tool không hỏi.\n"
        "Trong phiên tương đương Ctrl+O."
    ),
    "DashboardOpenLocationPicker": (
        "Mở picker đặt thư mục làm việc cho agent mới dispatch từ dashboard.\n"
        "Chạy agent ở repo/folder khác mà không rời dashboard.\n"
        "Chỉ ảnh hưởng dispatch mới."
    ),
    "DashboardToggleWorktree": (
        "Arm agent dashboard tiếp theo spawn trong git worktree mới, cô lập checkout.\n"
        "Chỉ khi working directory là git repo.\n"
        "Chỉ agent mới, không đụng agent đang chạy."
    ),
    "DashboardOverlayExit": (
        "Rời overlay phiên gắn kèm, về list dashboard, không dừng agent.\n"
        "Cũng: q trên scrollback, Esc trung tính, hoặc nút đóng.\n"
        "Muốn dừng agent: Ctrl+X."
    ),
    "DashboardOverlayStop": (
        "Trong overlay phiên, dừng agent gắn kèm và đóng, về list dashboard.\n"
        "Cần xác nhận: Ctrl+X hai lần.\n"
        "Ctrl+. vẫn mở cheatsheet; chỉ Ctrl+X dùng để dừng."
    ),
}


def extract_long_helps() -> dict[str, str]:
    text = DEFAULTS.read_text(encoding="utf-8")
    blocks = re.split(r"ActionDef\s*\{", text)[1:]
    out: dict[str, str] = {}
    for b in blocks:
        mid = re.search(r"id:\s*ActionId::(\w+)", b)
        if not mid:
            continue
        # Match Some("...") possibly multi-line with \"
        m = re.search(
            r'long_help:\s*Some\(\s*"((?:\\.|[^"\\])*)"\s*,?\s*\)',
            b,
            re.S,
        )
        if m:
            raw = m.group(1)
            # Unescape for storage then re-escape when writing
            un = (
                raw.replace("\\n", "\n")
                .replace('\\"', '"')
                .replace("\\\\", "\\")
            )
            out[mid.group(1)] = un
    return out


def write_actions_long_help_extension(en_map: dict[str, str]) -> None:
    """Append long_help functions into actions.rs or write separate file."""
    path = I18N / "actions_long_help.rs"
    lines = [
        "//! Action long_help (cheatsheet detail) en/vi by ActionId.",
        "use crate::actions::ActionId;",
        "use super::{Locale, locale};",
        "",
        "pub fn long_help(id: ActionId) -> Option<&'static str> {",
        "    match locale() {",
        "        Locale::Vi => long_help_vi(id),",
        "        Locale::En => long_help_en(id),",
        "    }",
        "}",
        "",
        "fn long_help_en(id: ActionId) -> Option<&'static str> {",
        "    match id {",
    ]
    for aid, en in sorted(en_map.items()):
        lines.append(f'        ActionId::{aid} => Some("{esc(en)}"),')
    lines += [
        "        _ => None,",
        "    }",
        "}",
        "",
        "fn long_help_vi(id: ActionId) -> Option<&'static str> {",
        "    match id {",
    ]
    for aid, en in sorted(en_map.items()):
        vi = LONG_HELP_VI.get(aid, en)  # fallback EN if missing
        lines.append(f'        ActionId::{aid} => Some("{esc(vi)}"),')
    lines += [
        "        _ => long_help_en(id),",
        "    }",
        "}",
        "",
    ]
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {path} ({len(en_map)} long_helps)")


# Full enum choice descriptions (canonical -> en, vi)
ENUM_CHOICES: dict[str, tuple[str, str, str, str]] = {
    # display_en, display_vi, desc_en, desc_vi
    "auto": (
        "Auto",
        "Tự động",
        "Follow system dark/light appearance.",
        "Theo giao diện tối/sáng của hệ thống.",
    ),
    "groknight": (
        "xVora Night",
        "xVora Night",
        "Neutral dark with magenta accent.",
        "Tối trung tính, accent magenta.",
    ),
    "grokday": (
        "xVora Day",
        "xVora Day",
        "Light theme for bright environments.",
        "Theme sáng cho môi trường nhiều ánh sáng.",
    ),
    "tokyonight": (
        "Tokyo Night",
        "Tokyo Night",
        "Dark + blue-tinted; needs truecolor.",
        "Tối xanh dương; cần truecolor.",
    ),
    "rosepine-moon": (
        "Rose Pine Moon",
        "Rose Pine Moon",
        "Muted dark with mauve accents; needs truecolor.",
        "Tối dịu accent mauve; cần truecolor.",
    ),
    "oscura-midnight": (
        "Oscura Midnight",
        "Oscura Midnight",
        "Deep dark with warm accents; needs truecolor.",
        "Tối sâu accent ấm; cần truecolor.",
    ),
    "default": (
        "Default",
        "Mặc định",
        "Use the agent's default permission behavior (currently equivalent to Ask).",
        "Dùng hành vi quyền mặc định của agent (hiện tương đương Hỏi).",
    ),
    "ask": (
        "Ask",
        "Hỏi",
        "Prompt for permission before tool actions.",
        "Hỏi quyền trước mỗi thao tác tool.",
    ),
    "always-approve": (
        "Always approve",
        "Luôn duyệt",
        "Auto-approve every tool action. Skips ALL permission prompts.",
        "Tự duyệt mọi tool. Bỏ qua mọi hộp hỏi quyền.",
    ),
    "opt-in": (
        "Opt in",
        "Cho phép",
        "Allow SpaceXAI to retain and use coding session data for training and product improvement.",
        "Cho phép SpaceXAI lưu và dùng dữ liệu phiên coding để huấn luyện và cải thiện sản phẩm.",
    ),
    "opt-out": (
        "Opt out",
        "Từ chối",
        "Do not retain coding session data. Code requests will not be used for training.",
        "Không lưu dữ liệu phiên coding. Code không dùng để huấn luyện.",
    ),
    "fullscreen": (
        "Fullscreen",
        "Toàn màn hình",
        "Open xVora in the standard fullscreen TUI. Default when unset.",
        "Mở xVora ở TUI toàn màn hình chuẩn. Mặc định khi chưa đặt.",
    ),
    "minimal": (
        "Minimal",
        "Tối giản",
        "Open xVora in scrollback-native (minimal) mode.",
        "Mở xVora ở chế độ tối giản (scrollback-native).",
    ),
    "on": (
        "On",
        "Bật",
        "Agent summarises a plan and asks for approval before running tools.",
        "Agent tóm tắt kế hoạch và xin duyệt trước khi chạy tool.",
    ),
    "off": (
        "Off",
        "Tắt",
        "Agent runs tools and edits files directly (default).",
        "Agent chạy tool và sửa file trực tiếp (mặc định).",
    ),
    "agent-only": (
        "Agent only",
        "Chỉ agent",
        "Track only files the agent edits (default).",
        "Chỉ theo dõi file agent sửa (mặc định).",
    ),
    "all-dirty": (
        "All dirty",
        "Mọi file dirty",
        "Track every git-dirty file, including external edits.",
        "Theo dõi mọi file git dirty, kể cả sửa ngoài.",
    ),
    "wheel": (
        "Mouse wheel",
        "Bánh xe chuột",
        "Always treat scrolling as wheel notches (fixed lines per tick).",
        "Luôn coi cuộn là bánh xe (số dòng cố định mỗi tick).",
    ),
    "trackpad": (
        "Trackpad",
        "Trackpad",
        "Always treat scrolling as a trackpad (fractional accumulation).",
        "Luôn coi cuộn là trackpad (tích lũy phân số).",
    ),
    "auto-detect": (
        "Auto-detect",
        "Tự nhận",
        "Detect wheel vs trackpad per gesture from event timing. Default.",
        "Tự nhận wheel vs trackpad theo timing gesture. Mặc định.",
    ),
    "flash": (
        "Flash after copy",
        "Nháy sau copy",
        "Brief highlight on mouse-up, then clear. Double-click toggles fold. Default.",
        "Nháy ngắn khi nhả chuột, rồi xóa. Double-click thu/mở. Mặc định.",
    ),
    "hold": (
        "Hold until dismissed",
        "Giữ đến khi bỏ",
        "Keep the selection visible until Esc, click, or scroll. Double-click toggles fold.",
        "Giữ vùng chọn đến Esc, click, hoặc cuộn. Double-click thu/mở.",
    ),
    "word": (
        "Word select (terminal-like)",
        "Chọn từ (kiểu terminal)",
        "Double-click selects & copies a word, triple-click a line; selection stays until dismissed.",
        "Double-click chọn & copy từ, triple-click cả dòng; giữ đến khi bỏ.",
    ),
    "toggle": (
        "Toggle",
        "Bật/tắt",
        "Ctrl+Space / F8 starts dictation; press again (or Esc/Enter) to stop.",
        "Ctrl+Space / F8 bắt đầu dictation; nhấn lại (hoặc Esc/Enter) để dừng.",
    ),
    "hold-to-talk": (
        "Hold to talk",
        "Giữ để nói",
        "Hold Ctrl+Space / F8 to record, release to stop. Needs a Kitty-protocol terminal.",
        "Giữ Ctrl+Space / F8 để ghi, thả để dừng. Cần terminal Kitty-protocol.",
    ),
    "system": (
        "System",
        "Hệ thống",
        "Use the system locale when it is a supported STT language; otherwise English.",
        "Dùng locale hệ thống nếu STT hỗ trợ; không thì tiếng Anh.",
    ),
}


def write_enum_full() -> None:
    path = I18N / "enums.rs"
    lines = [
        "//! Full enum choice display + description i18n.",
        "use super::{Locale, locale};",
        "",
        "pub fn display(canonical: &str, fallback: &'static str) -> &'static str {",
        "    match (locale(), canonical) {",
    ]
    for k, (de, dv, ee, ev) in ENUM_CHOICES.items():
        lines.append(f'        (Locale::Vi, "{esc(k)}") => "{esc(dv)}",')
        lines.append(f'        (Locale::En, "{esc(k)}") => "{esc(de)}",')
    # aliases used in defs
    aliases = {
        "always-approve": "always-approve",
        "mouse-wheel": "wheel",
        "Mouse wheel": "wheel",
    }
    lines += [
        "        _ => fallback,",
        "    }",
        "}",
        "",
        "pub fn description(canonical: &str, fallback: &'static str) -> &'static str {",
        "    match (locale(), canonical) {",
    ]
    for k, (de, dv, ee, ev) in ENUM_CHOICES.items():
        lines.append(f'        (Locale::Vi, "{esc(k)}") => "{esc(ev)}",')
        lines.append(f'        (Locale::En, "{esc(k)}") => "{esc(ee)}",')
    lines += [
        "        _ => fallback,",
        "    }",
        "}",
        "",
    ]
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {path}")


# Extra chrome strings for full UI
EXTRA_CHROME = {
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
    "settings.validate.empty": ("Value cannot be empty", "Giá trị không được để trống"),
    "settings.validate.whitespace": (
        "Value cannot contain whitespace",
        "Giá trị không được chứa khoảng trắng",
    ),
    "settings.validate.catalog_loading": (
        "Model catalog still loading — try again",
        "Catalog mô hình đang tải — thử lại",
    ),
    "settings.footer.nav": ("↑/↓/j/k nav", "↑/↓/j/k điều hướng"),
    "settings.footer.top_btm": ("g/G top/btm", "g/G đầu/cuối"),
    "settings.footer.space_enter": ("Space/Enter", "Space/Enter"),
    "settings.footer.expand": ("→ expand", "→ mở rộng"),
    "settings.footer.search": ("/ search", "/ tìm"),
    "settings.footer.reset": ("d reset", "d đặt lại"),
    "settings.footer.close": ("F2/Esc close", "F2/Esc đóng"),
    "settings.footer.type_filter": ("type to filter", "gõ để lọc"),
    "settings.footer.backspace": ("Backspace edit", "Backspace sửa"),
    "settings.footer.enter_commit": ("Enter commit", "Enter xác nhận"),
    "settings.footer.esc_clear": ("Esc clear", "Esc xóa"),
    "settings.footer.esc_revert": ("Esc revert", "Esc hoàn tác"),
    "settings.footer.esc_cancel": ("Esc cancel", "Esc hủy"),
    "settings.footer.toggle": ("Space/Enter toggle", "Space/Enter bật/tắt"),
    "settings.footer.esc_back": ("Esc back", "Esc quay lại"),
    "settings.footer.y_reset": ("y reset", "y đặt lại"),
    "settings.footer.n_cancel": ("n cancel", "n hủy"),
    "settings.footer.try": ("↑/↓ try", "↑/↓ thử"),
    "settings.reset.fallback": ("Reset setting to default?", "Đặt lại cài đặt về mặc định?"),
    "settings.toast.restart_suffix": ("(restart to apply)", "(khởi động lại để áp dụng)"),
    "settings.toast.already_default": ("already at default", "đã ở mặc định"),
    "shortcuts.press_again": ("press again to", "nhấn lại để"),
    "shortcuts.help.title": ("Keyboard Shortcuts", "Phím tắt"),
    "shortcuts.cat.essentials": ("Essentials", "Thiết yếu"),
    "shortcuts.cat.input": ("Input", "Nhập liệu"),
    "shortcuts.cat.conversation_nav": ("Conversation Navigation", "Điều hướng hội thoại"),
    "shortcuts.cat.conversation_action": ("Conversation Actions", "Thao tác hội thoại"),
    "shortcuts.cat.panels": ("Panels", "Panel"),
    "shortcuts.cat.session": ("Session", "Phiên"),
    "shortcuts.cat.dashboard": ("Dashboard", "Dashboard"),
    "shortcuts.pseudo.search": ("Search scrollback", "Tìm trong scrollback"),
    "shortcuts.pseudo.paste": (
        "Paste images (and text) from the clipboard",
        "Dán ảnh (và chữ) từ clipboard",
    ),
    "shortcuts.footer.detail.esc_back": ("Esc back", "Esc quay lại"),
    "shortcuts.footer.detail.scroll": ("↑/↓ scroll", "↑/↓ cuộn"),
    "shortcuts.footer.detail.close": ("Ctrl+./X close", "Ctrl+./X đóng"),
    "shortcuts.footer.browse.nav": ("↑/↓ nav", "↑/↓ điều hướng"),
    "shortcuts.footer.browse.filter_on": ("f show all", "f hiện tất cả"),
    "shortcuts.footer.browse.filter_off": ("f filter", "f lọc"),
    "shortcuts.footer.browse.expand": ("e/Space/→ expand", "e/Space/→ mở rộng"),
    "shortcuts.footer.browse.collapse": ("← collapse", "← thu gọn"),
    "shortcuts.footer.browse.details": ("Enter details", "Enter chi tiết"),
    "shortcuts.footer.browse.search": ("/ search", "/ tìm"),
    "shortcuts.footer.browse.esc_close": ("Esc close", "Esc đóng"),
    "shortcuts.detail.dimmed": (
        "(not active in current context)",
        "(không hoạt động trong ngữ cảnh hiện tại)",
    ),
    "slash.err.no_session": ("No active session", "Không có phiên đang mở"),
    "slash.err.unknown_model": ("Unknown model", "Mô hình không biết"),
}


def patch_settings_chrome() -> None:
    """Rewrite chrome() fully in settings.rs by regenerating chrome section only —
    easier to rewrite whole settings chrome via enums + merge into settings.rs end.
    """
    path = I18N / "chrome.rs"
    lines = [
        "//! Shared chrome / footer / toast fragments.",
        "use super::{Locale, locale};",
        "",
        "pub fn t(key: &str) -> &'static str {",
        "    match (locale(), key) {",
    ]
    for k, (e, v) in EXTRA_CHROME.items():
        lines.append(f'        (Locale::Vi, "{esc(k)}") => "{esc(v)}",')
        lines.append(f'        (Locale::En, "{esc(k)}") => "{esc(e)}",')
    lines += [
        '        _ => "",',
        "    }",
        "}",
        "",
    ]
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {path} ({len(EXTRA_CHROME)} keys)")


def main() -> None:
    en_lh = extract_long_helps()
    print(f"extracted {len(en_lh)} long_helps from defaults.rs")
    missing = [k for k in en_lh if k not in LONG_HELP_VI]
    if missing:
        print("WARNING missing VI long_help for:", missing)
    write_actions_long_help_extension(en_lh)
    write_enum_full()
    patch_settings_chrome()


if __name__ == "__main__":
    main()
