#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate colored;

#[cfg(test)]
extern crate serde_yaml;

#[cfg(test)]
mod tests;

/// Data structures describing the parsed document.
pub mod ast;

/// Error structures
pub mod error;

/// Utility functions and types
pub mod util;

/// Macros and data structures for source tree transformations.
#[macro_use]
pub mod transformations;

mod grammar;


/// Parse a mediawiki source document and build a syntax tree.
pub fn parse_document(input: &str) -> Result<ast::Element, error::ParseError> {
    let source_lines = ast::get_source_lines(&input);

    let mut result = match grammar::Document(&input, &source_lines) {
        Err(e) => Err(error::ParseError::from(&e, input)),
        Ok(r) => Ok(r)
    }?;

    result = transformations::fold_headings_transformation(result);
    result = transformations::fold_lists_transformation(result);
    Ok(result)
}


