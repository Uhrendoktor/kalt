use chumsky::{Parser, span::SimpleSpan};
use sertyp::{TypstError, chumsky::LocatingSequenceLike};

use crate::Expects;

pub mod atoms;
pub mod ops;
pub mod pratt;

pub type ParserError<'data, S = SimpleSpan> = chumsky::extra::Full<TypstError<'data, S>, (), ()>;

/// shorthand for sertyp::Attach exponents
///
/// Example:
/// pow!({SYMBOL_CC} ^ {SYMBOL_NN}) => $CC^NN$
#[macro_export]
macro_rules! pow {
    ($base:block ^ $exp:block) => {
        sertyp::math::Attach {
            t: Some(sertyp::content!($exp).into()),
            base: sertyp::content!($base).into(),
            ..Default::default()
        }
    };
}

/// shorthand for sertyp::Attach exponents
///
/// Example:
/// pow!({SYMBOL_CC} ^ {SYMBOL_NN}) => $CC^NN$
#[macro_export]
macro_rules! subscript {
    ($base:block _ $sub:block) => {
        sertyp::math::Attach {
            b: Some(sertyp::content!($sub).into()),
            base: sertyp::content!($base).into(),
            ..Default::default()
        }
    };
}

///
/// Args:
/// - `parser`: The parser to validate.
/// - `validator`: A function that takes the output of the parser and returns a Result.
///
/// Returns:
/// A new parser that validates the output of the given parser.
pub fn validate<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>, T, O>(
    parser: impl Parser<'this, I, Expects<'data, T>, ParserError<'data>>,
    validator: impl Fn(T, SimpleSpan) -> Expects<'data, O>,
) -> impl Parser<'this, I, Expects<'data, O>, ParserError<'data>> {
    parser.map_with(move |v, extra| match v {
        Ok(v) => validator(v, extra.span()),
        Err(e) => Err(e),
    })
}

pub mod validator {
    use chumsky::span::SimpleSpan;
    use num::Complex;
    use sertyp::{SYMBOL_CC, SYMBOL_NN, SYMBOL_RR, SYMBOL_in, TypstError, equation, sequence};

    use crate::{
        Expects,
        parser::atoms::{matrix::Matrix, tensor::Tensor},
    };

    /// Validator: tensor -> tensor
    pub fn tensor<'data>(tensor: Tensor, _span: SimpleSpan) -> Expects<'data, Tensor> {
        Ok(tensor)
    }

    /// Validator: tensor -> matrix
    pub fn matrix<'data>(tensor: Tensor, span: SimpleSpan) -> Expects<'data, Matrix> {
        match tensor {
            Tensor::Scalar(_) => Err(TypstError::full(span, "Type Error", "matrix", "scalar")),
            Tensor::Matrix(m) => Ok(m),
        }
    }

    /// Validator: tensor -> scalar
    pub fn scalar<'data>(tensor: Tensor, span: SimpleSpan) -> Expects<'data, Complex<f64>> {
        match tensor {
            Tensor::Scalar(s) => Ok(s),
            Tensor::Matrix(_) => Err(TypstError::full(span, "Type Error", "scalar", "matrix")),
        }
    }

    /// Validator: scalar -> real
    pub fn real<'data>(scalar: Complex<f64>, span: SimpleSpan) -> Expects<'data, f64> {
        if scalar.im == 0.0 {
            Ok(scalar.re)
        } else {
            Err(TypstError::full(
                span,
                "Real Value Error",
                equation!["f", SYMBOL_in, SYMBOL_RR],
                equation![scalar.to_string(), SYMBOL_in, SYMBOL_CC],
            ))
        }
    }

    /// Validator: real -> integer
    pub fn real_int<'data>(float: f64, span: SimpleSpan) -> Expects<'data, i64> {
        if float.fract() == 0.0 {
            Ok(float as i64)
        } else {
            Err(TypstError::full(
                span,
                "Integer Error",
                sequence!["i", SYMBOL_in, SYMBOL_NN],
                sequence![float.to_string(), SYMBOL_in, SYMBOL_RR],
            ))
        }
    }
}
