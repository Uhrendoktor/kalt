use chumsky::span::{Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_plus, chumsky::parser::character};

use crate::{operation_element_wise_same_shape, pratt_infix};

/// Condition for the pratt parser to apply.
#[kalt_macros::parser]
pub fn pratt_add_operator() -> char {
    character(SYMBOL_plus)
}

pratt_infix!(add => |_op, lhs: Spanned<_>, rhs: Spanned<_>| Ok(lhs.span.union(rhs.span).make_wrapped(add(lhs, rhs)?)));
operation_element_wise_same_shape!(add: +);
