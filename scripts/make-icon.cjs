// Generates a 1024x1024 RGBA PNG app icon without any dependencies.
// Design: minimal monochrome — dark rounded square, white "L" monogram.
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
const white = [235, 235, 240, 255]; // #ebebf0

// Inside-test for a rounded bar (the L strokes) with a soft AA edge.
const barMask = roundedRectMask;

const raw = Buffer.alloc(SIZE * (SIZE * 4 + 1));

for (let py = 0; py < SIZE; py++) {
  const rowStart = py * (SIZE * 4 + 1);
  raw[rowStart] = 0; // filter: none
  for (let px = 0; px < SIZE; px++) {
    const i = rowStart + 1 + px * 4;

    // outer dark rounded square (with transparency outside)
    const outer = roundedRectMask(32, 32, SIZE - 64, SIZE - 64, 200, px, py);
    // white "L" monogram: vertical stem + horizontal foot, rounded ends
    const stem = barMask(330, 250, 130, 520, 65, px, py);
    const foot = barMask(330, 640, 390, 130, 65, px, py);
    const mark = Math.max(stem, foot);

    let r = 0, g = 0, b = 0, a = 0;
    // start from bg
    r = bg[0]; g = bg[1]; b = bg[2]; a = bg[3] * outer;
    // blend white monogram over it
    r = r * (1 - mark) + white[0] * mark;
    g = g * (1 - mark) + white[1] * mark;
    b = b * (1 - mark) + white[2] * mark;

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
