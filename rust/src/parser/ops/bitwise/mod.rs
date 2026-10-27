mod and;
mod or;
mod xor;

pub use and::{bitwise_and, pratt_bitwise_and, pratt_bitwise_and_operator};
pub use or::{bitwise_or, pratt_bitwise_or, pratt_bitwise_or_operator};
pub use xor::{bitwise_xor, pratt_bitwise_xor, pratt_bitwise_xor_operator};

const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

fn validate_bitwise_range<'data>(
    value: f64,
    span: chumsky::span::SimpleSpan,
) -> crate::Expects<'data, ()> {
    use sertyp::{SYMBOL_RR, SYMBOL_ZZ, SYMBOL_in, SYMBOL_lt_eq, TypstError, content};
    if value.abs() <= MAX_SAFE_INTEGER {
        return Ok(());
    }
    Err(TypstError::full(
        span,
        "Bitwise Operand Error",
        content!(sertyp::sequence![
            "n", SYMBOL_in, SYMBOL_ZZ, ", |n|", SYMBOL_lt_eq, MAX_SAFE_INTEGER.to_string()
        ]),
        content!(sertyp::sequence![value.to_string(), SYMBOL_in, SYMBOL_RR]),
    ))
}

macro_rules! bitwise_operation {
    ($name:ident, $operator:tt, $symbol:expr) => {
        pub fn $name<'data>(
            lhs: chumsky::span::SimpleSpanned<crate::parser::atoms::tensor::Tensor>,
            rhs: chumsky::span::SimpleSpanned<crate::parser::atoms::tensor::Tensor>,
        ) -> crate::Expects<'data, crate::parser::atoms::tensor::Tensor> {
            crate::parser::ops::element_wise::element_wise2(
                |lhs, rhs| {
                    let left_real = crate::parser::validator::real(lhs.inner, lhs.span)?;
                    let right_real = crate::parser::validator::real(rhs.inner, rhs.span)?;
                    let left = crate::parser::validator::real_int(left_real, lhs.span)?;
                    let right = crate::parser::validator::real_int(right_real, rhs.span)?;
                    super::validate_bitwise_range(left_real, lhs.span)?;
                    super::validate_bitwise_range(right_real, rhs.span)?;
                    Ok(num::Complex::new((left $operator right) as f64, 0.0))
                },
                crate::parser::ops::element_wise::validate_same_shape(|lhs, rhs| {
                    sertyp::content!(sertyp::sequence![lhs, $symbol, rhs])
                }),
                lhs,
                rhs,
            )
        }

        crate::pratt_infix!($name => |_op, lhs: chumsky::span::Spanned<_>, rhs: chumsky::span::Spanned<_>| {
            Ok(chumsky::span::Span::union(lhs.span, rhs.span).make_wrapped($name(lhs, rhs)?))
        });
    };
}
pub(crate) use bitwise_operation;
