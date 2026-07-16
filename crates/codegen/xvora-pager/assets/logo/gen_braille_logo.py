#!/usr/bin/env python3
"""Convert source-logo.png into braille logoNN.txt assets for the TUI welcome screen."""

from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageFilter

HERE = Path(__file__).resolve().parent
SRC = HERE / "source-logo.png"
OUT = HERE

DOTS = [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (0, 3), (1, 3)]
BITS = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80]


def load_cropped(path: Path) -> Image.Image:
    img = Image.open(path).convert("RGBA")
    px = img.load()
    w, h = img.size
    min_x, min_y, max_x, max_y = w, h, 0, 0
    found = False
    for y in range(h):
        for x in range(w):
            r, g, b, a = px[x, y]
            if a > 30 and max(r, g, b) > 90:
                found = True
                min_x = min(min_x, x)
                min_y = min(min_y, y)
                max_x = max(max_x, x)
                max_y = max(max_y, y)
    if not found:
        bbox = img.getbbox() or (0, 0, w, h)
    else:
        pad = 20
        bbox = (
            max(0, min_x - pad),
            max(0, min_y - pad),
            min(w, max_x + 1 + pad),
            min(h, max_y + 1 + pad),
        )
    return img.crop(bbox)


def to_braille(binary: Image.Image) -> list[str]:
    bw, bh = binary.size
    pad_w = (2 - bw % 2) % 2
    pad_h = (4 - bh % 4) % 4
    if pad_w or pad_h:
        canvas = Image.new("1", (bw + pad_w, bh + pad_h), 0)
        canvas.paste(binary, (0, 0))
        binary = canvas
        bw, bh = binary.size
    data = binary.load()
    lines: list[str] = []
    for y in range(0, bh, 4):
        chars: list[str] = []
        for x in range(0, bw, 2):
            code = 0
            for i, (dx, dy) in enumerate(DOTS):
                if data[x + dx, y + dy] != 0:
                    code |= BITS[i]
            chars.append(chr(0x2800 + code))
        line = "".join(chars).rstrip("\u2800")
        lines.append(line if line else "\u2800")
    while lines and all(c == "\u2800" for c in lines[0]):
        lines.pop(0)
    while lines and all(c == "\u2800" for c in lines[-1]):
        lines.pop()
    # Drop purely blank interior rows (keep structure readable).
    # Keep at least one blank if sandwiched? No — drop empty.
    lines = [l for l in lines if any(c != "\u2800" for c in l)]
    return lines


def render_rows(
    img: Image.Image,
    target_rows: int,
    *,
    thr: int = 95,
    dilate: int = 2,
) -> list[str]:
    aspect = img.width / max(img.height, 1)
    ph = max(4, target_rows * 4)
    pw = max(2, int(round(ph * aspect)))
    if pw % 2:
        pw += 1

    # High-res threshold + dilate so sparse white dots survive downscale.
    up = 4
    big = img.resize((pw * up, ph * up), Image.Resampling.LANCZOS)
    gray = Image.new("L", big.size, 0)
    bp = big.load()
    gp = gray.load()
    for y in range(big.height):
        for x in range(big.width):
            r, g, b, a = bp[x, y]
            gp[x, y] = 0 if a < 20 else max(r, g, b)
    bw = gray.point(lambda v: 255 if v >= thr else 0, mode="L")
    for _ in range(max(0, dilate)):
        bw = bw.filter(ImageFilter.MaxFilter(3))
    small = bw.resize((pw, ph), Image.Resampling.NEAREST)
    binary = small.point(lambda v: 1 if v > 127 else 0, mode="1")
    return to_braille(binary)


def make_logo(img: Image.Image, target_rows: int) -> str:
    # Smaller sizes need more dilation so the X stays visible.
    if target_rows <= 6:
        dilate = 3
    elif target_rows <= 12:
        dilate = 2
    else:
        dilate = 2

    best: tuple[int, int, list[str]] | None = None
    for thr in (85, 95, 105, 75):
        for d in (dilate, dilate + 1, max(1, dilate - 1)):
            lines = render_rows(img, target_rows, thr=thr, dilate=d)
            if not lines:
                continue
            n = len(lines)
            ink = sum(1 for line in lines for c in line if c != "\u2800")
            score = abs(n - target_rows)
            cand = (score, -ink, lines)
            if best is None or cand[:2] < best[:2]:
                best = cand
            if score == 0 and ink >= target_rows * 3:
                break
        if best and best[0] == 0:
            break

    if best is None:
        raise RuntimeError(f"failed to render logo for {target_rows} rows")
    _, _, lines = best
    # Pad/trim to exact requested height for layout stability.
    if len(lines) < target_rows:
        # Prefer padding blank at bottom.
        lines = lines + ["\u2800"] * (target_rows - len(lines))
    else:
        lines = lines[:target_rows]
    return "\n".join(lines) + "\n"


def main() -> None:
    if not SRC.exists():
        raise SystemExit(f"missing source logo: {SRC}")
    img = load_cropped(SRC)
    print(f"source cropped size: {img.size}")
    sizes = [4, 5, 6, 7, 8, 9, 10, 12, 16, 20, 24]
    for s in sizes:
        art = make_logo(img, s)
        path = OUT / f"logo{s:02d}.txt"
        path.write_text(art, encoding="utf-8")
        lines = art.strip("\n").split("\n")
        width = max((len(l) for l in lines), default=0)
        print(f"wrote {path.name}: {len(lines)} lines, max width {width}")
        if s in (5, 7, 12):
            print(art)


if __name__ == "__main__":
    main()
