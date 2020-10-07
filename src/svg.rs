use crate::geometry::*;

use std::fs::File;
use std::io::prelude::*;

pub struct SvgBuilder {
    content: String,
}

impl SvgBuilder {
    pub fn new(width: u64, height: u64) -> Self {
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

    pub fn build(self, out_file: &mut File) -> std::io::Result<()> {
        out_file.write_all(self.content.as_bytes())?;
        writeln!(out_file, "</svg>\n")?;
        Ok(())
    }
}
