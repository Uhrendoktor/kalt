// Absolute value (norm/determinant) tests for kalt bindings

#import "../../lib.typ": comp, map
#import "../utils.typ": unary_operation

#unary_operation(
  "Absolute Value",
  [Unary absolute value (norm/determinant)],
  c1 => { $ lr(|#c1|) $ },
  (v: $3$, e: $3$),
  (v: $4i$, e: $4$),
  (v: $5-6i$, e: $sqrt(61)$),
  // vector norm
  (v: $mat(3; 4)$, e: $5$),
  (v: $mat(1; 2; 2)$, e: $3$),
  // determinant
  (v: $mat(1, 2; 3, 4)$, e: $-2$),
  (v: $mat(1, 2, 3; 4, 5, 6; 7, 8, 9)$, e: $0$),
)
