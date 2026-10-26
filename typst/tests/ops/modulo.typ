// Modulo tests for kalt bindings

#import "../utils.typ": binary_operation

#binary_operation(
  "Modulo",
  [Euclidean modulo for real scalar values and element-wise matrices],
  (c1, c2) => { $ #c1 % #c2 $ },
  (v1: $7$, v2: $4$, e: $3$),
  (v1: $-7$, v2: $4$, e: $1$),
  (v1: $7$, v2: $-4$, e: $3$),
  (v1: $mat(7, -7; 9, -9)$, v2: $4$, e: $mat(3, 1; 1, 3)$),
  (v1: $mat(7, 8)$, v2: $mat(3, 3)$, e: $mat(1, 2)$),
  (v1: $3i$, v2: $2$, error: "Modulo Error"),
  (v1: $5$, v2: $0$, error: "Modulo Error"),
)
