#[cfg(test)]
extern crate rand;
extern crate structopt;

use rose::*;

use config::*;
use geometry::*;
use svg::*;

use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rose", about = "A Penrose tiling generator")]
struct RoseArguments {
    /// Number of decomposition steps. A larger value results in more, smaller tiles. CAUTION:
    /// values larger than 10 may take a long time to run, and will result in a very large SVG file
    #[structopt(short, long, default_value = "6")]
    num_generations: u64,

    /// Seed to use for generating the tiling. Some seeds may result in tilings that look right, but
    /// aren't actual Penrose tilings
    #[structopt(
        long,
        possible_values = SeedArgument::variants(),
        default_value = SeedArgument::first(),
    )]
    seed: SeedArgument,

    /// Set a custom scale for the tiling. This number represents the side length of a rhombus
    /// before any decomposition, in SVG units. By default, the scale is half of the view box width
    #[structopt(long)]
    scale: Option<f64>,

    /// Set the SVG view box height
    #[structopt(long = "height", default_value = "1000")]
    view_box_height: u64,

    /// Set the SVG view box width
    #[structopt(long = "width", default_value = "1000")]
    view_box_width: u64,

    /// Draw each rhombus as two triangles
    #[structopt(short = "t", long)]
    draw_triangles: bool,

    /// Draw the matching arcs on the rhombuses
    #[structopt(short = "a", long)]
    draw_arcs: bool,

    /// Set the stroke width for the SVG, in SVG units
    #[structopt(long, default_value = "1")]
    stroke_width: f64,

    /// Set which color scheme to use
    #[structopt(
        short = "s",
        long,
        possible_values = ColorSchemeArgument::variants(),
        default_value = ColorSchemeArgument::first(),
    )]
    color_scheme: ColorSchemeArgument,

    /// Override the color scheme fill colors. Expects two valid CSS colors
    #[structopt(short, long, value_names = &["first-color", "second-color"])]
    colors: Vec<String>,

    /// Override the color scheme stroke color. Expects a valid CSS color
    #[structopt(long)]
    stroke_color: Option<String>,

    /// Set custom colors for the matching arcs. Expects two valid CSS colors. Can only be used if
    /// the "--draw-arcs" flag was set.
    #[structopt(long, requires = "draw-arcs", value_names = &["first-color", "second-color"])]
    arc_colors: Vec<String>,

    /// Output file
    #[structopt(required = true)]
    output_file: String,
}

fn main() -> std::io::Result<()> {
    let args: RoseArguments = RoseArguments::from_args();

    let scheme = ColorScheme::from_arg(args.color_scheme);
    let config = SvgConfig {
        view_box_width: args.view_box_width,
        view_box_height: args.view_box_height,
        stroke_width: args.stroke_width,

        // If the user didn't provide new stroke, quad or arc colors, we default to the color
        // scheme's colors
        stroke_color: args.stroke_color.as_deref().unwrap_or(scheme.stroke_color),
        quad_colors: if args.colors.is_empty() {
            scheme.quad_colors
        } else {
            (&args.colors[0], &args.colors[1])
        },
        arc_colors: if args.draw_arcs {
            if args.arc_colors.is_empty() {
                Some(scheme.arc_colors)
            } else {
                Some((&args.arc_colors[0], &args.arc_colors[1]))
            }
        } else {
            None
        },
    };
    let mut builder = SvgBuilder::new(config);

    let center = Point(
        args.view_box_width as f64 / 2.0,
        args.view_box_height as f64 / 2.0,
    );
    let scale = args.scale.unwrap_or(args.view_box_width as f64 / 2.0);
    let seed = get_seed_from_arg(args.seed).transform(center, scale);
    let triangles = tiling::generate_tiling(seed, args.num_generations);
    if args.draw_triangles {
        builder
            .add_all_polygons(triangles)
            .expect("Error writing to string");
    } else {
        // If the user didn't pass the "--draw-triangles" flag, we must merge the triangles and add
        // the resulting rhombuses to the SVG
        let quads = tiling::merge_pairs_hashing(triangles);
        builder
            .add_all_polygons(quads)
            .expect("Error writing to string");
    }
    let mut out_file = File::create(args.output_file)?;
    builder.build(&mut out_file)?;
    Ok(())
}
