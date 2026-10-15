use chumsky::{
    Parser, select,
    span::{SimpleSpan, Spanned, WrappingSpan},
};
use sertyp::{
    Content, LocatingSequence, SYMBOL_CC, SYMBOL_NN, SYMBOL_RR, SYMBOL_ZZ, TypstError,
    chumsky::{LocatingSequenceLike, Token},
    content,
    math::Attach,
};

use crate::{
    Expects, match_tensors, matrix_shape,
    parser::{
        ParserError,
        atoms::{complex::Complex, matrix::Matrix, tensor::Tensor},
        ops::{element_wise::validate_same_shape, transpose},
        pratt::pratt,
    },
    pow,
};

/// Formats a matrix of shape `d1 x d2` with an exponent `exponent` for typst.
///
/// $group^(d1 x d2)^exponent$
pub fn matrix_shape_exponent<'data>(
    d1: impl Into<Content<'data>>,
    d2: impl Into<Content<'data>>,
    exponent: impl Into<Content<'data>>,
) -> Content<'data> {
    content!(pow!(
        { matrix_shape!({SYMBOL_CC} ^ ({d1} x {d2})) } ^ { exponent.into() }
    ))
}

/// Computes c1^c2 for complex numbers
pub fn pow_c(c1: &num::Complex<f64>, c2: &num::Complex<f64>) -> num::Complex<f64> {
    c1.powc(*c2)
}

/// Computes m^c for a square matrix m and a non-negative integer c
pub fn pow_m<'data>(span: SimpleSpan, m: &Matrix, c: &num::Complex<f64>) -> Expects<'data, Matrix> {
    if !m.is_square() {
        return Err(TypstError::full(
            span,
            "Non square-matrix exponentitiation",
            matrix_shape_exponent('N', 'N', SYMBOL_NN),
            matrix_shape_exponent(
                m.dim().0.to_string(),
                m.dim().1.to_string(),
                content!(Complex::from(*c)),
            ),
        ));
    }
    if c.im != 0.0 {
        return Err(TypstError::full(
            span,
            "Non real square-matrix exponentitiation",
            matrix_shape_exponent('N', 'N', SYMBOL_RR),
            matrix_shape_exponent(
                m.dim().0.to_string(),
                m.dim().1.to_string(),
                content!(Complex::from(*c)),
            ),
        ));
    }
    if c.re.fract() != 0.0 {
        return Err(TypstError::full(
            span,
            "Non integer square-matrix exponentitiation",
            matrix_shape_exponent('N', 'N', SYMBOL_NN),
            matrix_shape_exponent(
                m.dim().0.to_string(),
                m.dim().1.to_string(),
                content!(Complex::from(*c)),
            ),
        ));
    }
    if c.re < 0.0 {
        return Err(TypstError::full(
            span,
            "Negative integer square-matrix exponentitiation",
            matrix_shape_exponent('N', 'N', SYMBOL_ZZ),
            matrix_shape_exponent(
                m.dim().0.to_string(),
                m.dim().1.to_string(),
                content!(Complex::from(*c)),
            ),
        ));
    }
    let n = c.re as u32;
    let mut result = Matrix::eye(m.nrows());
    for _ in 0..n {
        result = result.dot(m);
    }
    Ok(result)
}

/// Applies exponentiation to two tensors:
/// scalar ^ scalar -> scalar
/// scalar ^ matrix -> matrix (element-wise)
/// matrix ^ scalar -> matrix (dot product) only if matrix is square and scalar is non-negative real int
/// matrix ^ matrix -> matrix (element-wise) only if same shape
pub fn pow_t<'data>(t1: Spanned<Tensor>, t2: Spanned<Tensor>) -> Expects<'data, Tensor> {
    match_tensors!((t1, t2) => {
        (s1, s2) => |s1: Spanned<num::Complex<f64>>, s2: Spanned<num::Complex<f64>>| Ok(pow_c(&s1, &s2)),
        (s, m) => |s: Spanned<num::Complex<f64>>, mut m: Spanned<Matrix>| {
            m.inner.mapv_inplace(|el| pow_c(&s, &el));
            Ok(m.inner)
        },
        (m, s) => |m: Spanned<Matrix>, s: Spanned<num::Complex<f64>>| pow_m(m.span, &m, &s),
        (m1, m2) => |mut m1: Spanned<Matrix>, m2: Spanned<Matrix>| {
            validate_same_shape(|c1, c2| content!(pow!({c1} ^ {c2})))(m1.span.make_wrapped(&m1.inner), m2.span.make_wrapped(&m2.inner))?;
            m1.iter_mut().zip(m2.iter()).try_for_each(|(x, y)| {
                *x = pow_c(x, y);
                Ok::<_, TypstError>(())
            })?;
            Ok(m1.inner)
        },
    })
}

pub fn pow_parser<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    OB,
    OE,
    B: Parser<'this, LocatingSequence<'this, 'data>, OB, ParserError<'data>>,
    E: Parser<'this, LocatingSequence<'this, 'data>, OE, ParserError<'data>>,
>(
    base: impl Fn() -> B,
    exponent: impl Fn() -> E,
) -> impl chumsky::Parser<'this, I, (Spanned<OB>, Spanned<OE>), crate::parser::ParserError<'data>> {
    select!(Token::Raw(Content::MathAttach(attach @ Attach{ t: Some(_), ..})) => attach).try_map(
        move |attach, span: SimpleSpan| {
            let base = base()
                .parse(LocatingSequence::from(&**attach.base))
                .into_result()?;
            let exponent = exponent()
                .parse(LocatingSequence::from(&***attach.t.as_ref().unwrap()))
                .into_result()?;
            Ok((span.make_wrapped(base), span.make_wrapped(exponent)))
        },
    )
}

/// Parses a typst `attach` and tries to apply exponentiation to the base and exponent.
#[kalt_macros::parser]
pub fn pow<'data>() -> Expects<'data, Tensor> {
    pow_parser(pratt, pratt).map(|(b, e)| pow_t(transpose(b)?, transpose(e)?))
}
