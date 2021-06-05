use config::*;
use geometry::Point;
use rose::*;
use svg::{SvgBuilder, SvgConfig};
use wasm_bindgen::prelude::*;

// This function needs to have so many arguments because passing tuples or structs with `&str`s
// between javascript and rust is not supported
#[allow(clippy::clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn get_svg(
    num_generations: u8,
    seed: &str,
    color_scheme: &str,
    stroke_width: f64,
    draw_triangles: bool,
    draw_arcs: bool,
) -> String {
    let color_scheme = ColorScheme::from_arg(color_scheme.parse().unwrap());
    let svg_cfg = SvgConfig {
        view_box_width: 1000,
        view_box_height: 1000,
        stroke_width,
        stroke_color: color_scheme.stroke_color,
        quad_colors: color_scheme.quad_colors,
        arc_colors: if draw_arcs {
            Some(color_scheme.arc_colors)
        } else {
            None
        },
    };

    let seed = get_seed_from_arg(seed.parse().unwrap()).transform(Point(500.0, 500.0), 100.0);

    let triangles = tiling::generate_tiling(seed, num_generations as u64);
    let mut builder = SvgBuilder::new(svg_cfg);
    if draw_triangles {
        builder.add_all_polygons(triangles).unwrap();
    } else {
        let quads = tiling::merge_pairs_hashing(triangles);
        builder.add_all_polygons(quads).unwrap();
    }
    builder.build_to_string().unwrap()
}
