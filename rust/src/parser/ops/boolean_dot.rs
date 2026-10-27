use chumsky::{
    primitive::choice,
    span::{SimpleSpanned, Span, Spanned, WrappingSpan},
};
use num::Complex;
use sertyp::{TypstError, chumsky::parser::character};

use crate::{
    Expects, pratt_infix,
    parser::atoms::{matrix::Matrix, tensor::Tensor},
};

/// Parses the Boolean-semiring product operator `⊙`.
#[kalt_macros::parser]
pub fn pratt_boolean_dot_operator() -> char {
    choice((character('⊙'),))
}

fn bool_value<'data>(value: Complex<f64>, span: chumsky::span::SimpleSpan) -> Expects<'data, bool> {
    if value.im == 0.0 && (value.re == 0.0 || value.re == 1.0) {
        Ok(value.re == 1.0)
    } else {
        Err(TypstError::full(
            span,
            "Boolean Product Error",
            "values that are real and equal to 0 or 1",
            value.to_string(),
        ))
    }
}

fn validate_bool_matrix<'data>(
    matrix: &Matrix,
    span: chumsky::span::SimpleSpan,
) -> Expects<'data, ()> {
    for value in matrix.iter() {
        bool_value(*value, span)?;
    }
    Ok(())
}

fn indicator(value: bool) -> Complex<f64> {
    Complex::new(if value { 1.0 } else { 0.0 }, 0.0)
}

/// Boolean-semiring matrix/vector product. Pairwise AND is reduced with OR.
/// Numeric operands are accepted only when every entry is exactly real 0 or 1.
pub fn boolean_dot<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    let lhs_span = lhs.span;
    let rhs_span = rhs.span;
    let full_span = lhs_span.union(rhs_span);

    match (lhs.inner, rhs.inner) {
        (Tensor::Scalar(lhs), Tensor::Scalar(rhs)) => {
            Ok(Tensor::Scalar(indicator(bool_value(lhs, lhs_span)? && bool_value(rhs, rhs_span)?)))
        }
        (Tensor::Scalar(lhs), Tensor::Matrix(rhs)) => {
            let lhs = bool_value(lhs, lhs_span)?;
            validate_bool_matrix(&rhs, rhs_span)?;
            Ok(Tensor::Matrix(rhs.mapv(|value| indicator(lhs && value.re == 1.0))))
        }
        (Tensor::Matrix(lhs), Tensor::Scalar(rhs)) => {
            validate_bool_matrix(&lhs, lhs_span)?;
            let rhs = bool_value(rhs, rhs_span)?;
            Ok(Tensor::Matrix(lhs.mapv(|value| indicator(value.re == 1.0 && rhs))))
        }
        (Tensor::Matrix(lhs), Tensor::Matrix(rhs)) => {
            validate_bool_matrix(&lhs, lhs_span)?;
            validate_bool_matrix(&rhs, rhs_span)?;

            if lhs.ncols() != rhs.nrows() {
                return Err(TypstError::full(
                    full_span,
                    "Boolean Product Shape Error",
                    "matrices with compatible inner dimensions",
                    format!(
                        "{}×{} and {}×{}",
                        lhs.nrows(),
                        lhs.ncols(),
                        rhs.nrows(),
                        rhs.ncols()
                    ),
                ));
            }

            let mut output = Matrix::from_elem(
                (lhs.nrows(), rhs.ncols()),
                Complex::new(0.0, 0.0),
            );
            for row in 0..lhs.nrows() {
                for col in 0..rhs.ncols() {
                    let mut any_overlap = false;
                    for inner in 0..lhs.ncols() {
                        if lhs[[row, inner]].re == 1.0 && rhs[[inner, col]].re == 1.0 {
                            any_overlap = true;
                            break;
                        }
                    }
                    output[[row, col]] = indicator(any_overlap);
                }
            }
            Ok(Tensor::Matrix(output))
        }
    }
}

pratt_infix!(boolean_dot => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(boolean_dot(lhs, rhs)?))
});
