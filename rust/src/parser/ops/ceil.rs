use chumsky::{Parser, span::Spanned};
use sertyp::{
    SYMBOL_ceil_l, SYMBOL_ceil_r,
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

/// ceils a tensor:
/// scalar -> scalar (norm)
/// matrix -> scalar (element-wise)
pub fn ceil_t<'data>(t: Spanned<Tensor>) -> Expects<'data, Tensor> {
    match_tensor!((t) => {
        s => |s: Spanned<num::Complex<f64>>| Ok(num::Complex::<f64>::new(s.re.ceil(), s.im.ceil())),
        m => |mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(|el| num::Complex::<f64>::new(el.re.ceil(), el.im.ceil()));
            Ok(m.inner)
        }
    })
}

/// Parses the ceiling operator
///
/// EBNF:
/// "⌈" <tensor> "⌉"
#[kalt_macros::parser]
pub fn ceil<'data>(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    span(parser)
        .delimited_by(whitespaces(), whitespaces())
        .delimited_by(character(SYMBOL_ceil_l), character(SYMBOL_ceil_r))
        .map(|t| ceil_t(t?))
}
