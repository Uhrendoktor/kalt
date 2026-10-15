use chumsky::{Parser, primitive::choice, span::Spanned};
use sertyp::{
    SYMBOL_ceil_l, SYMBOL_ceil_r, SYMBOL_floor_l, SYMBOL_floor_r,
    chumsky::parser::{character, whitespaces},
};

use crate::{
    Expects, match_tensor,
    parser::{
        ParserError,
        atoms::{matrix::Matrix, tensor::Tensor},
        pratt::span,
    },
};

/// Rounds a tensor:
/// scalar -> scalar (norm)
/// matrix -> scalar (element-wise)
pub fn round_t<'data>(t: Spanned<Tensor>) -> Expects<'data, Tensor> {
    match_tensor!((t) => {
        s => |s: Spanned<num::Complex<f64>>| Ok(num::Complex::<f64>::new(s.re.round(), s.im.round())),
        m => |mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(|el| num::Complex::<f64>::new(el.re.round(), el.im.round()));
            Ok(m.inner)
        }
    })
}

/// Parses the rounding operator
///
/// EBNF:
/// ("⌊" <tensor> "⌉") |  ("⌈" <tensor> "⌋")
#[kalt_macros::parser]
pub fn round<'data>(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    let inner = span(parser)
        .delimited_by(whitespaces(), whitespaces())
        .boxed();
    choice((
        inner
            .clone()
            .delimited_by(character(SYMBOL_floor_l), character(SYMBOL_ceil_r)),
        inner.delimited_by(character(SYMBOL_floor_r), character(SYMBOL_ceil_l)),
    ))
    .map(|t| round_t(t?))
}
