use sertyp::{SYMBOL_and, chumsky::parser::character};

use crate::parser::ops::element_wise_binary_operator;

use super::{boolean_indicator, boolean_value};

element_wise_binary_operator!(
    logical_and,
    SYMBOL_and,
    char,
    |lhs, rhs| Ok(boolean_indicator(boolean_value(lhs)? && boolean_value(rhs)?))
);

#[kalt_macros::parser]
pub fn pratt_and_operator() -> char {
    character(SYMBOL_and)
}
