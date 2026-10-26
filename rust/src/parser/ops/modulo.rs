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

    if lhs.inner.im != 0.0 || rhs.inner.im != 0.0 {
        return Err(TypstError::full(
            span,
            "Modulo Error",
            content!("real operands"),
            content!("complex operands"),
        ));
    }

    if rhs.inner.re == 0.0 {
        return Err(TypstError::full(
            span,
            "Modulo Error",
            content!("a non-zero divisor"),
            content!("a zero divisor"),
        ));
    }

    Ok(Complex::new(lhs.inner.re.rem_euclid(rhs.inner.re), 0.0))
}

/// Applies Euclidean modulo to real scalar or tensor operands element-wise.
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
