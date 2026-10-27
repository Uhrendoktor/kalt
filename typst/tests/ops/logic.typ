// Logical operators use numeric Boolean indicators: 0=false and 1=true.
// Every logical operand must be real and exactly 0 or 1.

#import "../utils.typ": binary_operation, unary_operation

#unary_operation(
  "Logical NOT",
  [¬ accepts real 0/1 and returns 1 for 0 or 0 for 1.],
  a => { $ ¬ #a $ },
  (v: $0$, e: $1$),
  (v: $1$, e: $0$),
  (v: $op("true")$, e: $0$),
  (v: $op("false")$, e: $1$),
  (v: $2$, error: "{0, 1}"),
  (v: $1+i$, error: "ℂ"),
)

#binary_operation(
  "Logical AND",
  [∧ returns 1 only when both inputs are 1.],
  (a, b) => { $ #a ∧ #b $ },
  (v1: $1$, v2: $1$, e: $1$),
  (v1: $1$, v2: $0$, e: $0$),
  (v1: $0$, v2: $1$, e: $0$),
  (v1: $0$, v2: $0$, e: $0$),
  (v1: $2$, v2: $1$, error: "{0, 1}"),
  (v1: $0$, v2: $2$, error: "{0, 1}"),
)

#binary_operation(
  "Logical OR",
  [∨ returns 1 when at least one input is 1.],
  (a, b) => { $ #a ∨ #b $ },
  (v1: $1$, v2: $1$, e: $1$),
  (v1: $1$, v2: $0$, e: $1$),
  (v1: $0$, v2: $1$, e: $1$),
  (v1: $0$, v2: $0$, e: $0$),
  (v1: $1$, v2: $2$, error: "{0, 1}"),
)

#binary_operation(
  "Element-wise logical AND",
  [Logical operators apply element-wise to matrices of equal shape.],
  (a, b) => { $ #a ∧ #b $ },
  (
    v1: $mat(1, 0; 1, 1)$,
    v2: $mat(1, 1; 0, 1)$,
    e: $mat(1, 0; 0, 1)$,
  ),
  (
    v1: $mat(1, 0)$,
    v2: $mat(1; 0)$,
    error: "same shape",
  ),
)
