use chumsky::IterParser;
use chumsky::pratt::{infix, left, none, postfix};
use chumsky::primitive::just;
use chumsky::span::WrappingSpan;
use chumsky::span::{SimpleSpanned, Span};
use chumsky::{Parser, pratt::prefix, primitive::choice, recursive::recursive};
use sertyp::FromString;
use sertyp::chumsky::parser::character;
use sertyp::chumsky::{LocatingSequenceLike, Token};
use sertyp::locating::GroupType;
use sertyp::{TypstError, chumsky::parser::whitespaces};

use crate::parser::ops::abs::abs;
use crate::parser::ops::boolean_dot::{pratt_boolean_dot, pratt_boolean_dot_operator};
use crate::parser::ops::bitwise::{
    pratt_bitwise_and, pratt_bitwise_and_operator, pratt_bitwise_or, pratt_bitwise_or_operator,
    pratt_bitwise_xor, pratt_bitwise_xor_operator,
};
use crate::parser::ops::add::{pratt_add, pratt_add_operator};
use crate::parser::ops::binom::binom;
use crate::parser::ops::ceil::ceil;
use crate::parser::ops::conjugate::conjugate;
use crate::parser::ops::cross::{pratt_cross, pratt_cross_operator};
use crate::parser::ops::dot::{dot, pratt_dot, pratt_dot_operator};
use crate::parser::ops::factorial::{pratt_factorial, pratt_factorial_operator};
use crate::parser::ops::floor::floor;
use crate::parser::ops::func::ln::ln;
use crate::parser::ops::func::{
    sin::{asin, asinh, sin, sinh},
    cos::{acos, acosh, cos, cosh},
    tan::{atan, atanh, tan, tanh},
};
use crate::parser::ops::func::log::log;
use crate::parser::ops::func::re_im::{im, re};
use crate::parser::ops::index::{pratt_axes_index, pratt_axes_index_operator};
use crate::parser::ops::modulo::{pratt_modulo, pratt_modulo_operator};
use crate::parser::ops::mul::{pratt_mul, pratt_mul_operator};
use crate::parser::ops::logic::{
    boolean_literal, pratt_and_operator, pratt_eq, pratt_eq_operator, pratt_g,
    pratt_g_operator, pratt_geq, pratt_geq_operator, pratt_l, pratt_l_operator,
    pratt_leq, pratt_leq_operator, pratt_logical_and, pratt_logical_not,
    pratt_logical_or, pratt_neq, pratt_neq_operator, pratt_not_operator,
    pratt_or_operator,
};
use crate::parser::ops::pow::pow;
use crate::parser::ops::root::root;
use crate::parser::ops::sub::{pratt_sub, pratt_sub_operator};
use crate::parser::ops::transpose::transpose;
use crate::{
    Expects,
    parser::{
        ParserError,
        atoms::tensor::{Tensor, tensor},
        ops::{
            div::fraction,
            sign::{pratt_sign, pratt_sign_operator},
        },
    },
};

/// Spans Expects<'data, T> to Expects<'data, SimpleSpanned<T>>
pub fn span<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>, T: 'this>(
    parser: impl 'this + Parser<'this, I, Expects<'data, T>, ParserError<'data>>,
) -> impl 'this + Parser<'this, I, Expects<'data, SimpleSpanned<T>>, ParserError<'data>> {
    parser.map_with(|result, extra| Ok::<_, TypstError>(extra.span().make_wrapped(result?)))
}

/// Unspans Expects<'data, SimpleSpanned<T>> to Expects<'data, T>
pub fn unspan<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>, T: 'this>(
    parser: impl 'this + Parser<'this, I, Expects<'data, SimpleSpanned<T>>, ParserError<'data>>,
) -> impl 'this + Parser<'this, I, Expects<'data, T>, ParserError<'data>> {
    parser.map_with(|result, _extra| Ok::<_, TypstError>(result?.inner))
}

/// Parses a implicitly or explicitly delimited group as math expression
///
/// EBNF:
/// <group> ::= '(' {<atom>} ')' | '{' {<atom>} '}' | <groupmarker> {<atom>} <groupmarker>
pub fn group<'this, 'data: 'this, I: LocatingSequenceLike<'this, 'data>, O: 'this>(
    parser: impl 'this + Clone + Parser<'this, I, O, ParserError<'data>>,
) -> impl Parser<'this, I, O, ParserError<'data>> {
    choice((
        // implicitly delimited groups
        choice(
            [GroupType::Math, GroupType::Sequence, GroupType::LR].map(|group_type| {
                parser.clone().delimited_by(
                    just(Token::Open(group_type.clone())),
                    just(Token::Close(group_type)),
                )
            }),
        ),
        // explicitly delimited groups
        choice([('(', ')'), ('{', '}')].map(|(open, close)| {
            parser
                .clone()
                .delimited_by(character(open), character(close))
        })),
    ))
    .labelled(sertyp::Content::from_string("group"))
}

