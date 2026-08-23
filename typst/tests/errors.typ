// Expected-failure tests for the test library itself.

#import "../lib.typ": comp
#import "utils.typ": assert-error

#assert-error(
  "incompatible dot-product shapes",
  comp($mat(1, 2) dot mat(1, 2)$),
  contains: "compatible shapes",
)

#assert-error(
  "invalid logarithm base",
  comp($ log_1(8) $),
)

#assert-error(
  "zero root index",
  comp($ root(0, 8) $),
)
