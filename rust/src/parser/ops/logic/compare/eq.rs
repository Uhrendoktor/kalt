use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_eq, chumsky::parser::character};

use crate::{
    Expects, pratt_infix,
    parser::atoms::tensor::Tensor,
};

use super::super::boolean_indicator;
use super::compare_tensors;

#[kalt_macros::parser]
pub fn pratt_eq_operator() -> char {
    character(SYMBOL_eq)
}

pub fn compare_eq<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_eq, |lhs, rhs| {
        Ok(boolean_indicator(lhs.inner == rhs.inner))
    })
}

pratt_infix!(eq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_eq(lhs, rhs)?))
});
