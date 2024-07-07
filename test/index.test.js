const { quantize } = require("../buildjs");
const fs = require("fs/promises");
const { PNG } = require("pngjs");

async function test() {
  const file = await fs.readFile("./assets/nodejs.png");
  const png = PNG.sync.read(file);
  const result = quantize(png.data, png.width, png.height, {
    speed: 1,
    qMin: 0,
    qMax: 80,
    ditheringLevel: 1.0,
  });
  console.log(result);
}

test();
