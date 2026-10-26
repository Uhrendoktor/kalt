// Trigonometric and hyperbolic function tests for kalt bindings

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

#unary_operation(
  "Cosine",
  [Complex cosine, applied element-wise],
  c1 => { $cos(#c1)$ },
  (v: $0$, e: $1$),
  (v: $pi$, e: $-1$),
  (v: $mat(0, pi)$, e: $mat(1, -1)$),
)

#unary_operation(
  "Tangent",
  [Complex tangent, applied element-wise],
  c1 => { $tan(#c1)$ },
  (v: $0$, e: $0$),
  (v: $pi / 4$, e: $1$),
)

#unary_operation(
  "Inverse Sine",
  [Principal inverse sine in attach notation],
  c1 => { $sin^(-1)(#c1)$ },
  (v: $0$, e: $0$),
  (v: $1 / 2$, e: $pi / 6$),
)

#unary_operation(
  "Inverse Cosine",
  [Principal inverse cosine in attach notation],
  c1 => { $cos^(-1)(#c1)$ },
  (v: $1$, e: $0$),
  (v: $0$, e: $pi / 2$),
)

#unary_operation(
  "Inverse Tangent",
  [Principal inverse tangent in attach notation],
  c1 => { $tan^(-1)(#c1)$ },
  (v: $0$, e: $0$),
  (v: $1$, e: $pi / 4$),
)

#unary_operation(
  "Inverse Sine Alias",
  [The arcsin function name is an alias for inverse sine],
  c1 => { $arcsin(#c1)$ },
  (v: $0$, e: $0$),
)

#unary_operation(
  "Hyperbolic Sine",
  [Hyperbolic sine, applied element-wise],
  c1 => { $sinh(#c1)$ },
  (v: $0$, e: $0$),
  (v: $1$, e: $1.1752011936438014$),
)

#unary_operation(
  "Hyperbolic Cosine",
  [Hyperbolic cosine, applied element-wise],
  c1 => { $cosh(#c1)$ },
  (v: $0$, e: $1$),
)

#unary_operation(
  "Hyperbolic Tangent",
  [Hyperbolic tangent, applied element-wise],
  c1 => { $tanh(#c1)$ },
  (v: $0$, e: $0$),
)
