mod and;
mod or;
mod xor;

pub use and::{bitwise_and, pratt_bitwise_and, pratt_bitwise_and_operator};
pub use or::{bitwise_or, pratt_bitwise_or, pratt_bitwise_or_operator};
pub use xor::{bitwise_xor, pratt_bitwise_xor, pratt_bitwise_xor_operator};

const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

fn bitwise_integer<'data>(
    value: chumsky::span::SimpleSpanned<f64>,
) -> crate::Expects<'data, i64> {
    use sertyp::{SYMBOL_RR, SYMBOL_ZZ, SYMBOL_in, SYMBOL_lt_eq, TypstError, content};
    if value.inner.abs() > MAX_SAFE_INTEGER {
        return Err(TypstError::full(
            value.span,
            "Bitwise Operand Error",
            content!(sertyp::sequence![
                "n", SYMBOL_in, SYMBOL_ZZ, ", |n|", SYMBOL_lt_eq, MAX_SAFE_INTEGER.to_string()
            ]),
            content!(sertyp::sequence![value.inner.to_string(), SYMBOL_in, SYMBOL_RR]),
        ));
    }
    crate::parser::validator::real_int(value.inner, value.span)
}

