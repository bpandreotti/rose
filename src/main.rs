mod geometry;
mod svg;

use geometry::*;
use svg::*;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let (a, b) = (Point(100.0, 100.0), Point(200.0, 120.0));
    let t1 = RobinsonTriangle::from_base(a, b, RobinsonTriangleType::Large);
    let t2 = RobinsonTriangle::from_base(b, a, RobinsonTriangleType::Large);
    let mut builder = SvgBuilder::new(400, 400);
    for l in &t1.lines() {
        builder.add_line(*l, "#000", 2);
    }
    for l in &t2.lines() {
        builder.add_line(*l, "#000", 2);
    }
    let mut out_file = File::create("out.svg")?;
    builder.build(&mut out_file)?;
    Ok(())
}
