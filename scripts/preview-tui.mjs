#!/usr/bin/env node
/**
 * Interactive terminal mock of xVora welcome + Release Notes.
 * No cargo build — pure Node.
 *
 *   node scripts/preview-tui.mjs
 *   npm run preview:tui   (if wired)
 *
 * Keys:
 *   ↑/↓ or j/k   move menu
 *   Enter        open item
 *   Esc / q      back / quit
 *   c            toggle language label (en/vi chrome)
 *   1/2          logo size 5 / 7
 */
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import readline from "readline";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, "..");

function read(p) {
  try {
    return fs.readFileSync(p, "utf8").replace(/\r\n/g, "\n");
  } catch {
    return "";
  }
}

const logoDir = path.join(ROOT, "crates/codegen/xvora-pager/assets/logo");
const logos = {
  5: read(path.join(logoDir, "logo05.txt")).trimEnd(),
  7: read(path.join(logoDir, "logo07.txt")).trimEnd(),
};
const md = read(path.join(ROOT, "changelogs/CURRENT.external.md")).trimEnd();
let ver = "0.2.0";
try {
  ver = JSON.parse(read(path.join(ROOT, "changelogs/manifest.json"))).current_version || ver;
} catch {}

const bullets = md
  .split("\n")
  .map((l) => l.trim())
  .filter((l) => l.startsWith("- "))
  .map((l) => l.slice(2))
  .slice(0, 5);

const i18n = {
  vi: {
    changelog: "Nhật ký thay đổi",
    menu: [
      { id: "worktree", label: "Worktree mới", key: "ctrl+w" },
      { id: "resume", label: "Tiếp tục phiên", key: "ctrl+s" },
      { id: "notes", label: "Nhật ký thay đổi", key: "" },
      { id: "quit", label: "Thoát", key: "ctrl+q" },
    ],
    footer: "↑/↓ chọn  Enter mở  c:en/vi  1/2 logo  q thoát",
    notesFooter: "↑/↓ cuộn  Esc quay lại",
  },
  en: {
    changelog: "Changelog",
    menu: [
      { id: "worktree", label: "New worktree", key: "ctrl+w" },
      { id: "resume", label: "Resume session", key: "ctrl+s" },
      { id: "notes", label: "Changelog", key: "" },
      { id: "quit", label: "Quit", key: "ctrl+q" },
    ],
    footer: "↑/↓ select  Enter open  c:en/vi  1/2 logo  q quit",
    notesFooter: "↑/↓ scroll  Esc back",
  },
};

// ── state ──────────────────────────────────────────────────────────
let lang = "vi";
let logoSize = 7;
let screen = "welcome"; // welcome | notes | toast
let menuIdx = 0;
let notesScroll = 0;
let toast = "";
let toastUntil = 0;
let cols = process.stdout.columns || 80;
let rows = process.stdout.rows || 24;

// ── ANSI helpers ───────────────────────────────────────────────────
const ESC = "\x1b[";
const hide = () => process.stdout.write(`${ESC}?25l`);
const show = () => process.stdout.write(`${ESC}?25h`);
const clear = () => process.stdout.write(`${ESC}2J${ESC}H`);
const move = (r, c) => process.stdout.write(`${ESC}${r};${c}H`);
const style = {
  reset: `${ESC}0m`,
  dim: `${ESC}2m`,
  bold: `${ESC}1m`,
  orange: `${ESC}38;2;240;106;40m`,
  muted: `${ESC}38;2;140;140;140m`,
  text: `${ESC}38;2;220;220;220m`,
  green: `${ESC}38;2;61;214;140m`,
  blue: `${ESC}38;2;110;168;254m`,
  inv: `${ESC}7m`,
  border: `${ESC}38;2;80;80;80m`,
};

