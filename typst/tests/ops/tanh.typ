// Hyperbolic tangent tests for kalt bindings

#import "../utils.typ": unary_operation

#unary_operation(
  "Hyperbolic Tangent",
  [Hyperbolic tangent, applied element-wise],
  c1 => { $tanh(#c1)$ },
  (v: $0$, e: $0$),
  (v: $1$, e: $0.7615941559557649$),
)
