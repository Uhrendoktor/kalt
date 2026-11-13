use sertyp::{SYMBOL_xor, chumsky::parser::character};

#[kalt_macros::parser]
pub fn pratt_bitwise_xor_operator() -> char {
    character(SYMBOL_xor)
}

crate::element_wise_binary_operator!(
    bitwise_xor,
    SYMBOL_xor,
    char,
    |lhs, rhs| crate::parser::ops::element_wise::real_scalar_binary(lhs, rhs, |lhs, rhs| {
        Ok(num::Complex::new(
            (crate::parser::validator::real_int(lhs.inner, lhs.span)? ^ crate::parser::validator::real_int(rhs.inner, rhs.span)?) as f64,
            0.0,
        ))
    })
);
