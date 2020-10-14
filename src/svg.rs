use crate::geometry::*;

use std::fs::File;
use std::io::prelude::*;

pub struct SvgConfig {
    pub view_box_width: u64,
    pub view_box_height: u64,
    pub draw_triangles: bool,
    pub stroke_width: u64,
    pub stroke_color: String,
    pub quad_colors: (String, String),
    pub arc_colors: Option<(String, String)>,
}

pub struct SvgBuilder {
    config: SvgConfig,
    content: String,
}

// @TODO: Draw the arcs and triangle lines depending on config
impl SvgBuilder {
    pub fn new(config: SvgConfig) -> Self {
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

    pub fn build(mut self, out_file: &mut File) -> std::io::Result<()> {
        self.content += "  </g>\n";
        self.content += "</svg>\n";
        out_file.write_all(self.content.as_bytes())?;
        Ok(())
    }
}
