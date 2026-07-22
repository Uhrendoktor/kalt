use chumsky::{
    primitive::choice,
    select,
    span::{Span, Spanned, WrappingSpan},
};
use sertyp::{
    Content, TypstError,
    chumsky::{Token, parser::word},
    math::Op,
};

pub mod abs;
pub mod add;
pub mod binom;
pub mod ceil;
pub mod conjugate;
pub mod cross;
pub mod div;
pub mod dot;
pub mod factorial;
pub mod floor;
pub mod func;
pub mod index;
pub mod mul;
pub mod pow;
pub mod root;
pub mod round;
pub mod sign;
pub mod sub;
pub mod transpose;

/// Generates a pratt infix parser that combines two tensors using the specified operation.
///
/// Example:
/// ```
/// pratt_infix!(add => |op, lhs, rhs| ...);
/// ```
#[macro_export]
macro_rules! pratt_infix {
    ($name:ident => $map:expr) => {
        paste::paste! {
            #[doc = concat!("Application of pratt parser for operation:", stringify!($name),)]
            pub fn [<pratt_ $name>]<'this, 'data: 'this, I: sertyp::chumsky::LocatingSequenceLike<'this, 'data>>(
                lhs: $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>>,
                op: char,
                rhs: $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>>,
                _extra: &mut chumsky::input::MapExtra<'this, '_, I, $crate::parser::ParserError<'data>>,
            ) -> $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>> {
                #[allow(clippy::needless_question_mark)]
                ($map)(op, lhs?, rhs?)
            }
        }
    };
}

/// Generates a pratt postfix parser that combines two tensors using the specified operation.
///
/// Example:
/// ```
/// pratt_postfix!(factorial => |op, lhs| ...);
/// ```
#[macro_export]
macro_rules! pratt_postfix {
    ($name:ident => $map:expr) => {
        paste::paste! {
            #[doc = concat!("Application of pratt parser for operation:", stringify!($name),)]
            pub fn [<pratt_ $name>]<'this, 'data: 'this, I: sertyp::chumsky::LocatingSequenceLike<'this, 'data>>(
                lhs: $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>>,
                op: char,
                _extra: &mut chumsky::input::MapExtra<'this, '_, I, $crate::parser::ParserError<'data>>,
            ) -> $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>> {
                #[allow(clippy::needless_question_mark)]
                ($map)(op, lhs?)
            }
        }
    };
}

/// Generates a pratt postfix parser that combines two tensors using the specified operation.
///
/// Example:
/// ```
/// pratt_prefix!(operation => |op, lhs| ...);
/// ```
#[macro_export]
macro_rules! pratt_prefix {
    ($name:ident => $map:expr) => {
        paste::paste! {
            #[doc = concat!("Application of pratt parser for operation:", stringify!($name),)]
            pub fn [<pratt_ $name>]<'this, 'data: 'this, I: sertyp::chumsky::LocatingSequenceLike<'this, 'data>>(
                op: char,
                rhs: $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>>,
                _extra: &mut chumsky::input::MapExtra<'this, '_, I, $crate::parser::ParserError<'data>>,
            ) -> $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>> {
                #[allow(clippy::needless_question_mark)]
                ($map)(op, rhs?)
            }
        }
    };
}

#[macro_export]
macro_rules! match_tensor {
    (($t:expr) => {
        s => $s:expr,
        m => $m:expr
    }) => {{
        use chumsky::span::{SimpleSpanned, WrappingSpan};
        use $crate::parser::atoms::tensor::Tensor;
        match $t {
            SimpleSpanned {
                inner: Tensor::Scalar(s),
                span: sp,
            } => ($s)(sp.make_wrapped(s)).map(From::from),
            SimpleSpanned {
                inner: Tensor::Matrix(m),
                span: sp,
            } => ($m)(sp.make_wrapped(m)).map(From::from),
        }
    }};
}

