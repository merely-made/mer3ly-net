import fs from "node:fs";
import path from "node:path";

const PNG_SIGNATURE = Buffer.from([
  0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a,
]);
const RETAINED_CHUNKS = new Set([
  "IHDR",
  "PLTE",
  "IDAT",
  "IEND",
  "tRNS",
  "sRGB",
  "gAMA",
  "cHRM",
]);

const [input, output] = process.argv.slice(2);
if (!input || !output) {
  throw new Error(
    "Usage: node scripts/import-showcase-png.mjs INPUT.png OUTPUT.png",
  );
}

if (
  path.extname(input).toLowerCase() !== ".png" ||
  path.extname(output).toLowerCase() !== ".png"
) {
  throw new Error("showcase imports accept PNG input and output only");
}

const source = fs.readFileSync(input);
if (
  source.length < PNG_SIGNATURE.length ||
  !source.subarray(0, PNG_SIGNATURE.length).equals(PNG_SIGNATURE)
) {
  throw new Error("input does not have a valid PNG signature");
}

const outputChunks = [PNG_SIGNATURE];
const removedChunks = [];
let offset = PNG_SIGNATURE.length;
let sawHeader = false;
let sawImageData = false;
let sawEnd = false;

while (offset < source.length) {
  if (offset + 12 > source.length) {
    throw new Error("input ends inside a PNG chunk header");
  }

  const length = source.readUInt32BE(offset);
  const end = offset + 12 + length;
  if (end > source.length) {
    throw new Error("input ends inside a PNG chunk body");
  }

  const type = source.toString("ascii", offset + 4, offset + 8);
  if (!/^[A-Za-z]{4}$/.test(type)) {
    throw new Error("input contains an invalid PNG chunk type");
  }
  if (!sawHeader && type !== "IHDR") {
    throw new Error("IHDR is not the first PNG chunk");
  }
  if (type === "IHDR") {
    if (sawHeader || length !== 13) {
      throw new Error("input contains an invalid IHDR chunk");
    }
    sawHeader = true;
  }
  if (type === "IDAT") {
    sawImageData = true;
  }
  if (type === "IEND") {
    if (length !== 0) {
      throw new Error("input contains an invalid IEND chunk");
    }
    sawEnd = true;
  }

  if (RETAINED_CHUNKS.has(type)) {
    outputChunks.push(source.subarray(offset, end));
  } else {
    removedChunks.push(type);
  }

  offset = end;
  if (sawEnd) {
    break;
  }
}

if (!sawHeader || !sawImageData || !sawEnd || offset !== source.length) {
  throw new Error("input is not a complete single PNG image");
}

const normalized = Buffer.concat(outputChunks);
fs.mkdirSync(path.dirname(output), { recursive: true });
fs.writeFileSync(output, normalized);

process.stdout.write(
  `normalized ${path.basename(output)}: ${source.length} -> ${normalized.length} bytes; removed ${removedChunks.length} ancillary chunks\n`,
);
