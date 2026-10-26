use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use num::Complex;
use sertyp::{TypstError, chumsky::parser::character, content};

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
    character('%')
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
            content!("a non-zero divisor"),
            content!("a zero divisor"),
        ));
    }

    // Wolfram Language defines Mod[a, b] as a - b Floor[a / b].
    // Its Floor applies independently to real and imaginary components.
    if lhs.inner.im == 0.0 && rhs.inner.im == 0.0 {
        // Use Euclidean remainder for numerical stability, then match Wolfram's
        // convention that a real remainder has the sign of the divisor.
        let remainder = lhs.inner.re.rem_euclid(rhs.inner.re.abs());
        let remainder = if rhs.inner.re < 0.0 && remainder != 0.0 {
            remainder - rhs.inner.re.abs()
        } else {
            remainder
        };
        return Ok(Complex::new(remainder, 0.0));
    }

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
