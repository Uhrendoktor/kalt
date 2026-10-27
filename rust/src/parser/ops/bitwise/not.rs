use chumsky::span::{SimpleSpanned, Spanned, WrappingSpan};
use sertyp::{SYMBOL_not, chumsky::parser::character};

use crate::{
    Expects, pratt_prefix,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::element_wise_unary,
    },
};

/// Boolean indicators 0 and 1 keep logical NOT; other values use integer complement.
pub fn bitwise_not<'data>(value: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    element_wise_unary(
        |value| {
            let span = value.span;
            let real = crate::parser::validator::real(value.inner, span)?;
            let result = match real {
                0.0 => 1.0,
                1.0 => 0.0,
                _ => (!super::bitwise_integer(span.make_wrapped(real))?) as f64,
            };
            Ok(num::Complex::new(result, 0.0))
        },
        |_| Ok(()),
        value,
    )
}

#[kalt_macros::parser]
pub fn pratt_bitwise_not_operator() -> char {
    character(SYMBOL_not)
}

pratt_prefix!(bitwise_not => |_op, rhs: Spanned<_>| {
    let span = rhs.span;
    Ok(span.make_wrapped(bitwise_not(rhs)?))
});
