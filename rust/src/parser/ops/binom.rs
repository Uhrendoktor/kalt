use chumsky::{
    extra::Full,
    input::MapExtra,
    select,
    span::{Spanned, WrappingSpan},
};
use sertyp::{Content, LocatingSequence, TypstError, chumsky::Token, math::Binom};

use crate::{
    Expects,
    parser::{
        atoms::tensor::Tensor,
        ops::{
            element_wise::{element_wise2, validate_same_shape},
            factorial::factorial_c,
        },
        pratt::pratt,
    },
};

/// Computes the binomial coefficient of two complex numbers using the gamma function.
pub fn binom_c(n: &num::Complex<f64>, k: &num::Complex<f64>) -> num::Complex<f64> {
    factorial_c(n) / (factorial_c(k) * factorial_c(&(n - k)))
}

/// Applies the binomial coefficient to two tensors:
/// (scalar, scalar) -> scalar
/// (scalar, matrix) -> matrix (element-wise)
/// (matrix, scalar) -> matrix (element-wise)
/// (matrix, matrix) -> matrix (element-wise) only if same shape
pub fn binom_t<'data>(t1: Spanned<Tensor>, t2: Spanned<Tensor>) -> Expects<'data, Tensor> {
    element_wise2(
        |s1, s2| Ok(binom_c(&s1, &s2)),
        validate_same_shape(|m1, m2| {
            Binom {
                upper: m1.into(),
                lower: Some(vec![m2].into()),
            }
            .into()
        }),
        t1,
        t2,
    )
}

#[kalt_macros::parser]
pub fn binom<'data>() -> Expects<'data, Tensor> {
    select!(Token::Raw(Content::MathBinom(binom)) => binom).map_with(
        |binom_obj, extra: &mut MapExtra<'_, '_, I, Full<TypstError<'data>, (), ()>>| {
            let n = pratt::<LocatingSequence>()
                .parse(LocatingSequence::from(&**binom_obj.upper))
                .into_result()??;
            let k = if let Some(lower) = &binom_obj.lower
                && lower.len() == 1
            {
                &lower[0]
            } else {
                return Err(TypstError::full(
                    extra.span(),
                    "Binominal Argument Error",
                    "Binomial coefficients must have exactly one lower argument",
                    format!(
                        "{} arguments",
                        binom_obj.lower.as_ref().map(|v| v.len()).unwrap_or(0)
                    ),
                ));
            };
            let k = pratt::<LocatingSequence>()
                .parse(LocatingSequence::from(k))
                .into_result()??;

            binom_t(extra.span().make_wrapped(n), extra.span().make_wrapped(k))
        },
    )
}
