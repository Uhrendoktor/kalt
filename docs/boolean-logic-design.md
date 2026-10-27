# Boolean and Comparison Operators: Revised Design

**Status:** proposal for review; no evaluator code changes included.

## Core correction

Do **not** add Boolean values to Kalt's existing numeric `Tensor` enum or allow Boolean entries in its numeric matrix type. Kalt's `Tensor` is a numeric abstraction: complex scalar or matrix of complex numbers. Boolean semantics are not an extension of ordinary arithmetic.

However, if comparisons must return actual Boolean results and nest inside logical expressions, the evaluator needs a typed intermediate result somewhere. Recommended: introduce an outer `EvalValue` (or equivalent) that is either `Numeric(Tensor)` or `Boolean(BoolTensor)`; `BoolTensor` contains a scalar bool or a separate matrix of bools. Keep all existing numerical operation implementations and matrix storage numeric-only.

## Explicit operation domains

| Operation | Accepted operands | Result |
| --- | --- | --- |
| `+`, `-`, `*`, `/`, `^`, factorial, functions, numeric dot product | Numeric only | Numeric |
| `<`, `>`, `≤`, `≥` | Real-valued numeric values | Boolean |
| `=`, `≠` | Numeric/numeric or Boolean/Boolean | Boolean |
| `∧`, `∨`, `¬` | Boolean only | Boolean |
| `indicator(condition)` | Boolean | Numeric 1 or 0 |
| Mixed numeric/Boolean operands | Not implicitly allowed | One consistent type error |

Use a small shared dispatch/validation layer, such as `expect_numeric`, `expect_boolean`, and `binary_numeric`, so existing arithmetic implementations continue receiving `Tensor` and do not each grow their own Boolean-specific branches. An incompatible expression should be rejected in that central layer, with a source-spanned error naming the expected type and received type. This is unavoidable type checking, but it should not be handwritten independently inside every mathematical algorithm.

## What this means for the matrix concern

- A matrix remains numeric-only. `mat(1 < 2, 3)` is rejected with a direct message; use `indicator(1 < 2)` when a numeric matrix entry is intended.
- Comparing numeric matrices is element-wise and returns a separate Boolean mask of the same shape. AND/OR on masks is element-wise and preserves shape.
- Numeric multiplication, exponentiation, and matrix dot products reject Boolean operands. No implicit conversion to 0/1.
- Boolean-mask matrix multiplication is **not** assigned a meaning silently. Standard arithmetic dot product on `indicator(A < x)` and `indicator(B > y)` is numeric arithmetic on 0/1 matrices. If Kalt later needs Boolean-semiring multiplication (OR of pairwise ANDs), add an explicitly named operation such as `bool_dot`, with separate tests.
- Complex equality is allowed; ordering requires real-valued operands. No lexicographic ordering for complex numbers.
- `true + 1` and `2 ^ false` are type errors. If users intend a numeric encoding, they write `indicator(true) + 1` and `2 ^ indicator(false)`.

This is a deliberate, easy-to-explain rule: *numbers do mathematics; Boolean values do logic; convert explicitly when crossing domains.*

## Syntax and precedence

Prefer unambiguous mathematical syntax: comparisons `=`, `≠`, `<`, `≤`, `>`, `≥`; logic `∧`, `∨`, `¬`. Do not overload `&` and `|` contextually. Kalt already uses vertical bars for absolute value, `!` for factorial, and `^` for exponentiation. If bitwise operations are ever added, use explicit names and define integer constraints separately.

Precedence (tightest to loosest): existing arithmetic, comparisons, NOT, AND, OR. Implement `1 < 2 < 3` as a comparison chain equivalent to `(1 < 2) ∧ (2 < 3)`; ordinary left-associative infix comparison is incorrect. Do not promise short-circuit evaluation until the evaluator has a lazy evaluation model.

## Floating-point equality

Choose deliberately. Recommended: exact equality for `=` on evaluated values, and separate approximate equality `≈` / `approx_eq(a,b)`. A hidden tolerance can make equality non-transitive.

## Why this differs from Wolfram Language

Wolfram Language models `True` and `False` as symbolic truth values and provides `Boole` for explicit conversion to 1/0. Its broader symbolic-expression architecture can preserve or manipulate expressions that are not ordinary numeric values. Kalt currently evaluates into numeric scalars and numeric matrices; adopting Wolfram's behavior literally would require a substantially broader symbolic value model, not just another Tensor variant. The recommendation here borrows the clear separation of Boolean and numeric domains, not Wolfram's complete symbolic architecture.

## Implementation plan

1. Validate these semantic boundaries before coding.
2. Introduce the evaluator-level `EvalValue` wrapper while keeping `Tensor` and existing matrix storage numeric-only; centralize type checks at operator dispatch.
3. Add comparisons and a dedicated comparison-chain parser.
4. Add Boolean logic over scalar bools and Boolean masks, including shape checks.
5. Add explicit Boolean-to-number conversion and define Typst binding/display behavior.
6. Add table-driven Typst tests for valid expressions, each invalid cross-domain combination, matrix masks and shapes, complex ordering, precedence/chains, conversion, and regressions for absolute value, factorial, exponentiation, and numeric matrix products.

## Research basis

- [Wolfram True](https://reference.wolfram.com/language/ref/True.html): `True` is a symbolic Boolean value, not a numeric 1.
- [Wolfram Boole](https://reference.wolfram.com/language/ref/Boole.html): explicit conversion of true/false to 1/0; listable conversion.
- [Wolfram Logic & Boolean Algebra](https://reference.wolfram.com/language/guide/LogicAndBooleanAlgebra.html): symbolic Boolean operations and distinct `&&` / `||` operators.
- [Wolfram matrix/tensor operations](https://reference.wolfram.com/language/tutorial/LinearAlgebraMatrixAndTensorOperations.html): list/matrix operations and shape semantics in a more general expression system.
- [Kalt numeric Tensor](https://github.com/Uhrendoktor/kalt/blob/main/rust/src/parser/atoms/tensor.rs): scalar complex or numeric matrix.
- [Kalt numeric Matrix](https://github.com/Uhrendoktor/kalt/blob/main/rust/src/parser/atoms/matrix.rs): `Array2<Complex<f64>>` storage.
