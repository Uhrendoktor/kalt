use std::ops::Mul;

use chumsky::{
    Parser,
    primitive::choice,
    span::{SimpleSpan, SimpleSpanned, Span},
};
use derive_more::TryUnwrap;
use ndarray::Axis;
use num::{
    Zero,
    complex::{Complex64, ComplexFloat},
};
use sertyp::{
    Content, Item, SYMBOL_CC, SYMBOL_NN, SYMBOL_RR, SYMBOL_ZZ, SYMBOL_ast_basic, SYMBOL_ast_op,
    SYMBOL_compose, SYMBOL_dagger, SYMBOL_dot_op, SYMBOL_eq_not, SYMBOL_in, SYMBOL_times,
    SYMBOL_top, Sequence, TypstError, Underline,
    chumsky::{
        LocatingSequenceLike,
        parser::{Number, character},
    },
    content, equation,
    math::{Attach, Matrix, Vector},
    sequence,
};

use crate::{
    comp::{
        index::{AxisIndex, Range},
        pure::{Error, pratt},
    },
    complex::Complex,
};

#[derive(Clone, Debug, TryUnwrap)]
pub enum Tensor {
    Scalar(Complex64),
    Matrix(ndarray::Array2<Complex64>),
}

impl<'data> From<Tensor> for Content<'data> {
    fn from(tensor: Tensor) -> Self {
        match tensor {
            Tensor::Scalar(s) => content!(Complex::from(s)),
            Tensor::Matrix(m) => match m.dim() {
                (_, 1) => Vector {
                    children: m
                        .column(0)
                        .map(|&s| content!(Complex::from(s)))
                        .to_vec()
                        .into(),
                    ..Vector::default()
                }
                .into(),
                (_, _) => Matrix {
                    rows: m
                        .rows()
                        .into_iter()
                        .map(|row| row.map(|&s| content!(Complex::from(s))).to_vec().into())
                        .collect::<Vec<_>>()
                        .into(),
                    ..Matrix::default()
                }
                .into(),
            },
        }
    }
}

impl<'data> From<Tensor> for Item<'data> {
    fn from(tensor: Tensor) -> Self {
        content!(tensor).into()
    }
}

impl Default for Tensor {
    fn default() -> Self {
        Tensor::Scalar(Complex64::new(0.0, 0.0))
    }
}

pub fn validate<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>, T, O>(
    parser: impl Parser<'this, I, Result<T, TypstError<'data>>, Error<'data>>,
    validator: impl Fn(T, SimpleSpan) -> Result<O, TypstError<'data>>,
) -> impl Parser<'this, I, Result<O, TypstError<'data>>, Error<'data>> {
    parser.map_with(move |v, extra| match v {
        Ok(v) => validator(v, extra.span()),
        Err(e) => Err(e),
    })
}

pub fn matrix<'data>(
    tensor: Tensor,
    span: SimpleSpan,
) -> Result<ndarray::Array2<Complex64>, TypstError<'data>> {
    match tensor {
        Tensor::Scalar(_) => Err(TypstError::full(span, "Type Error", "matrix", "scalar")),
        Tensor::Matrix(m) => Ok(m),
    }
}

pub fn scalar<'data>(tensor: Tensor, span: SimpleSpan) -> Result<Complex64, TypstError<'data>> {
    match tensor {
        Tensor::Scalar(s) => Ok(s),
        Tensor::Matrix(_) => Err(TypstError::full(span, "Type Error", "scalar", "matrix")),
    }
}

pub fn real<'data>(scalar: Complex64, span: SimpleSpan) -> Result<f64, TypstError<'data>> {
    if scalar.im == 0.0 {
        Ok(scalar.re)
    } else {
        Err(TypstError::full(
            span,
            "Real Value Error",
            equation!["f", SYMBOL_in, SYMBOL_RR],
            equation![scalar.to_string(), SYMBOL_in, SYMBOL_CC],
        ))
    }
}

pub fn real_int<'data>(float: f64, span: SimpleSpan) -> Result<i64, TypstError<'data>> {
    if float.fract() == 0.0 {
        Ok(float as i64)
    } else {
        Err(TypstError::full(
            span,
            "Integer Error",
            sequence!["i", SYMBOL_in, SYMBOL_NN],
            sequence![float.to_string(), SYMBOL_in, SYMBOL_RR],
        ))
    }
}

impl std::ops::Add for Tensor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Tensor::Scalar(s1), Tensor::Scalar(s2)) => Tensor::Scalar(s1 + s2),
            (Tensor::Scalar(s), Tensor::Matrix(m)) | (Tensor::Matrix(m), Tensor::Scalar(s)) => {
                Tensor::Matrix(s + m)
            }
            (Tensor::Matrix(m1), Tensor::Matrix(m2)) => Tensor::Matrix(m1 + m2),
        }
    }
}

