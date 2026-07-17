#!/usr/bin/env node
/**
 * Convert source-logo.png into braille logoNN.txt (jimp 0.22 / CommonJS).
 */
const fs = require("fs");
const path = require("path");
const Jimp = require("jimp");

const HERE = __dirname;
const SRC = process.argv[2]
  ? path.resolve(process.argv[2])
  : path.join(HERE, "source-logo.png");
const OUT = HERE;

const DOTS = [
  [0, 0],
  [0, 1],
  [0, 2],
  [1, 0],
  [1, 1],
  [1, 2],
  [0, 3],
  [1, 3],
];
const BITS = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];

function lum(r, g, b) {
  return Math.max(r, g, b);
}

async function loadCropped(srcPath) {
  const img = await Jimp.read(srcPath);
  const w = img.bitmap.width;
  const h = img.bitmap.height;
  let minX = w,
    minY = h,
    maxX = 0,
    maxY = 0;
  let found = false;
  img.scan(0, 0, w, h, function (x, y, idx) {
    const r = this.bitmap.data[idx];
    const g = this.bitmap.data[idx + 1];
    const b = this.bitmap.data[idx + 2];
    const a = this.bitmap.data[idx + 3];
    if (a > 30 && lum(r, g, b) > 90) {
      found = true;
      minX = Math.min(minX, x);
      minY = Math.min(minY, y);
      maxX = Math.max(maxX, x);
      maxY = Math.max(maxY, y);
    }
  });
  if (!found) return img;
  const pad = 24;
  const x = Math.max(0, minX - pad);
  const y = Math.max(0, minY - pad);
  const cw = Math.min(w, maxX + 1 + pad) - x;
  const ch = Math.min(h, maxY + 1 + pad) - y;
  return img.crop(x, y, cw, ch);
}

function toBraille(binary, bw, bh) {
  const padW = (2 - (bw % 2)) % 2;
  const padH = (4 - (bh % 4)) % 4;
  const W = bw + padW;
  const H = bh + padH;
  const data = new Uint8Array(W * H);
  for (let y = 0; y < bh; y++) {
    for (let x = 0; x < bw; x++) {
      data[y * W + x] = binary[y * bw + x];
    }
  }
  const lines = [];
  for (let y = 0; y < H; y += 4) {
    let row = "";
    for (let x = 0; x < W; x += 2) {
      let code = 0;
      for (let i = 0; i < 8; i++) {
        const [dx, dy] = DOTS[i];
        if (data[(y + dy) * W + (x + dx)]) code |= BITS[i];
      }
      row += String.fromCharCode(0x2800 + code);
    }
    row = row.replace(/\u2800+$/g, "");
    lines.push(row || "\u2800");
  }
  while (lines.length && [...lines[0]].every((c) => c === "\u2800")) lines.shift();
  while (
    lines.length &&
    [...lines[lines.length - 1]].every((c) => c === "\u2800")
  )
    lines.pop();
  return lines.filter((l) => [...l].some((c) => c !== "\u2800"));
}

function renderRows(img, targetRows, thr, dilate) {
  const aspect = img.bitmap.width / Math.max(img.bitmap.height, 1);
  let ph = Math.max(4, targetRows * 4);
  let pw = Math.max(2, Math.round(ph * aspect));
  if (pw % 2) pw += 1;

  const up = 4;
  const big = img.clone().resize(pw * up, ph * up, Jimp.RESIZE_BILINEAR);
  const bw = big.bitmap.width;
  const bh = big.bitmap.height;
  let cur = new Uint8Array(bw * bh);
  big.scan(0, 0, bw, bh, function (x, y, idx) {
    const r = this.bitmap.data[idx];
    const g = this.bitmap.data[idx + 1];
    const b = this.bitmap.data[idx + 2];
    const a = this.bitmap.data[idx + 3];
    const v = a < 20 ? 0 : lum(r, g, b);
    cur[y * bw + x] = v >= thr ? 255 : 0;
  });

  for (let d = 0; d < dilate; d++) {
    const next = new Uint8Array(cur.length);
    for (let y = 0; y < bh; y++) {
      for (let x = 0; x < bw; x++) {
        let m = 0;
        for (let dy = -1; dy <= 1; dy++) {
          for (let dx = -1; dx <= 1; dx++) {
            const nx = x + dx,
              ny = y + dy;
            if (nx >= 0 && nx < bw && ny >= 0 && ny < bh) {
              m = Math.max(m, cur[ny * bw + nx]);
            }
          }
        }
        next[y * bw + x] = m;
      }
    }
    cur = next;
  }

  const binary = new Uint8Array(pw * ph);
  for (let y = 0; y < ph; y++) {
    for (let x = 0; x < pw; x++) {
      const sx = Math.min(bw - 1, Math.floor((x + 0.5) * (bw / pw)));
      const sy = Math.min(bh - 1, Math.floor((y + 0.5) * (bh / ph)));
      binary[y * pw + x] = cur[sy * bw + sx] > 127 ? 1 : 0;
    }
  }
  return toBraille(binary, pw, ph);
}

function makeLogo(img, targetRows) {
  let dilate = targetRows <= 6 ? 3 : 2;
  let best = null;
  for (const thr of [85, 95, 105, 75, 60, 120, 50]) {
    for (const d of [dilate, dilate + 1, Math.max(1, dilate - 1), dilate + 2]) {
      const lines = renderRows(img, targetRows, thr, d);
      if (!lines.length) continue;
      const n = lines.length;
      const ink = lines.reduce(
        (s, line) => s + [...line].filter((c) => c !== "\u2800").length,
        0
      );
      const score = Math.abs(n - targetRows);
      if (!best || score < best.score || (score === best.score && ink > best.ink)) {
        best = { score, ink, lines };
      }
      if (score === 0 && ink >= targetRows * 3) break;
    }
    if (best && best.score === 0) break;
  }
  if (!best) throw new Error("failed for " + targetRows);
  let lines = best.lines;
  if (lines.length < targetRows) {
    lines = lines.concat(Array(targetRows - lines.length).fill("\u2800"));
  } else {
    lines = lines.slice(0, targetRows);
  }
  return lines.join("\n") + "\n";
}

async function main() {
  if (!fs.existsSync(SRC)) {
    console.error("missing", SRC);
    process.exit(1);
  }
  // Ensure jimp can resolve from temp install
  const img = await loadCropped(SRC);
  console.log("source cropped:", img.bitmap.width, "x", img.bitmap.height);
  for (const s of [4, 5, 6, 7, 8, 9, 10, 12, 16, 20, 24]) {
    const art = makeLogo(img, s);
    const outPath = path.join(OUT, `logo${String(s).padStart(2, "0")}.txt`);
    fs.writeFileSync(outPath, art, "utf8");
    const lines = art.trimEnd().split("\n");
    console.log(
      `wrote ${path.basename(outPath)}: ${lines.length} lines, width ${Math.max(...lines.map((l) => l.length))}`
    );
    if (s === 5 || s === 7) process.stdout.write(art + "\n");
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
