use chumsky::{Parser, span::Spanned};
use sertyp::{
    SYMBOL_ceil_l, SYMBOL_ceil_r,
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

/// ceils a tensor:
/// scalar -> scalar (norm)
/// matrix -> scalar (element-wise)
pub fn ceil_t<'data>(t: Spanned<Tensor>) -> Expects<'data, Tensor> {
    unary(t, |value| num::Complex::<f64>::new(value.re.ceil(), value.im.ceil()))
}

/// Parses the ceiling operator
///
/// EBNF:
/// "⌈" <tensor> "⌉"
#[kalt_macros::parser]
pub fn ceil<'data>(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    span(parser)
        .delimited_by(whitespaces(), whitespaces())
        .delimited_by(character(SYMBOL_ceil_l), character(SYMBOL_ceil_r))
        .map(|t| ceil_t(t?))
}
