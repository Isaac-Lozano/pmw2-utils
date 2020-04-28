use std::fs::File;
use std::env;

use nxf::NxfObjGeom;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let f = File::open(filename).unwrap();
    let nxf = NxfObjGeom::from_read(f).unwrap();
    println!("{:#?}", nxf);
}