#[macro_export]
macro_rules! match_tensors {
    (($t1:expr, $t2:expr) => {
        (s1, s2) => $ss:expr,
        (s, m) => $sm:expr,
        (m, s) => $ms:expr,
        (m1, m2) => $mm:expr$(,)?
    }) => {{
        use chumsky::span::{SimpleSpanned, WrappingSpan};
        use $crate::parser::atoms::tensor::Tensor;
        match ($t1, $t2) {
            (
                SimpleSpanned {
                    inner: Tensor::Scalar(s1),
                    span: sp1,
                },
                SimpleSpanned {
                    inner: Tensor::Scalar(s2),
                    span: sp2,
                },
            ) => (($ss)(sp1.make_wrapped(s1), sp2.make_wrapped(s2))).map(From::from),
            (
                SimpleSpanned {
                    inner: Tensor::Scalar(s),
                    span: sps,
                },
                SimpleSpanned {
                    inner: Tensor::Matrix(m),
                    span: spm,
                },
            ) => (($sm)(sps.make_wrapped(s), spm.make_wrapped(m))).map(From::from),
            (
                SimpleSpanned {
                    inner: Tensor::Matrix(m),
                    span: spm,
                },
                SimpleSpanned {
                    inner: Tensor::Scalar(s),
                    span: sps,
                },
            ) => (($ms)(spm.make_wrapped(m), sps.make_wrapped(s))).map(From::from),
            (
                SimpleSpanned {
                    inner: Tensor::Matrix(m1),
                    span: sp1,
                },
                SimpleSpanned {
                    inner: Tensor::Matrix(m2),
                    span: sp2,
                },
            ) => ($mm)(sp1.make_wrapped(m1), sp2.make_wrapped(m2)).map(From::from),
        }
    }};
}

/// Generates a function that applies an operation to two tensors for basic operations
///
/// Usage:
/// ```rust
/// add!(add: +);
/// ```
/// where all custom functions are optional. If not provided, the default operation implementation will be used using the defined operator symbol.
/// Additionally, after the function name, a `?` can be added to indicate that the function returns an [Expects].
#[macro_export]
macro_rules! operation_element_wise_same_shape {
    ($name:ident: $op:tt) => {
        #[doc = concat!("Operation ", stringify!($name), "for two tensors:")]
        /// (scalar, scalar) -> scalar
        /// (scalar, matrix) -> matrix (element-wise)
        /// (matrix, scalar) -> matrix (element-wise)
        /// (matrix, matrix) -> matrix (element-wise) only if same shape
        pub fn $name<'data>(t1: chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>, t2: chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>) -> $crate::Expects<'data, $crate::parser::atoms::tensor::Tensor> {
            use $crate::parser::atoms::matrix::Matrix;
            $crate::match_tensors!((t1, t2) => {
                (s1, s2) => |s1: Spanned<num::Complex<f64>>, s2: Spanned<num::Complex<f64>>| Ok(*s1 $op *s2),
                (s, m) => |s: Spanned<num::Complex<f64>>, m: Spanned<Matrix>| Ok(*s $op &*m),
                (m, s) => |m: Spanned<Matrix>, s: Spanned<num::Complex<f64>>| Ok(&*m $op *s),
                (m1, m2) => |m1: Spanned<Matrix>, m2: Spanned<Matrix>| {
                    $crate::parser::ops::element_wise::validate_same_shape(|m1, m2| {
                        sertyp::sequence![
                            m1,
                            stringify!($op),
                            m2,
                        ]
                        .into()
                    })(m1.span.make_wrapped(&m1.inner), m2.span.make_wrapped(&m2.inner))?;
                    Ok(&*m1 $op &*m2)
                }
            })
        }
    };
}

pub fn map_spanned<I, O>(t: Spanned<I>, f: fn(I) -> O) -> Spanned<O> {
    t.span.make_wrapped(f(t.inner))
}

/// Transposes Spanned<Result<T, E>, S> -> Result<Spanned<T, S>, E>
pub fn transpose<T, E, S: Span + WrappingSpan<T, Spanned = Spanned<T, S>>>(
    s: Spanned<Result<T, E>, S>,
) -> Result<Spanned<T, S>, E> {
    match s.inner {
        Ok(t) => Ok(s.span.make_wrapped(t)),
        Err(e) => Err(e),
    }
}

pub mod element_wise {
    use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
    use sertyp::{Content, SYMBOL_CC, TypstError, content};

    use crate::{
        Expects, matrix_shape,
        parser::atoms::{matrix::Matrix, tensor::Tensor},
    };

