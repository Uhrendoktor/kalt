use chumsky::{Parser, span::Spanned};
use num::Zero;
use sertyp::{SYMBOL_CC, SYMBOL_eq_not, SYMBOL_in, TypstError, content, sequence};

use crate::{
    Expects,
    parser::{
        ParserError,
        atoms::tensor::Tensor,
        ops::{
            self,
            element_wise::{element_wise2, validate_same_shape},
            func::{func_parser, subscript_parser},
            word_or_op,
        },
        pratt::{pratt, span},
    },
    subscript,
};

use super::ln::ln_c;

/// Computes log_b(v) for complex numbers b and v
pub fn log_c(c1: &num::Complex<f64>, c2: &num::Complex<f64>) -> num::Complex<f64> {
    ln_c(c2) / ln_c(c1)
}

fn validate_base<'data>(base: &Spanned<num::Complex<f64>>) -> Expects<'data, ()> {
    if base.is_zero() {
        return Err(TypstError::full(
            base.span,
            "logarithm base cannot be zero",
            content!(sequence!(
                subscript!({"log"}_{"b"}),
                "(c) | b",
                SYMBOL_in,
                subscript!({SYMBOL_CC}_{sequence!(SYMBOL_eq_not, "1")})
            )),
            content!(sequence!(subscript!({"log"}_{"0"}), "(x)")),
        ));
    }
    Ok(())
}

/// Computes log_b(v):
/// (scalar, scalar) -> scalar
/// (scalar, matrix) -> matrix (element-wise)
/// (matrix, scalar) -> matrix (element-wise)
/// (matrix, matrix) -> matrix (element-wise) only if same shape
pub fn log_t<'data>(base: Spanned<Tensor>, value: Spanned<Tensor>) -> Expects<'data, Tensor> {
    element_wise2(
        |base, value| {
            validate_base(&base)?;
            Ok(log_c(&base, &value))
        },
        validate_same_shape(|base, value| {
            content!(sequence![subscript!({"log"}_{base}), '(', value, ')'])
        }),
        base,
        value,
    )
}

/// Parses the log keyword and returns the base
///
/// EBNF:
/// "log" _ <tensor>
#[kalt_macros::parser]
pub fn log_fn<'data>() -> Expects<'data, Spanned<Tensor>> {
    subscript_parser(|| word_or_op("log"), pratt).map(|(_, base)| ops::transpose(base))
}

#[kalt_macros::parser]
pub fn log(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(log_fn(), span(parser)).map(|(base, value)| log_t(base?, value?))
}
