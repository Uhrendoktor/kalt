use chumsky::{primitive::choice, select, span::{Span, WrappingSpan}};
use sertyp::{
    Content, SYMBOL_amp, chumsky::{Token, parser::character},
};

use super::bitwise_operation;

#[kalt_macros::parser]
pub fn pratt_bitwise_and_operator() -> char {
    choice((
        character(SYMBOL_amp),
        select!(
            Token::Raw(Content::Unknown(raw))
                if raw.func.full_name().is_some_and(|name| &*name == "align-point") => SYMBOL_amp
        ),
    ))
}

bitwise_operation!(bitwise_and, &, SYMBOL_amp);
