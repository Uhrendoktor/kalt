// NOT preserves Boolean indicators and complements other integers bitwise.

#import "../utils.typ": unary_operation

#unary_operation(
  "Boolean NOT and integer complement",
  [¬ maps Boolean indicators 0↔1 and applies two's-complement bitwise NOT to other real safe integers.],
  a => { $ ¬ #a $ },
  (v: $0$, e: $1$),
  (v: $1$, e: $0$),
  (v: $op("true")$, e: $0$),
  (v: $op("false")$, e: $1$),
  (v: $2$, e: $-3$),
  (v: $-6$, e: $5$),
  (v: $6.5$, error: "Integer Error"),
  (v: $1+i$, error: "Real Value Error"),
  (v: $9007199254740992$, error: "Bitwise Operand Error"),
)
