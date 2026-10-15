use chumsky::span::SimpleSpan;
use sertyp::TypstError;

pub mod parser;

/// An error wrapper that does not abort the parser, but throws an inline error message.
///
/// Example:
///
/// The parser already concluded that a matrix was detected, but one of the elements is not parsable.
/// In that case the parser should not abort or backtrack, but throw an appropriate error message and continue parsing the rest of the matrix.
pub type Expects<'data, T, Span = SimpleSpan> = Result<T, TypstError<'data, Span>>;
