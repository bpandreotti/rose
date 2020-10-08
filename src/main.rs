mod geometry;
mod svg;
mod tiling;

use geometry::*;
use svg::*;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let (a, c) = (Point(200.0, 2400.0), Point(2600.0, 2000.0));
    let seed = RobinsonTriangle::from_base(a, c, RobinsonTriangleType::Large);
    let mut triangles = vec![seed];
    for _ in 0..7 {
        triangles = tiling::inflate_all(triangles);
    }
    let colors = ["#399360", "#396c93", "#39938d"].iter().cycle();
    let mut builder = SvgBuilder::new(3000, 3000);
    for t in triangles {
        for (&l, c) in t.lines().iter().zip(colors.clone()) {
            builder.add_line(l, c, 2);
        }
        let [l1, l2] = t.arc_lines();
        builder.add_line(l1, "red", 2);
        builder.add_line(l2, "yellow", 2);
    }
    let mut out_file = File::create("out.svg")?;
    builder.build(&mut out_file)?;
    Ok(())
}
