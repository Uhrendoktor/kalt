use chumsky::IterParser;
use chumsky::extra::Full;
use chumsky::input::MapExtra;
use chumsky::span::{SimpleSpan, Span, Spanned, WrappingSpan};
use chumsky::{Parser, primitive::choice, span::SimpleSpanned};
use ndarray::Axis;
use num::ToPrimitive;
use sertyp::chumsky::parser::whitespaces;
use sertyp::{SYMBOL_eq_not, SYMBOL_in, equation, sequence};
use sertyp::{
    TypstError,
    chumsky::{
        LocatingSequenceLike,
        parser::{character, delimited_by_groups},
    },
};

use crate::Expects;
use crate::parser::atoms::matrix::Matrix;
use crate::parser::atoms::tensor::Tensor;
use crate::parser::ops::transpose;
use crate::parser::pratt::span;
use crate::parser::validator::{matrix, real, real_int, scalar};
use crate::parser::{ParserError, validate};

/// Converts negative indices into the correctly wrapped positive index.
///
/// Example:
/// -1 for len = 10 -> 9
pub fn cast_negative<'data>(
    len: usize,
    interval_open: bool,
) -> impl FnOnce(&SimpleSpanned<i64>) -> Expects<'data, usize> {
    move |index: &SimpleSpanned<i64>| {
        let mut x = **index;
        if x < 0 {
            x += len as i64
        }
        if x < 0 {
            return Err(TypstError::full(
                index.span,
                "negative index",
                equation!(sequence!("index", SYMBOL_in, format!("[-{}, -1]", len))),
                (x - len as i64).to_string(),
            ));
        }
        if (interval_open && x >= len as i64) || (!interval_open && x > len as i64) {
            return Err(TypstError::full(
                index.span,
                "index out of bounds",
                equation!(sequence!(
                    "index",
                    SYMBOL_in,
                    format!("[0, {}{}", len, if interval_open { ")" } else { "]" })
                )),
                x.to_string(),
            ));
        }
        Ok(x as usize)
    }
}

/// Converts an AxisIndex into a concrete list of indices
///
/// Example:
/// [1, 1, 2] -> [1, 1, 2]
/// 1:10:2 -> [1, 3, 5, 7, 9]
/// 1::-2 for len = 10 -> [9, 7, 5, 3, 1]
pub fn to_indices<'a>(
    len: usize,
    index: &AxisIndex,
) -> std::result::Result<Vec<usize>, TypstError<'a>> {
    let mut v = vec![];
    match index {
        AxisIndex::Range(Range { start, stop, step }) => {
            let start_v: usize = start
                .as_ref()
                .map(cast_negative(len, true))
                .unwrap_or(Ok(0))?;
            let stop_v: usize = stop
                .as_ref()
                .map(cast_negative(len, false))
                .unwrap_or(Ok(len))?;

            if start_v > stop_v {
                return Err(TypstError::full(
                    start.as_ref().unwrap().span,
                    "range start greater than end",
                    "value smaller than end of range",
                    equation!(format!("{} > {}", start_v, stop_v)),
                ));
            }

            let step_v: isize = step
                .as_deref()
                .cloned()
                .map(|n| n.to_isize().unwrap())
                .unwrap_or(1);
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
                let index = cast_negative(len, true)(index)?;
                v.push(index);
            }
        }
    };
    Ok(v)
}

/// Tries to index a matrix.
/// Errors are reported if index indices are out of bounds
pub fn indexm<'data>(matrix: &Matrix, indices: &[AxisIndex; 2]) -> Expects<'data, Matrix> {
    let rows = matrix.select(Axis(0), &to_indices(matrix.dim().0, &indices[0])?);
    let cols = rows.select(Axis(1), &to_indices(matrix.dim().1, &indices[1])?);
    Ok(cols)
}

/// Tries to index a tensor. Tensor indexing is only valid for matrices.
pub fn index<'data>(
    span: SimpleSpan,
    tensor: Tensor,
    indices: [AxisIndex; 2],
) -> Expects<'data, Tensor> {
    let matrix = matrix(tensor, span)?;
    indexm(&matrix, &indices).map(Tensor::Matrix)
}

/// Validates $[Tensor] in NN$
fn validate_real_int<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
    parser: impl Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> impl Parser<'this, I, Expects<'data, i64>, ParserError<'data>> {
    validate(validate(validate(parser, scalar), real), real_int)
}

