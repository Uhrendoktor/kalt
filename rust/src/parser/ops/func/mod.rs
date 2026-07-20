use chumsky::{
    Parser, select,
    span::{SimpleSpan, Spanned, WrappingSpan},
};
use sertyp::{
    Content, LocatingSequence,
    chumsky::{
        LocatingSequenceLike, Token,
        parser::{character, delimited_by_groups},
    },
    math::Attach,
};

use crate::parser::ParserError;

pub mod ln;
pub mod log;

/// Parses a typst `attach` with specific parsers for base and subscript (b)
pub fn subscript_parser<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>, B, S>(
    base: impl Parser<'this, LocatingSequence<'this, 'data>, B, ParserError<'data>>,
    subscript: impl Parser<'this, LocatingSequence<'this, 'data>, S, ParserError<'data>>,
) -> impl Parser<'this, I, (Spanned<B>, Spanned<S>), ParserError<'data>> {
    select!(Token::Raw(Content::MathAttach(attach @ Attach{ b: Some(_), ..})) => attach).try_map(
        move |attach, span: SimpleSpan| {
            let base = base
                .parse(LocatingSequence::from(&**attach.base))
                .into_result()?;
            let subscript = subscript
                .parse(LocatingSequence::from(&***attach.t.as_ref().unwrap()))
                .into_result()?;
            Ok((span.make_wrapped(base), span.make_wrapped(subscript)))
        },
    )
}

/// Parses a function call with a single argument, e.g. f(x)
pub fn func_parser<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>, T: 'this, F>(
    func: impl 'this + Parser<'this, I, F, ParserError<'data>>,
    parser: impl 'this + Parser<'this, I, T, ParserError<'data>>,
) -> impl Parser<'this, I, (F, T), ParserError<'data>> {
    func.then(delimited_by_groups(
        parser.delimited_by(character('('), character(')')),
    ))
}
