use chumsky::Parser;
use num::Complex;

use crate::{
    Expects,
    parser::{
        ParserError,
        atoms::tensor::Tensor,
        ops::{func::{func_parser, trig_common}, word_or_op},
        pratt::span,
    },
};

#[kalt_macros::parser]
fn cosh_fn() -> () {
    word_or_op("cosh").ignored()
}

/// Applies cosh to a scalar or element-wise to a matrix.
#[kalt_macros::parser]
pub fn cosh(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(cosh_fn(), span(parser))
        .map(|(_, value)| trig_common::unary(value?, |value: Complex<f64>| value.cosh()))
}
