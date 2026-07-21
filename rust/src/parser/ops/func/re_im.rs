use chumsky::{Parser, span::Spanned};
use sertyp::{SYMBOL_Im, SYMBOL_Re, TypstError, chumsky::parser::character};

use crate::{
    Expects, match_tensor,
    parser::{
        ParserError,
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::func::func_parser,
        pratt::span,
    },
};

/// Computes the imaginary part of a tensor:
/// scalar -> scalar (imaginary part)
/// matrix -> matrix (element-wise)
pub fn im_t(t: Spanned<Tensor>) -> Tensor {
    match_tensor!((t) => {
        s => |s: Spanned<num::Complex<f64>>| Ok::<_, TypstError>(num::Complex::<f64>::new(s.im, 0.0)),
        m => |mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(|el| num::Complex::<f64>::new(el.im, 0.0));
            Ok(m.inner)
        }
    })
    .unwrap()
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
    match_tensor!((t) => {
        s => |s: Spanned<num::Complex<f64>>| Ok::<_, TypstError>(num::Complex::<f64>::new(s.re, 0.0)),
        m => |mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(|el| num::Complex::<f64>::new(el.re, 0.0));
            Ok(m.inner)
        }
    })
    .unwrap()
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
