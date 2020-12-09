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

/// Like `clap::arg_enum!`, but allows you to customize the name of the argument associated with
/// each variant.
macro_rules! custom_arg_enum {
    (enum $enum_name:ident { $($variant_name:ident = $variant_value:expr),* $(,)? }) => {
        #[derive(Debug)]
        enum $enum_name {
            $($variant_name),*
        }

        impl std::str::FromStr for $enum_name {
            type Err = &'static str;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( $variant_value => Ok(Self::$variant_name), )*
                    _ => Err("invalid argument value")
                }
            }
        }

        impl $enum_name {
            fn variants() -> &'static [&'static str] { 
                &[ $($variant_value),* ]
            }
            fn first() -> &'static str {
                Self::variants()[0]
            }
        }
    }
}

custom_arg_enum! {
    enum SeedArgument {
        LargeRhombus = "large-rhombus",
        SmallRhombus = "small-rhombus",
        Rose = "rose",
    }
}

custom_arg_enum! {
    enum ColorSchemeArgument {
        Orange = "orange",
        Purple = "purple",
    }
}

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

    // @TODO: Should I change the height/width arguments shorthand form to '-y' and '-x', to allow
    // '-h' to mean '--help'? Or maybe just remove their shothand form all together?
    /// Set the SVG view box height
    #[structopt(short = "h", long = "height", default_value = "1000")]
    view_box_height: u64,

    /// Set the SVG view box width
    #[structopt(short = "w", long = "width", default_value = "1000")]
    view_box_width: u64,

    /// Draw each rhombus as two triangles
    #[structopt(short = "t", long)]
    draw_triangles: bool,

    /// Draw the matching arcs on the rhombuses
    #[structopt(short = "a", long)]
    draw_arcs: bool,

    /// Set the stroke width for the SVG, in SVG units
    #[structopt(long, default_value = "1")]
    stroke_width: u64,

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

struct ColorScheme {
    quad_colors: (&'static str, &'static str),
    stroke_color: &'static str,
    arc_colors: (&'static str, &'static str),
}

fn get_color_scheme_from_arg(arg: ColorSchemeArgument) -> ColorScheme {
    use ColorSchemeArgument::*;
    // @TODO: Add more color schemes
    match arg {
        Orange => ColorScheme {
            quad_colors: ("#ea4848", "#e8694c"),
            stroke_color: "black",
            arc_colors: ("blue", "green"),
        },
        Purple => ColorScheme {
            quad_colors: ("#8447d3", "#9654bc"),
            stroke_color: "white",
            arc_colors: ("green", "yellow"),
        },
    }
}

fn get_seed_from_arg(arg: SeedArgument) -> seeds::Seed {
    use SeedArgument::*;
    match arg {
        Rose => seeds::rose(),
        SmallRhombus => seeds::rhombus(RobinsonTriangleType::Small),
        LargeRhombus => seeds::rhombus(RobinsonTriangleType::Large),
    }
}

fn main() -> std::io::Result<()> {
    let args: RoseArguments = RoseArguments::from_args();

    let center = Point(
        args.view_box_width as f64 / 2.0,
        args.view_box_height as f64 / 2.0,
    );
    let scale = args.scale.unwrap_or(args.view_box_width as f64 / 2.0);
    let seed = get_seed_from_arg(args.seed).transform(center, scale);
    let quads = tiling::generate_tiling(seed, args.num_generations);

    let scheme = get_color_scheme_from_arg(args.color_scheme);
    let config = SvgConfig {
        view_box_width: args.view_box_width,
        view_box_height: args.view_box_height,
        draw_triangles: args.draw_triangles,
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
    builder.add_all_quads(quads);
    let mut out_file = File::create(args.output_file)?;
    builder.build(&mut out_file)?;
    Ok(())
}
