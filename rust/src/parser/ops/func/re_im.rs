use chumsky::{Parser, span::Spanned};
use sertyp::{SYMBOL_Im, SYMBOL_Re, chumsky::parser::character};

use crate::{
    Expects,
    parser::{
        ParserError,
        atoms::tensor::Tensor,
        ops::{element_wise::unary_tensor, func::func_parser},
        pratt::span,
    },
};

/// Computes the imaginary part of a tensor:
/// scalar -> scalar (imaginary part)
/// matrix -> matrix (element-wise)
pub fn im_t(t: Spanned<Tensor>) -> Tensor {
    unary_tensor(t.inner, |value| num::Complex::<f64>::new(value.im, 0.0))
}

/// Returns the imaginary part of a tensor:
///
/// EBNF:
/// <Im> "(" <tensor> ")"
#[kalt_macros::parser]
pub fn im<'data>(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(character(SYMBOL_Im), span(parser)).map(|(_op, t)| Ok(im_t(t?)))
}

/// Computes the real part of a tensor:
/// scalar -> scalar (real part)
/// matrix -> matrix (element-wise)
pub fn re_t(t: Spanned<Tensor>) -> Tensor {
    unary_tensor(t.inner, |value| num::Complex::<f64>::new(value.re, 0.0))
}

/// Returns the real part of a tensor:
///
/// EBNF:
/// <Re> "(" <tensor> ")"
#[kalt_macros::parser]
pub fn re<'data>(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    func_parser(character(SYMBOL_Re), span(parser)).map(|(_op, t)| Ok(re_t(t?)))
}
