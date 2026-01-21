mod state;
mod transition;
mod unique_bool;
mod standalone;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use clap::Parser;
use crate::standalone::Standalone;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source code or path to source file
    source: String,
    
    /// Automatically wrap state and transition names in '$'
    #[arg(short, long)]
    assume_math: bool,
    
    /// Skip path check
    #[arg(short, long)]
    ignore_path: bool,
}

fn is_file<P: AsRef<Path>>(path: P) -> bool {
    if let Some(ext) = path.as_ref().extension() {
        ext.eq_ignore_ascii_case("stag")
    } else {
        false
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> String {
    let mut source = String::new();
    let mut file = File::open(path).expect("Error: Unable to open file.");
    file.read_to_string(&mut source).expect("Error: Unable to read file.");
    source
}

fn compile(source: &String) -> Standalone {
   Standalone::new()
}

fn main() {
    let args = Args::parse();
    let source = if !args.ignore_path && is_file(&args.source) {
        read_file(args.source)
    } else {
        args.source
    };
    let document = compile(&source);
    println!("{}", document.build());
}
