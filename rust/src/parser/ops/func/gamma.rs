use chumsky::{Parser, span::Spanned};
use lazy_static::lazy_static;
use sertyp::SYMBOL_Gamma;

use crate::{
    Expects,
    parser::{
        ParserError,
        atoms::tensor::Tensor,
        ops::{element_wise::unary, factorial::gamma, func::func_parser, word_or_op},
        pratt::span,
    },
};

/// Computes ln(v):
/// scalar -> scalar
/// matrix -> matrix (element-wise)
pub fn gamma_t<'data>(value: Spanned<Tensor>) -> Expects<'data, Tensor> {
    unary(value, gamma::gamma)
}

lazy_static! {
    static ref SYMBOL_GAMMA_STR: String = SYMBOL_Gamma.to_string();
}

/// Parses the log keyword and returns the base
///
/// EBNF:
/// "Γ"
#[kalt_macros::parser]
pub fn gamma_fn() -> () {
    // convert SYMBOL_Gamma to static str
    word_or_op(SYMBOL_GAMMA_STR.as_str()).ignored()
}

#[kalt_macros::parser]
pub fn gamma(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(gamma_fn(), span(parser)).map(|(_, value)| gamma_t(value?))
}
