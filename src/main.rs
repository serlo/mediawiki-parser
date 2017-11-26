//! This program takes MediaWiki source code and produces a yaml syntax tree.
//!
//! It aims to provide fast offline processing with debug information
//! (element position) included. The resulting tree represents the input
//! document on a syntactic level. Please refer to the mediawiki_parser
//! documentation for a description of possible elements of the abstract
//! syntax tree.

extern crate mediawiki_parser;
extern crate serde_yaml;
extern crate argparse;

use std::io;
use std::process;
use argparse::{ArgumentParser, StoreTrue, Store};

macro_rules! DESCRIPTION {
() => (
"This program takes MediaWiki source code and produces
 a yaml syntax tree on stdout."
)
}

fn main() {
    let mut use_stdin = false;
    let mut input_file = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description(DESCRIPTION!());
        ap.refer(&mut use_stdin)
            .add_option(&["-s", "--stdin"], StoreTrue, "Use stdin as input file");

        ap.refer(&mut input_file)
            .add_option(&["-i", "--input"], Store, "Path to the input file");

        ap.parse_args_or_exit();
    }

    let input: String;
    if use_stdin {
        input = mediawiki_parser::read_stdin();
    } else if !input_file.is_empty() {
        input = mediawiki_parser::read_file(&input_file);
    } else {
        eprintln!("No input source specified!");
        process::exit(1);
    }

    let result = mediawiki_parser::parse_document(&input);

    match result {
        Ok(r) => {
            serde_yaml::to_writer(io::stdout(), &r).unwrap();
            println!();
        },
        Err(e) => {
            eprintln!("{}", e);
            serde_yaml::to_writer(io::stdout(), &e).unwrap();
            println!();
            process::exit(1);
        },
    }
}
