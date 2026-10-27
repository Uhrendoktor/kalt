use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use sertyp::{TypstError, chumsky::parser::character};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::element_wise::element_wise2,
    },
};

use super::logic::{boolean_indicator, boolean_value};

/// Parses the Boolean-semiring product operator `⊙`.
#[kalt_macros::parser]
pub fn pratt_boolean_dot_operator() -> char {
    character('⊙')
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

/// Boolean-semiring matrix/vector product. Pairwise AND is reduced with OR.
/// Scalar cases share `element_wise2`; the matrix-by-matrix case must use the
/// inner-dimension product rather than element-wise shape matching.
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
                boolean_indicator(false),
            );
            for row in 0..lhs.nrows() {
                for col in 0..rhs.ncols() {
                    // Boolean multiplication uses AND for each pair and OR for reduction.
                    let any_overlap = (0..lhs.ncols()).any(|inner| {
                        lhs[[row, inner]].re == 1.0 && rhs[[inner, col]].re == 1.0
                    });
                    output[[row, col]] = boolean_indicator(any_overlap);
                }
            }
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
