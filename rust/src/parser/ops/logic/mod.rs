use chumsky::{primitive::choice, span::SimpleSpanned};
use num::Complex;
use sertyp::{SYMBOL_CC, SYMBOL_RR, SYMBOL_ZZ, SYMBOL_in, TypstError, content};

use crate::{Expects, parser::atoms::tensor::Tensor};

mod and;
mod compare;
mod not;
mod or;

pub use and::{pratt_and_operator, pratt_logical_and};
pub use compare::{
    pratt_eq, pratt_eq_operator, pratt_g, pratt_g_operator, pratt_geq, pratt_geq_operator,
    pratt_l, pratt_l_operator, pratt_leq, pratt_leq_operator, pratt_neq, pratt_neq_operator,
};
pub use not::{pratt_logical_not, pratt_not_operator};
pub use or::{pratt_logical_or, pratt_or_operator};

pub(crate) fn boolean_value<'data>(
    value: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, bool> {
    if value.inner.im != 0.0 {
        return Err(TypstError::full(
            value.span,
            "Boolean Operand Error",
            content!(sertyp::sequence!["n", SYMBOL_in, SYMBOL_RR]),
            content!(sertyp::sequence![value.inner.to_string(), SYMBOL_in, SYMBOL_CC]),
        ));
    }

    match value.inner.re {
        0.0 => Ok(false),
        1.0 => Ok(true),
        _ => {
            let actual_domain = if value.inner.re.fract() == 0.0 { SYMBOL_ZZ } else { SYMBOL_RR };
            Err(TypstError::full(
                value.span,
                "Boolean Operand Error",
                content!(sertyp::sequence!["n", SYMBOL_in, "{0, 1}"]),
                content!(sertyp::sequence![value.inner.re.to_string(), SYMBOL_in, actual_domain]),
            ))
        }
    }
}

pub(crate) fn boolean_indicator(value: bool) -> Complex<f64> {
    Complex::new(value as f64, 0.0)
}

#[kalt_macros::parser]
pub fn boolean_literal() -> Expects<'data, Tensor> {
    choice((
        super::word_or_op("true").map(|_| Ok(Tensor::Scalar(boolean_indicator(true)))),
        super::word_or_op("false").map(|_| Ok(Tensor::Scalar(boolean_indicator(false)))),
    ))
}
