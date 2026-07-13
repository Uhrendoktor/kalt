use std::ops::{Add, Neg, Sub};

use chumsky::{
    IterParser, Parser,
    extra::{Full, ParserExtra, SimpleState},
    label::LabelError,
    pratt::{infix, left, postfix, prefix},
    primitive::{choice, just, one_of},
    recursive::recursive,
    select,
    span::SimpleSpan,
};
use num::{Complex, complex::Complex64};
use num_traits::Pow;
use sertyp::{
    Content::{self},
    FromString, LocatingSequence, SYMBOL_compose, SYMBOL_minus,
    chumsky::{
        LocatingSequenceLike, Token,
        parser::{
            MULTIPLY, as_token, auto_radix, character, delimited_by_groups, sign,
            unsigned_float_no_radix, whitespaces, word,
        },
    },
    error::TypstError,
    locating::GroupType,
    math::{Attach, Op},
};

use crate::{
    comp::{
        index::AxisIndex,
        tensor::{Tensor, log, scalar, tensor_exponent, validate},
    },
    complex::gamma::gamma,
};

// parses a constant complex number
// e.g. 1, 2.5, 3i, 5e3i, 2.6e-7i, π, e
pub fn complex<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>()
-> impl Parser<'this, I, Complex64, Error<'data>> {
    choice((
        auto_radix(unsigned_float_no_radix::<'this, 'data, f64, I, Error>, 10)
            .then(character('i').or_not())
            .map(|(f, i)| match i {
                Some(_) => Complex::new(0.0, f),
                None => Complex::new(f, 0.0),
            }),
        character('i').map(|_| Complex::new(0.0, 1.0)),
    ))
    .labelled(sertyp::Content::from_string("complex"))
}

