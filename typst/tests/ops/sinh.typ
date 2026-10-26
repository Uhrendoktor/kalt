// Hyperbolic sine tests for kalt bindings

#import "../utils.typ": unary_operation

#unary_operation(
  "Hyperbolic Sine",
  [Hyperbolic sine, applied element-wise],
  c1 => { $sinh(#c1)$ },
  (v: $0$, e: $0$),
  (v: $1$, e: $1.1752011936438014$),
)
