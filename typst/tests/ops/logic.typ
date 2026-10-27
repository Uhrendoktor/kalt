// Logical NOT is strict: it accepts only the numeric Boolean indicators 0 and 1.

#import "../utils.typ": unary_operation

#unary_operation(
  "Logical NOT",
  [¬ maps 0 to 1 and 1 to 0; non-Boolean indicators are rejected.],
  a => { $ ¬ #a $ },
  (v: $0$, e: $1$),
  (v: $1$, e: $0$),
  (v: $op("true")$, e: $0$),
  (v: $op("false")$, e: $1$),
  (v: $2$, error: "{0, 1}"),
  (v: $6.5$, error: "{0, 1}"),
  (v: $1+i$, error: "ℂ"),
)
