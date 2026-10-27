use chumsky::span::{Spanned, WrappingSpan};
use sertyp::{SYMBOL_not, chumsky::parser::character};

use crate::{
    pratt_prefix,
    parser::ops::element_wise::element_wise_unary,
};

use super::{boolean_indicator, boolean_value};

#[kalt_macros::parser]
pub fn pratt_not_operator() -> char {
    character(SYMBOL_not)
}

pratt_prefix!(logical_not => |_op, rhs: Spanned<_>| {
    Ok(rhs.span.make_wrapped(element_wise_unary(
        |value| Ok(boolean_indicator(!boolean_value(value)?)),
        |_| Ok(()),
        rhs,
    )?))
});
