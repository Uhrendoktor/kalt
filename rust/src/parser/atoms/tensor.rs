use chumsky::primitive::choice;
use derive_more::TryUnwrap;
use sertyp::{Content, Item, content};

use crate::{
    Expects,
    parser::atoms::{
        complex::{Complex, complex},
        matrix::{Matrix, matrix_like},
    },
};

/// A parsed tensor
#[derive(Clone, Debug, TryUnwrap)]
pub enum Tensor {
    Scalar(num::Complex<f64>),
    Matrix(Matrix),
}

/// Parses a tensor, which can be either a scalar or a matrix.
///
/// # EBNF
/// <complex> | <matrix>
#[kalt_macros::parser]
pub fn tensor<'data>() -> Expects<'data, Tensor> {
    choice((
        complex().map(|c| Ok(Tensor::Scalar(c))),
        matrix_like().map(|m| m.map(Tensor::Matrix)),
    ))
}

impl<'data> From<Tensor> for Content<'data> {
    fn from(tensor: Tensor) -> Self {
        match tensor {
            Tensor::Scalar(s) => content!(Complex::from(s)),
            Tensor::Matrix(m) => match m.dim() {
                (_, 1) => sertyp::math::Vector {
                    children: m
                        .column(0)
                        .map(|&s| content!(Complex::from(s)))
                        .to_vec()
                        .into(),
                    ..sertyp::math::Vector::default()
                }
                .into(),
                (_, _) => sertyp::math::Matrix {
                    rows: m
                        .rows()
                        .into_iter()
                        .map(|row| row.map(|&s| content!(Complex::from(s))).to_vec().into())
                        .collect::<Vec<_>>()
                        .into(),
                    ..sertyp::math::Matrix::default()
                }
                .into(),
            },
        }
    }
}

impl<'data> From<Tensor> for Item<'data> {
    fn from(tensor: Tensor) -> Self {
        content!(tensor).into()
    }
}

impl Default for Tensor {
    fn default() -> Self {
        Tensor::Scalar(num::Complex::<f64>::new(0.0, 0.0))
    }
}
