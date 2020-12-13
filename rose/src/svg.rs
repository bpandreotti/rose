use crate::geometry::*;

use std::fmt::Write;
use std::fs::File;
use std::write;

pub trait SvgPolygon {
    fn polygon_type(&self) -> RobinsonTriangleType;
    fn write_points(&self, builder: &mut SvgBuilder) -> std::fmt::Result;
    fn arcs(&self) -> (Arc, Arc);
}

impl SvgPolygon for RobinsonTriangle {
    fn polygon_type(&self) -> RobinsonTriangleType {
        self.triangle_type
    }

    fn write_points(&self, builder: &mut SvgBuilder) -> std::fmt::Result {
        write!(
            builder.content,
            "{:.4},{:.4} {:.4},{:.4} {:.4},{:.4}",
            self.a.0, self.a.1, self.b.0, self.b.1, self.c.0, self.c.1,
        )
    }

    fn arcs(&self) -> (Arc, Arc) {
        let ratio = match self.triangle_type {
            RobinsonTriangleType::Small => PHI,
            RobinsonTriangleType::Large => PHI_INVERSE,
        };
        let first_arc = (
            Line(self.a, self.b).median(),
            self.a,
            self.a + 0.5 * ratio * (self.c - self.a),
        );
        let second_arc = (
            Line(self.c, self.b).median(),
            self.c,
            self.c + 0.5 * ratio * (self.a - self.c),
        );
        (first_arc, second_arc)
    }
}

impl SvgPolygon for Quadrilateral {
    fn polygon_type(&self) -> RobinsonTriangleType {
        if Line(self.a, self.c).length() > Line(self.b, self.d).length() {
            RobinsonTriangleType::Large
        } else {
            RobinsonTriangleType::Small
        }
    }

    fn write_points(&self, builder: &mut SvgBuilder) -> std::fmt::Result {
        write!(
            builder.content,
            "{:.4},{:.4} {:.4},{:.4} {:.4},{:.4} {:.4},{:.4}",
            self.a.0, self.a.1, self.b.0, self.b.1, self.c.0, self.c.1, self.d.0, self.d.1
        )
    }

    fn arcs(&self) -> (Arc, Arc) {
        let first_arc = (
            Line(self.a, self.b).median(),
            self.a,
            Line(self.a, self.d).median(),
        );
        let second_arc = (
            Line(self.c, self.b).median(),
            self.c,
            Line(self.c, self.d).median(),
        );
        (first_arc, second_arc)
    }
}

pub struct SvgConfig<'a> {
    pub view_box_width: u64,
    pub view_box_height: u64,
    pub stroke_width: u64,
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
        let content = format!(
            "<svg width=\"100%\" height=\"100%\" viewBox=\"0 0 {} {}\" preserveAspectRatio=\
            \"xMidYMid slice\" xmlns=\"http://www.w3.org/2000/svg\">\n  <g stroke=\"{}\" \
            stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\">\n",
            config.view_box_width, config.view_box_height, config.stroke_color, config.stroke_width,
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
            ($type:expr, $color:expr) => {
                writeln!(self.content, r#"    <g fill="{}">"#, $color)?;
                let filtered = polys.iter().filter(|p| p.polygon_type() == $type);
                for p in filtered {
                    self.add_polygon(p)?
                }
                writeln!(self.content, "    </g>")?;
            };
        }
        add_polygon_group!(RobinsonTriangleType::Small, self.config.quad_colors.0);
        add_polygon_group!(RobinsonTriangleType::Large, self.config.quad_colors.1);

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

    fn add_arc(&mut self, (start, center, end): Arc) -> std::fmt::Result {
        let radius = Line(start, center).length();
        let sweep_flag = (start - center).cross(end - center) > 0.0;
        writeln!(
            self.content,
            "      <path d=\"M {} {} A {} {} 0 0 {} {} {}\" />",
            start.0, start.1, radius, radius, sweep_flag as u8, end.0, end.1
        )
    }
}