// parses a matrix or vector as ndarray
pub fn matrix<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>()
-> impl Parser<'this, I, Result<ndarray::Array2<Complex64>, TypstError<'data>>, Error<'data>> {
    choice((
        select!(Token::Raw(Content::MathMatrix(matrix)) => matrix)
            .map_with(|matrix, extra| {
                ndarray::Array2::from_shape_vec(
                    (matrix.rows.len(), matrix.rows[0].len()),
                    matrix
                        .rows
                        .iter()
                        .map(|row| {
                            row.iter()
                                .map(|cell| {
                                    validate(pratt::<LocatingSequence>(), scalar)
                                        .parse(LocatingSequence::from(cell))
                                        .into_result()?
                                })
                                .collect::<Result<Vec<_>, _>>()
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>(),
                )
                .map_err(|e| {
                    TypstError::full(
                        extra.span(),
                        "Matrix Error",
                        "Invalid matrix dimensions",
                        e.to_string(),
                    )
                })
            })
            .labelled(sertyp::Content::from_string("matrix")),
        select!(Token::Raw(Content::MathVector(vector)) => vector)
            .map(|vector| {
                Ok(ndarray::Array2::from_shape_vec(
                    (vector.children.len(), 1),
                    vector
                        .children
                        .iter()
                        .map(|cell| {
                            validate(pratt::<LocatingSequence>(), scalar)
                                .parse(LocatingSequence::from(cell))
                                .into_result()?
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )
                .unwrap())
            })
            .labelled(sertyp::Content::from_string("vector")),
    ))
    .labelled(sertyp::Content::from_string("matrix"))
}

pub fn tensor<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>()
-> impl Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>> {
    choice((
        complex().map(|c| Ok(Tensor::Scalar(c))),
        matrix().map(|m| m.map(Tensor::Matrix)),
    ))
    .labelled(sertyp::Content::from_string("tensor"))
}

pub fn fraction<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>()
-> impl Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>> {
    select!(Token::Raw(Content::MathFrac(f)) => f)
        .map_with(|frac, extra| {
            let denom = pratt::<LocatingSequence>()
                .parse(LocatingSequence::from(&**frac.denom))
                .into_result()?;
            let num = pratt::<LocatingSequence>()
                .parse(LocatingSequence::from(&**frac.num))
                .into_result()?;
            (num? / denom?)
                .map_err(|e| TypstError::full(extra.span(), "Fraction Error", "", e.to_string()))
        })
        .labelled(sertyp::Content::from_string("fraction"))
}

pub fn pow<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>()
-> impl Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>> {
    select!(Token::Raw(Content::MathAttach(attach @ Attach{ t: Some(_), ..})) => attach)
        .map_with(|attach, extra| {
            let base = pratt::<LocatingSequence>()
                .parse(LocatingSequence::from(&**attach.base))
                .into_result()?;
            let exponent = tensor_exponent()
                .parse(LocatingSequence::from(&***attach.t.as_ref().unwrap()))
                .into_result()?;
            base?.pow(extra.span(), exponent?)
        })
        .labelled(sertyp::Content::from_string("pow"))
}

pub fn binom<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>()
-> impl Parser<'this, I, Result<Complex64, TypstError<'data>>, Error<'data>> {
    select!(Token::Raw(Content::MathBinom(binom)) => binom)
        .map_with(|binom, extra| {
            let n = validate(pratt::<LocatingSequence>(), scalar)
                .parse(LocatingSequence::from(&**binom.upper))
                .into_result()??;
            let k = if let Some(lower) = &binom.lower
                && lower.len() == 1
            {
                &lower[0]
            } else {
                return Err(TypstError::full(
                    extra.span(),
                    "Binominal Argument Error",
                    "Binomial coefficients must have exactly one lower argument",
                    format!(
                        "{} arguments",
                        binom.lower.as_ref().map(|v| v.len()).unwrap_or(0)
                    ),
                ));
            };
            let k = validate(pratt::<LocatingSequence>(), scalar)
                .parse(LocatingSequence::from(k))
                .into_result()??;
            Ok(gamma(n) / (gamma(k) * gamma(n - k)))
        })
        .labelled(sertyp::Content::from_string("binom"))
}

pub fn root<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>()
-> impl Parser<'this, I, Result<Complex64, TypstError<'data>>, Error<'data>> {
    select!(Token::Raw(Content::MathRoot(root)) => root)
        .map(|root| {
            let radicand = validate(pratt::<LocatingSequence>(), scalar)
                .parse(LocatingSequence::from(&**root.radicand))
                .into_result()?;
            let index = root
                .index
                .as_ref()
                .map(|index| {
                    validate(pratt::<LocatingSequence>(), scalar)
                        .parse(LocatingSequence::from(&***index))
                        .into_result()
                })
                .unwrap_or(Ok(Ok(2f64.into())))?;
            Ok(radicand?.pow(1f64 / index?))
        })
        .labelled(sertyp::Content::from_string("root"))
}

pub fn group<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    O: 'this,
    P: 'this + Clone + Parser<'this, I, O, Error<'data>>,
>(
    parser: P,
) -> impl Parser<'this, I, O, Error<'data>> {
    delimited_by_groups(choice((
        // open new group for marker groups
        choice([GroupType::Math, GroupType::Sequence].map(|group_type| {
            parser.clone().delimited_by(
                just(Token::Open(group_type.clone())),
                just(Token::Close(group_type)),
            )
        })),
        // parse and open real groups
        choice([('(', ')'), ('{', '}')].map(|(open, close)| {
            parser
                .clone()
                .delimited_by(character(open), character(close))
        })),
    )))
    .labelled(sertyp::Content::from_string("group"))
}

pub fn built_in_fn<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
    parser: impl 'this + Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>>,
) -> impl Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>> {
    fn word_or_op<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>(
        name: &'static str,
    ) -> impl Parser<'this, I, String, Error<'data>> {
        choice((
            word(name),
            select!(Token::Raw(Content::MathOp(Op { text, .. })) => &***text).try_map(
                move |text, span| match text {
                    Content::Text(text) if &**text.text == name => Ok(text.text.to_string()),
                    _ => Err(TypstError::full(span, "Function Error", name, text.clone())),
                },
            ),
        ))
    }

    choice((
        word_or_op("ln").map(|c| Ok((c, None))),
        select!(Token::Raw(Content::MathAttach(Attach {
            base,
            b,
            ..
        })) => (base, b))
        .try_map(|(base, b), span| {
            let name = choice((word_or_op("log"),))
                .parse(LocatingSequence::from(&***base))
                .into_result()?;
            let b = b.as_ref().ok_or_else(|| {
                TypstError::full(
                    span,
                    "Function Error",
                    "Missing argument",
                    "function call must have an argument",
                )
            })?;
            let arg = pratt().parse(LocatingSequence::from(&***b)).into_result()?;
            Ok(arg.map(|arg| (name, Some(arg))))
        }),
    ))
    .then(whitespaces().ignored())
    .then(delimited_by_groups(
        parser.delimited_by(character('('), character(')')),
    ))
    .map_with(|((fn_info, _), value), extra| {
        let (name, arg) = fn_info?;
        let value = value?;
        match name.as_str() {
            "ln" => log(extra.span(), Tensor::Scalar(2.0.into()), value),
            "log" => {
                let arg = arg.ok_or_else(|| {
                    TypstError::full(extra.span(), "Logerithm needs Basis", "Basis", "No Basis")
                })?;
                log(extra.span(), arg, value)
            }
            _ => unreachable!(),
        }
    })
}

