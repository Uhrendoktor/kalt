use chumsky::span::{Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_and, chumsky::parser::character};

use crate::pratt_infix;

use super::logical_binary_tensor;

logical_binary_tensor!(logical_and, SYMBOL_and, &&);

#[kalt_macros::parser]
pub fn pratt_and_operator() -> char { character(SYMBOL_and) }

pratt_infix!(logical_and => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(logical_and(lhs, rhs)?))
});
