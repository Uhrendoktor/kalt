use chumsky::{Parser, span::SimpleSpan};
use sertyp::{FromString, TypstError, chumsky::LocatingSequenceLike};

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
    parser
        .map_with(move |v, extra| match v {
            Ok(v) => validator(v, extra.span()),
            Err(e) => Err(e),
        })
        .labelled(sertyp::Content::from_string("validate"))
}

pub mod validator {
    use chumsky::span::SimpleSpan;
    use num::Complex;
    use sertyp::{SYMBOL_CC, SYMBOL_RR, SYMBOL_ZZ, SYMBOL_in, TypstError, equation, sequence};

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

    /// Largest magnitude for which every integer is exactly representable by an f64.
    pub const MAX_EXACT_INTEGER: f64 =
        ((1_u64 << f64::MANTISSA_DIGITS) - 1) as f64;

    /// Validator: real -> signed 64-bit integer, rejecting values that would saturate on cast.
    pub fn real_int<'data>(float: f64, span: SimpleSpan) -> Expects<'data, i64> {
        if float.fract() != 0.0 {
            return Err(TypstError::full(
                span,
                "Integer Error",
                sequence!["i", SYMBOL_in, SYMBOL_ZZ],
                sequence![float.to_string(), SYMBOL_in, SYMBOL_RR],
            ));
        }

        // Reject integers that may already have been rounded when parsed as f64.
        if float.abs() > MAX_EXACT_INTEGER {
            return Err(TypstError::full(
                span,
                "Integer Range Error",
                sequence![
                    "i", SYMBOL_in,
                    format!("[-{}, {}]", MAX_EXACT_INTEGER, MAX_EXACT_INTEGER)
                ],
                sequence![float.to_string(), SYMBOL_in, SYMBOL_RR],
            ));
        }

        // MAX_EXACT_INTEGER is much smaller than i64::MAX, so this cast is in range.
        Ok(float as i64)
    }
}
