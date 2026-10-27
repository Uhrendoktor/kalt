#[kalt_macros::parser]
pub fn pratt_bitwise_xor_operator() -> String {
    super::super::word_or_op("bitxor")
}

crate::element_wise_binary_operator!(
    bitwise_xor,
    "bitxor",
    String,
    |lhs, rhs| crate::parser::ops::element_wise::real_scalar_binary(lhs, rhs, |lhs, rhs| {
        Ok(num::Complex::new(
            (super::bitwise_integer(lhs)? ^ super::bitwise_integer(rhs)?) as f64,
            0.0,
        ))
    })
);
