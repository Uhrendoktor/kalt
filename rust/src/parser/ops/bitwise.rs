use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use num::Complex;
use sertyp::{
    SYMBOL_CC, SYMBOL_RR, SYMBOL_ZZ, SYMBOL_in, SYMBOL_lt_eq, SYMBOL_union_sq, SYMBOL_xor,
    TypstError, chumsky::parser::character, content,
};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::{element_wise2, validate_same_shape},
    },
};

const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

fn integer_operand<'data>(value: SimpleSpanned<Complex<f64>>) -> Expects<'data, i64> {
    let valid = value.inner.im == 0.0
        && value.inner.re.is_finite()
        && value.inner.re.fract() == 0.0
        && value.inner.re.abs() <= MAX_SAFE_INTEGER;

    if !valid {
        let domain = if value.inner.im != 0.0 { SYMBOL_CC } else { SYMBOL_RR };
        return Err(TypstError::full(
            value.span,
            "Bitwise Operand Error",
            content!(sertyp::sequence![
                "n", SYMBOL_in, SYMBOL_ZZ, ", |n|", SYMBOL_lt_eq, MAX_SAFE_INTEGER.to_string()
            ]),
            content!(sertyp::sequence![value.inner.to_string(), SYMBOL_in, domain]),
        ));
    }

    Ok(value.inner.re as i64)
}

fn bitwise_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
    operation: impl Fn(i64, i64) -> i64,
) -> Expects<'data, Complex<f64>> {
    Ok(Complex::new(operation(integer_operand(lhs)?, integer_operand(rhs)?) as f64, 0.0))
}

fn bitwise_tensor<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
    operator: char,
    operation: impl Fn(i64, i64) -> i64 + Copy,
) -> Expects<'data, Tensor> {
    element_wise2(
        move |lhs, rhs| bitwise_scalar(lhs, rhs, operation),
        move |lhs, rhs| validate_same_shape(move |left, right| {
            content!(sertyp::sequence![left, operator, right])
        })(lhs, rhs),
        lhs,
        rhs,
    )
}

pub fn bitwise_and<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    bitwise_tensor(lhs, rhs, '&', |left, right| left & right)
}

pub fn bitwise_or<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    bitwise_tensor(lhs, rhs, SYMBOL_union_sq, |left, right| left | right)
}

pub fn bitwise_xor<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    bitwise_tensor(lhs, rhs, SYMBOL_xor, |left, right| left ^ right)
}

#[kalt_macros::parser]
pub fn pratt_bitwise_and_operator() -> char { character('&') }

#[kalt_macros::parser]
pub fn pratt_bitwise_or_operator() -> char { character(SYMBOL_union_sq) }

#[kalt_macros::parser]
pub fn pratt_bitwise_xor_operator() -> char { character(SYMBOL_xor) }

pratt_infix!(bitwise_and => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(bitwise_and(lhs, rhs)?))
});
pratt_infix!(bitwise_xor => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(bitwise_xor(lhs, rhs)?))
});
pratt_infix!(bitwise_or => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(bitwise_or(lhs, rhs)?))
});