impl std::ops::Sub for Tensor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Tensor::Scalar(s1), Tensor::Scalar(s2)) => Tensor::Scalar(s1 - s2),
            (Tensor::Scalar(s), Tensor::Matrix(m)) => Tensor::Matrix(s - m),
            (Tensor::Matrix(m), Tensor::Scalar(s)) => Tensor::Matrix(m - s),
            (Tensor::Matrix(m1), Tensor::Matrix(m2)) => Tensor::Matrix(m1 - m2),
        }
    }
}

pub fn matrix_shape<'data>(
    d1: impl Into<Content<'data>>,
    d2: impl Into<Content<'data>>,
) -> Content<'data> {
    content!(Attach {
        t: Some(content!(sequence![d1, SYMBOL_times, d2]).into()),
        base: content!(SYMBOL_CC).into(),
        ..Default::default()
    })
}

pub fn matrix_shapes<'data>(
    d11: impl Into<Content<'data>>,
    d12: impl Into<Content<'data>>,
    d21: impl Into<Content<'data>>,
    d22: impl Into<Content<'data>>,
    symbol: impl Into<Content<'data>>,
) -> Sequence<'data> {
    sequence![matrix_shape(d11, d12), symbol, matrix_shape(d21, d22)].flatten()
}

#[allow(clippy::should_implement_trait)]
impl Tensor {
    pub fn mul<'data, S: Span>(
        self,
        rhs: Self,
    ) -> Result<Self, impl FnOnce(S) -> TypstError<'data, S>> {
        match (self, rhs) {
            (Tensor::Scalar(s1), Tensor::Scalar(s2)) => Ok(Tensor::Scalar(s1 * s2)),
            (Tensor::Scalar(s), Tensor::Matrix(m)) | (Tensor::Matrix(m), Tensor::Scalar(s)) => {
                Ok(Tensor::Matrix(s * m))
            }
            (Tensor::Matrix(m1), Tensor::Matrix(m2)) => {
                if m1.dim().1 != m2.dim().0 {
                    return Err(TypstError::span_later(
                        "Matrix dimension mismatch",
                        matrix_shapes(
                            "N",
                            Underline::underline("M"),
                            Underline::underline("M"),
                            "P",
                            SYMBOL_dot_op,
                        ),
                        matrix_shapes(
                            m1.dim().0.to_string(),
                            Underline::underline(m1.dim().1.to_string()),
                            Underline::underline(m2.dim().0.to_string()),
                            m2.dim().1.to_string(),
                            SYMBOL_dot_op,
                        ),
                    ));
                }
                Ok(Tensor::Matrix(m1.dot(&m2)))
            }
        }
    }
}

impl std::ops::Neg for Tensor {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Tensor::Scalar(s) => Tensor::Scalar(-s),
            Tensor::Matrix(m) => Tensor::Matrix(-m),
        }
    }
}

impl std::ops::Div for Tensor {
    type Output = Result<Self, &'static str>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Tensor::Scalar(s1), Tensor::Scalar(s2)) => Ok(Tensor::Scalar(s1 / s2)),
            (Tensor::Scalar(s), Tensor::Matrix(m)) => Ok(Tensor::Matrix(s / m)),
            (Tensor::Matrix(m), Tensor::Scalar(s)) => Ok(Tensor::Matrix(m / s)),
            (Tensor::Matrix(_m1), Tensor::Matrix(_m2)) => Err(
                "Matrix division is not defined. Consider using the inverse of the second matrix and multiplying.",
            ),
        }
    }
}

pub fn tensor_exponent<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>()
-> impl Parser<'this, I, Result<TensorExponent, TypstError<'data>>, Error<'data>> {
    choice((
        choice((character(SYMBOL_top), character('T'))).map(|c| Ok(TensorExponent::Transpose(c))),
        choice((
            character(SYMBOL_ast_op),
            character(SYMBOL_ast_basic),
            character('H'),
            character(SYMBOL_dagger),
        ))
        .map(|c| Ok(TensorExponent::Conjugate(c))),
        pratt().map(|t| Ok(TensorExponent::Tensor(t?))),
    ))
}

pub enum TensorExponent {
    Tensor(Tensor),
    Transpose(char), // T or ⊤
    Conjugate(char), // * or H or † depending on the base
}

pub fn matrix_shape_exponent<'data>(
    d1: impl Into<Content<'data>>,
    d2: impl Into<Content<'data>>,
    exponent: impl Into<Content<'data>>,
) -> Content<'data> {
    content!(Attach {
        t: Some(content!(exponent.into()).into()),
        base: content!(matrix_shape(d1, d2)).into(),
        ..Default::default()
    })
}

