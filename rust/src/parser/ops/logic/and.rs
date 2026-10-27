use sertyp::{SYMBOL_and, chumsky::parser::character};

crate::element_wise_binary_operator!(
    logical_and,
    SYMBOL_and,
    char,
    |lhs, rhs| super::boolean_binary_scalar(lhs, rhs, |lhs, rhs| lhs && rhs)
);

#[kalt_macros::parser]
pub fn pratt_and_operator() -> char {
    character(SYMBOL_and)
}
