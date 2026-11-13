// Bare Typst math words `true` and `false` are numeric Boolean indicators.

#import "../../lib.typ": comp
#import "../utils.typ": assert-eq, unary_operation

#assert-eq("bare true literal", comp($#true$), $1$)
#assert-eq("bare false literal", comp($#false$), $0$)
#assert-eq("bare Boolean words in arithmetic", comp($#true + #true$), $2$)

#unary_operation(
  "Logical NOT",
  [¬ maps 0 to 1 and 1 to 0; non-Boolean indicators are rejected.],
  a => { $ ¬ #a $ },
  (v: $0$, e: $1$),
  (v: $1$, e: $0$),
  (v: $2$, error: "{0, 1}"),
  (v: $6.5$, error: "{0, 1}"),
  (v: $1+i$, error: "ℂ"),
)
