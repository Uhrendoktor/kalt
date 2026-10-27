use chumsky::{
    primitive::choice,
    span::{SimpleSpanned, Span, Spanned, WrappingSpan},
};
use num::Complex;
use sertyp::{
    SYMBOL_CC, SYMBOL_RR, SYMBOL_eq, SYMBOL_eq_not, SYMBOL_gt, SYMBOL_gt_eq,
    SYMBOL_in, SYMBOL_lt, SYMBOL_lt_eq, TypstError, chumsky::parser::character,
    content,
};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::tensor::Tensor,
        ops::{composed_operator, element_wise::{element_wise2, validate_same_shape}},
    },
};

use super::boolean_indicator;

#[kalt_macros::parser]
pub fn pratt_l_operator() -> char {
    character(SYMBOL_lt)
}

#[kalt_macros::parser]
pub fn pratt_leq_operator() -> char {
    choice((
        composed_operator(SYMBOL_lt, SYMBOL_eq).map(|_| SYMBOL_lt_eq),
        character(SYMBOL_lt_eq),
    ))
}

#[kalt_macros::parser]
pub fn pratt_g_operator() -> char {
    character(SYMBOL_gt)
}

#[kalt_macros::parser]
pub fn pratt_geq_operator() -> char {
    choice((
        composed_operator(SYMBOL_gt, SYMBOL_eq).map(|_| SYMBOL_gt_eq),
        character(SYMBOL_gt_eq),
    ))
}

#[kalt_macros::parser]
pub fn pratt_eq_operator() -> char {
    character(SYMBOL_eq)
}

#[kalt_macros::parser]
pub fn pratt_neq_operator() -> char {
    character(SYMBOL_eq_not)
}

fn compare_complex_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
    comparison: impl Fn(Complex<f64>, Complex<f64>) -> bool,
) -> Expects<'data, Complex<f64>> {
    Ok(boolean_indicator(comparison(lhs.inner, rhs.inner)))
}

fn compare_ordered_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
    operator: char,
    comparison: impl Fn(f64, f64) -> bool,
) -> Expects<'data, Complex<f64>> {
    if lhs.inner.im != 0.0 || rhs.inner.im != 0.0 {
        return Err(TypstError::full(
            lhs.span.union(rhs.span),
            "Comparison Error",
            content!(sertyp::sequence!["a, b", SYMBOL_in, SYMBOL_RR]),
            content!(sertyp::sequence![
                lhs.inner.to_string(), operator, rhs.inner.to_string()
            ]),
        ));
    }
    Ok(boolean_indicator(comparison(lhs.inner.re, rhs.inner.re)))
}

fn compare_tensors<'data, F>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
    operator: char,
    comparison: F,
) -> Expects<'data, Tensor>
where
    F: Fn(
        SimpleSpanned<Complex<f64>>,
        SimpleSpanned<Complex<f64>>,
    ) -> Expects<'data, Complex<f64>>,
{
    element_wise2(
        comparison,
        validate_same_shape(move |lhs, rhs| content!(sertyp::sequence![lhs, operator, rhs])),
        lhs,
        rhs,
    )
}

pub fn compare_less_than<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_lt, |lhs, rhs| {
        compare_ordered_scalar(lhs, rhs, SYMBOL_lt, |left, right| left < right)
    })
}

pub fn compare_less_than_or_equal<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_lt_eq, |lhs, rhs| {
        compare_ordered_scalar(lhs, rhs, SYMBOL_lt_eq, |left, right| left <= right)
    })
}

pub fn compare_greater_than<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_gt, |lhs, rhs| {
        compare_ordered_scalar(lhs, rhs, SYMBOL_gt, |left, right| left > right)
    })
}

pub fn compare_greater_than_or_equal<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_gt_eq, |lhs, rhs| {
        compare_ordered_scalar(lhs, rhs, SYMBOL_gt_eq, |left, right| left >= right)
    })
}

pub fn compare_eq<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_eq, |lhs, rhs| {
        compare_complex_scalar(lhs, rhs, |left, right| left == right)
    })
}

pub fn compare_neq<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_eq_not, |lhs, rhs| {
        compare_complex_scalar(lhs, rhs, |left, right| left != right)
    })
}

pratt_infix!(l, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_less_than(lhs, rhs)?))
});
pratt_infix!(leq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_less_than_or_equal(lhs, rhs)?))
});
pratt_infix!(g, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_greater_than(lhs, rhs)?))
});
pratt_infix!(geq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_greater_than_or_equal(lhs, rhs)?))
});
pratt_infix!(eq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_eq(lhs, rhs)?))
});
pratt_infix!(neq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_neq(lhs, rhs)?))
});
