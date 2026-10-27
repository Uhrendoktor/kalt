use chumsky::{primitive::choice, span::{Span, Spanned, WrappingSpan}};
use sertyp::{SYMBOL_eq, SYMBOL_lt, SYMBOL_lt_eq, chumsky::parser::character};

use crate::{
    pratt_infix,
    parser::ops::{composed_operator, element_wise::{element_wise_binary, real_scalar_binary, validate_same_shape}},
};

use super::super::boolean_indicator;

#[kalt_macros::parser]
pub fn pratt_leq_operator() -> char {
    choice((
        composed_operator(SYMBOL_lt, SYMBOL_eq).map(|_| SYMBOL_lt_eq),
        character(SYMBOL_lt_eq),
    ))
}

pratt_infix!(leq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(
        element_wise_binary(
            |lhs, rhs| real_scalar_binary(lhs, rhs, |lhs, rhs| Ok(boolean_indicator(lhs.inner <= rhs.inner))),
            validate_same_shape(|lhs, rhs| sertyp::content!(sertyp::sequence![lhs, SYMBOL_lt_eq, rhs])),
            lhs,
            rhs,
        )?
    ))
});
