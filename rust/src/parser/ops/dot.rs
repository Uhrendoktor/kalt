use chumsky::span::{Span, Spanned, WrappingSpan};
use num::Complex;
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
    character(SYMBOL_dot_op)
}

// dot product of two tensors
pratt_infix!(dot => |_op, lhs: Spanned<_>, rhs: Spanned<_>| Ok(lhs.span.union(rhs.span).make_wrapped(dot(lhs, rhs)?)));

/// Multiplies two matrices using a caller-provided product and reduction operation.
///
/// Returns no value if the matrices' inner dimensions are incompatible. Ordinary dot
/// product uses complex multiplication and addition; Boolean dot uses AND and OR.
/// Keeping the iteration and shape condition here prevents the two products from
/// drifting apart while allowing their algebraic rules to remain distinct.
pub(crate) fn matrix_product(
    lhs: &Matrix,
    rhs: &Matrix,
    zero: Complex<f64>,
    multiply: impl Fn(Complex<f64>, Complex<f64>) -> Complex<f64>,
    reduce: impl Fn(Complex<f64>, Complex<f64>) -> Complex<f64>,
) -> Option<Matrix> {
    if lhs.ncols() != rhs.nrows() {
        return None;
    }

    let mut output = Matrix::from_elem((lhs.nrows(), rhs.ncols()), zero);
    for row in 0..lhs.nrows() {
        for col in 0..rhs.ncols() {
            let mut accumulator = zero;
            for inner in 0..lhs.ncols() {
                accumulator = reduce(
                    accumulator,
                    multiply(lhs[[row, inner]], rhs[[inner, col]]),
                );
            }
            output[[row, col]] = accumulator;
        }
    }
    Some(output)
}

pub fn dot<'data>(t1: Spanned<Tensor>, t2: Spanned<Tensor>) -> Expects<'data, Tensor> {
    match_tensors!((t1, t2) => {
        (s1, s2) => |s1: Spanned<Complex<f64>>, s2: Spanned<Complex<f64>>| Ok(*s1 * *s2),
        (s, m) => |s: Spanned<Complex<f64>>, m: Spanned<Matrix>| Ok(*s * &*m),
        (m, s) => |m: Spanned<Matrix>, s: Spanned<Complex<f64>>| Ok(&*m * *s),
        (m1, m2) => |m1: Spanned<Matrix>, m2: Spanned<Matrix>| {
            let Some(product) = matrix_product(
                &m1,
                &m2,
                Complex::new(0.0, 0.0),
                |lhs, rhs| lhs * rhs,
                |sum, product| sum + product,
            ) else {
                return Err(TypstError::full(
                    m1.span.union(m2.span),
                    "dot product can only be applied to matrices with compatible shapes",
                    sequence![matrix_shape!({SYMBOL_CC} ^ ({'N'} x {Underline::underline('M')})), SYMBOL_dot_op, matrix_shape!({SYMBOL_CC} ^ ({Underline::underline('M')} x {'P'}))],
                    sequence![matrix_shape!({SYMBOL_CC} ^ ({m1.dim().0.to_string()} x {Underline::underline(m1.dim().1.to_string())})), SYMBOL_dot_op, matrix_shape!({SYMBOL_CC} ^ ({Underline::underline(m2.dim().0.to_string())} x {m2.dim().1.to_string()}))],
                ));
            };
            Ok(product)
        },
    })
}
