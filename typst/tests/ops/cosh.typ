// Hyperbolic cosine tests for kalt bindings

#import "../utils.typ": unary_operation

#unary_operation(
  "Hyperbolic Cosine",
  [Hyperbolic cosine, applied element-wise],
  c1 => { $cosh(#c1)$ },
  (v: $0$, e: $1$),
  (v: $1$, e: $1.5430806348152437$),
)
