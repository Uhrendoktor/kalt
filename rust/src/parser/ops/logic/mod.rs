use chumsky::{primitive::choice, span::SimpleSpanned};
use num::Complex;
use sertyp::{
    SYMBOL_CC, SYMBOL_RR, SYMBOL_ZZ, SYMBOL_in, TypstError, content,
    chumsky::parser::word,
};

use crate::{Expects, parser::atoms::tensor::Tensor};

mod compare;
mod not;

pub use not::{pratt_logical_not, pratt_not_operator};

pub use compare::{
    pratt_eq, pratt_eq_operator, pratt_g, pratt_g_operator, pratt_geq, pratt_geq_operator,
    pratt_l, pratt_l_operator, pratt_leq, pratt_leq_operator, pratt_neq, pratt_neq_operator,
};

/// Parses bare Typst math words `true` and `false` as numeric indicators.
/// Deliberately accepts words only, not math operators such as `op("true")`.
#[kalt_macros::parser]
pub fn boolean_literal() -> Expects<'data, Tensor> {
    choice((
        word("true").map(|_| Ok(Tensor::Scalar(boolean_indicator(true)))),
        word("false").map(|_| Ok(Tensor::Scalar(boolean_indicator(false)))),
    ))
}

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
    Complex::new(value as u8 as f64, 0.0)
}
