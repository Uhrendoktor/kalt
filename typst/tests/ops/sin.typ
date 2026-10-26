// Sine tests for kalt bindings

#import "../utils.typ": unary_operation

#unary_operation(
  "Sine",
  [Complex sine, applied element-wise],
  c1 => { $sin(#c1)$ },
  (v: $0$, e: $0$),
  (v: $pi / 2$, e: $1$),
  (v: $mat(0, pi / 2; pi, 3 pi / 2)$, e: $mat(0, 1; 0, -1)$),
  (v: $i pi$, e: $i sinh(pi)$),
)
