//! Action long_help (cheatsheet detail) en/vi by ActionId.
use super::{Locale, locale};
use crate::actions::ActionId;

pub fn long_help(id: ActionId) -> Option<&'static str> {
    match locale() {
        Locale::Vi => long_help_vi(id),
        Locale::En => long_help_en(id),
    }
}

fn long_help_en(id: ActionId) -> Option<&'static str> {
    match id {
        ActionId::BashMode => Some(
            "Runs a shell command without leaving the chat: type ! at the start of an empty prompt, then the command.\nThe command output is captured into the scrollback.\nDelete the leading ! to go back to a normal prompt.",
        ),
        ActionId::CancelTurn => Some(
            "Interrupts the agent's current turn and stops generation, keeping the session open.\nEsc cancels immediately while a turn is running in minimal mode or when vim scrollback mode is off (prompt or scrollback focused, even with a draft).\nCtrl+C cancels when the prompt is empty; with a non-empty draft it clears the prompt first and leaves the turn running.\nIt stops the turn, not the app; use the quit shortcut to exit.",
        ),
        ActionId::CommandPalette => Some(
            "Fuzzy-search every action and slash command, then run it by name.\nUseful when you don't remember a key binding.\nAlso opens with ? while the scrollback is focused.",
        ),
        ActionId::CopyBlockContent => Some(
            "Copies the selected block's body to the clipboard: message text, full tool output, or a code block's contents.\nOffered only on blocks that support copy.\nFor just the command or file path, use Y instead.",
        ),
        ActionId::CopyBlockMeta => Some(
            "Copies only the block's identifier: a tool call's command line or a file block's path, not the body.\nHandy to re-run a command or paste a path elsewhere.\nUse lowercase y to copy the full content instead.",
        ),
        ActionId::CycleMode => Some(
            "Steps the session mode: Normal -> Plan -> Always-Approve -> Normal.\nPlan keeps the agent planning first and writes no files; Always-Approve runs every tool call without asking.\nCtrl+O toggles auto-approve directly.",
        ),
        ActionId::DashboardCycleMode => Some(
            "Cycles the dispatch mode for agents you launch from the dashboard: Normal, Plan, then Always-Approve.\nPlan has new agents plan before changing files; Always-Approve runs their tools without prompting.\nMirrors the in-session Shift+Tab cycle, applied to new dispatches.",
        ),
        ActionId::DashboardExit => Some(
            "Closes the dashboard and returns to where you were.\nEsc is a cascade: it first dismisses an open peek or clears an active filter, and only exits once nothing else is pending.\nRebind this action to a different key to exit directly.",
        ),
        ActionId::DashboardOpenLocationPicker => Some(
            "Opens a picker to set the working directory that newly dispatched dashboard agents run in.\nLaunch agents against a different repo or folder without leaving the dashboard.\nAffects new dispatches only, not agents already running.",
        ),
        ActionId::DashboardOverlayExit => Some(
            "Leaves the attached session overlay and returns to the dashboard list, without stopping the agent.\nAlso reachable via q on the scrollback, a neutral Esc, or the close button.\nTo stop the agent instead of just detaching, use Ctrl+X.",
        ),
        ActionId::DashboardOverlayStop => Some(
            "Inside a session overlay, stops the attached agent and closes it, returning you to the dashboard list.\nRequires confirmation: press Ctrl+X twice.\nCtrl+. still opens the cheatsheet here; only Ctrl+X is taken over by stop.",
        ),
        ActionId::DashboardStop => Some(
            "On a busy top-level row, Ctrl+X cancels the running turn. Once the row is idle, press Ctrl+X again within 2s to permanently delete the session.\nOn a subagent row, Ctrl+X kills the subagent.",
        ),
        ActionId::DashboardToggleAutoApprove => Some(
            "Toggles auto-approve (YOLO) for the selected agent right from the dashboard, without attaching to it.\nWhile on, that agent runs every tool call with no per-action confirmation.\nThe per-session equivalent is Ctrl+O inside a session.",
        ),
        ActionId::DashboardToggleGrouping => Some(
            "Switches the dashboard between a flat list and rows grouped by state, such as working versus idle.\nGrouping surfaces the agents that need attention; the flat list keeps a stable order.\nYour choice persists across sessions.",
        ),
        ActionId::DashboardTogglePin => Some(
            "Pins or unpins the selected agent so it stays at the top of the list regardless of sorting or grouping.\nKeep the agents you care about in view as others come and go.\nPins persist across dashboard sessions.",
        ),
        ActionId::DashboardToggleWorktree => Some(
            "Arms the next dashboard-dispatched agent to spawn in a fresh git worktree, isolating its work on a separate checkout.\nOnly applies when the working directory is a git repo.\nAffects newly dispatched agents, not ones already running.",
        ),
        ActionId::EditPromptExternal => Some(
            "Opens the current prompt draft in $VISUAL or $EDITOR, falling back to vi when neither is set.\nSaving and closing the editor returns the updated text to the composer; it does not send the prompt.\nAvailable in minimal mode for ordinary attachment-free drafts.",
        ),
        ActionId::ExpandAllThinking => Some(
            "Shows or hides the agent's reasoning (thinking) blocks across the whole transcript in one keypress.\nReveal how the agent reached an answer, or hide reasoning to focus on results.\nSeparate from E, which folds every entry regardless of type.",
        ),
        ActionId::FocusScrollback => Some(
            "Moves focus from the prompt to the scrollback so you can navigate the transcript.\nTab works in both simple and vim scrollback modes.\nEsc is reserved for the cancel / clear / rewind policy, not focus.",
        ),
        ActionId::InterjectPrompt => Some(
            "Sends a message to the agent mid-turn without cancelling it (interject), so you can steer or add context while it keeps working.\nPlain Enter while a turn is running queues a follow-up for later; this chord merges composer text into the current turn instead.\nWith an empty composer, bare Enter (or this chord) force-sends the top queued follow-up from the prompt: no need to focus the queue pane. On the queue pane, this chord force-sends the selected row.\nReach for it to correct course without losing the turn's progress.",
        ),
        ActionId::KillBgTask => Some(
            "Terminates the background task owned by the selected task block (e.g. a long shell command sent to the background).\nReach for it to stop a runaway or no-longer-needed process.\nApplies only to a live task; finished ones are unaffected.",
        ),
        ActionId::ModelPicker => Some(
            "Opens the model picker to switch the model for this session; the choice applies to later turns.\nBound to Ctrl+M, but while the prompt is focused that chord toggles multiline instead.\nReach it from the scrollback or the command palette.",
        ),
        ActionId::NewSession => Some(
            "Starts a fresh session with empty scrollback and context.\nRequires confirmation: press it twice (the first press arms, the second starts)\nso you don't discard the current conversation by accident.",
        ),
        ActionId::OpenBlockViewer => Some(
            "Opens the selected block in a focused, scrollable full-screen viewer.\nBest for long tool output, large files, or code you want to read away from the surrounding transcript.\nEsc returns to the conversation.",
        ),
        ActionId::OpenDashboard => Some(
            "Opens the Agent Dashboard: a list of all your running and recent agents to monitor and switch between.\nWorks from anywhere, including the welcome screen and inside a session.\nFrom there you can dispatch, attach, stop, group, and reorder agents.",
        ),
        ActionId::OpenExtensions => Some(
            "Opens the extensions manager for MCP servers and plugins: see what's connected and the tools they add.\nUse it to confirm an integration loaded or browse available tools.\nDistinct from settings, which holds general app options.",
        ),
        ActionId::OpenSessions => Some(
            "Opens the session browser to resume or switch between past conversations.\nSelect one to reattach to its full history. `/resume` does the same.\nSeparate from the Agent Dashboard (Ctrl+\\), which manages many live agents at once.",
        ),
        ActionId::Quit => Some(
            "Exits the app. Requires confirmation: press twice in quick succession;\na lone press is treated as a stray key and ignored.\nBound to Ctrl+Q, with Ctrl+D as an alias (Ctrl+D is primary in VS Code's terminal).",
        ),
        ActionId::Rewind => Some(
            "Rewinds the conversation to an earlier turn, discarding later turns. File changes made after that turn are left as-is.\nPick a turn from the list; a running turn is offered for cancel first. When Confirm before rewind is on (default), each pick asks Yes / Yes, and don't ask again / No. Picking \"Yes, and don't ask again\" turns the setting off in /settings.\nDestructive: later turns are dropped.\nAlso reachable idle with an empty prompt via Esc Esc (within 800ms), same as `/rewind`.",
        ),
        ActionId::ShortcutsHelp => Some(
            "Opens this keyboard cheatsheet.\nBrowse with j/k, expand a row's inline help with e, or press Enter for a shortcut's full detail page.\nBound to both Ctrl+. and Ctrl+X; the bar advertises whichever your terminal sends reliably.",
        ),
        ActionId::StashPrompt => Some(
            "Stash your current prompt as a draft.\nCtrl+S sets the draft aside and clears the composer. Ctrl+S on an empty composer restores it. The draft also restores by itself after you send your next prompt. Use Alt+S if your terminal swallows Ctrl+S.\nOne draft at a time: a new stash replaces the old one.",
        ),
        ActionId::ToggleExpandAll => Some(
            "Folds or unfolds every scrollback entry at once, unlike e which toggles only the selected row.\nCollapse a long transcript to scan headers, then expand it all back.\nThinking blocks have their own toggle, Ctrl+E.",
        ),
        ActionId::ToggleFold => Some(
            "Folds or unfolds the selected scrollback entry to hide or show its full body.\nHandy for skimming long tool output or reasoning.\nRelated: E folds/unfolds every entry, Ctrl+E toggles all thinking blocks.",
        ),
        ActionId::ToggleMultiline => Some(
            "Toggles a persistent multi-line prompt so the editor stays expanded for composing longer messages.\nInsert newlines with Shift+Enter or Alt+Enter (or a trailing backslash); bare Enter still sends.\nCtrl+M toggles multiline in the prompt; off the prompt it opens the model picker.",
        ),
        ActionId::ToggleQueue => Some(
            "Shows or hides the prompt queue.\nThe queue lets you line up follow-up prompts while a turn is running; each is sent automatically when the agent finishes.\nLocal macOS VS Code family: Ctrl+4 primary (Ctrl+; / Ctrl+' alts). Otherwise Ctrl+; with Ctrl+' alt.",
        ),
        ActionId::ToggleRaw => Some(
            "Switches the selected entry between rendered markdown and its raw source text.\nUse it to copy exact markdown, inspect a link target, or see formatting the renderer hides.\nPress again to return to the rendered view.",
        ),
        ActionId::ToggleTasks => Some(
            "Shows or hides the tasks pane, which lists background tasks and their status.\nUse it to monitor or return to work you sent to the background with Ctrl+B.\nA side pane; toggle off to reclaim width.",
        ),
        ActionId::ToggleTodos => Some(
            "Shows or hides the todo pane: the agent's live task checklist for the current work.\nWatch what it plans to do and what's left as the turn runs.\nA side pane; toggle it off to reclaim width.",
        ),
        ActionId::ToggleYolo => Some(
            "Turns auto-approve (YOLO) on or off for this session.\nWhile on, the agent runs every tool call (edits, shell, deletes) with no per-action confirmation.\nSame state as the Shift+Tab cycle's Always-Approve; use with care.",
        ),
        ActionId::VoiceToggle => Some(
            "Microphone capture for dictation, bound to Ctrl+Space (or F8: handy where Ctrl+Space is taken, e.g. macOS input-source switching; use Fn+F8 on a laptop).\nBehavior follows the Voice capture setting: toggle (press to start, press again to stop) or hold-to-talk (hold to record, release to stop), where hold needs a Kitty-protocol terminal and falls back to toggle elsewhere. `/voice` toggles everywhere.\nSpeech is transcribed straight into the prompt.",
        ),
        _ => None,
    }
}