pub type Error<'data> = Full<TypstError<'data>, SimpleState<Tensor>, ()>;
pub fn pratt<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>>()
-> impl Parser<'this, I, Result<Tensor, TypstError<'data>>, Error<'data>>
where
    <Error<'data> as ParserExtra<'this, I>>::Error: LabelError<'this, I, sertyp::Content<'data>>,
{
    recursive(move |expr| {
        let atom = |expr: chumsky::prelude::Recursive<
            dyn Parser<
                    'this,
                    I,
                    Result<Tensor, TypstError<'data>>,
                    Full<TypstError<'data>, SimpleState<Tensor>, ()>,
                >,
        >| {
            whitespaces::<'this, 'data, I, Error>()
                .ignored()
                .then(choice((
                    group(expr.clone()),
                    tensor(),
                    fraction(),
                    pow(),
                    binom().map(|r| r.map(Tensor::Scalar)),
                    root().map(|r| r.map(Tensor::Scalar)),
                    built_in_fn(expr),
                )))
                .then(whitespaces().ignored())
                .map(|((_, e), _)| e)
                .labelled(sertyp::Content::from_string("atom"))
        };

        let atoms = atom(expr.clone())
            .map_with(|atom, extra| (atom, extra.span()))
            .then(
                atom(expr.clone())
                    .map_with(|atom, extra| (atom, extra.span()))
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(lhs, rhs)| {
                rhs.into_iter()
                    .fold(lhs, |(lhs, lspan), (rhs, rspan)| {
                        (
                            (|| {
                                lhs?.mul(rhs?)
                                    .map_err(|f| f((lspan.start..rspan.end).into()))
                            })(),
                            rspan,
                        )
                    })
                    .0
            });

        // --- Pratt parser ---
        atoms
            .map_with(|atom, extra| atom.map(|atom| (atom, extra.span())))
            .pratt((
                // Prefix unary minus
                prefix::<'_, _, _, _, _, _, Error>(
                    5,
                    sign(),
                    |op: char, rhs: Result<(Tensor, I::Span), _>, _| {
                        rhs.map(|(rhs, mut span)| {
                            span.start -= 1;
                            if [SYMBOL_minus, '-'].contains(&op) {
                                (rhs.neg(), span)
                            } else {
                                (rhs, span)
                            }
                        })
                    },
                ),
                // factorial
                postfix(
                    6,
                    character('!'),
                    |lhs: Result<(Tensor, I::Span), _>, _, _| match lhs {
                        Ok((Tensor::Scalar(lhs), mut span)) => {
                            span.end += 1;
                            Ok((Tensor::Scalar(gamma(lhs)), span))
                        }
                        Ok((_, span)) => Err(TypstError::full(
                            span,
                            "Type Error",
                            "factorial",
                            "only defined for scalars",
                        )),
                        Err(e) => Err(e),
                    },
                ),
                // indexing
                postfix(
                    5,
                    super::index::axes_index::<2, _>(expr)
                        .map_with(|index, extra| (index, extra.span())),
                    |lhs: Result<(Tensor, I::Span), _>,
                     (index, span2): (Result<[AxisIndex; 2], _>, SimpleSpan),
                     _| {
                        let (tensor, mut span) = lhs?;
                        span.end = span2.end;
                        tensor.index(span, &index?).map(|tensor| (tensor, span))
                    },
                ),
                // Multiplication / division
                infix::<'_, _, _, _, _, _, Error>(
                    left(3),
                    one_of(as_token(&MULTIPLY)),
                    |lhs: Result<(Tensor, I::Span), _>,
                     _op,
                     rhs: Result<(Tensor, I::Span), _>,
                     _| {
                        match (lhs, rhs) {
                            (Ok((lhs, lspan)), Ok((rhs, rspan))) => {
                                let span = (lspan.start..rspan.end).into();
                                lhs.mul(rhs).map_err(|f| f(span)).map(|res| (res, span))
                            }
                            (Err(e), _) | (_, Err(e)) => Err(e),
                        }
                    },
                ),
                // Element wise multiplication
                infix::<'_, _, _, _, _, _, Error>(
                    left(3),
                    character(SYMBOL_compose),
                    |lhs: Result<(Tensor, I::Span), _>,
                     _op,
                     rhs: Result<(Tensor, I::Span), _>,
                     _| {
                        match (lhs, rhs) {
                            (Ok((lhs, lspan)), Ok((rhs, rspan))) => {
                                let span = (lspan.start..rspan.end).into();
                                lhs.element_mul(span, &rhs).map(|res| (res, span))
                            }
                            (Err(e), _) | (_, Err(e)) => Err(e),
                        }
                    },
                ),
                // Addition / subtraction
                infix::<'_, _, _, _, _, _, Error>(
                    left(2),
                    just(Token::Char('+')),
                    |lhs: Result<(Tensor, I::Span), _>,
                     _op,
                     rhs: Result<(Tensor, I::Span), _>,
                     _| {
                        match (lhs, rhs) {
                            (Ok((lhs, lspan)), Ok((rhs, rspan))) => {
                                let span = (lspan.start..rspan.end).into();
                                Ok((lhs.add(rhs), span))
                            }
                            (Err(e), _) | (_, Err(e)) => Err(e),
                        }
                    },
                ),
                infix::<'_, _, _, _, _, _, Error>(
                    left(2),
                    choice((character(SYMBOL_minus), character('-')))
                        .labelled(sertyp::Content::from_string("subtraction operator")),
                    |lhs: Result<(Tensor, I::Span), _>,
                     _op,
                     rhs: Result<(Tensor, I::Span), _>,
                     _| match (lhs, rhs) {
                        (Ok((lhs, lspan)), Ok((rhs, rspan))) => {
                            let span = (lspan.start..rspan.end).into();
                            Ok((lhs.sub(rhs), span))
                        }
                        (Err(e), _) | (_, Err(e)) => Err(e),
                    },
                ),
            ))
            .map(|atom: _| atom.map(|(tensor, _)| tensor))
            .boxed()
    })
}
