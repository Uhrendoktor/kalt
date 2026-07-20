use chumsky::{Parser, span::Spanned};
use sertyp::TypstError;

use crate::{
    Expects, match_tensors,
    parser::{
        ParserError,
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::{
            self,
            func::{func_parser, subscript_parser},
            word_or_op,
        },
        pratt::{pratt, span},
    },
};

/// Computes log_b(v) for complex numbers b and v
pub fn log_c(c1: &num::Complex<f64>, c2: &num::Complex<f64>) -> num::Complex<f64> {
    c1.ln() / c2.ln()
}

/// Computes log_b(v):
/// (scalar, scalar) -> scalar
/// (scalar, matrix) -> matrix (element-wise)
/// (matrix, scalar) -> matrix (element-wise)
/// (matrix, matrix) -> matrix (element-wise) only if same shape
pub fn log_t<'data>(base: Spanned<Tensor>, value: Spanned<Tensor>) -> Expects<'data, Tensor> {
    match_tensors!((base, value) => {
        (s1, s2) => |s1: Spanned<num::Complex<f64>>, s2: Spanned<num::Complex<f64>>| Ok(log_c(&s1, &s2)),
        (s, m) => |s: Spanned<num::Complex<f64>>, mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(|el| log_c(&s, &el));
            Ok(m.inner)
        },
        (m, s) => |mut m: Spanned<Matrix>, s: Spanned<num::Complex<f64>>| {
            m.inner.mapv_inplace(|el| log_c(&el, &s));
            Ok(m.inner)
        },
        (m1, m2) => |mut m1: Spanned<Matrix>, m2: Spanned<Matrix>| {
            m1.iter_mut().zip(m2.iter()).try_for_each(|(x, y)| {
                *x = log_c(x, y);
                Ok::<_, TypstError>(())
            })?;
            Ok(m1.inner)
        }
    })
}

/// Parses the log keyword and returns the base
///
/// EBNF:
/// "log" _ <tensor>
#[kalt_macros::parser]
pub fn log_fn<'data>() -> Expects<'data, Spanned<Tensor>> {
    subscript_parser(word_or_op("log"), pratt()).map(|(_, base)| ops::transpose(base))
}

#[kalt_macros::parser]
pub fn log(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(log_fn(), span(parser)).map(|(base, value)| log_t(base?, value?))
}
