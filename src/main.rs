mod geometry;
mod svg;
mod tiling;

use geometry::*;
use svg::*;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let (a, c) = (Point(200.0, 1500.0), Point(2800.0, 1500.0));
    let seed1 = RobinsonTriangle::from_base(a, c, RobinsonTriangleType::Large, true);
    let seed2 = RobinsonTriangle::from_base(a, c, RobinsonTriangleType::Large, false);
    let mut triangles = vec![seed1, seed2];
    for _ in 0..7 {
        triangles = tiling::inflate_all(triangles);
    }
    let quads = tiling::merge_pairs(triangles);

    let mut builder = SvgBuilder::new(3000, 3000);
    for q in quads {
        // This is kind of hacky
        let fill_color = if q.a.distance_to(q.c) > q.b.distance_to(q.d) {
            "#aef26a"
        } else {
            "#71dd58"
        };
        builder.add_quadrilateral(q, fill_color, "#000", 1)
    }
    let mut out_file = File::create("out.svg")?;
    builder.build(&mut out_file)?;
    Ok(())
}
