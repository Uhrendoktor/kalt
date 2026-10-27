use sertyp::{SYMBOL_or, chumsky::parser::character};

use super::{boolean_indicator, boolean_value};

crate::element_wise_binary_operator!(
    logical_or,
    SYMBOL_or,
    char,
    |lhs, rhs| super::boolean_binary_scalar(lhs, rhs, |lhs, rhs| lhs || rhs)
);

#[kalt_macros::parser]
pub fn pratt_or_operator() -> char {
    character(SYMBOL_or)
}
