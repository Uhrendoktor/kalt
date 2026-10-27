use chumsky::span::{Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_eq_not, chumsky::parser::character};

use crate::{
    pratt_infix,
    parser::ops::element_wise::{element_wise_binary, validate_same_shape},
};

use super::super::boolean_indicator;

#[kalt_macros::parser]
pub fn pratt_neq_operator() -> char {
    character(SYMBOL_eq_not)
}

pratt_infix!(neq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(
        element_wise_binary(
            |lhs, rhs| Ok(boolean_indicator(lhs.inner != rhs.inner)),
            validate_same_shape(|lhs, rhs| sertyp::content!(sertyp::sequence![lhs, SYMBOL_eq_not, rhs])),
            lhs,
            rhs,
        )?
    ))
});
