use chumsky::{primitive::choice, span::{Span, Spanned, WrappingSpan}};
use sertyp::{SYMBOL_eq, SYMBOL_gt, SYMBOL_gt_eq, chumsky::parser::character};

use crate::{
    pratt_infix,
    parser::{
        ops::{composed_operator, element_wise::{element_wise_binary, real_scalar_binary, validate_same_shape}},
    },
};

use super::super::boolean_indicator;

#[kalt_macros::parser]
pub fn pratt_geq_operator() -> char {
    choice((
        composed_operator(SYMBOL_gt, SYMBOL_eq).map(|_| SYMBOL_gt_eq),
        character(SYMBOL_gt_eq),
    ))
}

pratt_infix!(geq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(
        element_wise_binary(
            |lhs, rhs| real_scalar_binary(lhs, rhs, |lhs, rhs| Ok(boolean_indicator(lhs.inner >= rhs.inner))),
            validate_same_shape(|lhs, rhs| sertyp::content!(sertyp::sequence![lhs, SYMBOL_gt_eq, rhs])),
            lhs,
            rhs,
        )?
    ))
});
