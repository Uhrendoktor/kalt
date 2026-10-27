use chumsky::{
    primitive::choice,
    span::{SimpleSpanned, Span, Spanned, WrappingSpan},
};
use num::Complex;
use sertyp::{
    SYMBOL_CC, SYMBOL_RR, SYMBOL_ZZ, SYMBOL_and, SYMBOL_eq, SYMBOL_eq_not, SYMBOL_gt,
    SYMBOL_gt_eq, SYMBOL_in, SYMBOL_lt, SYMBOL_lt_eq, SYMBOL_not, SYMBOL_or, TypstError,
    chumsky::parser::character, content,
};

use crate::{
    Expects, pratt_infix, pratt_prefix,
    parser::{
        atoms::tensor::Tensor,
        ops::{
            composed_operator,
            element_wise::{element_wise, element_wise2, validate_same_shape},
        },
    },
};

#[kalt_macros::parser]
pub fn pratt_l_operator() -> char {
    character(SYMBOL_lt)
}

#[kalt_macros::parser]
pub fn pratt_leq_operator() -> char {
    choice((
        composed_operator(SYMBOL_lt, SYMBOL_eq, SYMBOL_lt_eq),
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
        composed_operator(SYMBOL_gt, SYMBOL_eq, SYMBOL_gt_eq),
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

pub fn compare_less_thaneq<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_lt_eq, |lhs, rhs| {
        compare_ordered_scalar(lhs, rhs, SYMBOL_lt_eq, |left, right| left <= right)
    })
}

pub fn compare_greater_than<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_gt, |lhs, rhs| {
        compare_ordered_scalar(lhs, rhs, SYMBOL_gt, |left, right| left > right)
    })
}

pub fn compare_greater_thaneq<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
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
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_less_thaneq(lhs, rhs)?))
});
pratt_infix!(g, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_greater_than(lhs, rhs)?))
});
pratt_infix!(geq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_greater_thaneq(lhs, rhs)?))
});
pratt_infix!(eq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_eq(lhs, rhs)?))
});
pratt_infix!(neq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_neq(lhs, rhs)?))
});

pub(crate) fn boolean_value<'data>(
    value: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, bool> {
    if value.inner.im != 0.0 {
        return Err(TypstError::full(
            value.span,
            "Boolean Operand Error",
            content!(sertyp::sequence!["n", SYMBOL_in, SYMBOL_RR]),
            content!(sertyp::sequence![value.inner.to_string(), SYMBOL_in, SYMBOL_CC]),
        ));
    }

    match (value.inner.re == 0.0, value.inner.re == 1.0) {
        (true, false) => Ok(false),
        (false, true) => Ok(true),
        _ => {
            let actual_domain = if value.inner.re.fract() == 0.0 { SYMBOL_ZZ } else { SYMBOL_RR };
            Err(TypstError::full(
                value.span,
                "Boolean Operand Error",
                content!(sertyp::sequence!["n", SYMBOL_in, "{0, 1}"]),
                content!(sertyp::sequence![value.inner.re.to_string(), SYMBOL_in, actual_domain]),
            ))
        }
    }
}

pub(crate) fn boolean_indicator(value: bool) -> Complex<f64> {
    Complex::new(if value { 1.0 } else { 0.0 }, 0.0)
}

fn logical_binary_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
    operation: impl Fn(bool, bool) -> bool,
) -> Expects<'data, Complex<f64>> {
    Ok(boolean_indicator(operation(boolean_value(lhs)?, boolean_value(rhs)?)))
}

fn logical_and_scalar<'data>(lhs: SimpleSpanned<Complex<f64>>, rhs: SimpleSpanned<Complex<f64>>) -> Expects<'data, Complex<f64>> {
    logical_binary_scalar(lhs, rhs, |left, right| left && right)
}

fn logical_or_scalar<'data>(lhs: SimpleSpanned<Complex<f64>>, rhs: SimpleSpanned<Complex<f64>>) -> Expects<'data, Complex<f64>> {
    logical_binary_scalar(lhs, rhs, |left, right| left || right)
}

macro_rules! logical_binary_tensor {
    ($name:ident, $scalar:ident, $symbol:ident) => {
        pub fn $name<'data>(
            lhs: SimpleSpanned<Tensor>,
            rhs: SimpleSpanned<Tensor>,
        ) -> Expects<'data, Tensor> {
            element_wise2(
                $scalar,
                validate_same_shape(|lhs, rhs| content!(sertyp::sequence![lhs, $symbol, rhs])),
                lhs,
                rhs,
            )
        }
    };
}

logical_binary_tensor!(logical_and, logical_and_scalar, SYMBOL_and);
logical_binary_tensor!(logical_or, logical_or_scalar, SYMBOL_or);

#[kalt_macros::parser]
pub fn pratt_and_operator() -> char { character(SYMBOL_and) }

#[kalt_macros::parser]
pub fn pratt_or_operator() -> char { character(SYMBOL_or) }

pratt_infix!(logical_and => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(logical_and(lhs, rhs)?))
});
pratt_infix!(logical_or => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(logical_or(lhs, rhs)?))
});

pub fn logical_not<'data>(value: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    element_wise(
        |value| Ok(boolean_indicator(!boolean_value(value)?)),
        |_| Ok(()),
        value,
    )
}

#[kalt_macros::parser]
pub fn pratt_not_operator() -> char { character(SYMBOL_not) }

pratt_prefix!(logical_not => |_op, rhs: Spanned<_>| {
    let span = rhs.span;
    Ok(span.make_wrapped(logical_not(rhs)?))
});

#[kalt_macros::parser]
pub fn boolean_literal() -> Expects<'data, Tensor> {
    choice((
        super::word_or_op("true").map(|_| Ok(Tensor::Scalar(Complex::new(1.0, 0.0)))),
        super::word_or_op("false").map(|_| Ok(Tensor::Scalar(Complex::new(0.0, 0.0)))),
    ))
}
