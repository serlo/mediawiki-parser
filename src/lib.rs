#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::path::Path;
use std::fs;
use std::io::prelude::*;
use std::io;

#[cfg(test)]
mod tests;

/// Data structures describing the parsed document.
pub mod ast;

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

fn read_from_reader(reader: &mut io::Read) -> String {
    let mut buffer = io::BufReader::new(reader);
    let mut content = String::new();
    buffer.read_to_string(&mut content)
        .expect("Could not read fron file!");
    content
}

/**
 * Read a file from disk and store to string.
 */
pub fn read_file(filename: &str) -> String {
    let mut file = fs::File::open(Path::new(filename))
        .expect("Could not open file!");
    read_from_reader(&mut file)
}

/**
 * Read a file from stdin from to string.
 */
pub fn read_stdin() -> String {
    read_from_reader(&mut io::stdin())
}

/**
 * Parse a mediawiki source document and build a syntax tree.
 */
pub fn parse_document(input: &str) -> Result<ast::Element, grammar::ParseError> {
    let source_lines = ast::get_source_lines(&input);
    grammar::Document(&input, &source_lines)
}
