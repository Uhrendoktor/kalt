use chumsky::{
    select,
    span::{SimpleSpan, Span, Spanned, WrappingSpan},
};
use sertyp::{Content, chumsky::Token};

use crate::{
    Expects, operation_element_wise_same_shape,
    parser::{
        atoms::{parse_content, tensor::Tensor},
        validator,
    },
    pratt_infix,
};

/// evaluates a typst fraction
///
/// # EBNF
/// (<scalar> | <matrix>) / <scalar>
#[kalt_macros::parser]
pub fn fraction<'data>() -> Expects<'data, Tensor> {
    select!(Token::Raw(Content::MathFrac(f)) => f).map_with(|frac, extra| {
        let _span: SimpleSpan = extra.span();
        let span = |v| _span.make_wrapped(v);

        let denom = parse_content(validator::tensor)(&frac.denom).map(&span);
        let num = parse_content(validator::tensor)(&frac.num).map(&span);
        div(num?, denom?)
    })
}

pratt_infix!(div => |_op, lhs: Spanned<_>, rhs: Spanned<_>| Ok(lhs.span.union(rhs.span).make_wrapped(div(lhs, rhs)?)));
operation_element_wise_same_shape!(div: /);
