use chumsky::{
    primitive::choice,
    span::{Span, Spanned, WrappingSpan},
};
use sertyp::{SYMBOL_ast_basic, SYMBOL_ast_op, chumsky::parser::character};

use crate::{operation_element_wise_same_shape, pratt_infix};

/// Multiplication: Element wise
/// Condition for the pratt parser to apply.
#[kalt_macros::parser]
pub fn pratt_mul_operator() -> char {
    choice((character(SYMBOL_ast_op), character(SYMBOL_ast_basic)))
}

// element wise multiplication of two tensors
pratt_infix!(mul => |_op, lhs: Spanned<_>, rhs: Spanned<_>| Ok(lhs.span.union(rhs.span).make_wrapped(mul(lhs, rhs)?)));
operation_element_wise_same_shape!(mul: *);
