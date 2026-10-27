use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use sertyp::{
    SYMBOL_dot_o, SYMBOL_times, TypstError, chumsky::parser::character,
};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::element_wise::element_wise2,
    },
};

use super::{
    dot::matrix_product,
    logic::{boolean_indicator, boolean_value},
};

/// Parses the Boolean-semiring product operator ⊙.
#[kalt_macros::parser]
pub fn pratt_boolean_dot_operator() -> char {
    character(SYMBOL_dot_o)
}

fn validate_bool_matrix<'data>(
    matrix: &Matrix,
    span: chumsky::span::SimpleSpan,
) -> Expects<'data, ()> {
    for value in matrix.iter() {
        boolean_value(span.make_wrapped(*value))?;
    }
    Ok(())
}

/// Boolean-semiring matrix/vector product. Scalar and matrix-scalar cases use
/// the shared element-wise utility; matrix-matrix multiplication shares the
/// same shape check and product loop as ordinary dot, but uses AND followed by OR.
pub fn boolean_dot<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    let lhs_span = lhs.span;
    let rhs_span = rhs.span;
    let full_span = lhs_span.union(rhs_span);

    match (lhs.inner, rhs.inner) {
        (Tensor::Matrix(lhs), Tensor::Matrix(rhs)) => {
            // Validate every cell before applying the Boolean-semiring rule.
            validate_bool_matrix(&lhs, lhs_span)?;
            validate_bool_matrix(&rhs, rhs_span)?;

            let Some(output) = matrix_product(
                &lhs,
                &rhs,
                boolean_indicator(false),
                |lhs, rhs| boolean_indicator(lhs.re == 1.0 && rhs.re == 1.0),
                |left, right| boolean_indicator(left.re == 1.0 || right.re == 1.0),
            ) else {
                return Err(TypstError::full(
                    full_span,
                    "Boolean Product Shape Error",
                    "matrices with compatible inner dimensions",
                    format!(
                        "{}{}{} and {}{}{}",
                        lhs.nrows(),
                        SYMBOL_times,
                        lhs.ncols(),
                        rhs.nrows(),
                        SYMBOL_times,
                        rhs.ncols()
                    ),
                ));
            };

            Ok(Tensor::Matrix(output))
        }
        (lhs, rhs) => element_wise2(
            |lhs, rhs| {
                let result = boolean_value(lhs)? && boolean_value(rhs)?;
                Ok(boolean_indicator(result))
            },
            |_, _| Ok(()),
            lhs_span.make_wrapped(lhs),
            rhs_span.make_wrapped(rhs),
        ),
    }
}

pratt_infix!(boolean_dot => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(boolean_dot(lhs, rhs)?))
});
