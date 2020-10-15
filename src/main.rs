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
            fn first() -> &'static str { Self::variants()[0] }
        }
    }
}

custom_arg_enum! {
    enum SeedArgument {
        Rose = "rose",
        SmallRhombus = "small-rhombus",
        LargeRhombus = "large-rhombus",
    }
}

custom_arg_enum! {
    enum ColorSchemeArgument {
        Orange = "default",
        Purple = "purple",
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rose")]
struct RoseArguments {
    #[structopt(short, long, default_value = "6")]
    num_generations: u64,

    #[structopt(
        long,
        possible_values = SeedArgument::variants(),
        default_value = SeedArgument::first(),
    )]
    seed: SeedArgument,

    #[structopt(long)]
    scale: Option<f64>,

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

    #[structopt(
        short = "s",
        long,
        possible_values = ColorSchemeArgument::variants(),
        default_value = ColorSchemeArgument::first(),
    )]
    color_scheme: ColorSchemeArgument,

    #[structopt(short, long, value_names = &["first-color", "second-color"])]
    colors: Vec<String>,

    #[structopt(long)]
    stroke_color: Option<String>,

    #[structopt(long, value_names = &["first-color", "second-color"])]
    arc_colors: Vec<String>,
}

struct ColorScheme {
    quad_colors: (String, String),
    stroke_color: String,
    arc_colors: (String, String),
}

fn main() -> std::io::Result<()> {
    let args: RoseArguments = RoseArguments::from_args();
    println!("{:#?}", args);

    let center = Point(
        args.view_box_width as f64 / 2.0,
        args.view_box_height as f64 / 2.0,
    );
    let scale = args.scale.unwrap_or(args.view_box_width as f64 / 2.0) ;
    let seed = get_seed_from_arg(args.seed).transform(center, scale);
    let quads = tiling::generate_tiling(seed, args.num_generations);

    let scheme = get_color_scheme_from_arg(args.color_scheme);
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
    let mut builder = SvgBuilder::new(config);
    builder.add_all_quads(quads);
    let mut out_file = File::create("out.svg")?;
    builder.build(&mut out_file)?;
    Ok(())
}

fn get_color_scheme_from_arg(arg: ColorSchemeArgument) -> ColorScheme {
    use ColorSchemeArgument::*;
    // @TODO: Add more color schemes
    match arg {
        Orange => ColorScheme {
            quad_colors: ("#ea4848".into(), "#e8694c".into()),
            stroke_color: "black".into(),
            arc_colors: ("blue".into(), "green".into()),
        },
        Purple => ColorScheme {
            quad_colors: ("#8447d3".into(), "#9654bc".into()),
            stroke_color: "white".into(),
            arc_colors: ("green".into(), "yellow".into()),
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
