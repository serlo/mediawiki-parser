//! This program takes Media Wiki source code and produces a yaml syntax tree.
//!
//! It aims to provide fast offline processing with debug information
//! (element position) included. The resulting tree represents the input
//! document on a syntactic level. Please refer to the `mediawiki_parser`
//! documentation for a description of possible elements of the abstract
//! syntax tree.

use mediawiki_parser;
use serde_json;
use serde_yaml;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// This program takes MediaWiki source code and produces
/// a yaml syntax tree on stdout.
struct Args {
    /// Path to the input file.
    /// If none is provided, stdin is used.
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    pub input_file: Option<PathBuf>,

    /// Ouput the result as JSON
    #[structopt(short = "j", long = "json")]
    pub use_json: bool,
}

/// read contents of a `io::Reader` into a string
fn read_from_reader(reader: &mut io::Read) -> String {
    let mut buffer = io::BufReader::new(reader);
    let mut content = String::new();
    buffer
        .read_to_string(&mut content)
        .expect("Could not read fron file!");
    content
}

/// Read a file from disk and store to string.
fn read_file(filename: &PathBuf) -> String {
    let file = fs::File::open(filename).expect("Could not open file!");
    let mut reader = BufReader::new(file);
    read_from_reader(&mut reader)
}

/// Read a file from stdin from to string.
fn read_stdin() -> String {
    read_from_reader(&mut io::stdin())
}

fn main() {
    let args = Args::from_args();
    let input = if let Some(path) = args.input_file {
        read_file(&path)
    } else {
        read_stdin()
    };

    let result = mediawiki_parser::parse(&input);
    match result {
        Ok(r) => {
            if args.use_json {
                serde_json::to_writer(io::stdout(), &r).expect("could not serialize json!");
            } else {
                serde_yaml::to_writer(io::stdout(), &r).expect("could not serialize yaml!");
            };
            println!();
        }
        Err(e) => {
            eprintln!("{}", e);
            if args.use_json {
                serde_json::to_writer(io::stdout(), &e).expect("could not serialize json!");
            } else {
                serde_yaml::to_writer(io::stdout(), &e).expect("could not serialize yaml!");
            };
            println!();
            process::exit(1);
        }
    };
}
