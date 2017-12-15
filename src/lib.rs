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

mod general_transformations;

mod grammar;


fn apply_general_transformations(mut root: ast::Element) -> transformations::TResult {

    let settings = general_transformations::GeneralSettings {};
    root = general_transformations::fold_headings_transformation(root, &settings)?;
    root = general_transformations::fold_lists_transformation(root, &settings)?;
    root = general_transformations::whitespace_paragraphs_to_empty(root, &settings)?;
    root = general_transformations::collapse_paragraphs(root, &settings)?;
    root = general_transformations::collapse_consecutive_text(root, &settings)?;
    Ok(root)
}

/// Parse a mediawiki source document and build a syntax tree.
pub fn parse_document(input: &str) -> Result<ast::Element, error::MWError> {
    let source_lines = util::get_source_lines(&input);

    let result = match grammar::Document(&input, &source_lines) {
        Err(e) => Err(error::MWError::ParseError(error::ParseError::from(&e, input))),
        Ok(r) => Ok(r)
    }?;

    match apply_general_transformations(result) {
        Err(e) => Err(error::MWError::TransformationError(e)),
        Ok(r) => Ok(r)
    }
}


