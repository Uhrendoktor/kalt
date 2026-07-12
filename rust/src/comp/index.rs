use chumsky::IterParser;
use chumsky::{Parser, primitive::choice, span::SimpleSpanned};
use sertyp::chumsky::parser::whitespaces;
use sertyp::{
    TypstError,
    chumsky::{
        LocatingSequenceLike,
        parser::{character, delimited_by_groups},
    },
};

use crate::comp::pure::Error;
use crate::comp::tensor::{Tensor, real, real_int, scalar, validate};

#[derive(Default, Debug, Clone)]
pub struct Range {
    pub start: Option<SimpleSpanned<i64>>,
    pub stop: Option<SimpleSpanned<i64>>,
    pub step: Option<SimpleSpanned<i64>>,
}

fn validate_real_int<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
    parser: impl Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>>,
) -> impl Parser<'this, I, Result<i64, TypstError<'data>>, Error<'data>> {
    validate(validate(validate(parser, scalar), real), real_int)
}

pub fn range<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
    pratt: impl 'this + Clone + Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>>,
) -> impl Parser<'this, I, Result<Range, TypstError<'data>>, Error<'data>> {
    let s = || validate_real_int(pratt.clone()).spanned().or_not();

    (s().then_ignore(character(':')).then(s()).then(
        character(':')
            .ignored()
            .then(s())
            .map(|(_, s)| s)
            .or_not()
            .map(|step| step.flatten()),
    ))
    .map(|((start_t, stop_t), step_t)| {
        Ok(Range {
            start: start_t.map(map_spanned).transpose()?,
            stop: stop_t.map(map_spanned).transpose()?,
            step: step_t.map(map_spanned).transpose()?,
        })
    })
}

fn map_spanned<T, E>(s: SimpleSpanned<Result<T, E>>) -> Result<SimpleSpanned<T>, E> {
    match s.inner {
        Ok(t) => Ok(SimpleSpanned {
            inner: t,
            span: s.span,
        }),
        Err(e) => Err(e),
    }
}

pub fn indices<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
    pratt: impl 'this + Clone + Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>>,
) -> impl Parser<'this, I, Result<Vec<SimpleSpanned<i64>>, TypstError<'data>>, Error<'data>> {
    choice((
        delimited_by_groups(
            validate_real_int(pratt.clone())
                .spanned()
                .map(map_spanned)
                .separated_by(character(','))
                .at_least(1)
                // needs to be fully collected first to ensure that the error does
                // not bleed into the parser flow by aborting seperated_by too early
                .collect::<Vec<Result<_, _>>>()
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .delimited_by(character('['), character(']')),
        ),
        validate_real_int(pratt)
            .spanned()
            .map(|f| map_spanned(f).map(|f| vec![f])),
    ))
}

#[derive(Debug, Clone)]
pub enum AxisIndex {
    Range(Range),
    Indices(Vec<SimpleSpanned<i64>>),
}
pub fn axes_index<'this, 'data: 'this, const AXES: usize, I: LocatingSequenceLike<'this, 'data>>(
    pratt: impl 'this + Clone + Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>>,
) -> impl Parser<'this, I, Result<[AxisIndex; AXES], TypstError<'data>>, Error<'data>> {
    delimited_by_groups(
        choice((
            range(pratt.clone()).map(|r| r.map(AxisIndex::Range)),
            indices(pratt).map(|i| i.map(AxisIndex::Indices)),
        ))
        .delimited_by(whitespaces(), whitespaces())
        .separated_by(character(','))
        .exactly(AXES)
        // needs to be fully collected first to ensure that the error does
        // not bleed into the parser flow by aborting seperated_by too early
        .collect::<Vec<Result<_, _>>>()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map(|v| {
            v.map(|v: Vec<_>| {
                v.try_into()
                    .map_err(|_| ())
                    .expect("'exactly' should have prevented this")
            })
        })
        .delimited_by(character('['), character(']')),
    )
}
