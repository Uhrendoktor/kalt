use chumsky::span::{Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_or, chumsky::parser::character};

use crate::pratt_infix;

use super::logical_binary_tensor;

logical_binary_tensor!(logical_or, SYMBOL_or, ||);

#[kalt_macros::parser]
pub fn pratt_or_operator() -> char { character(SYMBOL_or) }

pratt_infix!(logical_or => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(logical_or(lhs, rhs)?))
});
