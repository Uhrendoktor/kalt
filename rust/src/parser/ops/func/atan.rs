use chumsky::{Parser, primitive::choice};
use num::{Complex, complex::ComplexFloat};

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
fn atan_fn() -> () {
    choice((
        pow_parser(|| word_or_op("tan"), pratt)
            .filter(|(_, exponent)| matches!(
                &exponent.inner,
                Ok(Tensor::Scalar(value)) if *value == Complex::new(-1.0, 0.0)
            ))
            .ignored(),
        word_or_op("arctan").ignored(),
    ))
}

/// Applies the principal inverse tan to a scalar or element-wise to a matrix.
#[kalt_macros::parser]
pub fn atan(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(atan_fn(), span(parser))
        .map(|(_, value)| trig_common::unary(value?, |value: Complex<f64>| value.atan()))
}
