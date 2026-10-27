use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_lt, chumsky::parser::character};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::real_scalar_binary,
    },
};

use super::super::boolean_indicator;
use super::compare_tensors;

#[kalt_macros::parser]
pub fn pratt_l_operator() -> char {
    character(SYMBOL_lt)
}

pub fn compare_lt<'data>(lhs: SimpleSpanned<Tensor>, rhs: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    compare_tensors(lhs, rhs, SYMBOL_lt, |lhs, rhs| {
        real_scalar_binary(lhs, rhs, |lhs, rhs| {
            Ok(boolean_indicator(lhs.inner < rhs.inner))
        })
    })
}

pratt_infix!(l, char => |_op: char, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(compare_lt(lhs, rhs)?))
});
