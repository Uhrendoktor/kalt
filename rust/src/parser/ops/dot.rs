use chumsky::{
    primitive::choice,
    span::{Span, Spanned, WrappingSpan},
};
use sertyp::{
    SYMBOL_CC, SYMBOL_dot_op, TypstError, Underline, chumsky::parser::character, sequence,
};

use crate::{
    Expects, match_tensors, matrix_shape,
    parser::atoms::{matrix::Matrix, tensor::Tensor},
    pratt_infix,
};

/// Multiplication: Dot product
/// Condition for the pratt parser to apply.
#[kalt_macros::parser]
pub fn pratt_dot_operator() -> char {
    choice((character(SYMBOL_dot_op),))
}

// dot product of two tensors
pratt_infix!(dot => |_op, lhs: Spanned<_>, rhs: Spanned<_>| Ok(lhs.span.union(rhs.span).make_wrapped(dot(lhs, rhs)?)));
pub fn dot<'data>(t1: Spanned<Tensor>, t2: Spanned<Tensor>) -> Expects<'data, Tensor> {
    match_tensors!((t1, t2) => {
        (s1, s2) => |s1: Spanned<num::Complex<f64>>, s2: Spanned<num::Complex<f64>>| Ok(*s1 * *s2),
        (s, m) => |s: Spanned<num::Complex<f64>>, m: Spanned<Matrix>| Ok(*s * &*m),
        (m, s) => |m: Spanned<Matrix>, s: Spanned<num::Complex<f64>>| Ok(&*m * *s),
        (m1, m2) => |m1: Spanned<Matrix>, m2: Spanned<Matrix>| {
            if m1.nrows() != m2.ncols() {
                return Err(TypstError::full(
                    m1.span.union(m2.span),
                    "dot product can only be applied to matrices with compatible shapes",
                    sequence![matrix_shape!({SYMBOL_CC} ^ ({'N'} x {Underline::underline('M')})), SYMBOL_dot_op, matrix_shape!({SYMBOL_CC} ^ ({Underline::underline('M')} x {'P'}))],
                    sequence![matrix_shape!({SYMBOL_CC} ^ ({m1.dim().0.to_string()} x {Underline::underline(m1.dim().1.to_string())})), SYMBOL_dot_op, matrix_shape!({SYMBOL_CC} ^ ({Underline::underline(m2.dim().0.to_string())} x {m2.dim().1.to_string()}))],
                ))
            }
            Ok(m1.dot(&*m2))
        },
    })
}
