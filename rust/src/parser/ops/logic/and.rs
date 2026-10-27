use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_and, chumsky::parser::character, content};

use crate::{
    Expects, pratt_infix,
    parser::{atoms::tensor::Tensor, ops::element_wise::{element_wise2, validate_same_shape}},
};

use super::{boolean_indicator, boolean_value};

pub fn logical_and<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    element_wise2(
        |lhs, rhs| Ok(boolean_indicator(boolean_value(lhs)? && boolean_value(rhs)?)),
        validate_same_shape(|lhs, rhs| content!(sertyp::sequence![lhs, SYMBOL_and, rhs])),
        lhs,
        rhs,
    )
}

#[kalt_macros::parser]
pub fn pratt_and_operator() -> char { character(SYMBOL_and) }

pratt_infix!(logical_and => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(logical_and(lhs, rhs)?))
});
