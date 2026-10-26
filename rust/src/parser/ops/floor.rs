use chumsky::{Parser, span::Spanned};
use sertyp::{
    SYMBOL_floor_l, SYMBOL_floor_r,
    chumsky::parser::{character, whitespaces},
};

use crate::{
    Expects,
    parser::{
        ParserError,
        atoms::tensor::Tensor,
        ops::element_wise::unary,
        pratt::span,
    },
};

/// floors a tensor:
/// scalar -> scalar (norm)
/// matrix -> scalar (element-wise)
pub fn floor_t<'data>(t: Spanned<Tensor>) -> Expects<'data, Tensor> {
    unary(t, |value| num::Complex::<f64>::new(value.re.floor(), value.im.floor()))
}

/// Parses the floor operator
///
/// EBNF:
/// "⌊" <tensor> "⌋"
#[kalt_macros::parser]
pub fn floor<'data>(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    span(parser)
        .delimited_by(whitespaces(), whitespaces())
        .delimited_by(character(SYMBOL_floor_l), character(SYMBOL_floor_r))
        .map(|t| floor_t(t?))
}
