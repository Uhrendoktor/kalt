use chumsky::span::{SimpleSpanned, Span, Spanned, WrappingSpan};
use sertyp::{Content, SYMBOL_CC, TypstError, content};

use crate::{
    Expects, matrix_shape,
    parser::atoms::{matrix::Matrix, tensor::Tensor},
};

/// Applies a pure complex-valued function to a tensor's scalar or matrix cells.
pub fn unary_tensor<F>(t: Tensor, function: F) -> Tensor
where
    F: Fn(num::Complex<f64>) -> num::Complex<f64> + Copy,
{
    match t {
        Tensor::Scalar(value) => Tensor::Scalar(function(value)),
        Tensor::Matrix(mut matrix) => {
            matrix.mapv_inplace(function);
            Tensor::Matrix(matrix)
        },
    }
}

/// Applies a pure complex-valued function to a scalar or element-wise to a matrix.
pub fn unary<'data, F>(
    t: SimpleSpanned<Tensor>,
    function: F,
) -> Expects<'data, Tensor>
where
    F: Fn(num::Complex<f64>) -> num::Complex<f64> + Copy,
{
    Ok(unary_tensor(t.inner, function))
}

pub fn element_wise<
    'data,
    F: Fn(SimpleSpanned<num::Complex<f64>>) -> Expects<'data, num::Complex<f64>>,
    MC: Fn(SimpleSpanned<&Matrix>) -> Expects<'data, ()>,
>(
    f: F,
    matrix_condition: MC,
    t: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    match_tensor!((t) => {
        s => &f,
        m => |mut m: Spanned<Matrix>| {
            let span = m.span;
            matrix_condition(span.make_wrapped(&m.inner))?;
            m.iter_mut().try_for_each(|x| {
                *x = f(span.make_wrapped(*x))?;
                Ok::<_, TypstError>(())
            })?;
            Ok(m.inner)
        }
    })
}

pub fn element_wise2<
    'data,
    F: Fn(
        SimpleSpanned<num::Complex<f64>>,
        SimpleSpanned<num::Complex<f64>>,
    ) -> Expects<'data, num::Complex<f64>>,
    MC: Fn(SimpleSpanned<&Matrix>, SimpleSpanned<&Matrix>) -> Expects<'data, ()>,
>(
    f: F,
    matrix_condition: MC,
    t1: SimpleSpanned<Tensor>,
    t2: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor> {
    fn sm<
        'data,
        F: Fn(
            SimpleSpanned<num::Complex<f64>>,
            SimpleSpanned<num::Complex<f64>>,
        ) -> Expects<'data, num::Complex<f64>>,
    >(
        s: Spanned<num::Complex<f64>>,
        mut m: Spanned<Matrix>,
        f: F,
    ) -> Expects<'data, Matrix> {
        let span = m.span;
        m.iter_mut().try_for_each(|x| {
            *x = f(s, span.make_wrapped(*x))?;
            Ok::<_, TypstError>(())
        })?;
        Ok(m.inner)
    }
    match_tensors!((t1, t2) => {
        (s1, s2) => &f,
        (s, m) => |s, m| sm(s, m ,f),
        (m, s) => |m, s| sm(s, m, |s1, s2| f(s2, s1)),
        (m1, m2) => |mut m1: Spanned<Matrix>, m2: Spanned<Matrix>|{
            matrix_condition(m1.span.make_wrapped(&m1), m2.span.make_wrapped(&m2))?;
            let sp1 = m1.span;
            let sp2 = m2.span;
            m1.iter_mut().zip(m2.iter()).try_for_each(|(x, y)| {
                *x = f(sp1.make_wrapped(*x), sp2.make_wrapped(*y))?;
                Ok::<_, TypstError>(())
            })?;
            Ok(m1.inner)
        }
    })
}

