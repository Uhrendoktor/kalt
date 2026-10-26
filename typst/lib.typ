#import "@preview/sertyp:0.1.5" as sertyp;
#let kalt = plugin("./bindings.wasm");

/// Recusrively evaluates the math content and returns the result.
/// Supports aribtrary expressions with complex numbers, matrices, and vectors.
///
/// Automatically reports formatted errors if the evaluation fails.
///
/// = Arguments
/// - `body`: The Typst math content to evaluate.
///
/// = Returns
/// A Typst float for real scalar results; otherwise the evaluated Typst content.
///
/// = Example
/// ```typst
/// $comp(3i*2i+1)$
/// // evaluates to $-5$
/// ```
#let comp(body) = {
  return sertyp.call(kalt.comp, body)
}

/// Maps one or more matrices/vectors element-wise using the given function.
///
/// - `fn(..elements, index) -> element`: A Typst function receiving one value from each matrix and the zero-based row-major cell index last.
/// - `..mats`: The matrices to map.
///
/// = Returns
/// A new matrix/vector containing the mapped elements.
///
/// = Note
/// Missing cells are passed as `none`.
/// Cells can be missing if the matrices have different sizes.
/// Numeric math results are converted to floats when possible; other content is preserved.
/// Maps matrices element-wise and passes each cell's zero-based row-major index last.
#let map(fn, ..mats) = {
  let elements = mats.pos().map(mat => sertyp.call(kalt.to_elements, mat))
  let max_rows = calc.max(..elements.map(element => element.len()))
  let max_cols = calc.max(..elements.map(element => calc.max(..element.map(row => row.len()))))

  let mapped = array(())
  let i = 0
  while (i < max_rows) {
    let j = 0
    mapped.push(array(()))
    while (j < max_cols) {
      let value = fn(
        ..elements.map(row => row.at(i, default: array(())).at(j, default: none)),
        i * max_cols + j,
      )
      if type(value) == content and repr(value).starts-with("content((func: \"math.equation\"") {
        let computed = comp(value)
        if type(computed) == float {
          value = computed
        }
      }
      mapped.at(i).push(value)
      j += 1
    }
    i += 1
  }

  if max_cols == 1 {
    return math.vec(..mapped.map(row => row.at(0)))
  }
  return math.mat(..mapped)
}

/// Returns the variant of a tensor (scalar or matrix).
///
/// - tensor (content):
/// -> str
#let tensor-variant(tensor) = {
  return sertyp.call(kalt.tensor_variant, tensor)
}

#let reduce(fn, value, ..mat) = {
  let elements = mat.pos().map(mat => sertyp.call(kalt.to_elements, mat))
  let max_rows = calc.max(..elements.map(element => element.len()))
  let max_cols = calc.max(..elements.map(element => calc.max(..element.map(row => row.len()))))

  let i = 0
  while (i < max_rows) {
    let j = 0
    while (j < max_cols) {
      value = fn(value, ..elements.map(row => row.at(i, default: array(())).at(j, default: none)))
      j += 1
    }
    i += 1
  }

  return value
}

/// Returns whether the predicate succeeds for at least one aligned matrix cell.
#let any(fn, ..mats) = reduce(
  (acc, ..elements) => acc or fn(..elements),
  false,
  ..mats,
)

/// Returns whether the predicate succeeds for every aligned matrix cell.
#let all(fn, ..mats) = reduce(
  (acc, ..elements) => acc and fn(..elements),
  true,
  ..mats,
)
