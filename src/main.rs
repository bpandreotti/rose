#[cfg(test)] extern crate rand;

#[macro_use] mod geometry;
mod seeds;
mod svg;
mod tiling;

use geometry::*;
use svg::*;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut triangles = seeds::rose().transform(Point(3500.0, 3500.0), 3000.0);
    for _ in 0..6 {
        triangles = tiling::inflate_all(triangles);
    }
    let quads = tiling::merge_pairs(triangles);

    let mut builder = SvgBuilder::new(7000, 7000, "#000", 6);
    builder.add_all_quads(quads, ("#ea4848", "#e8694c"));
    let mut out_file = File::create("out.svg")?;
    builder.build(&mut out_file)?;
    Ok(())
}
