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

/// Less-than comparison parser.
#[kalt_macros::parser]
pub fn pratt_l_operator() -> char {
    character(SYMBOL_lt)
}

/// Less-than-or-equal parser. The two-character alias is parsed as one operator.
#[kalt_macros::parser]
pub fn pratt_leq_operator() -> char {
    choice((
        composed_operator(SYMBOL_lt, SYMBOL_eq, SYMBOL_lt_eq),
        character(SYMBOL_lt_eq),
    ))
}

/// Greater-than comparison parser.
#[kalt_macros::parser]
pub fn pratt_g_operator() -> char {
    character(SYMBOL_gt)
}

/// Greater-than-or-equal parser. The two-character alias is parsed as one operator.
#[kalt_macros::parser]
pub fn pratt_geq_operator() -> char {
    choice((
        composed_operator(SYMBOL_gt, SYMBOL_eq, SYMBOL_gt_eq),
        character(SYMBOL_gt_eq),
    ))
}

/// Equality comparison parser.
#[kalt_macros::parser]
pub fn pratt_eq_operator() -> char {
    character(SYMBOL_eq)
}

/// Inequality comparison parser.
#[kalt_macros::parser]
pub fn pratt_neq_operator() -> char {
    character(SYMBOL_eq_not)
}

/// Compares two complex scalars directly (used for exact equality and inequality).
fn compare_complex_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
    comparison: impl Fn(Complex<f64>, Complex<f64>) -> bool,
) -> Expects<'data, Complex<f64>> {
    Ok(boolean_indicator(comparison(lhs.inner, rhs.inner)))
}

/// Compares the real parts after rejecting values with a nonzero imaginary part.
fn compare_ordered_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
    comparison: impl Fn(f64, f64) -> bool,
) -> Expects<'data, Complex<f64>> {
    let span = lhs.span.union(rhs.span);
    if lhs.inner.im != 0.0 || rhs.inner.im != 0.0 {
        return Err(TypstError::full(
            span,
            "Comparison Error",
            "real-valued operands",
            format!("{} and {}", lhs.inner, rhs.inner),
        ));
    }
    Ok(boolean_indicator(comparison(lhs.inner.re, rhs.inner.re)))
}

fn equal_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, Complex<f64>> {
    compare_complex_scalar(lhs, rhs, |lhs, rhs| lhs == rhs)
}

fn unequal_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, Complex<f64>> {
    compare_complex_scalar(lhs, rhs, |lhs, rhs| lhs != rhs)
}

fn less_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, Complex<f64>> {
    compare_ordered_scalar(lhs, rhs, |lhs, rhs| lhs < rhs)
}

fn less_or_equal_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, Complex<f64>> {
    compare_ordered_scalar(lhs, rhs, |lhs, rhs| lhs <= rhs)
}

fn greater_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, Complex<f64>> {
    compare_ordered_scalar(lhs, rhs, |lhs, rhs| lhs > rhs)
}

fn greater_or_equal_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, Complex<f64>> {
    compare_ordered_scalar(lhs, rhs, |lhs, rhs| lhs >= rhs)
}

/// Applies one explicit comparison to matching matrix cells. A scalar operand
/// is still broadcast by element_wise2; non-scalar matrices must match shapes.
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
        validate_same_shape(move |lhs, rhs| {
            content!(sertyp::sequence![lhs, operator, rhs])
        }),
        lhs,
        rhs,
    )
}

pub fn compare_l<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_lt, less_scalar)
}

pub fn compare_leq<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_lt_eq, less_or_equal_scalar)
}

pub fn compare_g<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_gt, greater_scalar)
}

pub fn compare_geq<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_gt_eq, greater_or_equal_scalar)
}

pub fn compare_eq<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_eq, equal_scalar)
}

pub fn compare_neq<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_eq_not, unequal_scalar)
}

