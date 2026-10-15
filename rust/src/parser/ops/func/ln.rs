use chumsky::{Parser, span::Spanned};

use crate::{
    Expects, match_tensor,
    parser::{
        ParserError,
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::{func::func_parser, word_or_op},
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
    match_tensor!((value) => {
        s => |s: Spanned<num::Complex<f64>>| Ok(ln_c(&s)),
        m => |mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(|el| ln_c(&el));
            Ok(m.inner)
        }
    })
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
