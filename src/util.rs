//! Utility functions and types

use ast;

/// The terminal width.
const TERMINAL_WIDTH: usize = 80;



/// Compiles a list of start and end positions of the input source lines.
///
/// This representation is used to calculate line and column position from the input offset.
pub fn get_source_lines(source: &str) -> Vec<ast::SourceLine> {

    let mut pos = 0;
    let mut result = Vec::new();

    for line in source.split('\n') {
        result.push(ast::SourceLine {
            start: pos,
            content: line,
            end: pos + line.len() + 1,
        });
        pos += line.len() + 1;
    }
    result
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


/// Shorten a string to fit into `TERMINAL_WIDTH`.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_whitespace() {
        for arg in &["", "   ", "\t", "\n", "\t\t\t", "\n\t "] {
            assert!(is_whitespace(arg), "is_whitespace({:?})", arg);
        }

        for arg in &["a", "    a", "\t\\", "   \nä\t\t\t "] {
            assert!(!is_whitespace(arg), "!is_whitespace({:?})", arg);
        }
    }
}
