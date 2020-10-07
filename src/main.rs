mod geometry;
mod svg;

use geometry::*;
use svg::*;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut builder = SvgBuilder::new(300, 300);
    builder.add_line(Line(Point(100, 100), Point(100, 200)), "#d30082", 3);
    builder.add_line(Line(Point(100, 200), Point(200, 200)), "#d30082", 3);
    builder.add_line(Line(Point(200, 200), Point(200, 100)), "#d30082", 3);
    builder.add_line(Line(Point(200, 100), Point(100, 100)), "#d30082", 3);

    let mut out_file = File::create("out.svg")?;
    builder.build(&mut out_file)?;
    Ok(())
}
