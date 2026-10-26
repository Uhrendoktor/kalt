use crate::{
    parser::ops::{self, pow::pow_parser},
    pow,
};
use chumsky::{primitive::choice, span::Spanned};
use sertyp::{
    SYMBOL_ast_basic, SYMBOL_ast_op, SYMBOL_dagger, TypstError, chumsky::parser::character,
    content, sequence,
};

use crate::{
    Expects,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::element_wise,
        pratt::pratt,
    },
};

/// Applies conjugate to a tensor:
/// scalar -> scalar
/// matrix -> matrix (element-wise)
pub fn conjugate_t<'data>(t: Spanned<Tensor>, op: char) -> Expects<'data, Tensor> {
    let span = t.span;
    element_wise(
        |value: Spanned<num::Complex<f64>>| match op {
            #[allow(non_upper_case_globals)]
            SYMBOL_ast_op | SYMBOL_ast_basic => Ok(value.inner.conj()),
            op => Err(TypstError::full(
                span,
                sequence!["Cannot conjugate a scalar using this operator. Try using", SYMBOL_ast_op, SYMBOL_ast_basic, "for scalars."],
                "matrix",
                content!(pow!({"scalar"} ^ {op})),
            )),
        },
        |_| Ok(()),
        t,
    )
}

/// Parses a tensor exponent
#[kalt_macros::parser]
pub fn conjugate_exponent() -> char {
    choice((
        character(SYMBOL_ast_op),
        character(SYMBOL_ast_basic),
        character('H'),
        character(SYMBOL_dagger),
    ))
}

/// Parses a typst `attach` and tries to apply matrix conjugate
#[kalt_macros::parser]
pub fn conjugate<'data>() -> Expects<'data, Tensor> {
    pow_parser(pratt, conjugate_exponent).map(|(t, op)| conjugate_t(ops::transpose(t)?, *op))
}
