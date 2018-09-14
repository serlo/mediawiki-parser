//! This program takes Media Wiki source code and produces a yaml syntax tree.
//!
//! It aims to provide fast offline processing with debug information
//! (element position) included. The resulting tree represents the input
//! document on a syntactic level. Please refer to the `mediawiki_parser`
//! documentation for a description of possible elements of the abstract
//! syntax tree.

extern crate mediawiki_parser;
extern crate serde_yaml;
#[macro_use]
extern crate structopt;

use std::io;
use std::io::prelude::*;
use std::fs;
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
}

/// read contents of a `io::Reader` into a string
fn read_from_reader(reader: &mut io::Read) -> String {
    let mut buffer = io::BufReader::new(reader);
    let mut content = String::new();
    buffer.read_to_string(&mut content).expect(
        "Could not read fron file!",
    );
    content
}


/// Read a file from disk and store to string.
fn read_file(filename: &PathBuf) -> String {
    let mut file = fs::File::open(filename)
        .expect("Could not open file!");
    read_from_reader(&mut file)
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
            serde_yaml::to_writer(io::stdout(), &r).unwrap();
            println!();
        }
        Err(e) => {
            eprintln!("{}", e);
            serde_yaml::to_writer(io::stdout(), &e).unwrap();
            println!();
            process::exit(1);
        }
    }
}