/// Parses a single atom.
/// This is either a tensor, a group, or a atomic like expression.
/// Atomic like expressions are caused by typst having nested expressions:
/// - fraction
/// - pow
/// - binom
/// - ...
///
/// # EBNF
/// <tensor> | <group> | <atomicexpr>
#[kalt_macros::parser]
pub fn atom_like(
    expr: impl 'this + Clone + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    // ordered by likelihood to improve performance
    choice((
        group(expr.clone()),
        boolean_literal(),
        tensor(),
        fraction(),
        pow(),
        root(),
        conjugate(),
        transpose(),
        binom(),
        ln(expr.clone()),
        log(expr.clone()),
        choice((
            asin(expr.clone()),
            acos(expr.clone()),
            atan(expr.clone()),
            asinh(expr.clone()),
            acosh(expr.clone()),
            atanh(expr.clone()),
        )),
        sin(expr.clone()),
        cos(expr.clone()),
        tan(expr.clone()),
        sinh(expr.clone()),
        cosh(expr.clone()),
        tanh(expr.clone()),
        abs(expr.clone()),
        floor(expr.clone()),
        ceil(expr.clone()),
        re(expr.clone()),
        im(expr.clone()),
    ))
    .labelled(sertyp::Content::from_string("atom"))
}

/// Parses directly consequtive atoms or atomic operations and applies implicit multiplication
///
/// # EBNF
/// <atom> {<atom>}
#[kalt_macros::parser]
pub fn atomic_operations(
    expr: impl 'this + Clone + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    unspan(
        span(atom_like(expr.clone()))
            .then(span(atom_like(expr)).repeated().collect::<Vec<_>>())
            // apply implicit multiplication
            .map(|(first, rest)| {
                rest.into_iter().fold(first, |lhs, rhs| {
                    let lhs = lhs?;
                    let rhs = rhs?;
                    let span = lhs.span.union(rhs.span);
                    dot(lhs, rhs).map(|t| span.make_wrapped(t))
                })
            }),
    )
}

/// Parses a full expression and condenses it to a single tensor
#[kalt_macros::parser]
pub fn pratt<'data>() -> Expects<'data, Tensor> {
    recursive(move |expr| {
        let expr = expr
            .delimited_by(whitespaces(), whitespaces())
            .boxed()
            .labelled(sertyp::Content::from_string("pratt"));
        unspan(
            span(atomic_operations(expr.clone()).delimited_by(whitespaces(), whitespaces())).pratt(
                (
                    // Tighter to looser: postfix, sign, products, addition, comparisons,
                    // bitwise AND/XOR/OR, then logical NOT/AND/OR.
                    infix(left(8), pratt_dot_operator(), pratt_dot),
                    infix(left(8), pratt_boolean_dot_operator(), pratt_boolean_dot),
                    infix(left(8), pratt_mul_operator(), pratt_mul),
                    infix(left(8), pratt_modulo_operator(), pratt_modulo),
                    infix(left(8), pratt_cross_operator(), pratt_cross),
                    infix(left(7), pratt_add_operator(), pratt_add),
                    infix(left(7), pratt_sub_operator(), pratt_sub),
                    // Comparisons are non-associative; combine them with explicit logical operators.
                    // The composed ASCII aliases are parsed as whole operators.
                    infix(none(6), pratt_leq_operator(), pratt_leq),
                    infix(none(6), pratt_geq_operator(), pratt_geq),
                    infix(none(6), pratt_l_operator(), pratt_l),
                    infix(none(6), pratt_g_operator(), pratt_g),
                    infix(none(6), pratt_eq_operator(), pratt_eq),
                    infix(none(6), pratt_neq_operator(), pratt_neq),
                    infix(left(5), pratt_bitwise_and_operator(), pratt_bitwise_and),
                    infix(left(4), pratt_bitwise_xor_operator(), pratt_bitwise_xor),
                    infix(left(3), pratt_bitwise_or_operator(), pratt_bitwise_or),
                    prefix(2, pratt_not_operator(), pratt_logical_not),
                    infix(left(1), pratt_and_operator(), pratt_logical_and),
                    infix(left(0), pratt_or_operator(), pratt_logical_or),
                    prefix(9, pratt_sign_operator(), pratt_sign),
                    postfix(11, pratt_factorial_operator(), pratt_factorial),
                    postfix(10, pratt_axes_index_operator(expr.clone()), pratt_axes_index),
                ),
            ),
        )
        .boxed()
    })
    .delimited_by(whitespaces(), whitespaces())
}
