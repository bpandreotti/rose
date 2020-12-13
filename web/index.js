import * as wasm from "rose-wasm";

document.getElementById("button-generate").onclick = () => {
    const COLOR_SCHEMES = {
        "red": ["#97332b", "#c05150"],
        "green": ["#2c6e49", "#4c956c"],
        "blue": ["#1f4a77", "#416d9f"],
    };
    let num_generations = +document.getElementById("input-num-generations").value;
    let colors = COLOR_SCHEMES[document.getElementById("input-color-scheme").value];
    let stroke_width = +document.getElementById("input-stroke-width").value;
    let svg = wasm.get_svg(num_generations, colors[0], colors[1], stroke_width);
    let container = document.getElementById("svg-container");
    container.innerHTML = svg;
}