/// General matrix product for operations defined by a multiplication and reduction operation.
///
/// Returns None when inner dimensions are incompatible. The identity and operations
/// are supplied by the caller, so this is reusable for arithmetic and semiring products.
pub fn matrix_product<T: Copy>(
    lhs: &ndarray::Array2<T>,
    rhs: &ndarray::Array2<T>,
    identity: T,
    multiply: impl Fn(T, T) -> T,
    reduce: impl Fn(T, T) -> T,
) -> Option<ndarray::Array2<T>> {
    if lhs.ncols() != rhs.nrows() {
        return None;
    }
    Some(ndarray::Array2::from_shape_fn(
        (lhs.nrows(), rhs.ncols()),
        |(row, col)| {
            let mut accumulator = identity;
            for inner in 0..lhs.ncols() {
                accumulator = reduce(
                    accumulator,
                    multiply(lhs[[row, inner]], rhs[[inner, col]]),
                );
            }
            accumulator
        },
    ))
}

/// Dispatches a binary tensor operation to a matrix product for matrix-matrix
/// inputs and to element-wise evaluation whenever at least one operand is scalar.
pub fn binary_tensor_operation<'data, F, M>(
    scalar_operation: F,
    matrix_operation: M,
    lhs: SimpleSpanned<Tensor>,
    rhs: SimpleSpanned<Tensor>,
) -> Expects<'data, Tensor>
where
    F: Fn(
        SimpleSpanned<num::Complex<f64>>,
        SimpleSpanned<num::Complex<f64>>,
    ) -> Expects<'data, num::Complex<f64>>,
    M: Fn(
        SimpleSpanned<&Matrix>,
        SimpleSpanned<&Matrix>,
    ) -> Expects<'data, Matrix>,
{
    match (lhs, rhs) {
        (
            SimpleSpanned {
                inner: Tensor::Matrix(lhs),
                span: lhs_span,
            },
            SimpleSpanned {
                inner: Tensor::Matrix(rhs),
                span: rhs_span,
            },
        ) => matrix_operation(
            lhs_span.make_wrapped(&lhs),
            rhs_span.make_wrapped(&rhs),
        )
        .map(Tensor::Matrix),
        (lhs, rhs) => element_wise2(scalar_operation, |_, _| Ok(()), lhs, rhs),
    }
}

/// Validates that two matrices have the same shape and returns a formattable matrix shape error if they do not.
///
/// Args:
/// - f: A function that formats the operation applied to the two matrices.
///   It receives two formatted matrix dimensions as arguments.
pub fn validate_same_shape<'data>(
    f: impl Fn(Content<'data>, Content<'data>) -> Content<'data>,
) -> impl Fn(SimpleSpanned<&Matrix>, SimpleSpanned<&Matrix>) -> Expects<'data, ()> {
    move |m1, m2| {
        if m1.shape() == m2.shape() {
            return Ok(());
        }
        Err(TypstError::full(
            m1.span.union(m2.span),
            "Element-wise operation requires matrices of the same shape",
            f(
                content!(
                    matrix_shape!({SYMBOL_CC} ^ ({m1.dim().0.to_string()} x {m1.dim().1.to_string()}))
                ),
                content!(
                    matrix_shape!({SYMBOL_CC} ^ ({m1.dim().0.to_string()} x {m1.dim().1.to_string()}))
                ),
            ),
            f(
                content!(
                    matrix_shape!({SYMBOL_CC} ^ ({m1.dim().0.to_string()} x {m1.dim().1.to_string()}))
                ),
                content!(
                    matrix_shape!({SYMBOL_CC} ^ ({m2.dim().0.to_string()} x {m2.dim().1.to_string()}))
                ),
            ),
        ))
    }
}

/// Validates that a matrix is square and returns a formattable matrix shape error if it is not.
///
/// Args:
/// - f: A function that formats the matrix operation.
///   It receives a formatted matrix's dimensions as input.
pub fn validate_square<'data>(
    f: impl Fn(Content<'data>) -> Content<'data>,
) -> impl Fn(SimpleSpanned<&Matrix>) -> Expects<'data, ()> {
    move |m| {
        if m.is_square() {
            return Ok(());
        }
        Err(TypstError::full(
            m.span,
            "Matrix must be square",
            f(content!(matrix_shape!({SYMBOL_CC} ^ ({'N'} x {'N'})))),
            f(content!(
                matrix_shape!({SYMBOL_CC} ^ ({m.dim().0.to_string()} x {m.dim().1.to_string()}))
            )),
        ))
    }
}