fn long_help_vi(id: ActionId) -> Option<&'static str> {
    match id {
        ActionId::BashMode => Some(
            "Chạy lệnh shell trong chat: gõ ! đầu prompt trống, rồi lệnh.\nOutput được ghi vào scrollback.\nXóa ! để về prompt thường.",
        ),
        ActionId::CancelTurn => Some(
            "Ngắt lượt agent hiện tại và dừng generate, giữ phiên mở.\nCtrl+C hủy khi prompt trống; có nháp thì xóa prompt trước, lượt vẫn chạy.\nDừng lượt, không thoát app; dùng phím thoát để thoát.",
        ),
        ActionId::CommandPalette => Some(
            "Tìm mờ mọi action và slash command, chạy theo tên.\nHữu ích khi quên phím tắt.\nCũng mở bằng ? khi focus scrollback.",
        ),
        ActionId::CopyBlockContent => Some(
            "Sao chép nội dung khối đang chọn: tin nhắn, output tool, hoặc code block.\nChỉ có trên khối hỗ trợ copy.\nChỉ cần lệnh/đường dẫn: dùng Y.",
        ),
        ActionId::CopyBlockMeta => Some(
            "Chỉ copy định danh khối: dòng lệnh tool hoặc đường dẫn file, không phải body.\nTiện chạy lại lệnh hoặc dán path.\nDùng y thường để copy toàn bộ nội dung.",
        ),
        ActionId::CycleMode => Some(
            "Xoay chế độ phiên: Thường → Kế hoạch → Luôn duyệt → Thường.\nKế hoạch: agent lên plan trước, không ghi file; Luôn duyệt: chạy mọi tool không hỏi.\nCtrl+O bật/tắt always-approve trực tiếp.",
        ),
        ActionId::DashboardCycleMode => Some(
            "Xoay chế độ dispatch agent mới từ dashboard: Thường, Kế hoạch, Luôn duyệt.\nKế hoạch: plan trước khi sửa file; Luôn duyệt: tool không hỏi.\nGiống chu kỳ Shift+Tab trong phiên, áp cho dispatch mới.",
        ),
        ActionId::DashboardExit => Some(
            "Đóng dashboard và về chỗ trước đó.\nEsc theo tầng: đóng peek/filter trước, rồi mới thoát.\nĐổi phím action này để thoát thẳng.",
        ),
        ActionId::DashboardOpenLocationPicker => Some(
            "Mở picker đặt thư mục làm việc cho agent mới dispatch từ dashboard.\nChạy agent ở repo/folder khác mà không rời dashboard.\nChỉ ảnh hưởng dispatch mới.",
        ),
        ActionId::DashboardOverlayExit => Some(
            "Rời overlay phiên gắn kèm, về list dashboard, không dừng agent.\nCũng: q trên scrollback, Esc trung tính, hoặc nút đóng.\nMuốn dừng agent: Ctrl+X.",
        ),
        ActionId::DashboardOverlayStop => Some(
            "Trong overlay phiên, dừng agent gắn kèm và đóng, về list dashboard.\nCần xác nhận: Ctrl+X hai lần.\nCtrl+. vẫn mở cheatsheet; chỉ Ctrl+X dùng để dừng.",
        ),
        ActionId::DashboardStop => Some(
            "Dừng agent đang chọn và xóa hàng khỏi dashboard; ngắt lượt đang chạy trước.\nDọn agent xong/không cần mà không attach.\nTrong overlay (Ctrl+X) có xác nhận trước khi dừng.",
        ),
        ActionId::DashboardToggleAutoApprove => Some(
            "Bật/tắt always-approve (YOLO) cho agent đang chọn ngay từ dashboard, không cần attach.\nKhi bật, agent chạy mọi tool không hỏi.\nTrong phiên tương đương Ctrl+O.",
        ),
        ActionId::DashboardToggleGrouping => Some(
            "Chuyển dashboard giữa list phẳng và nhóm theo trạng thái (đang làm / idle).\nNhóm nổi agent cần chú ý; list phẳng giữ thứ tự ổn định.\nLựa chọn lưu qua phiên.",
        ),
        ActionId::DashboardTogglePin => Some(
            "Ghim/bỏ ghim agent để luôn ở đầu danh sách bất kể sort/group.\nGiữ agent quan trọng trong tầm nhìn.\nGhim lưu qua các lần mở dashboard.",
        ),
        ActionId::DashboardToggleWorktree => Some(
            "Arm agent dashboard tiếp theo spawn trong git worktree mới, cô lập checkout.\nChỉ khi working directory là git repo.\nChỉ agent mới, không đụng agent đang chạy.",
        ),
        ActionId::EditPromptExternal => Some(
            "Opens the current prompt draft in $VISUAL or $EDITOR, falling back to vi when neither is set.\nSaving and closing the editor returns the updated text to the composer; it does not send the prompt.\nAvailable in minimal mode for ordinary attachment-free drafts.",
        ),
        ActionId::ExpandAllThinking => Some(
            "Hiện hoặc ẩn mọi khối suy nghĩ (reasoning) của agent trên toàn transcript.\nXem agent suy luận thế nào, hoặc ẩn để tập trung kết quả.\nKhác E — E thu/mở mọi loại entry.",
        ),
        ActionId::FocusScrollback => Some(
            "Chuyển focus từ prompt sang scrollback để điều hướng transcript.\nTab hoạt động ở cả simple và vim scrollback.\nEsc dành cho clear/rewind (idle), không phải focus.",
        ),
        ActionId::InterjectPrompt => Some(
            "Gửi tin cho agent giữa lượt mà không hủy (interject), để chỉnh hướng hoặc thêm ngữ cảnh.\nEnter thường khi lượt chạy sẽ xếp follow-up; tổ hợp này gộp text composer vào lượt hiện tại.\nComposer trống: Enter (hoặc chord) force-gửi follow-up đầu hàng đợi.\nDùng để chỉnh hướng mà không mất tiến độ lượt.",
        ),
        ActionId::KillBgTask => Some(
            "Dừng tác vụ nền của khối task đang chọn (vd. lệnh shell chạy nền).\nDùng để dừng process runaway hoặc không còn cần.\nChỉ tác động task đang sống.",
        ),
        ActionId::ModelPicker => Some(
            "Mở chọn mô hình cho phiên; áp dụng các lượt sau.\nGán Ctrl+M, nhưng khi focus prompt chord đó bật multiline.\nVào từ scrollback hoặc command palette.",
        ),
        ActionId::NewSession => Some(
            "Bắt đầu phiên mới với scrollback và context trống.\nCần xác nhận: nhấn hai lần (lần 1 chờ, lần 2 bắt đầu)\nđể không vô tình bỏ hội thoại hiện tại.",
        ),
        ActionId::OpenBlockViewer => Some(
            "Mở khối đang chọn trong trình xem toàn màn hình, cuộn được.\nTốt cho output tool dài, file lớn, hoặc code cần đọc riêng.\nEsc để về hội thoại.",
        ),
        ActionId::OpenDashboard => Some(
            "Mở Agent Dashboard: danh sách agent đang chạy/gần đây để theo dõi và chuyển.\nHoạt động từ welcome và trong phiên.\nDispatch, attach, dừng, nhóm, sắp xếp agent.",
        ),
        ActionId::OpenExtensions => Some(
            "Mở quản lý extension cho MCP và plugin: xem kết nối và tool thêm.\nXác nhận integration đã load hoặc duyệt tool.\nKhác Settings (tùy chọn chung).",
        ),
        ActionId::OpenSessions => Some(
            "Mở trình duyệt phiên để resume hoặc chuyển hội thoại cũ.\nChọn một phiên để gắn lại lịch sử đầy đủ.\nKhác Agent Dashboard (Ctrl+\\) quản lý nhiều agent sống.",
        ),
        ActionId::Quit => Some(
            "Thoát app. Cần xác nhận: nhấn hai lần liên tiếp;\nnhấn một lần coi như phím lạc.\nGán Ctrl+Q, alias Ctrl+D (Ctrl+D chính trong terminal VS Code).",
        ),
        ActionId::Rewind => Some(
            "Hoàn tác hội thoại về lượt trước, khôi phục snapshot file lúc đó và bỏ thay đổi sau.\nChọn lượt và phạm vi khôi phục (tất cả, chỉ hội thoại, hoặc chỉ file); lượt đang chạy có thể hủy trước.\nPhá hủy: các lượt sau bị xóa.\nCũng vào được khi idle prompt trống bằng Esc Esc (trong 800ms), giống /rewind.",
        ),
        ActionId::ShortcutsHelp => Some(
            "Mở bảng phím tắt này.\nDuyệt j/k, mở rộng help bằng e, Enter xem trang chi tiết.\nGán Ctrl+. và Ctrl+X; thanh gợi ý phím terminal gửi ổn định.",
        ),
        ActionId::StashPrompt => Some(
            "Stash your current prompt as a draft.\nCtrl+S sets the draft aside and clears the composer. Ctrl+S on an empty composer restores it. The draft also restores by itself after you send your next prompt. Use Alt+S if your terminal swallows Ctrl+S.\nOne draft at a time: a new stash replaces the old one.",
        ),
        ActionId::ToggleExpandAll => Some(
            "Thu gọn hoặc mở rộng mọi mục scrollback cùng lúc, khác e chỉ tác động hàng đang chọn.\nThu gọn transcript dài để quét tiêu đề, rồi mở lại tất cả.\nKhối suy nghĩ có phím riêng Ctrl+E.",
        ),
        ActionId::ToggleFold => Some(
            "Thu gọn hoặc mở rộng mục scrollback đang chọn để ẩn/hiện toàn bộ nội dung.\nHữu ích khi lướt output tool hoặc reasoning dài.\nLiên quan: E thu/mở mọi mục, Ctrl+E bật/tắt mọi khối suy nghĩ.",
        ),
        ActionId::ToggleMultiline => Some(
            "Bật/tắt prompt nhiều dòng cố định để soạn tin dài.\nXuống dòng bằng Shift+Enter hoặc Alt+Enter (hoặc \\ cuối dòng); Enter gửi.\nCtrl+M bật multiline ở prompt; ngoài prompt thì mở model picker.",
        ),
        ActionId::ToggleQueue => Some(
            "Hiện/ẩn hàng đợi prompt.\nXếp prompt follow-up trong khi lượt đang chạy; gửi tự động khi agent xong.\nVS Code macOS local: Ctrl+4 chính (Ctrl+; / Ctrl+' phụ). Còn lại Ctrl+; với Ctrl+' phụ.",
        ),
        ActionId::ToggleRaw => Some(
            "Chuyển mục đang chọn giữa markdown đã render và mã nguồn thô.\nDùng để copy markdown đúng, xem link, hoặc định dạng renderer ẩn.\nNhấn lại để về bản đã render.",
        ),
        ActionId::ToggleTasks => Some(
            "Hiện/ẩn panel tác vụ nền và trạng thái.\nTheo dõi hoặc quay lại việc đã gửi nền bằng Ctrl+G.\nPanel bên; tắt để lấy lại chiều ngang.",
        ),
        ActionId::ToggleTodos => Some(
            "Hiện/ẩn panel todo: checklist việc agent đang làm.\nTheo dõi kế hoạch và phần còn lại khi lượt chạy.\nPanel bên; tắt để lấy lại chiều ngang.",
        ),
        ActionId::ToggleYolo => Some(
            "Bật/tắt always-approve (YOLO) cho phiên này.\nKhi bật, agent chạy mọi tool (sửa, shell, xóa) không hỏi từng bước.\nCùng trạng thái Always-Approve của Shift+Tab; dùng cẩn thận.",
        ),
        ActionId::VoiceToggle => Some(
            "Thu mic dictation, gán Ctrl+Space (hoặc F8 — khi Ctrl+Space bị chiếm, vd. đổi IME macOS).\nTheo cài Voice capture: bật/tắt hoặc giữ-để-nói (cần terminal Kitty-protocol).\n/voice bật/tắt mọi nơi. Lời nói được chuyển thẳng vào prompt.",
        ),
        _ => long_help_en(id),
    }
}
