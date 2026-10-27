use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use ndarray::Array2;
use sertyp::{SYMBOL_dot_o, chumsky::parser::character};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::element_wise::{element_wise_binary_combinatorial, matrix_product},
    },
};

use super::logic::{boolean_indicator, boolean_value};

/// Parses the Boolean-semiring product operator ⊙.
#[kalt_macros::parser]
pub fn pratt_boolean_dot_operator() -> char {
    character(SYMBOL_dot_o)
}

fn boolean_matrix<'data>(matrix: SimpleSpanned<&Matrix>) -> Expects<'data, Array2<bool>> {
    let mut result = Array2::from_elem(matrix.inner.raw_dim(), false);
    for ((row, col), value) in matrix.inner.indexed_iter() {
        result[[row, col]] = boolean_value(matrix.span.make_wrapped(*value))?;
    }
    Ok(result)
}

/// Boolean-semiring product: pairwise AND is reduced with OR. The shared tensor
/// utility dispatches scalar cases to element-wise evaluation and matrix pairs to
/// the semiring product loop.
pub fn boolean_dot<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    element_wise_binary_combinatorial(
        |lhs, rhs| Ok(boolean_indicator(boolean_value(lhs)? && boolean_value(rhs)?)),
        |lhs: SimpleSpanned<&Matrix>, rhs: SimpleSpanned<&Matrix>| {
            let lhs_bool = boolean_matrix(lhs)?;
            let rhs_bool = boolean_matrix(rhs)?;
            let product = matrix_product(
                lhs.span.make_wrapped(&lhs_bool),
                rhs.span.make_wrapped(&rhs_bool),
                false,
                |a, b| a && b,
                |a, b| a || b,
                SYMBOL_dot_o,
            )?;
            Ok(product.mapv(boolean_indicator))
        },
        lhs,
        rhs,
    )
}

pratt_infix!(boolean_dot => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(boolean_dot(lhs, rhs)?))
});
