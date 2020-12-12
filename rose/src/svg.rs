use crate::geometry::*;

use std::fs::File;
use std::io::prelude::*;

pub trait SvgPolygon {
    fn polygon_type(&self) -> RobinsonTriangleType;
    fn points_string(&self) -> String;
    fn arcs(&self) -> (Arc, Arc);
}

impl SvgPolygon for RobinsonTriangle {
    fn polygon_type(&self) -> RobinsonTriangleType {
        self.triangle_type
    }

    fn points_string(&self) -> String {
        format!(
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

    fn points_string(&self) -> String {
        format!(
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
        let mut content = format!(
            r#"<svg width="100%" height="100%" viewBox="0 0 {} {}" "#,
            config.view_box_width, config.view_box_height
        );
        content += r#"preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg">"#;
        content += "\n";

        content += &format!(
            r#"  <g stroke="{}" stroke-width="{}" stroke-linecap="round" stroke-linejoin="round">"#,
            config.stroke_color, config.stroke_width
        );
        content += "\n";

        SvgBuilder { config, content }
    }

    pub fn build(mut self, out_file: &mut File) -> std::io::Result<()> {
        let declaration = r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string() + "\n";
        out_file.write_all(declaration.as_bytes())?;
        self.content += "  </g>\n";
        self.content += "</svg>\n";
        out_file.write_all(self.content.as_bytes())?;
        Ok(())
    }

    pub fn build_to_string(self) -> String {
        self.content
    }

    pub fn add_all_polygons<T: SvgPolygon>(&mut self, polys: Vec<T>) {
        macro_rules! add_polygon_group {
            ($type:expr, $color:expr) => {
                self.content += &format!(r#"    <g fill="{}">"#, $color);
                self.content += "\n";
                let filtered = polys.iter().filter(|p| p.polygon_type() == $type);
                for p in filtered {
                    self.add_polygon(p)
                }
                self.content += "    </g>\n";
            };
        }
        add_polygon_group!(RobinsonTriangleType::Small, self.config.quad_colors.0);
        add_polygon_group!(RobinsonTriangleType::Large, self.config.quad_colors.1);

        if let Some((color_1, color_2)) = self.config.arc_colors {
            let (arcs_1, arcs_2): (Vec<_>, Vec<_>) = polys.iter().map(SvgPolygon::arcs).unzip();
            self.add_arc_group(arcs_1, &color_1);
            self.add_arc_group(arcs_2, &color_2);
        }
    }

    fn add_polygon(&mut self, polygon: &dyn SvgPolygon) {
        self.content += "      ";
        self.content += &format!(r#"<polygon points="{}" />"#, polygon.points_string());
        self.content += "\n";
    }

    fn add_arc_group(&mut self, arcs: Vec<Arc>, color: &str) {
        self.content += &format!(r#"    <g fill="none" stroke="{}">"#, color);
        self.content += "\n";
        for a in arcs {
            self.add_arc(a);
        }
        self.content += "    </g>\n";
    }

    fn add_arc(&mut self, (start, center, end): Arc) {
        let radius = Line(start, center).length();
        let sweep_flag = (start - center).cross(end - center) > 0.0;
        let path = format!(
            "M {} {} A {} {} 0 0 {} {} {}",
            start.0, start.1, radius, radius, sweep_flag as u8, end.0, end.1
        );
        self.content += "      ";
        self.content += &format!(r#"<path d="{}" />"#, path);
        self.content += "\n";
    }
}
