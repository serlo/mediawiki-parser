mod ast;
mod error;
#[cfg_attr(
    feature = "cargo-clippy",
    allow(
        unit_arg,
        cyclomatic_complexity,
        len_zero,
        single_match,
        naive_bytecount,
        suspicious_else_formatting
    )
)]
mod grammar;
#[cfg(test)]
mod tests;
mod traversion;
mod util;

// public exports
pub use self::ast::*;
pub use self::error::*;
pub use self::traversion::Traversion;

pub mod transformations;

mod default_transformations;
use self::default_transformations::*;

/// Parse the input document to generate a document tree.
/// After parsing, some transformations are applied to the result.
pub fn parse(input: &str) -> Result<Element, MWError> {
    let source_lines = util::get_source_lines(input);

    #[cfg(feature = "ptime")]
    let starttime = time::precise_time_ns();

    let result = match grammar::document(input, &source_lines) {
        Err(e) => Err(error::MWError::ParseError(error::ParseError::from(
            &e, input,
        ))),
        Ok(r) => Ok(r),
    }?;

    #[cfg(feature = "ptime")]
    let parsedtime = time::precise_time_ns();

    let settings = GeneralSettings {};
    let trans_result = apply_transformations(result, &settings);

    #[cfg(feature = "ptime")]
    {
        eprintln!(
            "Parse Timer: Parsing took {} ms.",
            ((parsedtime - starttime) as f64) / 1.0e6
        );
        eprintln!(
            "Parse Timer: Transformation took {} ms.",
            ((time::precise_time_ns() - parsedtime) as f64) / 1.0e6
        );
    }

    trans_result.map_err(error::MWError::TransformationError)
}

fn apply_transformations(
    mut root: Element,
    settings: &GeneralSettings,
) -> transformations::TResult {
    root = fold_headings_transformation(root, settings)?;
    root = fold_lists_transformation(root, settings)?;
    root = whitespace_paragraphs_to_empty(root, settings)?;
    root = collapse_paragraphs(root, settings)?;
    root = collapse_consecutive_text(root, settings)?;
    root = enumerate_anon_args(root, settings)?;
    Ok(root)
}
