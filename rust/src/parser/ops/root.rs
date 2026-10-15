use chumsky::{
    select,
    span::{SimpleSpan, Spanned, WrappingSpan},
};
use num::{Zero, complex::ComplexFloat};
use sertyp::{
    Content, LocatingSequence, SYMBOL_CC, SYMBOL_eq_not, SYMBOL_in, TypstError, chumsky::Token,
    content, math::Root, sequence,
};

use crate::{
    Expects, match_tensors,
    parser::{
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::element_wise::validate_same_shape,
        pratt::pratt,
    },
    subscript,
};

/// Computes root(c1, c2) for complex numbers
pub fn root_c(radicand: &num::Complex<f64>, index: &num::Complex<f64>) -> num::Complex<f64> {
    let mut radicand = *radicand;
    // required to return more expected result for e.g. sqrt(-1) = +- i
    if radicand.im().is_zero() {
        radicand.im = 0.0;
    }
    if *index == num::Complex::new(2.0, 0.0) {
        // extra case since for more optimized performance
        return radicand.sqrt();
    }
    radicand.powc(1.0 / *index)
}

fn validate_index<'data>(index: &Spanned<num::Complex<f64>>) -> Expects<'data, ()> {
    if index.is_zero() {
        return Err(TypstError::full(
            index.span,
            "root index cannot be zero",
            content!(sequence!(
                Root {
                    index: Some(content!("r").into()),
                    radicand: content!("x").into()
                },
                "| r",
                SYMBOL_in,
                subscript!({SYMBOL_CC}_{sequence!(SYMBOL_eq_not, "0")})
            )),
            content!(Root {
                index: Some(content!("0").into()),
                radicand: content!("x").into()
            }),
        ));
    }
    Ok(())
}

/// Applies root to two tensors:
/// scalar root scalar -> scalar
/// scalar root matrix -> matrix (element-wise)
/// matrix root scalar -> matrix (element-wise)
/// matrix root matrix -> matrix (element-wise) only if same shape
pub fn root_t<'data>(radicand: Spanned<Tensor>, index: Spanned<Tensor>) -> Expects<'data, Tensor> {
    match_tensors!((radicand, index) => {
        (s1, s2) => |s1: Spanned<num::Complex<f64>>, s2: Spanned<num::Complex<f64>>| Ok(root_c(&s1, &s2)),
        (s, m) => |s: Spanned<num::Complex<f64>>, mut m: Spanned<Matrix>| {
            m.iter().try_for_each(|v| validate_index(&m.span.make_wrapped(*v)))?;
            m.inner.mapv_inplace(|el| root_c(&s, &el));
            Ok(m.inner)
        },
        (m, s) => |mut m: Spanned<Matrix>, s: Spanned<num::Complex<f64>>| {
            validate_index(&s)?;
            m.inner.mapv_inplace(|el| root_c(&el, &s));
            Ok(m.inner)
        },
        (m1, m2) => |mut m1: Spanned<Matrix>, m2: Spanned<Matrix>| {
            validate_same_shape(|c1, c2| content!(Root{ index: Some(c2.into()), radicand: c1.into() }))(m1.span.make_wrapped(&m1.inner), m2.span.make_wrapped(&m2.inner))?;
            m1.iter_mut().zip(m2.iter()).try_for_each(|(x, y)| {
                validate_index(&m2.span.make_wrapped(*y))?;
                *x = root_c(x, y);
                Ok::<_, TypstError>(())
            })?;
            Ok(m1.inner)
        },
    })
}

/// Parses a typst `attach` and tries to apply root to the radicand and index.
#[kalt_macros::parser]
pub fn root<'data>() -> Expects<'data, Tensor> {
    select!(Token::Raw(Content::MathRoot(Root{ index, radicand })) => (index, radicand)).map_with(
        |(index, radicand), extra| {
            let span: SimpleSpan = extra.span();
            let radicand = pratt::<LocatingSequence>()
                .parse(LocatingSequence::from(&***radicand))
                .into_result()?
                .map(|t| span.make_wrapped(t));
            let index = span.make_wrapped(
                index
                    .as_ref()
                    .map(|index| {
                        pratt::<LocatingSequence>()
                            .parse(LocatingSequence::from(&***index))
                            .into_result()?
                    })
                    .transpose()?
                    .unwrap_or(Tensor::Scalar(num::Complex::new(2f64, 0f64))),
            );
            root_t(radicand?, index)
        },
    )
}