impl Tensor {
    pub fn pow<'data, S: Span>(
        self,
        span: S,
        exponent: TensorExponent,
    ) -> Result<Tensor, TypstError<'data, S>> {
        match self {
            Tensor::Scalar(base) => {
                match exponent {
                    TensorExponent::Tensor(Tensor::Scalar(s)) => {
                        // check undefined cases
                        if (base.is_zero() && s.is_zero())
                            || (base.is_infinite() && s.is_zero())
                            || (base.is_zero() && s.is_infinite())
                            || base.is_nan()
                            || s.is_nan()
                        {
                            return Err(TypstError::full(
                                span,
                                "undefined scalar exponentiation",
                                "",
                                "",
                            ));
                        }
                        Ok(Tensor::Scalar(base.powc(s)))
                    }
                    TensorExponent::Tensor(Tensor::Matrix(m)) => {
                        Ok(Tensor::Matrix(m.mapv_into(|el| base.powc(el))))
                    }
                    #[allow(non_upper_case_globals)]
                    TensorExponent::Conjugate(SYMBOL_ast_op | SYMBOL_ast_basic) => {
                        Ok(Tensor::Scalar(base.conj()))
                    }
                    TensorExponent::Conjugate(c) => Err(TypstError::full(
                        span,
                        "Invalid conjugate operation for scalar",
                        format!("{SYMBOL_ast_basic} or {SYMBOL_ast_op}"),
                        format!("{c}"),
                    )),
                    TensorExponent::Transpose(_) => Err(TypstError::full(
                        span,
                        "Scalars cannot be transposed",
                        "",
                        "",
                    )),
                }
            }
            Tensor::Matrix(base) => {
                fn found<'a>(
                    base: &ndarray::Array2<Complex64>,
                    exponent: Content<'a>,
                ) -> Content<'a> {
                    matrix_shape_exponent(
                        base.dim().0.to_string(),
                        base.dim().1.to_string(),
                        exponent,
                    )
                }
                fn expected<'a>(exponent: char, group: char) -> Content<'a> {
                    content!(
                        sequence![
                            matrix_shape_exponent('N', 'N', exponent),
                            "where",
                            exponent,
                            SYMBOL_in,
                            group,
                        ]
                        .flatten()
                    )
                }
                match exponent {
                    TensorExponent::Tensor(Tensor::Scalar(s)) => {
                        if !base.is_square() {
                            return Err(TypstError::full(
                                span,
                                "Non square-matrix exponentitiation",
                                expected('i', SYMBOL_NN),
                                found(&base, content!(Complex::from(s))),
                            ));
                        }
                        if s.im != 0.0 {
                            return Err(TypstError::full(
                                span,
                                "Non real square-matrix exponentitiation",
                                expected('i', SYMBOL_RR),
                                found(&base, content!(Complex::from(s))),
                            ));
                        }
                        if s.re.fract() != 0.0 {
                            return Err(TypstError::full(
                                span,
                                "Non integer square-matrix exponentitiation",
                                expected('i', SYMBOL_NN),
                                found(&base, content!(Complex::from(s))),
                            ));
                        }
                        if s.re < 0.0 {
                            return Err(TypstError::full(
                                span,
                                "Negative integer square-matrix exponentitiation",
                                expected('i', SYMBOL_ZZ),
                                found(&base, content!(Complex::from(s))),
                            ));
                        }
                        let n = s.re as u32;
                        let mut result = ndarray::Array2::<Complex64>::eye(base.nrows());
                        for _ in 0..n {
                            result = result.dot(&base);
                        }
                        Ok(Tensor::Matrix(result))
                    }
                    TensorExponent::Tensor(Tensor::Matrix(m)) => Err(TypstError::full(
                        span,
                        content!("Matrix exponentiation is not defined"),
                        expected('n', SYMBOL_ZZ),
                        found(
                            &base,
                            matrix_shape(m.dim().0.to_string(), m.dim().1.to_string()),
                        ),
                    )),
                    TensorExponent::Conjugate('H') => {
                        Ok(Tensor::Matrix(base.mapv_into(|el| el.conj())))
                    }
                    TensorExponent::Conjugate(c) => Err(TypstError::full(
                        span,
                        "Invalid conjugate operation for matrix",
                        'H',
                        c,
                    )),
                    TensorExponent::Transpose(_) => Ok(Tensor::Matrix(base.t().to_owned())),
                }
            }
        }
    }
}

