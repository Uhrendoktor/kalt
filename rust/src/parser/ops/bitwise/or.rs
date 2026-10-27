use chumsky::span::{Span, WrappingSpan};
use sertyp::{SYMBOL_bar_v, chumsky::parser::character};

use super::bitwise_operation;

#[kalt_macros::parser]
pub fn pratt_bitwise_or_operator() -> char {
    character(SYMBOL_bar_v)
}

bitwise_operation!(bitwise_or, |, SYMBOL_bar_v);
