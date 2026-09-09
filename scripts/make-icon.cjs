// Generates a 1024x1024 RGBA PNG app icon without any dependencies.
// Design: dark rounded square, purple inner rounded square, dark sparkle mark.
const zlib = require("zlib");
const fs = require("fs");
const path = require("path");

const SIZE = 1024;

// --- drawing helpers -------------------------------------------------------
function clamp(v, lo, hi) {
  return Math.max(lo, Math.min(hi, v));
}

function roundedRectMask(x, y, w, h, r, px, py) {
  // 1 inside the rounded rect, 0 outside (with 1px AA edge)
  const cx = clamp(px, x + r, x + w - r);
  const cy = clamp(py, y + r, y + h - r);
  const dx = px - cx;
  const dy = py - cy;
  const inside = dx * dx + dy * dy <= r * r;
  const dist = Math.sqrt(dx * dx + dy * dy) - r;
  if (inside) return 1;
  return clamp(0.5 - dist, 0, 1);
}

const bg = [10, 12, 16, 255]; // #0a0c10
const purple = [168, 85, 247, 255]; // #a855f7
const dark = [13, 13, 13, 255]; // #0d0d0d

// Inside-test for a diamond (|dx|/a + |dy|/b <= 1) with a soft AA edge.
function diamondMask(cx, cy, a, b, px, py) {
  const d = Math.abs(px - cx) / a + Math.abs(py - cy) / b;
  return clamp((1 - d) * 40, 0, 1);
}

function circleMask(cx, cy, r, px, py) {
  const d = Math.hypot(px - cx, py - cy) - r;
  return clamp(0.5 - d, 0, 1);
}

const raw = Buffer.alloc(SIZE * (SIZE * 4 + 1));

for (let py = 0; py < SIZE; py++) {
  const rowStart = py * (SIZE * 4 + 1);
  raw[rowStart] = 0; // filter: none
  for (let px = 0; px < SIZE; px++) {
    const i = rowStart + 1 + px * 4;

    // outer dark rounded square (with transparency outside)
    const outer = roundedRectMask(32, 32, SIZE - 64, SIZE - 64, 200, px, py);
    // inner purple rounded square
    const inner = roundedRectMask(160, 160, SIZE - 320, SIZE - 320, 130, px, py);
    // dark 4-point sparkle in the middle (union of two thin diamonds)
    const sparkle = Math.max(
      diamondMask(512, 512, 95, 300, px, py),
      diamondMask(512, 512, 300, 95, px, py),
    );
    // small dark accent dot (top-right of the sparkle)
    const dot = circleMask(720, 330, 62, px, py);
    const mark = Math.max(sparkle, dot);

    let r = 0, g = 0, b = 0, a = 0;
    // start from bg
    r = bg[0]; g = bg[1]; b = bg[2]; a = bg[3] * outer;
    // blend purple over it
    r = r * (1 - inner) + purple[0] * inner;
    g = g * (1 - inner) + purple[1] * inner;
    b = b * (1 - inner) + purple[2] * inner;
    // blend dark sparkle over that
    r = r * (1 - mark) + dark[0] * mark;
    g = g * (1 - mark) + dark[1] * mark;
    b = b * (1 - mark) + dark[2] * mark;

    raw[i] = Math.round(clamp(r, 0, 255));
    raw[i + 1] = Math.round(clamp(g, 0, 255));
    raw[i + 2] = Math.round(clamp(b, 0, 255));
    raw[i + 3] = Math.round(clamp(a, 0, 255));
  }
}

// --- PNG encoding -----------------------------------------------------------
function crc32(buf) {
  let c, crc = 0xffffffff;
  for (let n = 0; n < buf.length; n++) {
    c = (crc ^ buf[n]) & 0xff;
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    crc = (crc >>> 8) ^ c;
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length);
  const typeBuf = Buffer.from(type, "ascii");
  const crcBuf = Buffer.alloc(4);
  crcBuf.writeUInt32BE(crc32(Buffer.concat([typeBuf, data])));
  return Buffer.concat([len, typeBuf, data, crcBuf]);
}

const ihdr = Buffer.alloc(13);
ihdr.writeUInt32BE(SIZE, 0);
ihdr.writeUInt32BE(SIZE, 4);
ihdr[8] = 8; // bit depth
ihdr[9] = 6; // color type RGBA
ihdr[10] = 0; // compression
ihdr[11] = 0; // filter
ihdr[12] = 0; // interlace

const png = Buffer.concat([
  Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
  chunk("IHDR", ihdr),
  chunk("IDAT", zlib.deflateSync(raw, { level: 9 })),
  chunk("IEND", Buffer.alloc(0)),
]);

const out = path.join(__dirname, "app-icon.png");
fs.writeFileSync(out, png);
console.log("wrote", out, png.length, "bytes");
