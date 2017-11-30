#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate colored;

#[cfg(test)]
extern crate serde_yaml;

use std::path::Path;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::error;
use std::fmt;
use colored::*;

#[cfg(test)]
mod tests;

/// Data structures describing the parsed document.
pub mod ast;

/// Macros and data structures for source tree transformations.
#[macro_use]
pub mod transformations;

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

/// The number of lines to display as error context.
const ERROR_CONTEXT_LINES: usize = 5;

/// The terminal width.
const TERMINAL_WIDTH: usize = 80;


/// The parser error with source code context.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="lowercase", deny_unknown_fields)]
pub struct ParseError {
    pub position: ast::Position,
    pub expected: Vec<String>,
    pub context: Vec<String>,
    pub context_start: usize,
    pub context_end: usize,
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

fn is_whitespace(input: &str) -> bool {
    for c in input.chars() {
        if !c.is_whitespace() {
            return false;
        }
    }
    true
}

fn shorten_str(input: &str) -> String {

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


/**
 * Parse a mediawiki source document and build a syntax tree.
 */
pub fn parse_document(input: &str) -> Result<ast::Element, ParseError> {
    let source_lines = ast::get_source_lines(&input);

    let mut result = match grammar::Document(&input, &source_lines) {
        Err(e) => Err(ParseError::from(&e, input)),
        Ok(r) => Ok(r)
    }?;

    result = transformations::fold_headings_transformation(result);
    result = transformations::fold_lists_transformation(result);
    Ok(result)
}

impl ParseError {
    pub fn from(err: &grammar::ParseError, input: &str) -> Self {

        let source_lines = ast::get_source_lines(&input);
        let line_count = source_lines.len();

        let line = if err.line <= line_count {
            err.line
        } else {
            source_lines.len()
        } - 1;

        let start = if line < ERROR_CONTEXT_LINES {
            0
        } else {
            line - ERROR_CONTEXT_LINES
        };

        let end = if line + ERROR_CONTEXT_LINES >= line_count {
            line_count - 1
        } else {
            line + ERROR_CONTEXT_LINES
        };

        let mut token_str = vec![];
        for token in &err.expected {
            token_str.push(String::from(*token));
        }


        let mut context = vec![];
        for sloc in source_lines[start..end + 1].iter() {
            context.push(String::from(sloc.content));
        }

        ParseError {
            position: ast::Position::new(err.offset, &source_lines),
            context: context,
            expected:  token_str,
            context_start: start,
            context_end: end,
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Could not continue to parse, because no rules could be matched."
    }
}

impl fmt::Display for ParseError {

    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let error_message = format!("ERROR in line {} at column {}: Could not continue to parse, expected one of: ",
            self.position.line, self.position.col).red().bold();

        let mut token_str = vec![];
        for token in &self.expected {
            if is_whitespace(token) {
                token_str.push(format!("{:?}", token));
            } else {
                token_str.push(format!("{}", token));
            }
        }

        write!(f, "{}", error_message)?;
        write!(f, "{}\n", token_str.join(", ").blue().bold())?;

        for (i, content) in self.context.iter().enumerate() {

            let lineno = format!("{} |", self.context_start + i + 1);
            let lineno_col;

            let formatted_content;
            // the erroneous line
            if self.context_start + i + 1 == self.position.line {
                formatted_content = content.red();
                lineno_col = lineno.red().bold();
            } else {
                formatted_content = shorten_str(content).normal();
                lineno_col = lineno.blue().bold()
            }

            writeln!(f, "{} {}", lineno_col, formatted_content)?;
        }

        Ok(())
    }
}
