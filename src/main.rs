use std::path::PathBuf;
use clap::Parser;
use crate::args::Args;
use std::fs;
use crate::parser::Stag;

mod args;
mod parser;

fn main() {
    let args = Args::parse();
    let source = if args.force_string || !fs::exists(&args.source).is_ok_and(|x| x) {
        args.source
    } else {
        fs::read_to_string(&args.source).unwrap() // TODO: implement error handling
    };
    Stag::parse_source(source);
}