#import "../lib.typ": comp, reduce, tensor-variant

#let test-state = state("kalt-test-results", ())
#let ci-mode = sys.inputs.at("ci", default: "false") == "true"

/// Returns true when a value returned by `comp` is an inline error sequence.
#let is-error(output) = "body" not in output.fields()

/// Records one test result without throwing on a failed assertion.
#let record-test(name, output, expected: none, error: none) = {
  let actual-error = is-error(output)
  let passed = if error != none {
    actual-error and error in repr(output)
  } else {
    not actual-error and validate(output, expected)
  }

  test-state.update(results => {
    results.push((
      name: name,
      passed: passed,
      expected-error: error,
      actual-error: actual-error,
    ))
    results
  })

  passed
}

/// Assertion for a successful calculation. Returns the pass/fail state and
/// records it for the test runner without panicking on inline errors.
#let assert-eq(name, output, expected) = record-test(name, output, expected: expected)

/// Assertion for a calculation expected to return an inline error.
/// If `contains` is provided, the rendered error must contain that text.
#let assert-error(name, output, contains: none) = record-test(name, output, error: contains)

/// Emits the final report as queryable metadata.
#let emit-test-report() = context {
  let results = test-state.get()
  let passed = results.filter(result => result.passed).len()
  let report = (
    total: results.len(),
    passed: passed,
    failed: results.len() - passed,
    failures: results.filter(result => not result.passed),
  )
  metadata(report) <kalt-test-report>
}

#let validate-scalar(output, expected) = {
  let is-nan-or-inf(val) = {
    if val in ("NaN", "nan", "Inf", "inf") {
      return true
    }
    return false
  }
  let part(op, output) = {
    let c = comp($#op (#output)$)
    if ("body" not in c.fields()) {
      return none
    }
    let val = c.body.children.at(0).text
    if is-nan-or-inf(val) {
      return val
    }
    eval(val)
  }
  let (real, imag) = (part($Re$, output), part($Im$, output))
  let (expected-real, expected-imag) = (part($Re$, expected), part($Im$, expected))

  if real == none or imag == none or expected-real == none or expected-imag == none {
    return false
  }

  let dist(a, b) = {
    if is-nan-or-inf(a) and is-nan-or-inf(b) {
      return 0
    } else if is-nan-or-inf(a) or is-nan-or-inf(b) {
      return 1e100000
    }
    calc.abs(a - b)
  }
  if dist(real, expected-real) > 1e-9 or dist(imag, expected-imag) > 1e-9 {
    return false
  }
  true
}

#let validate(output, expected) = {
  if type(output) != type(expected) {
    return false
  }
  if tensor-variant(output) == "matrix" {
    reduce(
      (acc, output, expected) => acc and validate-scalar(output, expected),
      true,
      output,
      expected,
    )
  } else {
    validate-scalar(output, expected)
  }
}

#let validate-format(output, expected) = {
  if is-error(output) {
    "FAIL"
  } else if validate(output, expected) {
    "PASS"
  } else {
    "FAIL"
  }
}

/// Runs and records binary operation cases. In CI mode it produces no visual
/// output while executing exactly the same cases as the visual suite.
#let binary-operation(title, description, op, ..cases) = {
  let rows = cases.pos().map((case, index) => {
    let expression = op(case.v1, case.v2)
    let output = comp(expression)
    let expected-error = if "error" in case { case.error } else { none }
    let passed = record-test(
      title + " case " + str(index + 1),
      output,
      expected: if "e" in case { case.e } else { none },
      error: expected-error,
    )

    if ci-mode {
      return ()
    }

    (
      block(expression),
      block($#output$),
      if "e" in case { block($#case.e$) } else { [expected error: #case.error] },
      if passed { "PASS" } else { "FAIL" },
    )
  })

  if ci-mode {
    return ()
  }

  [
    == #title
    #description
    #table(
      columns: 4,
      align: left,
      [*Input*], [*Output*], [*Expected*], [*Result*],
      ..rows.flatten(),
    )
  ]
}

/// Runs and records unary operation cases. In CI mode it produces no visual
/// output while executing exactly the same cases as the visual suite.
#let unary-operation(title, description, op, ..cases) = {
  let rows = cases.pos().map((case, index) => {
    let expression = op(case.v)
    let output = comp(expression)
    let expected-error = if "error" in case { case.error } else { none }
    let passed = record-test(
      title + " case " + str(index + 1),
      output,
      expected: if "e" in case { case.e } else { none },
      error: expected-error,
    )

    if ci-mode {
      return ()
    }

    (
      block(expression),
      block($#output$),
      if "e" in case { block($#case.e$) } else { [expected error: #case.error] },
      if passed { "PASS" } else { "FAIL" },
    )
  })

  if ci-mode {
    return ()
  }

  [
    == #title
    #description
    #table(
      columns: 4,
      align: left,
      [*Input*], [*Output*], [*Expected*], [*Result*],
      ..rows.flatten(),
    )
  ]
}
