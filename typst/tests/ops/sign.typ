// Sign/negation tests for kalt bindings

#import "../../lib.typ": comp
#import "../utils.typ": unary_operation

#unary_operation(
  "Negation",
  [Unary negation],
  c1 => { $ -#c1 $ },
  (v: $3$, e: $-3$),
  (v: $4-5i$, e: $-4+5i$),
  (v: $mat(1, -2; 3, -4)$, e: $mat(-1, 2; -3, 4)$),
)
