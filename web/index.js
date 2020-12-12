import * as wasm from "rose-wasm";

let svg = wasm.get_svg(6, "#1f4a77", "#416d9f");
var container = document.getElementById("svg-container");
container.innerHTML = svg;
