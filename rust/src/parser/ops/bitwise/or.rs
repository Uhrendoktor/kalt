use sertyp::{SYMBOL_bar_v, chumsky::parser::character};

use crate::parser::ops::element_wise_binary_operator;

#[kalt_macros::parser]
pub fn pratt_bitwise_or_operator() -> char {
    character(SYMBOL_bar_v)
}

element_wise_binary_operator!(
    bitwise_or,
    SYMBOL_bar_v,
    char,
    |lhs, rhs| crate::parser::ops::element_wise::real_scalar_binary(lhs, rhs, |lhs, rhs| {
        Ok(num::Complex::new(
            (super::bitwise_integer(lhs)? | super::bitwise_integer(rhs)?) as f64,
            0.0,
        ))
    })
);
