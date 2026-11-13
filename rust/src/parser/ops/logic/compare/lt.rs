use chumsky::span::{Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_lt, chumsky::parser::character};

use crate::{
    pratt_infix,
    parser::ops::element_wise::{element_wise_binary, real_scalar_binary, validate_same_shape},
};

use super::super::boolean_indicator;

#[kalt_macros::parser]
pub fn pratt_l_operator() -> char {
    character(SYMBOL_lt)
}

pratt_infix!(l, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(
        element_wise_binary(
            |lhs, rhs| real_scalar_binary(lhs, rhs, |lhs, rhs| Ok(boolean_indicator(lhs.inner < rhs.inner))),
            validate_same_shape(|lhs, rhs| sertyp::content!(sertyp::sequence![lhs, SYMBOL_lt, rhs])),
            lhs,
            rhs,
        )?
    ))
});
