use crate::geometry::*;

use std::fs::File;
use std::io::prelude::*;

pub struct SvgConfig<'a> {
    pub view_box_width: u64,
    pub view_box_height: u64,
    pub draw_triangles: bool,
    pub stroke_width: u64,
    pub stroke_color: &'a str,
    pub quad_colors: (&'a str, &'a str),
    pub arc_colors: Option<(&'a str, &'a str)>,
}

pub struct SvgBuilder<'a> {
    config: SvgConfig<'a>,
    content: String,
}

// @TODO: Draw the triangle lines depending on config
impl<'a> SvgBuilder<'a> {
    pub fn new(config: SvgConfig<'a>) -> Self {
        let mut content = r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string() + "\n";
        content += &format!(
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

    pub fn add_all_quads(&mut self, quads: Vec<Quadrilateral>) {
        macro_rules! add_quad_group {
            ($type:expr, $color:expr) => {
                self.content += &format!(r#"    <g fill="{}">"#, $color);
                self.content += "\n";
                let qs = quads.iter().filter(|q| q.quadrilateral_type() == $type);
                for q in qs {
                    self.add_quad(&q)
                }
                self.content += "    </g>\n";
            };
        }
        add_quad_group!(RobinsonTriangleType::Small, self.config.quad_colors.0);
        add_quad_group!(RobinsonTriangleType::Large, self.config.quad_colors.1);

        if let Some((color_1, color_2)) = self.config.arc_colors {
            let (arcs_1, arcs_2): (Vec<_>, Vec<_>) = quads.iter().map(Quadrilateral::arcs).unzip();
            self.add_arc_group(arcs_1, &color_1);
            self.add_arc_group(arcs_2, &color_2);
        }
    }

    fn add_quad(&mut self, quad: &Quadrilateral) {
        let points = &format!(
            "{:.4},{:.4} {:.4},{:.4} {:.4},{:.4} {:.4},{:.4}",
            quad.a.0, quad.a.1, quad.b.0, quad.b.1, quad.c.0, quad.c.1, quad.d.0, quad.d.1
        );
        self.content += "      ";
        self.content += &format!(r#"<polygon points="{}" />"#, points);
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

    pub fn build(mut self, out_file: &mut File) -> std::io::Result<()> {
        self.content += "  </g>\n";
        self.content += "</svg>\n";
        out_file.write_all(self.content.as_bytes())?;
        Ok(())
    }
}
