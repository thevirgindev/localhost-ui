// Wraps the generated PNGs into a Windows .ico (PNG-compressed entries).
const zlib = require("zlib");
const fs = require("fs");
const path = require("path");

function makePng(size) {
  // reuse the drawing code at a smaller scale by re-rendering with SIZE=size
  const code = fs.readFileSync(path.join(__dirname, "make-icon.cjs"), "utf8");
  const patched = code.replace("const SIZE = 1024;", `const SIZE = ${size};`).replace(
    /const out = path\.join[\s\S]*?console\.log[\s\S]*?;/,
    "module.exports = png;"
  );
  const mod = { exports: {} };
  new Function("module", "exports", "require", "__dirname", patched)(
    mod,
    mod.exports,
    require,
    __dirname
  );
  return mod.exports;
}

function crc32(buf) {
  let c;
  let crc = 0xffffffff;
  for (let n = 0; n < buf.length; n++) {
    c = (crc ^ buf[n]) & 0xff;
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    crc = (crc >>> 8) ^ c;
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function pngChunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length);
  const typeBuf = Buffer.from(type, "ascii");
  const crcBuf = Buffer.alloc(4);
  crcBuf.writeUInt32BE(crc32(Buffer.concat([typeBuf, data])));
  return Buffer.concat([len, typeBuf, data, crcBuf]);
}

function makeIcoPng(size) {
  // Render icon at `size` using same shape math as make-icon.
  const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
  function rrect(x, y, w, h, r, px, py) {
    const cx = clamp(px, x + r, x + w - r);
    const cy = clamp(py, y + r, y + h - r);
    const dx = px - cx;
    const dy = py - cy;
    const dist = Math.sqrt(dx * dx + dy * dy) - r;
    return dist <= 0 ? 1 : clamp(0.5 - dist, 0, 1);
  }
  const bg = [10, 12, 16, 255];
  const purple = [168, 85, 247, 255];
  const dark = [13, 13, 13, 255];
  function diamond(cx, cy, a, b, px, py) {
    const d = Math.abs(px - cx) / a + Math.abs(py - cy) / b;
    return clamp((1 - d) * 40, 0, 1);
  }
  function circle(cx, cy, r, px, py) {
    const d = Math.hypot(px - cx, py - cy) - r * s;
    return clamp(0.5 - d, 0, 1);
  }
  const raw = Buffer.alloc(size * (size * 4 + 1));
  const s = size / 1024;
  for (let py = 0; py < size; py++) {
    const rowStart = py * (size * 4 + 1);
    raw[rowStart] = 0;
    for (let px = 0; px < size; px++) {
      const i = rowStart + 1 + px * 4;
      const outer = rrect(32 * s, 32 * s, size - 64 * s, size - 64 * s, 200 * s, px, py);
      const inner = rrect(160 * s, 160 * s, size - 320 * s, size - 320 * s, 130 * s, px, py);
      const sparkle = Math.max(
        diamond(512 * s, 512 * s, 95 * s, 300 * s, px, py),
        diamond(512 * s, 512 * s, 300 * s, 95 * s, px, py),
      );
      const dot = circle(720 * s, 330 * s, 62, px, py);
      const mark = Math.max(sparkle, dot);
      let r = bg[0], g = bg[1], b = bg[2], a = bg[3] * outer;
      r = r * (1 - inner) + purple[0] * inner;
      g = g * (1 - inner) + purple[1] * inner;
      b = b * (1 - inner) + purple[2] * inner;
      r = r * (1 - mark) + dark[0] * mark;
      g = g * (1 - mark) + dark[1] * mark;
      b = b * (1 - mark) + dark[2] * mark;
      raw[i] = Math.round(clamp(r, 0, 255));
      raw[i + 1] = Math.round(clamp(g, 0, 255));
      raw[i + 2] = Math.round(clamp(b, 0, 255));
      raw[i + 3] = Math.round(clamp(a, 0, 255));
    }
  }
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(size, 0);
  ihdr.writeUInt32BE(size, 4);
  ihdr[8] = 8;
  ihdr[9] = 6;
  const png = Buffer.concat([
    Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    pngChunk("IHDR", ihdr),
    pngChunk("IDAT", zlib.deflateSync(raw, { level: 9 })),
    pngChunk("IEND", Buffer.alloc(0)),
  ]);
  return png;
}

const sizes = [16, 24, 32, 48, 64, 128, 256];
const entries = [];
for (const size of sizes) {
  entries.push({ size, png: makeIcoPng(size) });
}

// ICO container
const header = Buffer.alloc(6);
header.writeUInt16LE(0, 0);
header.writeUInt16LE(1, 2);
header.writeUInt16LE(entries.length, 4);

const dirEntries = [];
let offset = 6 + entries.length * 16;
for (const e of entries) {
  const d = Buffer.alloc(16);
  d[0] = e.size === 256 ? 0 : e.size;
  d[1] = e.size === 256 ? 0 : e.size;
  d[2] = 0;
  d[3] = 0;
  d.writeUInt16LE(1, 4);
  d.writeUInt16LE(32, 6);
  d.writeUInt32LE(e.png.length, 8);
  d.writeUInt32LE(offset, 12);
  offset += e.png.length;
  dirEntries.push(d);
}

const ico = Buffer.concat([header, ...dirEntries, ...entries.map((e) => e.png)]);
const out = path.join(__dirname, "..", "src-tauri", "icons", "icon.ico");
fs.writeFileSync(out, ico);
console.log("wrote", out, ico.length, "bytes");
