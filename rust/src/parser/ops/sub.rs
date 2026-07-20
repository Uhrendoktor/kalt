use chumsky::primitive::choice;
use chumsky::span::Spanned;
use chumsky::span::{Span, WrappingSpan};
use sertyp::SYMBOL_minus;
use sertyp::chumsky::parser::character;

use crate::{operation_element_wise_same_shape, pratt_infix};

/// Condition for the pratt parser to apply.
#[kalt_macros::parser]
pub fn pratt_sub_operator() -> char {
    choice((character(SYMBOL_minus), character('-')))
}

pratt_infix!(sub => |_op, lhs: Spanned<_>, rhs: Spanned<_>| Ok(lhs.span.union(rhs.span).make_wrapped(sub(lhs, rhs)?)));
operation_element_wise_same_shape!(sub: -);
