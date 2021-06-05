use crate::geometry::TileType;
use crate::seeds;

/// Like `clap::arg_enum!`, but allows you to customize the name of the argument associated with
/// each variant.
macro_rules! custom_arg_enum {
    ($v:vis enum $enum_name:ident { $($variant_name:ident = $variant_value:expr),* $(,)? }) => {
        #[derive(Debug)]
        $v enum $enum_name {
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
            $v fn variants() -> &'static [&'static str] {
                &[ $($variant_value),* ]
            }
            $v fn first() -> &'static str {
                Self::variants()[0]
            }
        }
    }
}

custom_arg_enum! {
    pub enum SeedArgument {
        Rose = "rose",
        LargeRhombus = "large-rhombus",
        SmallRhombus = "small-rhombus",
        Kite = "kite",
        Dart = "dart",
        Pizza = "pizza",
    }
}

custom_arg_enum! {
    pub enum ColorSchemeArgument {
        Red = "red",
        Green = "green",
        Blue = "blue",
        Purple = "purple",
        Grey = "grey",
        Yellow = "yellow",
    }
}

pub struct ColorScheme {
    pub quad_colors: (&'static str, &'static str),
    pub stroke_color: &'static str,
    pub arc_colors: (&'static str, &'static str),
}

impl ColorScheme {
    pub fn from_arg(arg: ColorSchemeArgument) -> ColorScheme {
        use ColorSchemeArgument::*;
        match arg {
            Red => ColorScheme {
                quad_colors: ("#97332b", "#c05150"),
                stroke_color: "white",
                arc_colors: ("#50d35b", "#30bbe5"),
            },
            Green => ColorScheme {
                quad_colors: ("#2c6e49", "#4c956c"),
                stroke_color: "white",
                arc_colors: ("#d17432", "#8d31ce"),
            },
            Blue => ColorScheme {
                quad_colors: ("#1f4a77", "#416d9f"),
                stroke_color: "white",
                arc_colors: ("#d13232", "#a9d132"),
            },
            Purple => ColorScheme {
                quad_colors: ("#674593", "#915eae"),
                stroke_color: "white",
                arc_colors: ("#a9d132", "#d17432"),
            },
            Grey => ColorScheme {
                quad_colors: ("#404040", "#545454"),
                stroke_color: "white",
                arc_colors: ("black", "#202020"),
            },
            Yellow => ColorScheme {
                quad_colors: ("#e0be4e", "#f9d96d"),
                stroke_color: "#9b6a01",
                arc_colors: ("#4e5de0", "#884ee0"),
            },
        }
    }
}
pub fn get_seed_from_arg(arg: SeedArgument) -> seeds::Seed {
    use SeedArgument::*;
    match arg {
        Rose => seeds::rose(),
        LargeRhombus => seeds::tile(TileType::LargeRhombus),
        SmallRhombus => seeds::tile(TileType::SmallRhombus),
        Kite => seeds::tile(TileType::Kite),
        Dart => seeds::tile(TileType::Dart),
        Pizza => seeds::pizza(),
    }
}
