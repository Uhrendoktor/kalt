use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use sertyp::{SYMBOL_not, chumsky::parser::character};

use crate::{
    Expects, pratt_prefix,
    parser::{atoms::tensor::Tensor, ops::element_wise::element_wise},
};

use super::{boolean_indicator, boolean_value};

pub fn logical_not<'data>(value: SimpleSpanned<Tensor>) -> Expects<'data, Tensor> {
    element_wise(
        |value| Ok(boolean_indicator(!boolean_value(value)?)),
        |_| Ok(()),
        value,
    )
}

#[kalt_macros::parser]
pub fn pratt_not_operator() -> char { character(SYMBOL_not) }

pratt_prefix!(logical_not => |_op, rhs: Spanned<_>| {
    let span = rhs.span;
    Ok(span.make_wrapped(logical_not(rhs)?))
});
