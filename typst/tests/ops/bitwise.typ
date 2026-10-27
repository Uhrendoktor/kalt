// Bitwise operators act on signed integers, independently of numeric Boolean logic.

#import "../utils.typ": binary_operation

#binary_operation(
  "Bitwise AND",
  [Computes the bitwise AND of two real integers.],
  (a, b) => { $ #a & #b $ },
  (v1: $6$, v2: $3$, e: $2$),
  (v1: $6$, v2: $2$, e: $2$),
  (v1: $5$, v2: $2$, e: $0$),
  (v1: $mat(6, 2)$, v2: $mat(3, 1)$, e: $mat(2, 0)$),
  (v1: $6.5$, v2: $3$, error: "ℤ"),
  (v1: $1+i$, v2: $3$, error: "ℂ"),
)

#binary_operation(
  "Bitwise OR",
  [Computes the bitwise OR of two real integers.],
  (a, b) => { $ #a ⊔ #b $ },
  (v1: $6$, v2: $3$, e: $7$),
  (v1: $4$, v2: $1$, e: $5$),
)

#binary_operation(
  "Bitwise XOR",
  [Computes the bitwise exclusive OR of two real integers.],
  (a, b) => { $ #a ⊕ #b $ },
  (v1: $6$, v2: $3$, e: $5$),
  (v1: $7$, v2: $7$, e: $0$),
)
