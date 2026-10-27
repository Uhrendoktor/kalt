use chumsky::span::{SimpleSpanned, Spanned, WrappingSpan};
use sertyp::chumsky::parser::character;

use crate::{
    Expects, pratt_prefix,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::element_wise_unary,
    },
};

/// Applies two's-complement bitwise NOT to each real integer in a tensor.
pub fn bitwise_not<'data>(value: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    element_wise_unary(
        |value| {
            Ok(num::Complex::new(
                (!crate::parser::validator::real_int(
                    crate::parser::validator::real(value.inner, value.span)?,
                    value.span,
                )?) as f64,
                0.0,
            ))
        },
        |_| Ok(()),
        value,
    )
}

#[kalt_macros::parser]
pub fn pratt_bitwise_not_operator() -> char {
    character('~')
}

pratt_prefix!(bitwise_not => |_op, rhs: Spanned<_>| {
    Ok(rhs.span.make_wrapped(bitwise_not(rhs)?))
});
