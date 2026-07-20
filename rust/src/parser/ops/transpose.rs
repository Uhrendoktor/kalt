use chumsky::{primitive::choice, span::Spanned};
use sertyp::{SYMBOL_top, TypstError, chumsky::parser::character};

use crate::{
    Expects, match_tensor,
    parser::{
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::{self, pow::pow_parser},
        pratt::pratt,
    },
};

/// Applies transpose to a tensor:
/// scalar -> invalid
/// matrix -> matrix (transpose)
pub fn transpose_t<'data>(t: Spanned<Tensor>) -> Expects<'data, Tensor> {
    match_tensor!((t) => {
        s => |_s| Err(TypstError::full(
            t.span,
            "Cannot transpose a scalar",
            "scalar",
            "matrix",
        )),
        m => |mut m: Spanned<Matrix>| {
            m.reverse_axes();
            Ok(m.inner)
        }
    })
}

/// Parses a tensor exponent
#[kalt_macros::parser]
pub fn transpose_exponent() -> char {
    choice((character(SYMBOL_top), character('T')))
}

/// Parses a typst `attach` and tries to apply matrix transpose
#[kalt_macros::parser]
pub fn transpose<'data>() -> Expects<'data, Tensor> {
    pow_parser(pratt(), transpose_exponent()).map(|(t, _)| transpose_t(ops::transpose(t)?))
}
