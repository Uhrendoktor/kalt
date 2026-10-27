use chumsky::{
    primitive::choice,
    select,
    span::{Span, Spanned, WrappingSpan},
};
use sertyp::{
    Content, TypstError,
    chumsky::{Token, parser::{character, word}},
    math::Op,
};

/// Generates an element-wise binary tensor operation and its Pratt application.
#[macro_export]
macro_rules! element_wise_binary_operator {
    ($name:ident, $symbol:expr, $operator_type:ty, $operation:expr) => {
        pub fn $name<'data>(
            lhs: chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>,
            rhs: chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>,
        ) -> $crate::Expects<'data, $crate::parser::atoms::tensor::Tensor> {
            $crate::parser::ops::element_wise::element_wise_binary(
                $operation,
                $crate::parser::ops::element_wise::validate_same_shape(|lhs, rhs| {
                    sertyp::content!(sertyp::sequence![lhs, $symbol, rhs])
                }),
                lhs,
                rhs,
            )
        }

        $crate::pratt_infix!($name, $operator_type => |_op: $operator_type, lhs: chumsky::span::Spanned<_>, rhs: chumsky::span::Spanned<_>| {
            use chumsky::span::{Span, WrappingSpan};
            Ok(lhs.span.union(rhs.span).make_wrapped($name(lhs, rhs)?))
        });
    };
}
pub(crate) use element_wise_binary_operator;

pub mod abs;
pub mod add;
pub mod binom;
pub mod bitwise;
pub mod boolean_dot;
pub mod ceil;
pub mod conjugate;
pub mod cross;
pub mod div;
pub mod dot;
pub mod factorial;
pub mod floor;
pub mod func;
pub mod index;
pub mod logic;
pub mod mul;
pub mod modulo;
pub mod pow;
pub mod root;
pub mod round;
pub mod sign;
pub mod sub;
pub mod transpose;

/// Consumes a two-character operator alias, leaving its meaning to the caller.
#[kalt_macros::parser]
pub fn composed_operator(first: char, second: char) -> (char, char) {
    character(first).then(character(second))
}

/// Generates a pratt infix parser that combines two tensors using the specified operation.
///
/// Example:
/// ```
/// pratt_infix!(add => |op, lhs, rhs| ...);
/// ```
#[macro_export]
macro_rules! pratt_infix {
    ($name:ident, $operator_type:ty => $map:expr) => {
        paste::paste! {
            #[doc = concat!("Application of pratt parser for operation:", stringify!($name),)]
            pub fn [<pratt_ $name>]<'this, 'data: 'this, I: sertyp::chumsky::LocatingSequenceLike<'this, 'data>>(
                lhs: $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>>,
                op: $operator_type,
                rhs: $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>>,
                _extra: &mut chumsky::input::MapExtra<'this, '_, I, $crate::parser::ParserError<'data>>,
            ) -> $crate::Expects<'data, chumsky::span::SimpleSpanned<$crate::parser::atoms::tensor::Tensor>> {
                #[allow(clippy::needless_question_mark)]
                ($map)(op, lhs?, rhs?)
            }
        }
    };
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
            $crate::parser::ops::element_wise::element_wise_binary(
                |lhs, rhs| Ok(lhs.inner $op rhs.inner),
                $crate::parser::ops::element_wise::validate_same_shape(|lhs, rhs| {
                    sertyp::sequence![lhs, stringify!($op), rhs].into()
                }),
                t1,
                t2,
            )
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

pub mod element_wise;

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