function stripAnsi(s) {
  return s.replace(/\x1b\[[0-9;]*m/g, "");
}
function pad(s, w) {
  const n = [...stripAnsi(s)].length;
  if (n >= w) return s;
  return s + " ".repeat(w - n);
}
function trunc(s, w) {
  const chars = [...s];
  if (chars.length <= w) return s;
  if (w <= 1) return "…";
  return chars.slice(0, w - 1).join("") + "…";
}

// ── frames ─────────────────────────────────────────────────────────
function frameWelcome() {
  const t = i18n[lang];
  const logo = logos[logoSize] || logos[7] || "";
  const logoLines = logo.split("\n");
  const logoW = Math.max(0, ...logoLines.map((l) => [...l].length));

  const bulletLines = bullets.map((b) => `  • ${trunc(b, Math.max(20, cols - 28 - logoW))}`);
  const title = `${style.bold}${style.text}xVora${style.reset} ${style.muted}Beta${style.reset}  ${style.muted}${ver}${style.reset}`;
  const head = [
    title,
    "",
    `${style.muted}${t.changelog}${style.reset}`,
    ...bulletLines.map((l) => `${style.muted}${l}${style.reset}`),
    "",
  ];

  const menuLines = t.menu.map((m, i) => {
    const sel = i === menuIdx;
    const label = sel
      ? `${style.inv}${style.bold} ${m.label} ${style.reset}`
      : `${style.text} ${m.label} ${style.reset}`;
    const key = m.key ? `${style.dim}${m.key}${style.reset}` : "";
    return { label, key, rawLabel: m.label, rawKey: m.key };
  });

  // Build left (logo) + right (content) as side-by-side
  const contentLines = [
    ...head,
    ...menuLines.map((m) => {
      const mid = Math.max(24, Math.min(48, cols - logoW - 16));
      return pad(m.label, mid) + m.key;
    }),
  ];

  const bodyH = Math.max(logoLines.length, contentLines.length);
  const leftPad = Math.max(2, Math.floor((cols - (logoW + 4 + 50)) / 2));
  const out = [];

  // top margin
  const topPad = Math.max(1, Math.floor((rows - bodyH - 6) / 2));
  for (let i = 0; i < topPad; i++) out.push("");

  const boxW = Math.min(cols - 4, logoW + 6 + 52);
  const innerW = boxW - 2;
  out.push(
    " ".repeat(Math.max(0, leftPad - 1)) +
      style.border +
      "┌" +
      "─".repeat(innerW) +
      "┐" +
      style.reset
  );

  for (let i = 0; i < bodyH; i++) {
    const L = logoLines[i] ?? "";
    const R = contentLines[i] ?? "";
    const logoPart =
      style.orange +
      pad(L, logoW) +
      style.reset +
      "  ";
    const row = logoPart + R;
    const visible = [...stripAnsi(row)].length;
    const padR = Math.max(0, innerW - visible - 1);
    out.push(
      " ".repeat(Math.max(0, leftPad - 1)) +
        style.border +
        "│" +
        style.reset +
        " " +
        row +
        " ".repeat(padR) +
        style.border +
        "│" +
        style.reset
    );
  }

  out.push(
    " ".repeat(Math.max(0, leftPad - 1)) +
      style.border +
      "└" +
      "─".repeat(innerW) +
      "┘" +
      style.reset
  );

  out.push("");
  out.push(center(`${style.dim}${t.footer}${style.reset}`, cols));

  if (toast && Date.now() < toastUntil) {
    out.push("");
    out.push(center(`${style.green}✓ ${toast}${style.reset}`, cols));
  }

  return out;
}

function notesLines() {
  const lines = [];
  for (const line of md.split("\n")) {
    if (line.startsWith("# ")) {
      lines.push({ kind: "h1", text: line.slice(2) });
    } else if (line.startsWith("## ")) {
      lines.push({ kind: "h2", text: line.slice(3) });
    } else if (line.startsWith("- ")) {
      lines.push({ kind: "li", text: line.slice(2) });
    } else if (line.trim() === "") {
      lines.push({ kind: "sp", text: "" });
    } else {
      lines.push({ kind: "p", text: line });
    }
  }
  return lines;
}

function frameNotes() {
  const t = i18n[lang];
  const boxW = Math.min(cols - 4, 72);
  const leftPad = Math.max(1, Math.floor((cols - boxW) / 2));
  const inner = boxW - 2;
  const contentH = Math.max(8, rows - 8);
  const parsed = notesLines();
  const maxScroll = Math.max(0, parsed.length - contentH);
  notesScroll = Math.min(notesScroll, maxScroll);

  const out = [];
  const title = " Release Notes ";
  const side = Math.max(0, Math.floor((inner - title.length) / 2));
  const sideR = Math.max(0, inner - title.length - side);
  out.push("");
  out.push(
    " ".repeat(leftPad) +
      style.border +
      "┌" +
      "─".repeat(side) +
      title +
      "─".repeat(sideR) +
      "┐" +
      style.reset
  );

  const view = parsed.slice(notesScroll, notesScroll + contentH);
  for (let i = 0; i < contentH; i++) {
    const item = view[i];
    let text = "";
    if (!item) {
      text = "";
    } else if (item.kind === "h1") {
      text = `${style.green}${style.bold}${item.text}${style.reset}`;
    } else if (item.kind === "h2") {
      text = `${style.blue}${item.text}${style.reset}`;
    } else if (item.kind === "li") {
      text = `${style.text}  • ${trunc(item.text, inner - 6)}${style.reset}`;
    } else {
      text = `${style.muted}${trunc(item.text, inner - 2)}${style.reset}`;
    }
    const vis = [...stripAnsi(text)].length;
    const padR = Math.max(0, inner - vis);
    out.push(
      " ".repeat(leftPad) +
        style.border +
        "│" +
        style.reset +
        text +
        " ".repeat(padR) +
        style.border +
        "│" +
        style.reset
    );
  }

  out.push(
    " ".repeat(leftPad) + style.border + "└" + "─".repeat(inner) + "┘" + style.reset
  );
  out.push("");
  out.push(center(`${style.dim}${t.notesFooter}${style.reset}`, cols));
  return out;
}

function center(s, w) {
  const n = [...stripAnsi(s)].length;
  const padL = Math.max(0, Math.floor((w - n) / 2));
  return " ".repeat(padL) + s;
}

function draw() {
  cols = process.stdout.columns || 80;
  rows = process.stdout.rows || 24;
  const lines = screen === "notes" ? frameNotes() : frameWelcome();
  clear();
  // clip to terminal height
  const clipped = lines.slice(0, rows - 1);
  process.stdout.write(clipped.join("\n"));
  if (clipped.length < rows) process.stdout.write("\n".repeat(rows - clipped.length - 1));
  move(rows, 1);
}

function showToast(msg, ms = 1800) {
  toast = msg;
  toastUntil = Date.now() + ms;
  draw();
  setTimeout(() => {
    if (Date.now() >= toastUntil) draw();
  }, ms + 50);
}

function openMenu() {
  const id = i18n[lang].menu[menuIdx]?.id;
  if (id === "quit") {
    cleanup();
    process.exit(0);
  }
  if (id === "notes") {
    screen = "notes";
    notesScroll = 0;
    draw();
    return;
  }
  if (id === "worktree") {
    showToast(lang === "vi" ? "Mock: Worktree mới (preview)" : "Mock: New worktree (preview)");
    return;
  }
  if (id === "resume") {
    showToast(lang === "vi" ? "Mock: Tiếp tục phiên (preview)" : "Mock: Resume session (preview)");
    return;
  }
}

function onKey(key) {
  if (key === "\u0003") {
    // ctrl+c
    cleanup();
    process.exit(0);
  }

  if (screen === "notes") {
    if (key === "\u001b" || key === "q" || key === "Q") {
      screen = "welcome";
      draw();
      return;
    }
    if (key === "\u001b[A" || key === "k") {
      notesScroll = Math.max(0, notesScroll - 1);
      draw();
      return;
    }
    if (key === "\u001b[B" || key === "j") {
      notesScroll++;
      draw();
      return;
    }
    return;
  }

  // welcome
  if (key === "q" || key === "Q") {
    cleanup();
    process.exit(0);
  }
  if (key === "c" || key === "C") {
    lang = lang === "vi" ? "en" : "vi";
    menuIdx = Math.min(menuIdx, i18n[lang].menu.length - 1);
    draw();
    return;
  }
  if (key === "1") {
    logoSize = 5;
    draw();
    return;
  }
  if (key === "2") {
    logoSize = 7;
    draw();
    return;
  }
  if (key === "\u001b[A" || key === "k") {
    menuIdx = (menuIdx + i18n[lang].menu.length - 1) % i18n[lang].menu.length;
    draw();
    return;
  }
  if (key === "\u001b[B" || key === "j") {
    menuIdx = (menuIdx + 1) % i18n[lang].menu.length;
    draw();
    return;
  }
  if (key === "\r" || key === "\n") {
    openMenu();
    return;
  }
  // ctrl+w / ctrl+s / ctrl+q shortcuts
  if (key === "\u0017") {
    menuIdx = 0;
    openMenu();
    return;
  }
  if (key === "\u0013") {
    menuIdx = 1;
    openMenu();
    return;
  }
  if (key === "\u0011") {
    menuIdx = 3;
    openMenu();
    return;
  }
}

function cleanup() {
  try {
    if (process.stdin.isTTY) process.stdin.setRawMode(false);
  } catch {}
  show();
  process.stdout.write("\n");
}

function main() {
  if (!process.stdout.isTTY) {
    console.error("Cần chạy trong terminal TTY (Windows Terminal / PowerShell).");
    process.exit(1);
  }
  hide();
  readline.emitKeypressEvents(process.stdin);
  process.stdin.setRawMode(true);
  process.stdin.resume();
  process.stdin.setEncoding("utf8");

  let buf = "";
  process.stdin.on("data", (chunk) => {
    buf += chunk;
    // handle multi-byte escape sequences
    while (buf.length) {
      if (buf.startsWith("\u001b[")) {
        // wait for full CSI
        if (buf.length < 3) return;
        const seq = buf.slice(0, 3);
        buf = buf.slice(3);
        onKey(seq);
        continue;
      }
      const ch = buf[0];
      buf = buf.slice(1);
      onKey(ch);
    }
  });

  process.stdout.on("resize", draw);
  process.on("SIGINT", () => {
    cleanup();
    process.exit(0);
  });
  process.on("exit", cleanup);

  draw();
}

main();
