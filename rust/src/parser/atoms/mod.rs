use chumsky::{Parser, span::SimpleSpan};
use sertyp::{Content, LocatingSequence, chumsky::parser::delimited_by_groups};

use crate::{
    Expects,
    parser::{atoms::tensor::Tensor, pratt::pratt, validate},
};

pub mod complex;
pub mod matrix;
pub mod tensor;

pub use tensor::tensor;

/// Parses content into a complex number.
/// Wraps parser errors and expects into expects.
pub fn parse_content<'data, O>(
    validator: impl Clone + Fn(Tensor, SimpleSpan) -> Expects<'data, O>,
) -> impl Fn(&Content<'data>) -> Expects<'data, O> {
    move |content| {
        validate(
            delimited_by_groups(pratt::<LocatingSequence>()),
            validator.clone(),
        )
        .parse(LocatingSequence::from(content))
        // ? merges parser errors and expects into expects
        .into_result()?
    }
}
