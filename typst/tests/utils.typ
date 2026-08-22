#import "../lib.typ": comp, reduce, tensor-variant

/// Checks if two scalars (complex numbers) are equal within a tolerance. Returns true if they are equal, false otherwise.
///
/// - output (content): The output scalar to validate.
/// - expected (content): The expected scalar to compare against.
/// -> boolean
#let validate_scalar(output, expected) = {
  let is_nan_or_inf(val) = {
    if val in ("NaN", "nan", "Inf", "inf") {
      return true
    }
    return false
  }
  let part(op, output) = {
    let c = comp($#op (#output)$)
    if ("body" not in c.fields()) {
      panic(c)
    }
    let val = c.body.children.at(0).text
    if is_nan_or_inf(val) {
      return val
    }
    return eval(val)
  }
  let (real, imag) = (part($Re$, output), part($Im$, output))
  let (expected_real, expected_imag) = (part($Re$, expected), part($Im$, expected))

  let dist(a, b) = {
    if is_nan_or_inf(a) and is_nan_or_inf(b) {
      return 0
    } else if is_nan_or_inf(a) or is_nan_or_inf(b) {
      return 1e100000
    }
    return calc.abs(a - b)
  }
  if dist(real, expected_real) > 1e-9 or dist(imag, expected_imag) > 1e-9 {
    return false
  }
  return true
}

/// Checks if two tensors (scalar or matrices/vectors) are equal within a tolerance. Returns true if they are equal, false otherwise.
///
/// - output (content): The output tensor to validate.
/// - expected (content): The expected tensor to compare against.
/// -> boolean
#let validate(output, expected) = {
  if type(output) != type(expected) {
    return false
  }
  if tensor-variant(output) == "matrix" {
    reduce(
      (acc, output, expected) => {
        return acc and validate_scalar(output, expected)
      },
      true,
      output,
      expected,
    )
  } else {
    return validate_scalar(output, expected)
  }
}

#let validate_format(output, expected) = {
  if validate(output, expected) {
    return "✅"
  } else {
    return "❌"
  }
}

/// Renders a validation table for each permutation of tensor operands. Automatically verifies the output.
///
/// - title (content):
/// - description (content):
/// - op (function): (content, content) -> content. kalt binary operator to test.
/// - format-op (function): (content, content) -> content. formatting function for the operator.
/// - cases (array): ((v1: scalar, v2: scalar, e: tensor), ...)
/// -> table
#let binary_operation(
  title,
  description,
  op,
  ..cases,
) = [
  == #title
  #description

  #table(
    columns: 4,
    align: left,
    [*Input*], [*Output*], [*Expected*], [*Result*],
    ..cases
      .pos()
      .map(case => {
        let output = comp(op(case.v1, case.v2))
        return (
          block(op(case.v1, case.v2)),
          block($#output$),
          block($#case.e$),
          validate_format(output, case.e),
        )
      })
      .flatten(),
  )
]

/// Renders a validation table for each permutation of unary tensor operands. Automatically verifies the output.
///
/// - title (content):
/// - description (content):
/// - op (function): (content) -> content. kalt unary operator to test.
/// - format-op (function): (content) -> content. formatting function for the operator.
/// - scalar (dictionary): (v: scalar, e: tensor)
/// - matrix (dictionary): (v: matrix, e: tensor)
/// ->
#let unary_operation(title, description, op, ..cases) = [
  == #title
  #description

  #table(
    columns: 4,
    align: left,
    [*Input*], [*Output*], [*Expected*], [*Result*],
    ..cases
      .pos()
      .map(case => {
        let output = comp(op(case.v))
        return (
          block(op(case.v)),
          block($#output$),
          block($#case.e$),
          validate_format(output, case.e),
        )
      })
      .flatten(),
  )
]
