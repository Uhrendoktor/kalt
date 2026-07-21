use chumsky::{Parser, span::Spanned};
use sertyp::{
    chumsky::parser::{character, whitespaces},
    content, equation,
};

use crate::{
    Expects, match_tensor,
    parser::{
        ParserError,
        atoms::{matrix::Matrix, tensor::Tensor},
        ops::element_wise::validate_square,
        pratt::span,
    },
};

/// Computes the determinant of a square matrix
/// using Guassian elimination with partial pivoting.
pub fn determinant(mut m: Matrix) -> num::Complex<f64> {
    let mut det: num::Complex<f64> = num::one();

    let d = m.dim().0;
    for i in 0..d {
        // Find pivot
        let mut pivot = i;
        for j in i + 1..d {
            if m[[j, i]].norm() > m[[pivot, i]].norm() {
                pivot = j;
            }
        }

        // Singular
        if m[[pivot, i]].norm() < f64::EPSILON {
            return num::zero();
        }

        // Swap rows
        if pivot != i {
            for k in 0..d {
                m.swap([i, k], [pivot, k]);
            }
            det = -det;
        }

        let pivot_val = m[[i, i]];
        det *= pivot_val;

        // Eliminate
        for j in i + 1..d {
            let factor = m[[j, i]] / pivot_val;
            for k in i + 1..d {
                let _v = m[[i, k]];
                m[[j, k]] -= factor * _v;
            }
        }
    }

    det
}

/// Computes the absolute value of a tensor:
/// scalar -> scalar (norm)
/// matrix -> scalar (determinant) only if square
pub fn abs_t<'data>(t: Spanned<Tensor>) -> Expects<'data, num::Complex<f64>> {
    match_tensor!((t) => {
        s => |s: Spanned<num::Complex<f64>>| Ok(num::Complex::<f64>::new(s.norm(), 0.0)),
        m => |m: Spanned<Matrix>| {
            validate_square(|c| content!(equation!['|', c, '|']))(m.span.make_wrapped(&m.inner))?;
            Ok(determinant(m.inner))
        }
    })
}

/// Parses the absolute value operator
///
/// EBNF:
/// "|" <tensor> "|"
#[kalt_macros::parser]
pub fn abs<'data>(
    parser: impl 'this + Parser<'this, I, Expects<'data, Tensor>, ParserError<'data>>,
) -> Expects<'data, Tensor> {
    span(parser)
        .delimited_by(whitespaces(), whitespaces())
        .delimited_by(character('|'), character('|'))
        .map(|t| Ok(Tensor::Scalar(abs_t(t?)?)))
}
