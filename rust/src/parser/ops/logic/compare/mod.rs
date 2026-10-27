use chumsky::span::{SimpleSpanned, Span};
use num::Complex;
use sertyp::content;

use crate::{
    Expects,
    parser::{
        atoms::tensor::Tensor,
        ops::element_wise::{element_wise_binary, validate_same_shape},
    },
};

mod eq;
mod ge;
mod gt;
mod le;
mod lt;
mod neq;

pub use eq::{compare_eq, pratt_eq, pratt_eq_operator};
pub use ge::{compare_ge, pratt_geq, pratt_geq_operator};
pub use gt::{compare_gt, pratt_g, pratt_g_operator};
pub use le::{compare_le, pratt_leq, pratt_leq_operator};
pub use lt::{compare_lt, pratt_l, pratt_l_operator};
pub use neq::{compare_neq, pratt_neq, pratt_neq_operator};

pub(super) fn compare_tensors<'data, F>(
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
    operator: char,
    comparison: F,
) -> Expects<'data, Tensor>
where
    F: Fn(
        SimpleSpanned<Complex<f64>>,
        SimpleSpanned<Complex<f64>>,
    ) -> Expects<'data, Complex<f64>>,
{
    element_wise_binary(
        comparison,
        validate_same_shape(move |lhs, rhs| content!(sertyp::sequence![lhs, operator, rhs])),
        lhs,
        rhs,
    )
}
