use crate::geometry::*;

use std::fmt::Write;
use std::fs::File;
use std::write;

const SCALING_FACTOR: u64 = 1000;

// Since writing a float to string is so slow, it is much better to convert it to an integer
// (scaling it first so as not to lose too much precision) and write the integer to the string
// instead. Using this technique to avoid writing floats to the SVG improves performance
// considerably.
fn scale_float(x: f64) -> i64 {
    (x * SCALING_FACTOR as f64) as i64
}

pub trait SvgPolygon {
    fn polygon_type(&self) -> TileType;
    fn write_points(&self, builder: &mut SvgBuilder) -> std::fmt::Result;
    fn arcs(&self) -> (Arc, Arc);
}

impl SvgPolygon for RobinsonTriangle {
    fn polygon_type(&self) -> TileType {
        self.triangle_type
    }

    fn write_points(&self, builder: &mut SvgBuilder) -> std::fmt::Result {
        write!(
            builder.content,
            "{},{} {},{} {},{}",
            scale_float(self.a.0),
            scale_float(self.a.1),
            scale_float(self.b.0),
            scale_float(self.b.1),
            scale_float(self.c.0),
            scale_float(self.c.1),
        )
    }

    fn arcs(&self) -> (Arc, Arc) {
        let first_radius = Line(Line(self.a, self.b).median(), self.a).length();
        let first_arc = (
            Line(self.a, self.b).median(),
            self.a,
            self.a + first_radius * (self.c - self.a).normalized(),
            false,
        );
        let second_radius = Line(Line(self.c, self.b).median(), self.c).length();
        let second_arc = (
            Line(self.c, self.b).median(),
            self.c,
            self.c + second_radius * (self.a - self.c).normalized(),
            false,
        );
        (first_arc, second_arc)
    }
}

impl SvgPolygon for Quadrilateral {
    fn polygon_type(&self) -> TileType {
        RobinsonTriangle::infer_triangle_type(self.a, self.b, self.c)
    }

    fn write_points(&self, builder: &mut SvgBuilder) -> std::fmt::Result {
        write!(
            builder.content,
            "{},{} {},{} {},{} {},{}",
            scale_float(self.a.0),
            scale_float(self.a.1),
            scale_float(self.b.0),
            scale_float(self.b.1),
            scale_float(self.c.0),
            scale_float(self.c.1),
            scale_float(self.d.0),
            scale_float(self.d.1),
        )
    }

    fn arcs(&self) -> (Arc, Arc) {
        let first_arc = (
            Line(self.a, self.b).median(),
            self.a,
            Line(self.a, self.d).median(),
            false,
        );
        let second_arc = (
            Line(self.c, self.b).median(),
            self.c,
            Line(self.c, self.d).median(),
            // Only in the dart tile, the second arc has an angle of more than 180 degrees
            self.polygon_type() == TileType::Dart,
        );
        (first_arc, second_arc)
    }
}

pub struct SvgConfig<'a> {
    pub view_box_width: u64,
    pub view_box_height: u64,
    pub stroke_width: f64,
    pub stroke_color: &'a str,
    pub quad_colors: (&'a str, &'a str),
    pub arc_colors: Option<(&'a str, &'a str)>,
}

pub struct SvgBuilder<'a> {
    config: SvgConfig<'a>,
    content: String,
}

impl<'a> SvgBuilder<'a> {
    pub fn new(config: SvgConfig<'a>) -> Self {
        // As we are scaling all coordinates by SCALING_FACTOR, we must also scale the view box.
        let content = format!(
            "<svg width=\"100%\" height=\"100%\" viewBox=\"0 0 {} {}\" preserveAspectRatio=\
            \"xMidYMid slice\" xmlns=\"http://www.w3.org/2000/svg\">\n  <g stroke=\"{}\" \
            stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\">\n",
            config.view_box_width * SCALING_FACTOR,
            config.view_box_height * SCALING_FACTOR,
            config.stroke_color,
            scale_float(config.stroke_width),
        );
        SvgBuilder { config, content }
    }

    pub fn build(self, out_file: &mut File) -> std::io::Result<()> {
        use std::io::prelude::*;
        let declaration = b"<?xml version=\"1.0\" encoding=\"utf-8\"?>\n";
        out_file.write_all(declaration)?;
        out_file.write_all(self.content.as_bytes())?;
        out_file.write_all(b"  </g>\n</svg>\n")?;
        Ok(())
    }

    pub fn build_to_string(mut self) -> Result<String, std::fmt::Error> {
        write!(self.content, "  </g>\n</svg>\n")?;
        Ok(self.content)
    }

    pub fn add_all_polygons<T: SvgPolygon>(&mut self, polys: Vec<T>) -> std::fmt::Result {
        macro_rules! add_polygon_group {
            ($color:expr, $($type:expr),*) => {
                writeln!(self.content, r#"    <g fill="{}">"#, $color)?;
                let filtered = polys.iter().filter(
                    |p| $(p.polygon_type() == $type)||*
                );
                for p in filtered {
                    self.add_polygon(p)?
                }
                writeln!(self.content, "    </g>")?;
            };
        }
        add_polygon_group!(
            self.config.quad_colors.0,
            TileType::SmallRhombus,
            TileType::Dart
        );
        add_polygon_group!(
            self.config.quad_colors.1,
            TileType::LargeRhombus,
            TileType::Kite
        );

        if let Some((color_1, color_2)) = self.config.arc_colors {
            let (arcs_1, arcs_2): (Vec<_>, Vec<_>) = polys.iter().map(SvgPolygon::arcs).unzip();
            self.add_arc_group(arcs_1, &color_1)?;
            self.add_arc_group(arcs_2, &color_2)?;
        }
        Ok(())
    }

    fn add_polygon(&mut self, polygon: &dyn SvgPolygon) -> std::fmt::Result {
        write!(self.content, "      <polygon points=\"")?;
        polygon.write_points(self)?;
        writeln!(self.content, "\" />")
    }

    fn add_arc_group(&mut self, arcs: Vec<Arc>, color: &str) -> std::fmt::Result {
        writeln!(self.content, "    <g fill=\"none\" stroke=\"{}\">", color)?;
        for a in arcs {
            self.add_arc(a)?;
        }
        writeln!(self.content, "    </g>")
    }

    fn add_arc(&mut self, (start, center, end, large_angle_flag): Arc) -> std::fmt::Result {
        let radius = scale_float(Line(start, center).length());

        // This uses the cross product between the start and end directions in order to determine
        // if the shortest rotation from start to end is clockwise or counterclockwise. If the arc
        // is supposed to represent the largest rotation, we must invert the result.
        let sweep_flag = (start - center).cross(end - center) > 0.0;
        let sweep_flag = if large_angle_flag {
            !sweep_flag
        } else {
            sweep_flag
        };
        writeln!(
            self.content,
            "      <path d=\"M {} {} A {} {} 0 {} {} {} {}\" />",
            scale_float(start.0),
            scale_float(start.1),
            radius,
            radius,
            large_angle_flag as u8,
            sweep_flag as u8,
            scale_float(end.0),
            scale_float(end.1),
        )
    }
}
