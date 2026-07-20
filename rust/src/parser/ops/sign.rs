use chumsky::{extra::Full, input::MapExtra, span::Spanned};
use sertyp::{
    SYMBOL_minus, TypstError,
    chumsky::{LocatingSequenceLike, parser::sign},
};

use crate::{Expects, parser::atoms::tensor::Tensor};

/// Applies negation to a tensor:
/// (scalar) -> scalar
/// (matrix) -> matrix (element-wise)
pub fn neg_t(t: &Tensor) -> Tensor {
    match t {
        Tensor::Scalar(s) => Tensor::Scalar(-s),
        Tensor::Matrix(m) => Tensor::Matrix(-m),
    }
}

/// Condition for the pratt parser to apply.
#[kalt_macros::parser]
pub fn pratt_sign_operator() -> char {
    sign()
}

/// Application of pratt parser for negation of a tensor
pub fn pratt_sign<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
    op: char,
    rhs: Expects<'data, Spanned<Tensor>>,
    _extra: &mut MapExtra<'this, '_, I, Full<TypstError<'data>, (), ()>>,
) -> Expects<'data, Spanned<Tensor>> {
    rhs.map(|mut rhs| {
        rhs.span.start -= 1;
        if [SYMBOL_minus, '-'].contains(&op) {
            rhs.inner = neg_t(&rhs.inner);
        }
        rhs
    })
}
