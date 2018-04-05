#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate colored;

#[cfg(test)]
extern crate serde_yaml;

#[cfg(test)]
mod tests;

mod traversion;
mod ast;
mod error;
mod util;
#[cfg_attr(feature = "cargo-clippy", allow(unit_arg, cyclomatic_complexity,
     len_zero, single_match, naive_bytecount, suspicious_else_formatting))]
mod grammar;


// public exports
pub use ast::*;
pub use traversion::Traversion;
pub use error::*;

pub mod transformations;

mod default_transformations;
use default_transformations::*;


/// Parse the input document to generate a document tree.
/// After parsing, some transformations are applied to the result.
pub fn parse(input: &str) -> Result<Element, MWError> {

    let source_lines = util::get_source_lines(input);

    let result = match grammar::document(input, &source_lines) {
        Err(e) => Err(error::MWError::ParseError(
            error::ParseError::from(&e, input),
        )),
        Ok(r) => Ok(r),
    }?;

    let settings = GeneralSettings {};
    match apply_transformations(result, &settings) {
        Err(e) => Err(error::MWError::TransformationError(e)),
        Ok(r) => Ok(r),
    }
}

fn apply_transformations(mut root: Element, settings: &GeneralSettings)
    -> transformations::TResult {

    root = fold_headings_transformation(root, settings)?;
    root = fold_lists_transformation(root, settings)?;
    root = whitespace_paragraphs_to_empty(root, settings)?;
    root = collapse_paragraphs(root, settings)?;
    root = collapse_consecutive_text(root, settings)?;
    root = enumerate_anon_args(root, settings)?;
    Ok(root)
}