pratt_infix!(l, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_l(lhs, rhs)?))
});
pratt_infix!(leq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_leq(lhs, rhs)?))
});
pratt_infix!(g, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_g(lhs, rhs)?))
});
pratt_infix!(geq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_geq(lhs, rhs)?))
});
pratt_infix!(eq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_eq(lhs, rhs)?))
});
pratt_infix!(neq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_neq(lhs, rhs)?))
});

/// Validates Kalt's numeric Boolean encoding, shared by logical operators
/// and the Boolean-semiring product. Keep the real-valued and indicator checks
/// separate so diagnostics explain which constraint failed.
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

    if value.inner.re != 0.0 && value.inner.re != 1.0 {
        let actual_domain = if value.inner.re.fract() == 0.0 {
            SYMBOL_ZZ
        } else {
            SYMBOL_RR
        };
        return Err(TypstError::full(
            value.span,
            "Boolean Operand Error",
            content!(sertyp::sequence!["n", SYMBOL_in, "{0, 1}"]),
            content!(sertyp::sequence![value.inner.re.to_string(), SYMBOL_in, actual_domain]),
        ));
    }

    Ok(value.inner.re == 1.0)
}

/// Converts a logical result back to Kalt's numeric 0/1 representation.
pub(crate) fn boolean_indicator(value: bool) -> Complex<f64> {
    Complex::new(if value { 1.0 } else { 0.0 }, 0.0)
}

fn logical_binary_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
    operation: impl Fn(bool, bool) -> bool,
) -> Expects<'data, Complex<f64>> {
    let left = boolean_value(lhs)?;
    let right = boolean_value(rhs)?;
    Ok(boolean_indicator(operation(left, right)))
}

fn logical_and_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, Complex<f64>> {
    logical_binary_scalar(lhs, rhs, |left, right| left && right)
}

fn logical_or_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, Complex<f64>> {
    logical_binary_scalar(lhs, rhs, |left, right| left || right)
}

/// Element-wise logical AND. Every operand must be real and exactly 0 or 1.
pub fn logical_and<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    element_wise2(
        logical_and_scalar,
        validate_same_shape(|lhs, rhs| content!(sertyp::sequence![lhs, SYMBOL_and, rhs])),
        lhs,
        rhs,
    )
}

/// Element-wise logical OR. Every operand must be real and exactly 0 or 1.
pub fn logical_or<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    element_wise2(
        logical_or_scalar,
        validate_same_shape(|lhs, rhs| content!(sertyp::sequence![lhs, SYMBOL_or, rhs])),
        lhs,
        rhs,
    )
}

#[kalt_macros::parser]
pub fn pratt_and_operator() -> char {
    character(SYMBOL_and)
}

#[kalt_macros::parser]
pub fn pratt_or_operator() -> char {
    character(SYMBOL_or)
}

pratt_infix!(logical_and => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(logical_and(lhs, rhs)?))
});

pratt_infix!(logical_or => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(logical_or(lhs, rhs)?))
});

/// Applies logical NOT element-wise. Each value must be real and exactly 0 or 1.
pub fn logical_not<'data>(value: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    element_wise(
        |value| {
            let result = !boolean_value(value)?;
            Ok(boolean_indicator(result))
        },
        |_| Ok(()),
        value,
    )
}

#[kalt_macros::parser]
pub fn pratt_not_operator() -> char {
    character(SYMBOL_not)
}

pratt_prefix!(logical_not => |_op, rhs: Spanned<_>| {
    let span = rhs.span;
    Ok(span.make_wrapped(logical_not(rhs)?))
});

/// Parses op("true") / op("false") (or the equivalent text tokens)
/// as the numeric Boolean indicators 1 and 0.
#[kalt_macros::parser]
pub fn boolean_literal() -> Expects<'data, Tensor> {
    choice((
        super::word_or_op("true").map(|_| Ok(Tensor::Scalar(Complex::new(1.0, 0.0)))),
        super::word_or_op("false").map(|_| Ok(Tensor::Scalar(Complex::new(0.0, 0.0)))),
    ))
}
