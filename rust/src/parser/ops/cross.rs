use chumsky::{
    primitive::choice,
    span::{Span, Spanned, WrappingSpan},
};
use ndarray::arr2;
use sertyp::{SYMBOL_CC, SYMBOL_times, TypstError, chumsky::parser::character, sequence};

use crate::{
    Expects, match_tensors, matrix_shape,
    parser::atoms::{matrix::Matrix, tensor::Tensor},
    pow, pratt_infix,
};

/// Calculates the cross product of two complex 3D vectors.
pub fn cross_c(m1: &Matrix, m2: &Matrix) -> Matrix {
    arr2(&[[
        m1[(1, 0)] * m2[(2, 0)] - m1[(2, 0)] * m2[(1, 0)],
        m1[(2, 0)] * m2[(0, 0)] - m1[(0, 0)] * m2[(2, 0)],
        m1[(0, 0)] * m2[(1, 0)] - m1[(1, 0)] * m2[(0, 0)],
    ]])
}

/// Vector cross product
/// Condition for the pratt parser to apply.
#[kalt_macros::parser]
pub fn pratt_cross_operator() -> char {
    choice((character(SYMBOL_times),))
}

// cross product of two vectors
pratt_infix!(cross => |_op, lhs: Spanned<_>, rhs: Spanned<_>| Ok(lhs.span.union(rhs.span).make_wrapped(cross(lhs, rhs)?)));
pub fn cross<'data>(t1: Spanned<Tensor>, t2: Spanned<Tensor>) -> Expects<'data, Tensor> {
    let span = t1.span.union(t2.span);
    let sm = |_s: Spanned<num::Complex<f64>>, m: Spanned<Matrix>| {
        Err(TypstError::full(
            span,
            "cross product is only defined for vectors",
            sequence![
                pow!({ SYMBOL_CC } ^ { 3.to_string() }),
                SYMBOL_times,
                pow!({ SYMBOL_CC } ^ { 3.to_string() })
            ],
            sequence![
                pow!({ SYMBOL_CC } ^ { 1.to_string() }),
                SYMBOL_times,
                matrix_shape!({SYMBOL_CC} ^ ({m.dim().0.to_string()} x {m.dim().1.to_string()}))
            ],
        ))
    };
    match_tensors!((t1, t2) => {
        (s1, s2) => |_s1: Spanned<num::Complex<f64>>, _s2: Spanned<num::Complex<f64>>| Err(TypstError::full(
            span,
            "cross product is only defined for vectors",
            sequence![pow!({SYMBOL_CC} ^ {3.to_string()}), SYMBOL_times, pow!({SYMBOL_CC} ^ {3.to_string()})],
            sequence![pow!({SYMBOL_CC} ^ {1.to_string()}), SYMBOL_times, pow!({SYMBOL_CC} ^ {1.to_string()})]
        )),
        (s, m) => sm,
        (m, s) => |m, s| sm(s, m),
        (m1, m2) => |m1: Spanned<Matrix>, m2: Spanned<Matrix>| {
            if m1.shape() != [3,1] || m2.shape() != [3,1] {
                return Err(TypstError::full(
                    span,
                    "cross product is only defined for vectors of shape 3x1",
                    sequence![pow!({SYMBOL_CC} ^ {3.to_string()}), SYMBOL_times, pow!({SYMBOL_CC} ^ {3.to_string()})],
                    sequence![matrix_shape!({SYMBOL_CC} ^ ({m1.dim().0.to_string()} x {m1.dim().1.to_string()})), SYMBOL_times, matrix_shape!({SYMBOL_CC} ^ ({m2.dim().0.to_string()} x {m2.dim().1.to_string()}))]
                ))
            }
            Ok(cross_c(&m1, &m2))
        },
    })
}
