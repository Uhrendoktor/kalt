use sertyp::{SYMBOL_or, chumsky::parser::character};

use crate::parser::ops::element_wise_binary_operator;

use super::{boolean_indicator, boolean_value};

element_wise_binary_operator!(
    logical_or,
    SYMBOL_or,
    char,
    |lhs, rhs| Ok(boolean_indicator(boolean_value(lhs)? || boolean_value(rhs)?))
);

#[kalt_macros::parser]
pub fn pratt_or_operator() -> char {
    character(SYMBOL_or)
}
