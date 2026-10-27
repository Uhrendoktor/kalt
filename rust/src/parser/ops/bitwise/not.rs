use chumsky::span::{Spanned, WrappingSpan};
use sertyp::chumsky::parser::character;

use crate::{
    pratt_prefix,
    parser::ops::element_wise::element_wise_unary,
};

#[kalt_macros::parser]
pub fn pratt_bitwise_not_operator() -> char {
    character('~')
}

pratt_prefix!(bitwise_not => |_op, rhs: Spanned<_>| {
    Ok(rhs.span.make_wrapped(element_wise_unary(
        |value| Ok(num::Complex::new(
            (!crate::parser::validator::real_int(
                crate::parser::validator::real(value.inner, value.span)?,
                value.span,
            )?) as f64,
            0.0,
        )),
        |_| Ok(()),
        rhs,
    )?))
});
