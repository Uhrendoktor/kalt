use chumsky::span::{Span, WrappingSpan};
use sertyp::{SYMBOL_xor, chumsky::parser::character};

use super::bitwise_operation;

#[kalt_macros::parser]
pub fn pratt_bitwise_xor_operator() -> char {
    character(SYMBOL_xor)
}

bitwise_operation!(bitwise_xor, ^, SYMBOL_xor);
