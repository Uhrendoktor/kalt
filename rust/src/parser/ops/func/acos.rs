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
fn acos_fn() -> () {
    choice((
        pow_parser(|| word_or_op("cos"), pratt)
            .filter(|(_, exponent)| matches!(
                &exponent.inner,
                Ok(Tensor::Scalar(value)) if *value == Complex::new(-1.0, 0.0)
            ))
            .ignored(),
        word_or_op("arccos").ignored(),
    ))
}

/// Applies the principal inverse cos to a scalar or element-wise to a matrix.
#[kalt_macros::parser]
pub fn acos(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(acos_fn(), span(parser))
        .map(|(_, value)| trig_common::unary(value?, |value: Complex<f64>| value.acos()))
}
