use crate::geometry::*;

use std::fs::File;
use std::io::prelude::*;

pub struct SvgBuilder {
    content: String,
}

// @TODO: Add a proper way to draw the matching arcs
impl SvgBuilder {
    pub fn new(width: u64, height: u64, stroke_color: &str, stroke_width: u64) -> Self {
        let mut content = r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string() + "\n";
        content += &format!(
            r#"<svg width="100%" height="100%" viewBox="0 0 {} {}" "#,
            width, height
        );
        content += r#"preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg">"#;
        content += "\n";

        content += &format!(
            r#"  <g stroke="{}" stroke-width="{}" stroke-linecap="round" stroke-linejoin="round">"#,
            stroke_color, stroke_width
        );

        SvgBuilder { content }
    }

    pub fn add_line(&mut self, line: Line) {
        let Line(Point(x1, y1), Point(x2, y2)) = line;
        self.content += "    ";
        self.content += &format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" />"#,
            x1, y1, x2, y2
        );
        self.content += "\n"
    }

    pub fn add_robinson_triangle(&mut self, rt: RobinsonTriangle, fill_color: &str) {
        self.content += "    ";
        self.content += &format!(
            r#"<path d="M {} {} L {} {} L {} {}" fill="{}" />"#,
            rt.a.0, rt.a.1, rt.b.0, rt.b.1, rt.c.0, rt.c.1, fill_color
        );
        self.content += "\n";
    }

    pub fn add_quadrilateral(&mut self, quad: Quadrilateral, fill_color: &str) {
        let path = &format!(
            "M {} {} L {} {} L {} {} L {} {} Z",
            quad.a.0, quad.a.1, quad.b.0, quad.b.1, quad.c.0, quad.c.1, quad.d.0, quad.d.1
        );
        self.content += "    ";
        self.content += &format!(
            r#"<path d="{}" fill="{}" />"#,
            path, fill_color
        );
        self.content += "\n";
    }

    pub fn build(mut self, out_file: &mut File) -> std::io::Result<()> {
        self.content += "  </g>\n";
        self.content += "</svg>\n";
        out_file.write_all(self.content.as_bytes())?;
        Ok(())
    }
}
