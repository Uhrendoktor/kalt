use chumsky::{primitive::choice, span::{SimpleSpanned, Span, Spanned, WrappingSpan}};
use sertyp::{SYMBOL_eq, SYMBOL_gt, SYMBOL_gt_eq, chumsky::parser::character};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::tensor::Tensor,
        ops::{composed_operator, element_wise::real_scalar_binary},
    },
};

use super::super::boolean_indicator;
use super::compare_tensors;

#[kalt_macros::parser]
pub fn pratt_geq_operator() -> char {
    choice((
        composed_operator(SYMBOL_gt, SYMBOL_eq).map(|_| SYMBOL_gt_eq),
        character(SYMBOL_gt_eq),
    ))
}

pub fn compare_ge<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_gt_eq, |lhs, rhs| {
        real_scalar_binary(lhs, rhs, |lhs, rhs| {
            Ok(boolean_indicator(lhs.inner >= rhs.inner))
        })
    })
}

pratt_infix!(geq, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_ge(lhs, rhs)?))
});
