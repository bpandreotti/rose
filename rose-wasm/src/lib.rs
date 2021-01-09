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
    first_quad_color: &str,
    second_quad_color: &str,
    stroke_color: &str,
    first_arc_color: &str,
    second_arc_color: &str,
    stroke_width: u8,
    draw_triangles: bool,
    draw_arcs: bool,
) -> String {
    let svg_cfg = SvgConfig {
        view_box_width: 1000,
        view_box_height: 1000,
        stroke_width: stroke_width as u64,
        stroke_color,
        quad_colors: (first_quad_color, second_quad_color),
        arc_colors: if draw_arcs {
            Some((first_arc_color, second_arc_color))
        } else {
            None
        },
    };

    let center = Point(500.0, 500.0);
    let seed = match seed {
        "rose" => seeds::rose().transform(center, 125.0),
        "pizza" => seeds::pizza().transform(center, 250.0),
        "rhombus" => seeds::rhombus(geometry::RobinsonTriangleType::Large).transform(center, 500.0),
        _ => panic!(),
    };

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
