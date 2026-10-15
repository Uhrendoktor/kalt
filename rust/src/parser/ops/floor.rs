use chumsky::{Parser, span::Spanned};
use sertyp::{
    SYMBOL_floor_l, SYMBOL_floor_r,
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

/// floors a tensor:
/// scalar -> scalar (norm)
/// matrix -> scalar (element-wise)
pub fn floor_t<'data>(t: Spanned<Tensor>) -> Expects<'data, Tensor> {
    match_tensor!((t) => {
        s => |s: Spanned<num::Complex<f64>>| Ok(num::Complex::<f64>::new(s.re.floor(), s.im.floor())),
        m => |mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(|el| num::Complex::<f64>::new(el.re.floor(), el.im.floor()));
            Ok(m.inner)
        }
    })
}

/// Parses the floor operator
///
/// EBNF:
/// "⌊" <tensor> "⌋"
#[kalt_macros::parser]
pub fn floor<'data>(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    span(parser)
        .delimited_by(whitespaces(), whitespaces())
        .delimited_by(character(SYMBOL_floor_l), character(SYMBOL_floor_r))
        .map(|t| floor_t(t?))
}
