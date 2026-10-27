// Bitwise operators act on signed integers, independently of numeric Boolean logic.

#import "../utils.typ": binary_operation

#binary_operation(
  "Bitwise AND",
  [Computes the bitwise AND of two real integers.],
  (a, b) => { $ #a op("bitand") #b $ },
  (v1: $6$, v2: $3$, e: $2$),
  (v1: $6$, v2: $2$, e: $2$),
  (v1: $5$, v2: $2$, e: $0$),
  (v1: $-6$, v2: $3$, e: $2$),
  (v1: $9007199254740991$, v2: $1$, e: $1$),
  (v1: $9007199254740992$, v2: $1$, error: "Bitwise Operand Error"),
  (v1: $mat(6, 2)$, v2: $mat(3, 1)$, e: $mat(2, 0)$),
  (v1: $mat(1, 2)$, v2: $mat(1; 2)$, error: "same shape"),
  (v1: $6.5$, v2: $3$, error: "Integer Error"),
  (v1: $1+i$, v2: $3$, error: "Real Value Error"),
)

#binary_operation(
  "Bitwise OR",
  [Computes the bitwise OR of two real integers.],
  (a, b) => { $ #a op("bitor") #b $ },
  (v1: $6$, v2: $3$, e: $7$),
  (v1: $4$, v2: $1$, e: $5$),
  (v1: $-6$, v2: $3$, e: $-5$),
)

#binary_operation(
  "Bitwise XOR",
  [Computes the bitwise exclusive OR of two real integers.],
  (a, b) => { $ #a op("bitxor") #b $ },
  (v1: $6$, v2: $3$, e: $5$),
  (v1: $7$, v2: $7$, e: $0$),
  (v1: $-6$, v2: $3$, e: $-7$),
)