/// Collects a parser of impl Iterator<Item = Expects<'data, T>>
/// into a single Expects<'data, impl Iterator<T>>.
///
/// Directly collecting into Expects<'data, impl Iterator<T>> is possible,
/// but bleeds the preemptive abortion of the parser into the parser flow.
pub fn collect_expects<
    'this,
    'data: 'this,
    C: FromIterator<T>,
    T,
    I: LocatingSequenceLike<'this, 'data>,
>(
    parser: impl IterParser<'this, I, Expects<'data, T>, ParserError<'data>>,
) -> impl Parser<'this, I, Expects<'data, C>, ParserError<'data>> {
    parser
        .collect::<Vec<Expects<'data, T>>>()
        .into_iter()
        .collect::<Expects<'data, C>>()
}

/// A range of indices for slices of list like objects.
/// `start`, `stop` and `step` have span metadata stored.
///
/// # EBNF
/// range = [<start>]':'[<stop>][':'[<step>]]
#[derive(Default, Debug, Clone)]
pub struct Range<S: Span = SimpleSpan> {
    pub start: Option<Spanned<i64, S>>,
    pub stop: Option<Spanned<i64, S>>,
    pub step: Option<Spanned<i64, S>>,
}

/// Represents a single dimension of index into a list like object.
/// The syntax is very similar to python.
/// An index can either be a range or a list of indices.
///
/// # EBNF
/// range = [<start>]':'[<stop>][':'[<step>]]
/// indices = <int> | '[' {<int>','} ']'
/// axes_index = '[' {<range> | <indices> ','} ']'
///
/// where <int> in (-dim, dim)
#[derive(Debug, Clone)]
pub enum AxisIndex {
    Range(Range),
    Indices(Vec<SimpleSpanned<i64>>),
}

/// Parses a range of indices for slices of list like objects.
///
/// # EBNF
/// range = [<start>]':'[<stop>][':'[<step>]]
///
/// where <int> in (-dim, dim)
pub fn range<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
    pratt: impl 'this + Clone + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> impl Parser<'this, I, Expects<'data, Range>, ParserError<'data>> {
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
            start: start_t.map(transpose).transpose()?,
            stop: stop_t.map(transpose).transpose()?,
            step: step_t.map(transpose).transpose()?,
        })
    })
}

/// Parses a single or a list of indices for slices of list like objects.
///
/// # EBNF
/// indices = <int> | '[' {<int>','} ']'
///
/// where <int> in (-dim, dim)
pub fn indices<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
    pratt: impl 'this + Clone + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> impl Parser<'this, I, Expects<'data, Vec<SimpleSpanned<i64>>>, ParserError<'data>> {
    choice((
        delimited_by_groups(
            collect_expects::<Vec<_>, _, _>(
                validate_real_int(pratt.clone())
                    .spanned()
                    .map(transpose)
                    .separated_by(character(','))
                    .at_least(1),
            )
            .delimited_by(character('['), character(']')),
        ),
        validate_real_int(pratt)
            .spanned()
            .map(|f| transpose(f).map(|f| vec![f])),
    ))
}

/// Parses an [AxisIndex]. A single axis index to create a slice of a list like object.
///
/// # EBNF
/// see [AxisIndex]
pub fn axes_index<'this, 'data: 'this, const AXES: usize, I: LocatingSequenceLike<'this, 'data>>(
    pratt: impl 'this + Clone + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> impl Parser<'this, I, Expects<'data, [AxisIndex; AXES]>, ParserError<'data>> {
    delimited_by_groups(
        collect_expects::<Vec<_>, _, _>(
            choice((
                range(pratt.clone()).map(|r| r.map(AxisIndex::Range)),
                indices(pratt).map(|i| i.map(AxisIndex::Indices)),
            ))
            .delimited_by(whitespaces(), whitespaces())
            .separated_by(character(','))
            .exactly(AXES),
        )
        // Expects<Vec<AxisIndex>> -> Expects<[AxisIndex; AXES]>
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

/// Condition for the pratt parser to apply.
#[kalt_macros::parser]
pub fn pratt_axes_index_operator(
    parser: impl 'this + Clone + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Spanned<[AxisIndex; 2]>> {
    span(axes_index::<2, I>(parser))
}

/// Application of pratt parser for negation of a tensor
pub fn pratt_axes_index<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
    lhs: Expects<'data, Spanned<Tensor>>,
    op: Expects<'data, Spanned<[AxisIndex; 2]>>,
    _extra: &mut MapExtra<'this, '_, I, Full<TypstError<'data>, (), ()>>,
) -> Expects<'data, Spanned<Tensor>> {
    let lhs = lhs?;
    let indices = op?;

    let span = lhs.span.union(indices.span);
    transpose(span.make_wrapped(index(span, lhs.inner, indices.inner)))
}
