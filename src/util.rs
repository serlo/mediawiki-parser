use std::path::Path;
use std::fs;
use std::io::prelude::*;
use std::io;

/// The terminal width.
const TERMINAL_WIDTH: usize = 80;


/// read contents of a io::Reader into a string
fn read_from_reader(reader: &mut io::Read) -> String {
    let mut buffer = io::BufReader::new(reader);
    let mut content = String::new();
    buffer.read_to_string(&mut content)
        .expect("Could not read fron file!");
    content
}

/// Read a file from disk and store to string.
pub fn read_file(filename: &str) -> String {
    let mut file = fs::File::open(Path::new(filename))
        .expect("Could not open file!");
    read_from_reader(&mut file)
}


/// Read a file from stdin from to string.
pub fn read_stdin() -> String {
    read_from_reader(&mut io::stdin())
}

/// Tests if a string is entirely whitespace
pub fn is_whitespace(input: &str) -> bool {
    for c in input.chars() {
        if !c.is_whitespace() {
            return false;
        }
    }
    true
}

/// Shorten a string to fit into TERMINAL_WIDTH.
pub fn shorten_str(input: &str) -> String {

    let input_len = input.chars().count();

    if input.len() < TERMINAL_WIDTH {
        return String::from(input);
    }

    let filler = " .. ";
    let mut result = String::new();
    let half_text_size = (TERMINAL_WIDTH - filler.chars().count()) / 2;

    for (char_count, c) in input.chars().enumerate() {
        if char_count < half_text_size {
            result.push(c);
        }
        if char_count == half_text_size {
            result.push_str(filler);
        }
        if char_count >= input_len - half_text_size {
            result.push(c);
        }
    }
    result
}


