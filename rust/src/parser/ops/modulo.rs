use chumsky::{Parser, primitive::choice, span::{SimpleSpanned, Span, Spanned, WrappingSpan}};
use num::Complex;
use sertyp::{SYMBOL_eq_not, TypstError, chumsky::parser::character, content, equation};

use crate::{
    Expects, pratt_infix,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::{element_wise2, validate_same_shape},
    },
};

/// Parses the infix modulo operator.
#[kalt_macros::parser]
pub fn pratt_modulo_operator() -> char {
    choice((
        character('%'),
        super::word_or_op("mod").map(|_| '%'),
    ))
}

fn modulo_c<'data>(
    lhs: SimpleSpanned<Complex<f64>>,
    rhs: SimpleSpanned<Complex<f64>>,
) -> Expects<'data, Complex<f64>> {
    let span = lhs.span.union(rhs.span);

    if rhs.inner.re == 0.0 && rhs.inner.im == 0.0 {
        return Err(TypstError::full(
            span,
            "Modulo Error",
            equation!["b", SYMBOL_eq_not, '0'],
            equation![rhs.inner.to_string(), '=', '0'],
        ));
    }

    // Match Wolfram Language: Mod[a, b] = a - b Floor[a / b].
    // Floor is applied independently to the real and imaginary parts.
    let ratio = lhs.inner / rhs.inner;
    let quotient = Complex::new(ratio.re.floor(), ratio.im.floor());
    Ok(lhs.inner - quotient * rhs.inner)
}

/// Applies modulo element-wise to real or complex scalar/tensor operands.
pub fn modulo<'data>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    element_wise2(
        modulo_c,
        validate_same_shape(|lhs, rhs| content!(sertyp::sequence![lhs, "%", rhs])),
        lhs,
        rhs,
    )
}

pratt_infix!(modulo => |_op, lhs: Spanned<_>, rhs: Spanned<_>| {
    Ok(lhs.span.union(rhs.span).make_wrapped(modulo(lhs, rhs)?))
});
