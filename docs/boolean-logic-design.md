# Boolean and Comparison Operators: Numeric-Embedding Proposal

**Status:** design proposal for review; no evaluator code changes included.

## Recommendation

Do not create a strict Boolean domain that breaks arithmetic, matrices, powers, or dot products. Given Kalt's numeric-first evaluator and the goal of reducing cross-type edge cases, the simplest coherent model is to represent Boolean outcomes through the numeric embedding **false = 0, true = 1**.

This lets Kalt retain its existing numeric `Tensor` and matrix storage. If it is important for a top-level result to display as the words `true`/`false`, a Boolean display tag can be preserved at the API/output boundary, but it must not block numeric promotion. A full per-element Boolean type is not needed merely to make comparisons composable.

## Semantics to validate

| Expression | Result | Rationale |
| --- | --- | --- |
| `3 > 2` | `1` | Comparison results are numeric indicators. |
| `3 < 2` | `0` | False is zero. |
| `true`, `false` | `1`, `0` | Optional readable aliases for the indicator values. |
| `2^true` | `2^1 = 2` | Boolean results are accepted by numeric operators. |
| `true + true` | `2` | Arithmetic is ordinary arithmetic, not closed Boolean algebra. |
| `true = 1` | `1` | Equality uses the same 0/1 numeric embedding. |
| `true ∧ false` | `0` | Logical operations return numeric indicators. |
| `3 ∧ 2` | `1` | Under the proposed numeric/truth-value rule, zero means false and nonzero means true. |
| `¬0`, `¬5` | `1`, `0` | NOT is an element-wise zero test. |

The rule for logical operators should be explicit: a numeric entry is false exactly when it is zero and true otherwise. Then AND/OR/NOT can be applied element-wise to numeric scalars and matrices and return 0/1. This includes complex values: zero means both real and imaginary parts are zero; nonzero means true. NaN/infinity behavior should be tested and documented separately rather than accidentally inherited.

This intentionally chooses numeric composability over strict Boolean typing. Users who expect a type error for `3 ∧ 2` may find this permissive, but the behavior is predictable once the zero/nonzero rule is stated and it eliminates numeric/Boolean cross-product errors.

## Matrices and dot products

- Matrix cells can contain comparisons or Boolean expressions because each element is parsed/evaluated before the containing matrix is assembled; the result is normalized to numeric 0/1 in the current numeric matrix representation.
- Comparison on matrices is element-wise and returns a numeric 0/1 matrix with the same shape.
- AND/OR/NOT on matrices is element-wise, respects Kalt's existing shape/broadcast rules, and returns numeric 0/1 entries.
- Ordinary `dot` retains its existing arithmetic meaning. For Boolean-encoded vectors `[1, 1]` and `[1, 1]`, the dot product is `2`, counting the positions where both entries are true. This is often useful.
- Boolean-semiring matrix multiplication is a different operation: pairwise multiplication is AND and the accumulation is OR, so the result tells whether any overlap exists rather than counting overlaps. Do not change Kalt's ordinary dot product to this interpretation. Add a separately named `bool_dot` only if that use case emerges.

## Algebra terminology

The set `{0,1}` under *ordinary* integer addition and multiplication is not itself a ring, because `1 + 1 = 2` leaves the set. This proposal instead embeds Boolean outcomes into the integers (or real-valued numeric representation): a Boolean value can enter any numeric operation, and the result may be any number.

There are other valid algebras, but they must not be confused:
- **Integer embedding:** `true = 1`, `false = 0`; ordinary arithmetic; dot product counts overlaps.
- **Boolean semiring:** OR acts as addition and AND as multiplication; Boolean matrix multiplication answers whether any overlap exists.
- **Boolean ring / GF(2):** addition is XOR and multiplication is AND; `1 + 1 = 0` modulo 2.

For Kalt, the first model aligns best with its existing numeric tensor and general equation-evaluation purpose.

## Operator syntax

- Comparisons: `<`, `>`, `≤`, `≥`, equality and inequality. They return numeric indicators.
- Logic: use unambiguous mathematical symbols `∧`, `∨`, `¬`. They apply the zero/nonzero rule and return numeric indicators.
- Do not make `&` and `|` context-dependent. Kalt already uses vertical bars for absolute value. If bitwise operations are ever needed, add explicitly named functions or reserve `&`/`|` exclusively for integer bitwise operations.
- Implement chained comparisons such as `1 < 2 < 3` as `(1 < 2) ∧ (2 < 3)`, not ordinary left-associative infix comparisons.
- Precedence (tightest to loosest): existing arithmetic; comparisons; NOT; AND; OR. Parentheses can always disambiguate.

## Implementation plan

1. Validate the numeric-embedding semantics, especially whether logical operators should accept arbitrary numeric values using zero/nonzero truth.
2. Add comparison operators that return real numeric 0/1 values, plus Boolean literal aliases if desired.
3. Add `∧`, `∨`, `¬` as element-wise zero/nonzero operations, using existing tensor shape and broadcasting machinery.
4. Add dedicated comparison-chain parsing.
5. Add tests through the real Typst interface for precedence, aliases, comparison boundaries, complex ordering, matrix expressions and shapes, numeric promotion in powers/arithmetic/dot products, NaN/infinity, and regressions for absolute value, factorial, and exponentiation.
6. Decide whether direct comparison output should display as `0/1` (simplest and honest representation) or as `false/true` while preserving the numeric 0/1 semantics internally. Do not allow this presentation decision to create type barriers.

## Research basis

- [Wolfram True](https://reference.wolfram.com/language/ref/True.html): Wolfram represents `True` as a symbolic Boolean value.
- [Wolfram Boole](https://reference.wolfram.com/language/ref/Boole.html): explicit conversion of truth values to 1/0.
- [Wolfram Dot](https://reference.wolfram.com/language/ref/Dot.html): ordinary dot products contract indices using the usual operation defined for their entries.
- [Wolfram Logic & Boolean Algebra](https://reference.wolfram.com/language/guide/LogicAndBooleanAlgebra.html): distinguishes symbolic Boolean values, logical operators, and conversion to 0/1.
- [Python Boolean type](https://docs.python.org/3/library/stdtypes.html#boolean-type): a contrasting design where booleans are a subclass of integers, while documentation warns against relying on the numeric behavior.
- [Boolean matrix multiplication over the Boolean semiring](https://cs.au.dk/~larsen/papers/BooleanMatrixVectorLB.pdf): distinguishes OR/AND matrix multiplication from ordinary numeric multiplication.
- [Kalt numeric Tensor](https://github.com/Uhrendoktor/kalt/blob/main/rust/src/parser/atoms/tensor.rs): currently represents scalar complex numbers and numeric matrices.
- [Kalt numeric Matrix](https://github.com/Uhrendoktor/kalt/blob/main/rust/src/parser/atoms/matrix.rs): currently stores matrices as `Array2<Complex<f64>>`.
