use chumsky::{primitive::choice, select};
use num::Complex;
use sertyp::{Content, SYMBOL_CC, TypstError, chumsky::Token, content, equation, sequence};

use crate::{
    Expects,
    parser::{atoms::parse_content, validator::scalar},
};

/// A parsed matrix or vector as 2D complex ndarray
///
/// Is obtained by using [matrix], [vector] or [matrix_like] parsers.
pub type Matrix = ndarray::Array2<Complex<f64>>;

/// Formatted matrix shape
///
/// Args:
/// - `group`: The mathematical group of the matrice's elements
/// - `d1`: The first dimension
/// - `d2`: The second dimension
///
/// Returns:
/// $group^(m times n)$
#[macro_export]
macro_rules! matrix_shape {
    ($base:block ^ ($d1:block x $d2:block)) => {
        $crate::pow!($base ^ { sertyp::sequence![$d1, sertyp::SYMBOL_times, $d2] })
    };
}

/// Formatted matrix shapes
///
/// Args:
/// - `m1`: The first matrix
/// - `m2`: The second matrix
/// - `symbol`: The symbol to use between the matrix shapes
///
/// Returns:
/// $CC^(m1.rows times m1.cols) symbol CC^(m2.rows times m2.cols)$
pub fn matrix_shapes<'data>(
    d11: impl Into<Content<'data>>,
    d12: impl Into<Content<'data>>,
    d21: impl Into<Content<'data>>,
    d22: impl Into<Content<'data>>,
    symbol: impl Into<Content<'data>>,
) -> Content<'data> {
    content!(equation![
        matrix_shape!({SYMBOL_CC} ^ ({d11} x {d12})),
        symbol,
        matrix_shape!({SYMBOL_CC} ^ ({d21} x {d22}))
    ])
}

/// parses a matrix
///
/// # EBNF
/// {{<expr->complex>}}
#[kalt_macros::parser]
pub fn matrix<'data>() -> Expects<'data, Matrix> {
    select!(Token::Raw(Content::MathMatrix(matrix)) => matrix).map_with(|matrix, extra| {
        let rows = matrix.rows.len();
        let cols = matrix.rows.iter().map(|r| r.len()).max().unwrap_or(0);

        ndarray::Array2::from_shape_vec(
            (rows, cols),
            matrix
                .rows
                .iter()
                .map(|row| {
                    row.iter()
                        .map(parse_content(scalar))
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
                "Matrix Dimension Error",
                sequence!(
                    "homogenous matrix of",
                    matrix_shape!({SYMBOL_CC} ^ ({rows.to_string()} x {cols.to_string()}))
                ),
                e.to_string(),
            )
        })
    })
}

/// parses a vector
///
/// # EBNF
/// {<expr->complex>}
#[kalt_macros::parser]
pub fn vector<'data>() -> Expects<'data, Matrix> {
    select!(Token::Raw(Content::MathVector(vector)) => vector).map(|vector| {
        Ok(ndarray::Array2::from_shape_vec(
            (vector.children.len(), 1),
            vector
                .children
                .iter()
                .map(parse_content(scalar))
                .collect::<Result<Vec<_>, _>>()?,
        )
        // can never fail since vecs are one dimensional
        .unwrap())
    })
}

/// parses a matrix or vector
///
/// # EBNF
/// <matrix> | <vector>
#[kalt_macros::parser]
pub fn matrix_like<'data>() -> Expects<'data, ndarray::Array2<Complex<f64>>> {
    choice((matrix(), vector()))
}
