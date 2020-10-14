#[cfg(test)]
extern crate rand;
extern crate structopt;

#[macro_use]
mod geometry;
mod seeds;
mod svg;
mod tiling;

use geometry::*;
use svg::*;

use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rose")]
struct SvgArguments {
    #[structopt(short = "w", long = "width", default_value = "1000")]
    view_box_width: u64,

    #[structopt(short = "h", long = "height", default_value = "1000")]
    view_box_height: u64,

    #[structopt(short = "t", long)]
    draw_triangles: bool, // @WIP

    #[structopt(short = "a", long)]
    draw_arcs: bool, // @WIP

    #[structopt(long, default_value = "1")]
    stroke_width: u64,

    #[structopt(short = "s", long, default_value = "default")]
    color_scheme: String,

    #[structopt(short, long, value_names=&["first-color", "second-color"])]
    colors: Vec<String>,

    #[structopt(long)]
    stroke_color: Option<String>,

    #[structopt(long, value_names=&["first-color", "second-color"])]
    arc_colors: Vec<String>,
}

struct ColorScheme {
    quad_colors: (String, String),
    stroke_color: String,
    arc_colors: (String, String),
}

fn main() -> std::io::Result<()> {
    let args: SvgArguments = SvgArguments::from_args();
    println!("{:#?}", args);
    let scheme = get_color_scheme(&args.color_scheme);
    let config = SvgConfig {
        view_box_width: args.view_box_width,
        view_box_height: args.view_box_height,
        draw_triangles: args.draw_triangles,
        stroke_width: args.stroke_width,
        stroke_color: args.stroke_color.unwrap_or(scheme.stroke_color),

        quad_colors: if args.colors.is_empty() {
            scheme.quad_colors
        } else {
            (args.colors[0].clone(), args.colors[1].clone())
        },

        arc_colors: if args.draw_arcs {
            if args.arc_colors.is_empty() {
                Some(scheme.arc_colors)
            } else {
                Some((args.arc_colors[0].clone(), args.arc_colors[1].clone()))
            }
        } else {
            None
        },
    };

    let seed = seeds::rose().transform(Point(500.0, 500.0), 400.0);
    let quads = tiling::generate_tiling(seed, 6);

    let mut builder = SvgBuilder::new(config);
    builder.add_all_quads(quads);
    let mut out_file = File::create("out.svg")?;
    builder.build(&mut out_file)?;
    Ok(())
}

// @TODO: Add error handling in case the name is not a valid color scheme
fn get_color_scheme(name: &str) -> ColorScheme {
    // @TODO: Add more color schemes
    match name {
        "default" => ColorScheme {
            quad_colors: ("#ea4848".into(), "#e8694c".into()),
            stroke_color: "black".into(),
            arc_colors: ("blue".into(), "green".into()),
        },
        _ => panic!(),
    }
}
