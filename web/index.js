import * as wasm from "rose-wasm";

const COLOR_SCHEMES = {
    "red": {
        quad_colors: ["#97332b", "#c05150"],
        stroke_color: "white",
        arc_colors: ["#50d35b", "#30bbe5"],
    },
    "green": {
        quad_colors: ["#2c6e49", "#4c956c"],
        stroke_color: "white",
        arc_colors: ["#d17432", "#8d31ce"],
    },
    "blue": {
        quad_colors: ["#1f4a77", "#416d9f"],
        stroke_color: "white",
        arc_colors: ["#d13232", "#a9d132"],
    },
    "purple": {
        quad_colors: ["#674593", "#915eae"],
        stroke_color: "white",
        arc_colors: ["#a9d132", "#d17432"],
    },
    "grey": {
        quad_colors: ["#404040", "#545454"],
        stroke_color: "white",
        arc_colors: ["black", "#202020"],
    },
    "yellow": {
        quad_colors: ["#e0be4e", "#f9d96d"],
        stroke_color: "#9b6a01",
        arc_colors: ["#4e5de0", "#884ee0"],
    },
};

const svg_container = document.getElementById("svg-container");

const generate = () => {
    let num_generations = +document.getElementById("input-num-generations").value;
    let seed = document.getElementById("input-seed").value;
    let color_scheme = COLOR_SCHEMES[document.getElementById("input-color-scheme").value];
    let stroke_width = +document.getElementById("input-stroke-width").value;
    let draw_triangles = !!document.getElementById("input-draw-triangles").checked;
    let draw_arcs = !!document.getElementById("input-draw-arcs").checked;
    
    let svg = wasm.get_svg(
        num_generations,
        seed,
        ...color_scheme.quad_colors,
        color_scheme.stroke_color,
        ...color_scheme.arc_colors,
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
