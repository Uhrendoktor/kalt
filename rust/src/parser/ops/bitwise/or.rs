use sertyp::{SYMBOL_or, chumsky::parser::character};

#[kalt_macros::parser]
pub fn pratt_bitwise_or_operator() -> char {
    character(SYMBOL_or)
}

crate::element_wise_binary_operator!(
    bitwise_or,
    SYMBOL_or,
    char,
    |lhs, rhs| crate::parser::ops::element_wise::real_scalar_binary(lhs, rhs, |lhs, rhs| {
        Ok(num::Complex::new(
            (super::bitwise_integer(lhs)? | super::bitwise_integer(rhs)?) as f64,
            0.0,
        ))
    })
);
