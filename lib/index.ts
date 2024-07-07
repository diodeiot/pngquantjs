const addon = require("../index.node");

export type Result = {
  palette: Buffer;
  pixels: Buffer;
  quality: number;
};

export type Options = {
  speed: number;
  qMin: number;
  qMax: number;
  ditheringLevel: number;
};

export function quantize(
  buffer: Buffer,
  width: number,
  height: number,
  options?: Options
): Result {
  return addon.quantize(
    buffer,
    width,
    height,
    options?.speed ?? 4,
    options?.qMin ?? 0,
    options?.qMax ?? 100,
    options?.ditheringLevel ?? 1.0
  );
}
