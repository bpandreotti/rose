use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut out_file = File::create("out.svg")?;
    write_header(&mut out_file, 300, 300)?;
    writeln!(
        out_file,
        r#"  <line x1="50" y1="50" x2="150" y2="150" stroke="blue" stroke-width="2"/>"#
    )?;
    writeln!(out_file, "</svg>")?;
    Ok(())
}

fn write_header(f: &mut File, width: u64, height: u64) -> std::io::Result<()> {
    writeln!(f, r#"<?xml version="1.0" encoding="utf-8"?>"#)?;
    write!(f, r#"<svg width="100%" height="100%" viewBox="0 0 {} {}" "#, width, height)?;
    writeln!(f, r#"preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg">"#)?;
    Ok(())
}
