use chumsky::span::Spanned;
use num::Complex;

use crate::{
    Expects, match_tensor,
    parser::atoms::{matrix::Matrix, tensor::Tensor},
};

/// Applies a complex-valued unary function to a scalar or each matrix entry.
pub fn unary<'data, F>(value: Spanned<Tensor>, function: F) -> Expects<'data, Tensor>
where
    F: Fn(Complex<f64>) -> Complex<f64> + Copy,
{
    match_tensor!((value) => {
        s => |s: Spanned<Complex<f64>>| Ok(function(*s)),
        m => |mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(function);
            Ok(m.inner)
        }
    })
}
/// Returns whether a parsed exponent is exactly -1, used by inverse trig parsers.
pub fn is_negative_one(exponent: &crate::Expects<'_, Tensor>) -> bool {
    matches!(
        exponent,
        Ok(Tensor::Scalar(value)) if *value == Complex::new(-1.0, 0.0)
    )
}
