mod geometry;
mod svg;
mod tiling;

use geometry::*;
use svg::*;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut triangles = rose_seed(Point(1500.0, 1500.0), 3000.0);
    for _ in 0..5 {
        triangles = tiling::inflate_all(triangles);
    }
    let quads = tiling::merge_pairs(triangles);

    let mut builder = SvgBuilder::new(3000, 3000);
    for q in quads {
        // This is kind of hacky
        let fill_color = if q.a.distance_to(q.c) > q.b.distance_to(q.d) {
            "#e8694c"
        } else {
            "#ea4848"
        };
        builder.add_quadrilateral(q, fill_color, "#000", 6)
    }
    let mut out_file = File::create("out.svg")?;
    builder.build(&mut out_file)?;
    Ok(())
}

fn rose_seed(center: Point, scale: f64) -> Vec<RobinsonTriangle> {
    const RAD_TO_DEG: f64 = std::f64::consts::PI / 180.0;
    let a = Point(1.0, 0.0);
    let b = Point(f64::cos(36.0 * RAD_TO_DEG), -f64::sin(36.0 * RAD_TO_DEG));
    let c = Point(f64::cos(72.0 * RAD_TO_DEG), -f64::sin(72.0 * RAD_TO_DEG));
    let d = Point(-c.0, c.1);
    let e = Point(-b.0, b.1);
    let f = -a;
    let mut top_half = vec![
        RobinsonTriangle::new(a, Point::ZERO, b),
        RobinsonTriangle::new(c, Point::ZERO, b),
        RobinsonTriangle::new(c, Point::ZERO, d),
        RobinsonTriangle::new(e, Point::ZERO, d),
        RobinsonTriangle::new(e, Point::ZERO, f),
    ];
    let mut bottom_half = top_half
        .iter()
        .map(|t| {
            let RobinsonTriangle { a, b, c, .. } = t;
            RobinsonTriangle::new(Point(a.0, -a.1), *b, Point(c.0, -c.1))
        })
        .collect::<Vec<_>>();
    top_half.append(&mut bottom_half);
    top_half
        .iter()
        .map(|t| {
            let RobinsonTriangle { a, b, c, .. } = *t;
            RobinsonTriangle::new(scale * a + center, scale * b + center, scale * c + center)
        })
        .collect()
}
