use chumsky::{Parser, primitive::choice};
use num::Complex;

use crate::{
    Expects,
    parser::{
        ParserError,
        atoms::tensor::Tensor,
        ops::{func::{func_parser, trig_common}, pow::pow_parser, word_or_op},
        pratt::{pratt, span},
    },
};

#[kalt_macros::parser]
fn asin_fn() -> () {
    choice((
        pow_parser(|| word_or_op("sin"), pratt)
            .filter(|(_, exponent)| matches!(
                &exponent.inner,
                Ok(Tensor::Scalar(value)) if *value == Complex::new(-1.0, 0.0)
            ))
            .ignored(),
        word_or_op("arcsin").ignored(),
    ))
}

/// Applies the principal inverse sin to a scalar or element-wise to a matrix.
#[kalt_macros::parser]
pub fn asin(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(asin_fn(), span(parser))
        .map(|(_, value)| trig_common::unary(value?, |value: Complex<f64>| value.asin()))
}
