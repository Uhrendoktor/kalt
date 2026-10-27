use chumsky::{
    primitive::choice,
    span::{SimpleSpanned, Span, Spanned, WrappingSpan},
};
use num::Complex;
use sertyp::{
    TypstError,
    chumsky::parser::character,
    content,
};

use crate::{
    Expects, pratt_infix, pratt_prefix, pratt_prefix,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::{element_wise, element_wise2, validate_same_shape},
    },
};

/// Parses mathematical comparisons. ASCII <= and >= are accepted as aliases
/// for the corresponding single-character mathematical operators.
#[kalt_macros::parser]
pub fn pratt_compare_operator() -> String {
    choice((
        character('<').then_ignore(character('=')).map(|_| "<=".to_string()),
        character('>').then_ignore(character('=')).map(|_| ">=".to_string()),
        character('≤').map(|_| "≤".to_string()),
        character('≥').map(|_| "≥".to_string()),
        character('≠').map(|_| "≠".to_string()),
        character('<').map(|_| "<".to_string()),
        character('>').map(|_| ">".to_string()),
        character('=').map(|_| "=".to_string()),
    ))
}

fn compare_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
    operator: &str,
) -> Expects<'data, Complex<f64>> {
    let span = lhs.span.union(rhs.span);
    let result = match operator {
        "=" => lhs.inner == rhs.inner,
        "≠" => lhs.inner != rhs.inner,
        "<" | "<=" | "≤" | ">" | ">=" | "≥" => {
            if lhs.inner.im != 0.0 || rhs.inner.im != 0.0 {
                return Err(TypstError::full(
                    span,
                    "Comparison Error",
                    "real-valued operands",
                    format!("{} and {}", lhs.inner, rhs.inner),
                ));
            }
            match operator {
                "<" => lhs.inner.re < rhs.inner.re,
                "<=" | "≤" => lhs.inner.re <= rhs.inner.re,
                ">" => lhs.inner.re > rhs.inner.re,
                ">=" | "≥" => lhs.inner.re >= rhs.inner.re,
                _ => unreachable!(),
            }
        }
        _ => unreachable!("comparison parser only produces known operators"),
    };
    Ok(Complex::new(if result { 1.0 } else { 0.0 }, 0.0))
}

/// Applies a comparison element-wise. Comparisons return numeric 0/1 values.
/// Ordered comparisons require real operands; equality also supports complex values.
pub fn compare<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
    operator: String,
) -> Expects<'data, Tensor> {
    let shape_operator = operator.clone();
    element_wise2(
        move |lhs, rhs| compare_scalar(lhs, rhs, &operator),
        validate_same_shape(move |lhs, rhs| {
            content!(sertyp::sequence![lhs, shape_operator.clone(), rhs])
        }),
        lhs,
        rhs,
    )
}

pratt_infix!(compare, String => |op: String, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare(lhs, rhs, op)?))
});

/// Validates the numeric encoding used for logical operators: real 0 is false
/// and real 1 is true. Other values are rejected rather than treated as truthy.
fn boolean_value<'data>(value: SimpleSpanned<Complex<f64>>) -> Expects<'data, bool> {
    if value.inner.im == 0.0 && (value.inner.re == 0.0 || value.inner.re == 1.0) {
        Ok(value.inner.re == 1.0)
    } else {
        Err(TypstError::full(
            value.span,
            "Boolean Logic Error",
            "a real value equal to 0 or 1",
            value.inner.to_string(),
        ))
    }
}

fn logical_scalar<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
    is_and: bool,
) -> Expects<'data, Complex<f64>> {
    let left = boolean_value(lhs)?;
    let right = boolean_value(rhs)?;
    let result = if is_and { left && right } else { left || right };
    Ok(Complex::new(if result { 1.0 } else { 0.0 }, 0.0))
}

/// Element-wise logical AND. Every operand must be real and exactly 0 or 1.
pub fn logical_and<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    element_wise2(
        |lhs, rhs| logical_scalar(lhs, rhs, true),
        validate_same_shape(|lhs, rhs| content!(sertyp::sequence![lhs, '∧', rhs])),
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
        |lhs, rhs| logical_scalar(lhs, rhs, false),
        validate_same_shape(|lhs, rhs| content!(sertyp::sequence![lhs, '∨', rhs])),
        lhs,
        rhs,
    )
}

#[kalt_macros::parser]
pub fn pratt_and_operator() -> char {
    character('∧')
}

#[kalt_macros::parser]
pub fn pratt_or_operator() -> char {
    character('∨')
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
            Ok(Complex::new(if result { 1.0 } else { 0.0 }, 0.0))
        },
        |_| Ok(()),
        value,
    )
}

#[kalt_macros::parser]
pub fn pratt_not_operator() -> char {
    character('¬')
}

pratt_prefix!(logical_not => |_op, rhs: Spanned<_>| {
    let span = rhs.span;
    Ok(span.make_wrapped(logical_not(rhs)?))
});

/// Parses `op("true")` / `op("false")` (or the equivalent text tokens)
/// as the numeric Boolean indicators 1 and 0.
#[kalt_macros::parser]
pub fn boolean_literal() -> Expects<'data, Tensor> {
    choice((
        super::word_or_op("true").map(|_| Ok(Tensor::Scalar(Complex::new(1.0, 0.0)))),
        super::word_or_op("false").map(|_| Ok(Tensor::Scalar(Complex::new(0.0, 0.0)))),
    ))
}
