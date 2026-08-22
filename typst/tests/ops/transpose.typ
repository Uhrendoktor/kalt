// Matrix transpose tests for kalt bindings

#import "../../lib.typ": comp
#import "../utils.typ": unary_operation

#unary_operation(
  "Transpose",
  [Matrix transpose],
  c1 => { $ #c1^T $ },
  (v: $mat(1, 2; 3, 4)$, e: $mat(1, 3; 2, 4)$),
  (v: $mat(1, 2, 3; 4, 5, 6)$, e: $mat(1, 4; 2, 5; 3, 6)$),
)
