import * as wasm from "rose-wasm";

const svg_container = document.getElementById("svg-container");

const generate = () => {
    let num_generations = +document.getElementById("input-num-generations").value;
    let seed = document.getElementById("input-seed").value;
    let color_scheme = document.getElementById("input-color-scheme").value;
    let stroke_width = +document.getElementById("input-stroke-width").value;
    let draw_triangles = !!document.getElementById("input-draw-triangles").checked;
    let draw_arcs = !!document.getElementById("input-draw-arcs").checked;
    
    let svg = wasm.get_svg(
        num_generations,
        seed,
        color_scheme,
        stroke_width,
        draw_triangles,
        draw_arcs,
    );

    let svg_container = document.getElementById("svg-container");
    svg_container.innerHTML = svg;
    svg_container.children[0].style.transform = `scale(${scale})`;
};

document.getElementById("button-generate").onclick = generate;

let scale = 1.0;
const zoom = (event) => {
    event.preventDefault();
    const zoom_sensitivity = 0.04;
    scale += event.deltaY * -zoom_sensitivity * (scale - 0.3);
    scale = Math.min(Math.max(1.0, scale), 16.0);
    svg_container.children[0].style.transform = `scale(${scale})`;
};

svg_container.onwheel = zoom;
generate();
