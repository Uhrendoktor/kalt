// Principal inverse cosine tests for kalt bindings

#import "../utils.typ": unary_operation

#unary_operation(
  "Inverse Cosine",
  [Principal inverse cosine using the existing exponent-attach parser],
  c1 => { $cos^(-1)(#c1)$ },
  (v: $1$, e: $0$),
  (v: $0$, e: $pi / 2$),
)

#unary_operation(
  "Inverse Cosine Alias",
  [The arccos function name is an alias for inverse cosine],
  c1 => { $arccos(#c1)$ },
  (v: $1$, e: $0$),
)

#unary_operation(
  "Inverse Cosine Short Alias",
  [The acos function name is an alias for inverse cosine],
  c1 => { $acos(#c1)$ },
  (v: $1$, e: $0$),
)
