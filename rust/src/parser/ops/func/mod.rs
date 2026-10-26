use chumsky::{
    Parser, select,
    span::{SimpleSpan, Spanned, WrappingSpan},
};
use sertyp::{
    Content, LocatingSequence,
    chumsky::{
        LocatingSequenceLike, Token,
        parser::{character, delimited_by_groups, whitespaces},
    },
    math::Attach,
};

use crate::parser::ParserError;

/// Generates a trigonometric function and its inverse.
macro_rules! trig_function {
    (
        $function_name:ident,
        $description:literal,
        $inverse_description:literal $(,)?
    ) => {
        paste::paste! {
            #[kalt_macros::parser]
            fn [<$function_name _fn>]() -> () {
                $crate::parser::ops::word_or_op(stringify!($function_name)).ignored()
            }

            #[doc = $description]
            #[kalt_macros::parser]
            pub fn $function_name(
                parser: impl 'this + chumsky::Parser<
                    'this,
                    I,
                    $crate::Expects<'data, $crate::parser::atoms::tensor::Tensor>,
                    $crate::parser::ParserError<'data>,
                >,
            ) -> $crate::Expects<'data, $crate::parser::atoms::tensor::Tensor> {
                $crate::parser::ops::func::func_parser(
                    [<$function_name _fn>](),
                    $crate::parser::pratt::span(parser),
                )
                .map(|(_, value)| {
                    $crate::parser::ops::func::trig_common::unary(value?, |value: num::Complex<f64>| {
                        value.$function_name()
                    })
                })
            }

            #[kalt_macros::parser]
            fn [<a $function_name _fn>]() -> () {
                chumsky::primitive::choice((
                    $crate::parser::ops::pow::pow_parser(
                        || $crate::parser::ops::word_or_op(stringify!($function_name)),
                        $crate::parser::pratt::pratt,
                    )
                    .filter(|(_, exponent)| {
                        $crate::parser::ops::func::trig_common::is_negative_one(&exponent.inner)
                    })
                    .ignored(),
                    chumsky::primitive::choice((
                        $crate::parser::ops::word_or_op(concat!("a", stringify!($function_name))),
                        $crate::parser::ops::word_or_op(concat!("arc", stringify!($function_name))),
                    ))
                    .ignored(),
                ))
            }

            #[doc = $inverse_description]
            #[kalt_macros::parser]
            pub fn [<a $function_name>](
                parser: impl 'this + chumsky::Parser<
                    'this,
                    I,
                    $crate::Expects<'data, $crate::parser::atoms::tensor::Tensor>,
                    $crate::parser::ParserError<'data>,
                >,
            ) -> $crate::Expects<'data, $crate::parser::atoms::tensor::Tensor> {
                $crate::parser::ops::func::func_parser(
                    [<a $function_name _fn>](),
                    $crate::parser::pratt::span(parser),
                )
                .map(|(_, value)| {
                    $crate::parser::ops::func::trig_common::unary(value?, |value: num::Complex<f64>| {
                        value.[<a $function_name>]()
                    })
                })
            }
        }
    };
}
pub(crate) use trig_function;

pub mod cos;
pub mod gamma;
pub mod ln;
pub mod log;
pub mod re_im;
pub mod sin;
pub mod tan;
pub mod trig_common;

/// Parses a typst `attach` with specific parsers for base and subscript (b)
pub fn subscript_parser<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    OB,
    OS,
    B: Parser<'this, LocatingSequence<'this, 'data>, OB, ParserError<'data>>,
    S: Parser<'this, LocatingSequence<'this, 'data>, OS, ParserError<'data>>,
>(
    base: impl Fn() -> B,
    subscript: impl Fn() -> S,
) -> impl Parser<'this, I, (Spanned<OB>, Spanned<OS>), ParserError<'data>> {
    select!(Token::Raw(Content::MathAttach(attach @ Attach{ b: Some(_), ..})) => attach).try_map(
        move |attach, span: SimpleSpan| {
            let base = base()
                .parse(LocatingSequence::from(&**attach.base))
                .into_result()?;
            let subscript = subscript()
                .parse(LocatingSequence::from(&***attach.b.as_ref().unwrap()))
                .into_result()?;
            Ok((span.make_wrapped(base), span.make_wrapped(subscript)))
        },
    )
}

/// Parses a function call with a single argument, e.g. f(x)
pub fn func_parser<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    T: 'this,
    F: 'this,
>(
    func: impl 'this + Parser<'this, I, F, ParserError<'data>>,
    parser: impl 'this + Parser<'this, I, T, ParserError<'data>>,
) -> impl Parser<'this, I, (F, T), ParserError<'data>> {
    delimited_by_groups(func)
        .then_ignore(whitespaces())
        .then(delimited_by_groups(
            parser
                .delimited_by(whitespaces(), whitespaces())
                .delimited_by(character('('), character(')')),
        ))
}
