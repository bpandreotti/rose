use crate::geometry::*;

use std::fs::File;
use std::io::prelude::*;

pub struct SvgBuilder {
    content: String,
}

impl SvgBuilder {
    pub fn new(width: u64, height: u64) -> Self {
        // @TODO: Add the stroke color and width in a <g> tag
        let mut content = r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string() + "\n";
        content += &format!(
            r#"<svg width="100%" height="100%" viewBox="0 0 {} {}" "#,
            width, height
        );
        content += r#"preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg">"#;
        content += "\n";

        SvgBuilder { content }
    }

    pub fn add_line(&mut self, line: Line, color: &str, stroke_width: u64) {
        let Line(Point(x1, y1), Point(x2, y2)) = line;
        self.content += "  ";
        self.content += &format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}"/>"#,
            x1, y1, x2, y2, color, stroke_width
        );
        self.content += "\n"
    }

    pub fn add_robinson_triangle(
        &mut self,
        rt: RobinsonTriangle,
        fill_color: &str,
        stroke_color: &str,
        stroke_width: u64,
        arc_colors: Option<(&str, &str)>,
    ) {
        self.content += "  ";
        self.content += &format!(
            r#"<path d="M {} {} L {} {} L {} {}" fill="{}" stroke="{}" stroke-width="{}"/>"#,
            rt.a.0, rt.a.1, rt.b.0, rt.b.1, rt.c.0, rt.c.1, fill_color, stroke_color, stroke_width
        );
        self.content += "\n";
        if let Some((a_color, b_color)) = arc_colors {
            let [a, b] = rt.arc_lines();
            self.add_line(a, a_color, stroke_width);
            self.add_line(b, b_color, stroke_width);
        }
    }

    pub fn add_quadrilateral(
        &mut self,
        quad: Quadrilateral,
        fill_color: &str,
        stroke_color: &str,
        stroke_width: u64,
    ) {
        self.content += "  ";
        self.content += &format!(
            r#"<path d="M {} {} L {} {} L {} {} L {} {} Z" "#,
            quad.a.0, quad.a.1, quad.b.0, quad.b.1, quad.c.0, quad.c.1, quad.d.0, quad.d.1,
        );
        self.content += &format!(
            r#"fill="{}" stroke="{}" stroke-width="{}" "#,
            fill_color, stroke_color, stroke_width,
        );
        self.content += r#"stroke-linecap="round" stroke-linejoin="round" />"#;
        self.content += "\n";
    }

    pub fn build(self, out_file: &mut File) -> std::io::Result<()> {
        out_file.write_all(self.content.as_bytes())?;
        writeln!(out_file, "</svg>\n")?;
        Ok(())
    }
}
