use chumsky::{Parser, primitive::choice, span::Spanned};
use num::{Complex, complex::ComplexFloat};

use crate::{
    Expects, match_tensor,
    parser::{
        ParserError,
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::{
            func::{func_parser, superscript_parser},
            word_or_op,
        },
        pratt::{pratt, span},
    },
};

#[derive(Clone, Copy)]
enum TrigFunction {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
}

/// Recognizes common trigonometric and hyperbolic function names.
///
/// Inverse circular functions accept attach notation with exponent -1 and
/// explicit arcsin/arccos/arctan aliases.
#[kalt_macros::parser]
fn trig_fn() -> TrigFunction {
    choice((
        superscript_parser(|| word_or_op("sin"), pratt)
            .filter(|(_, exponent)| matches!(&exponent.inner, Tensor::Scalar(value) if *value == Complex::new(-1.0, 0.0)))
            .to(TrigFunction::Asin),
        superscript_parser(|| word_or_op("cos"), pratt)
            .filter(|(_, exponent)| matches!(&exponent.inner, Tensor::Scalar(value) if *value == Complex::new(-1.0, 0.0)))
            .to(TrigFunction::Acos),
        superscript_parser(|| word_or_op("tan"), pratt)
            .filter(|(_, exponent)| matches!(&exponent.inner, Tensor::Scalar(value) if *value == Complex::new(-1.0, 0.0)))
            .to(TrigFunction::Atan),
        word_or_op("arcsin").to(TrigFunction::Asin),
        word_or_op("arccos").to(TrigFunction::Acos),
        word_or_op("arctan").to(TrigFunction::Atan),
        word_or_op("sinh").to(TrigFunction::Sinh),
        word_or_op("cosh").to(TrigFunction::Cosh),
        word_or_op("tanh").to(TrigFunction::Tanh),
        word_or_op("sin").to(TrigFunction::Sin),
        word_or_op("cos").to(TrigFunction::Cos),
        word_or_op("tan").to(TrigFunction::Tan),
    ))
}

fn apply(value: Complex<f64>, function: TrigFunction) -> Complex<f64> {
    match function {
        TrigFunction::Sin => value.sin(),
        TrigFunction::Cos => value.cos(),
        TrigFunction::Tan => value.tan(),
        TrigFunction::Asin => value.asin(),
        TrigFunction::Acos => value.acos(),
        TrigFunction::Atan => value.atan(),
        TrigFunction::Sinh => value.sinh(),
        TrigFunction::Cosh => value.cosh(),
        TrigFunction::Tanh => value.tanh(),
    }
}

/// Applies a trigonometric or hyperbolic function to a scalar or each matrix cell.
fn trig_t<'data>(
    function: TrigFunction,
    value: Spanned<Tensor>,
) -> Expects<'data, Tensor> {
    match_tensor!((value) => {
        s => |s: Spanned<Complex<f64>>| Ok(apply(*s, function)),
        m => |mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(|element| apply(element, function));
            Ok(m.inner)
        }
    })
}

#[kalt_macros::parser]
pub fn trig(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(trig_fn(), span(parser))
        .map(|(function, value)| trig_t(function, value?))
}
