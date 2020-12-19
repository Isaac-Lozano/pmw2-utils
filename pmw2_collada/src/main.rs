mod nxf2collada;
mod sf2collada;
mod matrix;

use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process;

use nxf::NxfObjGeom;
use sf::SceneTemplate;
use getopts::Options;

use nxf2collada::Nxf2Collada;
use sf2collada::Sf2Collada;

trait UnwrapOrBarfExt<T> {
    fn unwrap_or_barf(self, err_str: &str) -> T;
}

impl<T, E> UnwrapOrBarfExt<T> for Result<T, E>
    where E: Error
{
    fn unwrap_or_barf(self, err_desc: &str) -> T {
        self.unwrap_or_else(|err| {
            let err_string = format!("{}: {}", err_desc, err);
            barf(&err_string);
        })
    }
}

impl<T> UnwrapOrBarfExt<T> for Option<T> {
    fn unwrap_or_barf(self, err_desc: &str) -> T {
        self.unwrap_or_else(|| {
            let err_string = format!("{}", err_desc);
            barf(&err_string);
        })
    }
}

fn barf(msg: &str) -> ! {
    println!("Error: {}", msg);
    process::exit(-1);
}

fn print_help(program: &str, opts: Options) {
    println!("pmw2_collada v{}", env!("CARGO_PKG_VERSION"));
    println!("Written by OnVar");
    println!();
    let brief = format!("Usage: {} [options] OUT_FILE", program);
    print!("{}", opts.usage(&brief));
}

enum Operation {
    SfDecode(String),
    NxfDecode(String),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].as_str();

    let mut opts = Options::new();
    opts.optopt("", "sf", "SF input file", "FILE").long_only(true);
    opts.optopt("", "nxf", "NXF input file", "FILE").long_only(true);
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "placements", "include placements (bounding boxes and points)");
    let matches = opts.parse(&args[1..])
        .map_err(|err| barf(&err.to_string()))
        .unwrap();

    if matches.opt_present("h") {
        print_help(program, opts);
        return;
    }

    let include_placments = matches.opt_present("p");

    let out_filename = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_help(program, opts);
        return;
    };

    let operations: Vec<Operation> = vec![
        matches.opt_str("sf").map(|v| Operation::SfDecode(v)),
        matches.opt_str("nxf").map(|v| Operation::NxfDecode(v)),
    ]
        .into_iter()
        .filter_map(|v| v)
        .collect();

    if operations.len() > 1 {
        barf("Multiple input files specified");
    }
    
    if operations.len() == 0 {
        barf("No input files specified")
    }

    let operation = operations.into_iter().next().unwrap();

    match operation {
        Operation::SfDecode(in_filename) => {
            let fin = File::open(&in_filename).unwrap();
            let fout = File::create(out_filename).unwrap();

            let sf = SceneTemplate::from_read(fin).unwrap();
            let mut converter = Sf2Collada::new(sf, fout, include_placments);
            converter.write_collada().unwrap();
            println!("Successfully converted SF file to collada.");
        }
        Operation::NxfDecode(in_filename) => {
            let fin = File::open(&in_filename).unwrap();
            let fout = File::create(out_filename).unwrap();

            let in_file = Path::new(&in_filename)
                .file_name()
                .and_then(|f| Path::new(f).file_stem())
                .and_then(|f| f.to_str())
                .unwrap_or_else(|| barf("Could not get base file name"));

            let nxf = NxfObjGeom::from_read(fin).unwrap();
            let mut converter = Nxf2Collada::new(in_file.into(), nxf, fout);
            converter.write_collada().unwrap();
            println!("Successfully converted NXF file to collada.");
        }
    }
}
