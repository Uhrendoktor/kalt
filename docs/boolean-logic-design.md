# Boolean and Comparison Operators: Design Proposal

**Status:** proposal for review; no evaluator code changes included.

## Goal

Add intuitive comparisons and Boolean logic without making operators guess whether a value is a number, a truth value, or a bit pattern. Preserve Kalt's math-first interface, complex-number support, matrix behavior, and useful error messages.

## Recommended semantics

### 1. Booleans are a separate type

Comparisons return a Boolean, not numeric 0 or 1. Logical operators accept only Booleans. Numeric arithmetic accepts only numbers.

| Expression | Recommended behavior |
| --- | --- |
| `3 < 4` | `true` |
| `(3 < 4) ∧ (5 > 2)` | `true` |
| `true + 1`, `true * 3` | Clear type error; no implicit conversion |
| `true ∧ 1` | Clear type error; numbers are not truthy/falsy |
| `indicator(3 < 4)` | `1` |
| `indicator(3 > 4)` | `0` |

Provide one explicit Boolean-to-number helper (recommended name: `indicator(condition)`, with `1` for true and `0` for false). Do not let booleans silently participate in arithmetic. For equality, allow comparisons within a type; a Boolean compared to a number should be a type error, not silently equated to 0 or 1.

### 2. Prefer mathematical syntax; keep bitwise semantics separate

Canonical comparison operators: `=`, `≠`, `<`, `≤`, `>`, `≥`. Optionally accept common ASCII aliases (`==`, `!=`, `<=`, `>=`) if Typst's tokenization can normalize them reliably.

Canonical logic operators: `∧` (AND), `∨` (OR), and `¬` (NOT). Optional aliases such as `&&` and `||` can be considered after parsing is verified.

**Do not give `&` and `|` context-dependent meanings.** They commonly mean bitwise operations in programming, and `|x|` is already Kalt's absolute-value syntax. Likewise, `!` is already used as postfix factorial, and `^` is exponentiation. If bitwise support is added later, expose explicit names such as `bitand(a, b)` and `bitor(a, b)`; accept only real, finite, integer-valued operands and define negative-integer behavior deliberately. Do not implement bitwise behavior in the first Boolean release.

### 3. Precedence and chains

Use this precedence, from tightest to loosest:

1. Existing arithmetic and unary operations.
2. Comparisons.
3. Boolean NOT.
4. Boolean AND.
5. Boolean OR.

Parentheses remain available for clarity. Support natural chained comparisons: `1 < 2 < 3` means `(1 < 2) ∧ (2 < 3)`, not `(1 < 2) < 3`. This needs dedicated comparison-chain parsing (or an equivalent intermediate representation); implementing comparisons as ordinary left-associative infix operations will get it wrong.

Kalt currently evaluates while parsing and has no observable side effects in ordinary math expressions, so the first version need not promise short-circuit evaluation. Do not promise that `A ∧ B` skips evaluating `B`; true lazy evaluation would need a separate design if conditional or effectful expressions are added.

### 4. Numeric and matrix rules

- `<`, `≤`, `>`, `≥`: allow real-valued numbers only. Reject a complex operand with nonzero imaginary part using an explanatory error; do not invent a lexicographic order for complex numbers.
- `=` and `≠`: allow real or complex numbers. Decide the floating-point equality policy explicitly before implementation. Recommendation: exact equality of the evaluated values for `=`, plus an explicit approximate operator/helper such as `≈` or `approx_eq`; avoid a hidden tolerance because approximate equality is not transitive.
- Comparisons on matrices/vectors are element-wise and return a Boolean matrix/vector with the same shape. Scalar-to-matrix comparisons follow Kalt's existing scalar/matrix broadcasting behavior; matrix-to-matrix operations require compatible shapes.
- Boolean logic on Boolean matrices is element-wise, preserves shape, and requires compatible matrix shapes. Scalar Boolean values may broadcast over a Boolean matrix.
- Define and test behavior for NaN and infinities instead of letting edge behavior emerge accidentally from the implementation.

### 5. Representation and Typst output

The current Rust `Tensor` has numeric scalar and numeric matrix variants. For a contained first implementation, add distinct Boolean-scalar and Boolean-matrix variants, and centralize type checking at operator dispatch. Keep the existing numeric matrix representation intact initially; do not silently turn all numeric matrices into mixed-type arrays.

Decide the bindings contract alongside the type: Boolean results must remain Boolean values internally and must not be returned as floats. Ideally scalar results are usable as native Typst Boolean values in code contexts and can also be represented readably as `true`/`false` in displayed math. Verify what the existing `comp` return type supports before locking the public API; if needed, add a dedicated Boolean accessor rather than corrupting type semantics.

## Implementation battle plan

1. **Lock semantics first.** Validate this proposal, especially equality/approximation, comparison syntax, and Boolean output in Typst.
2. **Add typed values and diagnostics.** Add Boolean scalar/matrix representations, explicit conversion, and a central “expected number / expected Boolean” error path. Existing arithmetic on Boolean inputs should fail cleanly.
3. **Implement comparisons and precedence.** Add relational/equality operators, real-only ordering checks, and dedicated chained-comparison parsing.
4. **Implement Boolean logic.** Add NOT/AND/OR with Boolean-only operands; then add element-wise Boolean matrix behavior and shape checks.
5. **Wire output and documentation.** Update Rust-to-Typst conversions, the public operation list, and examples.
6. **Test through the real Typst interface.** Follow Kalt's existing operation-test conventions and update the test manifest. Keep the change split by phase so failures identify parser, typing, matrix, or output problems.

## Acceptance tests

Cover true/false comparison results and boundary equality; operator precedence and parentheses; `1 < 2 < 3`; invalid numeric/Boolean mixing; complex equality versus invalid complex ordering; NaN/infinity behavior; scalar/matrix broadcasting; equal and mismatched matrix shapes; Boolean output in Typst code and displayed math; explicit conversion to 0/1; and regression tests for absolute value, factorial, and exponentiation.

## Research basis

- [Wolfram Less](https://reference.wolfram.com/language/ref/Less.html) and [Greater](https://reference.wolfram.com/language/ref/Greater.html): numeric ordering and real-domain constraints.
- [Wolfram logical operators](https://reference.wolfram.com/language/howto/UseLogicalOperators.html): `&&` / `||` as AND / OR.
- [Wolfram Boole](https://reference.wolfram.com/language/ref/Boole.html): explicit conversion of true/false to 1/0.
- [Wolfram BitAnd](https://reference.wolfram.com/language/ref/BitAnd.html): integer-only bitwise semantics.
- [Python Boolean type](https://docs.python.org/3/library/stdtypes.html#boolean-type) and [Rust operator expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html): contrasting choices around booleans, integers, bitwise operators, and logical operators.