    pub fn element_wise<
        'data,
        F: Fn(SimpleSpanned<num::Complex<f64>>) -> Expects<'data, num::Complex<f64>>,
        MC: Fn(SimpleSpanned<&Matrix>) -> Expects<'data, ()>,
    >(
        f: F,
        matrix_condition: MC,
        t: SimpleSpanned<Tensor>,
    ) -> Expects<'data, Tensor> {
        match_tensor!((t) => {
            s => &f,
            m => |mut m: Spanned<Matrix>| {
                let span = m.span;
                matrix_condition(span.make_wrapped(&m.inner))?;
                m.iter_mut().try_for_each(|x| {
                    f(span.make_wrapped(*x))?;
                    Ok::<_, TypstError>(())
                })?;
                Ok(m.inner)
            }
        })
    }

    pub fn element_wise2<
        'data,
        F: Fn(
            SimpleSpanned<num::Complex<f64>>,
            SimpleSpanned<num::Complex<f64>>,
        ) -> Expects<'data, num::Complex<f64>>,
        MC: Fn(SimpleSpanned<&Matrix>, SimpleSpanned<&Matrix>) -> Expects<'data, ()>,
    >(
        f: F,
        matrix_condition: MC,
        t1: SimpleSpanned<Tensor>,
        t2: SimpleSpanned<Tensor>,
    ) -> Expects<'data, Tensor> {
        let sm = |s, mut m: Spanned<Matrix>| {
            let span = m.span;
            m.iter_mut().try_for_each(|x| {
                f(s, span.make_wrapped(*x))?;
                Ok::<_, TypstError>(())
            })?;
            Ok(m.inner)
        };
        match_tensors!((t1, t2) => {
            (s1, s2) => &f,
            (s, m) => sm,
            (m, s) => |m, s| sm(s, m),
            (m1, m2) => |mut m1: Spanned<Matrix>, m2: Spanned<Matrix>|{
                matrix_condition(m1.span.make_wrapped(&m1), m2.span.make_wrapped(&m2))?;
                let sp1 = m1.span;
                let sp2 = m2.span;
                m1.iter_mut().zip(m2.iter()).try_for_each(|(x, y)| {
                    *x = f(sp1.make_wrapped(*x), sp2.make_wrapped(*y))?;
                    Ok::<_, TypstError>(())
                })?;
                Ok(m1.inner)
            }
        })
    }

    /// Validates that two matrices have the same shape and returns a formattable matrix shape error if they do not.
    ///
    /// Args:
    /// - f: A function that is sipposed to format the operation of the two matrices.
    ///   It receives two formatted matrix dimensions as input.
    pub fn validate_same_shape<'data>(
        f: impl Fn(Content<'data>, Content<'data>) -> Content<'data>,
    ) -> impl Fn(SimpleSpanned<&Matrix>, SimpleSpanned<&Matrix>) -> Expects<'data, ()> {
        move |m1, m2| {
            if m1.shape() == m2.shape() {
                return Ok(());
            }
            Err(TypstError::full(
                m1.span.union(m2.span),
                "Binomial coefficient can only be applied to matrices of the same shape",
                f(
                    content!(
                        matrix_shape!({SYMBOL_CC} ^ ({m1.dim().0.to_string()} x {m1.dim().1.to_string()}))
                    ),
                    content!(
                        matrix_shape!({SYMBOL_CC} ^ ({m1.dim().0.to_string()} x {m1.dim().1.to_string()}))
                    ),
                ),
                f(
                    content!(
                        matrix_shape!({SYMBOL_CC} ^ ({m1.dim().0.to_string()} x {m1.dim().1.to_string()}))
                    ),
                    content!(
                        matrix_shape!({SYMBOL_CC} ^ ({m2.dim().0.to_string()} x {m2.dim().1.to_string()}))
                    ),
                ),
            ))
        }
    }

    /// Validates that a matrix is square and returns a formattable matrix shape error if it is not.
    ///
    /// Args:
    /// - f: A function that is sipposed to format the operation of the matrix.
    ///   It receives a formatted matrix's dimensions as input.
    pub fn validate_square<'data>(
        f: impl Fn(Content<'data>) -> Content<'data>,
    ) -> impl Fn(SimpleSpanned<&Matrix>) -> Expects<'data, ()> {
        move |m| {
            if m.is_square() {
                return Ok(());
            }
            Err(TypstError::full(
                m.span,
                "Matrix must be square",
                f(content!(matrix_shape!({SYMBOL_CC} ^ ({'N'} x {'N'})))),
                f(content!(
                    matrix_shape!({SYMBOL_CC} ^ ({m.dim().0.to_string()} x {m.dim().1.to_string()}))
                )),
            ))
        }
    }
}

/// Parses a specific keyword as text or math operator.
///
/// EBNF:
/// "<keyword>"
#[kalt_macros::parser]
fn word_or_op(name: &'static str) -> String {
    choice((
        word(name),
        select!(Token::Raw(Content::MathOp(Op { text, .. })) => &***text).try_map(
            move |text, span| match text {
                Content::Text(text) if &**text.text == name => Ok(text.text.to_string()),
                _ => Err(TypstError::full(
                    span,
                    "Unexpected keyword",
                    name,
                    text.clone(),
                )),
            },
        ),
    ))
}