impl Tensor {
    pub fn index<'data>(
        &self,
        span: SimpleSpan,
        indices: &[AxisIndex; 2],
    ) -> Result<Tensor, TypstError<'data>> {
        match self {
            Tensor::Scalar(s) => Err(TypstError::full(
                span,
                "Indexing a scalar",
                "matrix",
                Complex::from(*s),
            )),
            Tensor::Matrix(m) => {
                pub fn to_indices<'a>(
                    len: usize,
                    index: &AxisIndex,
                ) -> std::result::Result<Vec<usize>, TypstError<'a>> {
                    let cast_negative = |interval_open: bool| {
                        move |token: &SimpleSpanned<i64>| {
                            let mut x = token.inner;
                            if x < 0 {
                                x += len as i64
                            }
                            if x < 0 {
                                return Err(TypstError::full(
                                    token.span,
                                    "negative index",
                                    equation!(sequence!(
                                        "index",
                                        SYMBOL_in,
                                        format!("[-{}, -1]", len)
                                    )),
                                    (x - len as i64).to_string(),
                                ));
                            }
                            if (interval_open && x >= len as i64)
                                || (!interval_open && x > len as i64)
                            {
                                return Err(TypstError::full(
                                    token.span,
                                    "index out of bounds",
                                    equation!(sequence!(
                                        "index",
                                        SYMBOL_in,
                                        format!(
                                            "[0, {}{}",
                                            len,
                                            if interval_open { ")" } else { "]" }
                                        )
                                    )),
                                    x.to_string(),
                                ));
                            }
                            Ok(x as usize)
                        }
                    };
                    let mut v = vec![];
                    match index {
                        AxisIndex::Range(Range { start, stop, step }) => {
                            let start_v: usize =
                                start.as_ref().map(cast_negative(true)).unwrap_or(Ok(0))?;
                            let stop_v: usize =
                                stop.as_ref().map(cast_negative(false)).unwrap_or(Ok(len))?;

                            if start_v > stop_v {
                                return Err(TypstError::full(
                                    start.as_ref().unwrap().span,
                                    "range start greater than end",
                                    "value smaller than end of range",
                                    equation!(format!("{} > {}", start_v, stop_v)),
                                ));
                            }

                            let step_v: isize =
                                step.as_deref().cloned().map(|n| n.as_isize()).unwrap_or(1);
                            if step_v == 0 {
                                return Err(TypstError::full(
                                    step.as_ref().unwrap().span,
                                    "range step is zero",
                                    equation!(sequence!("step", SYMBOL_eq_not, "0")),
                                    "0",
                                ));
                            }

                            let push_i = |i: usize| v.push(i);
                            // step with step size
                            if step_v > 0 {
                                (start_v..stop_v)
                                    .step_by(step_v.unsigned_abs())
                                    .for_each(push_i);
                            }
                            // reverse if step is negative
                            else {
                                (start_v..stop_v)
                                    .step_by(step_v.unsigned_abs())
                                    .rev()
                                    .for_each(push_i);
                            }
                        }
                        AxisIndex::Indices(indices) => {
                            for index in indices {
                                let index = cast_negative(true)(index)?;
                                v.push(index);
                            }
                        }
                    };
                    Ok(v)
                }
                let rows = m.select(Axis(0), &to_indices(m.dim().0, &indices[0])?);
                let cols = rows.select(Axis(1), &to_indices(m.dim().1, &indices[1])?);
                Ok(Tensor::Matrix(cols))
            }
        }
    }
}

impl Tensor {
    pub fn element_mul<'data, S: Span>(
        &self,
        span: S,
        rhs: &Self,
    ) -> Result<Tensor, TypstError<'data, S>> {
        match (self, rhs) {
            (Tensor::Matrix(a), Tensor::Matrix(b)) => {
                if a.dim() != b.dim() {
                    return Err(TypstError::full(
                        span,
                        "Matrix dimension mismatch for element-wise multiplication",
                        matrix_shapes(
                            a.dim().0.to_string(),
                            a.dim().1.to_string(),
                            a.dim().0.to_string(),
                            a.dim().1.to_string(),
                            SYMBOL_compose,
                        ),
                        matrix_shapes(
                            a.dim().0.to_string(),
                            a.dim().1.to_string(),
                            b.dim().0.to_string(),
                            b.dim().1.to_string(),
                            SYMBOL_compose,
                        ),
                    ));
                }
                Ok(Tensor::Matrix(a.mul(b)))
            }
            _ => Err(TypstError::full(
                span,
                "Element-wise multiplication is only defined for matrices",
                "matrix",
                "scalar",
            )),
        }
    }
}

pub fn log<'data, S: Span>(
    span: S,
    base: Tensor,
    value: Tensor,
) -> Result<Tensor, TypstError<'data, S>> {
    match (base, value) {
        (Tensor::Scalar(base), Tensor::Scalar(value)) => {
            Ok(Tensor::Scalar(value.log10() / base.log10()))
        }
        _ => Err(TypstError::full(
            span,
            "Logarithm is only defined for scalars",
            "scalar",
            "matrix",
        )),
    }
}
