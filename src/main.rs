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
    let mut builder = SvgBuilder::new(3000, 3000);
    for t in triangles {
        let fill_color = match t.triangle_type {
            RobinsonTriangleType::Large => "#aef26a",
            RobinsonTriangleType::Small => "#71dd58",
        };
        builder.add_robinson_triangle(t, fill_color, "#000", 1, Some(("blue", "red")))
    }
    let mut out_file = File::create("out.svg")?;
    builder.build(&mut out_file)?;
    Ok(())
}
