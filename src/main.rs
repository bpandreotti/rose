#[cfg(test)] extern crate rand;

#[macro_use] mod geometry;
mod seeds;
mod svg;
mod tiling;

use geometry::*;
use svg::*;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut triangles = seeds::rose().transform(Point(1500.0, 1500.0), 3000.0);
    for _ in 0..5 {
        triangles = tiling::inflate_all(triangles);
    }
    let quads = tiling::merge_pairs(triangles);

    let mut builder = SvgBuilder::new(7000, 7000);
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
