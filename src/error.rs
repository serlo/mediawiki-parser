//! Error structures

use crate::ast::{Element, Position, Span};
use crate::grammar;
use crate::util::{get_source_lines, is_whitespace, shorten_str};
use colored::*;
use serde_derive::{Deserialize, Serialize};
use std::error;
use std::fmt;

/// The number of lines to display as error context.
const ERROR_CONTEXT_LINES: usize = 5;

/// Generic error type for high-level errors of this libaray.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub enum MWError {
    ParseError(ParseError),
    TransformationError(TransformationError),
}

/// The parser error with source code context.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct ParseError {
    pub position: Position,
    pub expected: Vec<String>,
    pub context: Vec<String>,
    pub context_start: usize,
    pub context_end: usize,
}

/// Error structure for syntax tree transformations.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct TransformationError {
    pub cause: String,
    pub position: Span,
    pub transformation_name: String,
    pub tree: Element,
}

impl ParseError {
    pub fn from(err: &grammar::ParseError, input: &str) -> Self {
        let source_lines = get_source_lines(input);
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
        for sloc in source_lines[start..=end].iter() {
            context.push(String::from(sloc.content));
        }

        ParseError {
            position: Position::new(err.offset, &source_lines),
            context,
            expected: token_str,
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
        let error_message = format!(
            "ERROR in line {} at column {}: Could not continue to parse, expected one of: ",
            self.position.line, self.position.col
        )
        .red()
        .bold();

        let mut token_str = vec![];
        for token in &self.expected {
            if is_whitespace(token) {
                token_str.push(format!("{:?}", token));
            } else {
                token_str.push(token.to_string());
            }
        }

        write!(f, "{}", error_message)?;
        writeln!(f, "{}", token_str.join(", ").blue().bold())?;

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

impl error::Error for TransformationError {
    fn description(&self) -> &str {
        &self.cause
    }
}

impl fmt::Display for TransformationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let message = format!(
            "ERROR applying transformation \"{}\" to Elemtn at {}:{} to {}:{}: {}",
            self.transformation_name,
            self.position.start.line,
            self.position.start.col,
            self.position.end.line,
            self.position.end.col,
            self.cause
        );
        writeln!(f, "{}", message.red().bold())
    }
}

impl error::Error for MWError {
    fn description(&self) -> &str {
        match *self {
            MWError::ParseError(ref e) => e.description(),
            MWError::TransformationError(ref e) => e.description(),
        }
    }
}

impl fmt::Display for MWError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            MWError::ParseError(ref e) => write!(f, "{}", e),
            MWError::TransformationError(ref e) => write!(f, "{}", e),
        }
    }
}
