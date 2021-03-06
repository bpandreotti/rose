# rose

[Check out the live demo!](https://bpandreotti.github.io/rose/)

<img src="images/rose.png" width="200"/>

`rose` is a [Penrose tiling](https://en.wikipedia.org/wiki/Penrose_tiling) generator written in Rust

## Usage

Clone the repository and run `cargo run -- out.svg` to generate a tiling using the default options. `rose` will create an SVG file that can be viewed in a web browser.

## Options

Run `rose --help` to see a full list of arguments.

### Number of generations

<img src="images/generations.gif" width="300"/>

Use `-n <num-generations>` to control how many decomposition steps should be made.

### Seeds

You can change the starting seed for the tiling with `--seed <seed>`. There are 6 available seeds: `rose`, `pizza`, `large-rhombus`, `small-rhombus`, `kite` and `dart`. This will change the shape of the Penrose tiling. The first four seed use the rhombus (P3) tiles, while the `kite` and `dart` seeds use the kite and dart (P2) tiles.

### Appearance

Use `-s <color-scheme>` to change the appearance of the tiling. There are 6 available color schemes: `red`, `green`, `blue`, `purple`, `grey` and `yellow`,

<img src="images/color-schemes.png" width="600"/>

You can also use the `--colors` and `--stroke-color` options to create your own custom color scheme. See `--help` for more ways to customize the appearance of the Penrose tiling.

### Show arcs and triangles

You can use the `--draw-triangles` flag to skip the triangle merging step and render the Robinson triangles used to generate the tiling. The `--draw-arcs` flag will render colored arcs to show the tile matching rules.

## Building and running the WebAssembly demo

To build the WebAssembly version, you will need to have `wasm-pack` and `npm` installed in your system. First, `cd` into the `rose-wasm` directory and run `wasm-pack build --release`. Then, go the the `web` directory and run `npm install`. Finally, you can either serve the website with `npm run serve`, or build it in the `dist` folder with `npm run build`.
