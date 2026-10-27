use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_dot_o, chumsky::parser::character};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::{element_wise_binary, matrix_product},
    },
};

use super::logic::{boolean_indicator, boolean_value};

/// Parses the Boolean-semiring product operator ⊙.
#[kalt_macros::parser]
pub fn pratt_boolean_dot_operator() -> char {
    character(SYMBOL_dot_o)
}

/// Boolean-semiring product: pairwise AND is reduced with OR.
pub fn boolean_dot<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    match (lhs, rhs) {
        (
            SimpleSpanned {
                inner: Tensor::Matrix(lhs),
                span: lhs_span,
            },
            SimpleSpanned {
                inner: Tensor::Matrix(rhs),
                span: rhs_span,
            },
        ) => {
            for (matrix, span) in [(&lhs, lhs_span), (&rhs, rhs_span)] {
                for value in matrix {
                    boolean_value(span.make_wrapped(*value))?;
                }
            }
            matrix_product(
                lhs_span.make_wrapped(&lhs),
                rhs_span.make_wrapped(&rhs),
                false,
                |lhs, rhs| lhs.re == 1.0 && rhs.re == 1.0,
                |lhs, rhs| lhs || rhs,
                SYMBOL_dot_o,
            )
            .map(|product| Tensor::Matrix(product.mapv(boolean_indicator)))
        }
        (lhs, rhs) => element_wise_binary(
            |lhs, rhs| super::logic::boolean_binary_scalar(lhs, rhs, |lhs, rhs| lhs && rhs),
            |_, _| Ok(()),
            lhs,
            rhs,
        ),
    }
}

pratt_infix!(boolean_dot => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(boolean_dot(lhs, rhs)?))
});
