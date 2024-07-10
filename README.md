# pngquantjs

pngquantjs is a native nodejs module for pngquant library
see https://github.com/ImageOptim/libimagequant

> [!WARNING]  
> Curently this package can only run on unix-like systems

> [!WARNING]  
> Currently this package doesn't use pre-built binaries. It builds addon from source code while package is installed. Compilation process uses `cargo` so this binary should be in the path.

## Install

```sh
npm install pngquantjs
```

## Example

```sh
npm install pngjs
```

```js
import fs from "fs/promises";
import { PNG } from "pngjs";
import { quantize } from "pngquantjs";

async function test() {
  const file = await fs.readFile("node_modules/pngquantjs/assets/nodejs.png");
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
```

## Contribution

### Development

```sh
git clone https://github.com/diodeiot/pngquantjs --recursive
cd pngquantjs
yarn
yarn build
```

When contributing code, please write relevant tests and run `npm test` before submitting pull requests.

## License

MIT
