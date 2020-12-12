use wasm_bindgen::prelude::*;
use rose::*;
use svg::{SvgBuilder, SvgConfig};
use geometry::Point;

#[wasm_bindgen]
pub fn get_svg(num_generations: u8, first_color: &str, second_color: &str) -> String {
    let svg_cfg = SvgConfig {
        view_box_width: 1000,
        view_box_height: 1000,
        stroke_width: 3,
        stroke_color: "white",
        quad_colors: (first_color, second_color),
        arc_colors: None,
    };

    let seed = seeds::pizza().transform(Point(500.0, 500.0), 500.0);
    let triangles = tiling::generate_tiling(seed, num_generations as u64);
    let quads = tiling::merge_pairs(triangles);

    let mut builder = SvgBuilder::new(svg_cfg);
    builder.add_all_polygons(quads);
    builder.build_to_string()
}
