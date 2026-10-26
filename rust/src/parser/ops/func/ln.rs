use chumsky::{Parser, span::Spanned};

use crate::{
    Expects,
    parser::{
        ParserError,
        atoms::tensor::Tensor,
        ops::{element_wise::unary, func::func_parser, word_or_op},
        pratt::span,
    },
};

/// Computes ln(v) for complex numbers
pub fn ln_c(c: &num::Complex<f64>) -> num::Complex<f64> {
    c.ln()
}

/// Computes ln(v):
/// scalar -> scalar
/// matrix -> matrix (element-wise)
pub fn ln_t<'data>(value: Spanned<Tensor>) -> Expects<'data, Tensor> {
    unary(value, |value| ln_c(&value))
}

/// Parses the log keyword and returns the base
///
/// EBNF:
/// "ln"
#[kalt_macros::parser]
pub fn ln_fn() -> () {
    word_or_op("ln").ignored()
}

#[kalt_macros::parser]
pub fn ln(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(ln_fn(), span(parser)).map(|(_, value)| ln_t(value?))
}
