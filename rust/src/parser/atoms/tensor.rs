use chumsky::primitive::choice;
use derive_more::{From, IsVariant, TryUnwrap};
use sertyp::{Content, Item, LocatingSequence, Sequence, content};

use crate::{
    Expects,
    parser::{
        atoms::{
            complex::{Complex, complex},
            matrix::{Matrix, matrix_like},
        },
        pratt::pratt,
    },
};

/// A parsed tensor
#[derive(Clone, Debug, TryUnwrap, IsVariant, From)]
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

impl<'data> TryFrom<Sequence<'data>> for Tensor {
    type Error = std::string::String;

    fn try_from(seq: Sequence<'data>) -> Result<Self, Self::Error> {
        let seq = LocatingSequence::from(&seq);
        use chumsky::Parser;
        let tensor = match (pratt()).parse(&seq).into_result() {
            Ok(arr) => arr,
            Err(_) => {
                return Err("unable to parse content".to_string());
            }
        };
        match tensor {
            Ok(tensor) => Ok(tensor),
            Err(_) => Err("error while parsing".to_string()),
        }
    }
}

impl Default for Tensor {
    fn default() -> Self {
        Tensor::Scalar(num::Complex::<f64>::new(0.0, 0.0))
    }
}
