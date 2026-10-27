use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_eq_not, chumsky::parser::character};

use crate::{
    Expects, pratt_infix,
    parser::atoms::tensor::Tensor,
};

use super::super::boolean_indicator;
use super::compare_tensors;

#[kalt_macros::parser]
pub fn pratt_neq_operator() -> char {
    character(SYMBOL_eq_not)
}

pub fn compare_neq<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_eq_not, |lhs, rhs| {
        Ok(boolean_indicator(lhs.inner != rhs.inner))
    })
}

pratt_infix!(neq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_neq(lhs, rhs)?))
});
