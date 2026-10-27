use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use num::Complex;
use sertyp::{
    SYMBOL_RR, SYMBOL_ZZ, SYMBOL_bar_v, SYMBOL_in, SYMBOL_lt_eq, SYMBOL_xor,
    TypstError, chumsky::parser::character, content,
};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::{element_wise2, validate_same_shape},
        validator::{real, real_int},
    },
};

const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

/// The shared real/integer validators handle operand type and integrality.
/// This bound prevents values outside Kalt's exact integer range from being rounded.
fn validate_bitwise_range<'data>(
    value: f64,
    span: chumsky::span::SimpleSpan,
) -> Expects<'data, ()> {
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
    ($name:ident, $operator:tt, $symbol:expr, $parser:ident, $parsed:expr) => {
        pub fn $name<'data>(
            lhs: SimpleSpanned<Tensor>,
            rhs: SimpleSpanned<Tensor>,
        ) -> Expects<'data, Tensor> {
            element_wise2(
                |lhs, rhs| {
                    let left_real = real(lhs.inner, lhs.span)?;
                    let right_real = real(rhs.inner, rhs.span)?;
                    let left = real_int(left_real, lhs.span)?;
                    let right = real_int(right_real, rhs.span)?;
                    validate_bitwise_range(left_real, lhs.span)?;
                    validate_bitwise_range(right_real, rhs.span)?;
                    Ok(Complex::new((left $operator right) as f64, 0.0))
                },
                validate_same_shape(|lhs, rhs| {
                    content!(sertyp::sequence![lhs, $symbol, rhs])
                }),
                lhs,
                rhs,
            )
        }

        #[kalt_macros::parser]
        pub fn $parser() -> char {
            character($parsed)
        }

        pratt_infix!($name => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
            Ok(lhs.span.union(rhs.span).make_wrapped($name(lhs, rhs)?))
        });
    };
}

bitwise_operation!(bitwise_and, &, "&", pratt_bitwise_and_operator, '&');
bitwise_operation!(bitwise_or, |, SYMBOL_bar_v, pratt_bitwise_or_operator, '|');
bitwise_operation!(bitwise_xor, ^, SYMBOL_xor, pratt_bitwise_xor_operator, SYMBOL_xor);
