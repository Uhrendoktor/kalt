# Boolean and Comparison Operators: Typed Values with Numeric Promotion

**Status:** proposal for review; no evaluator code changes included.

## Recommendation

Keep Boolean values as a real value domain, available anywhere the parser can produce a value, including matrix entries. Do not immediately erase them into numeric 0/1, because that loses the information needed to choose Boolean-specific operations.

At the same time, allow Booleans to be promoted to numbers in ordinary numeric contexts. This avoids the main failure mode of strict typing: expressions such as `2^true` and `true + 1` need not fail just because one operand originated as Boolean.

The working model is:

- `Bool` is a first-class value type, with Boolean scalar and Boolean tensor/matrix results.
- `Numeric` remains Kalt's existing complex scalar / numeric matrix domain.
- A Boolean can be converted to numeric 0/1 when a numeric operation needs it.
- A numeric value does **not** automatically become a Boolean merely because its value is 0 or 1. The syntax and evaluated type determine the domain; never infer intent from values alone.

This preserves both information and composability, without changing Kalt's numeric matrix storage into an arbitrary heterogeneous array.

## Rules to validate

| Expression | Recommended result | Why |
| --- | --- | --- |
| `3 > 2` | Boolean `true` | Comparisons produce actual Boolean values. |
| `2^true` | `2^1 = 2` | Arithmetic context promotes Boolean to 0/1. |
| `true + true` | Numeric `2` | Arithmetic is ordinary arithmetic, not Boolean algebra. |
| `true = 1` | `true` | In a mixed Boolean/numeric comparison, promote the Boolean to 1. |
| `true ∧ false` | Boolean `false` | Logical operations remain in the Boolean domain. |
| `¬(5 > 0)` | Boolean `false` | Negate a proposition. |
| `¬5` | Clear type error | A number is not a proposition. Do not silently invent truthiness. |
| `A < B` | Boolean matrix | Element-wise comparison preserves Boolean type information. |

For `¬5`, treating every nonzero number as true is a possible *truthiness* policy, but it comes from programming-language conventions rather than Boolean algebra. It should only be adopted as a deliberate product decision, not as a side effect of encoding Booleans as 0/1. The recommended default for a math-first evaluator is to require a Boolean operand: `¬(5 != 0)` is valid, `¬5` is not. This avoids confusion with the nonsensical arithmetic complement `1 - 5 = -4`.

Wolfram Language similarly distinguishes symbolic truth values and provides `Boole` for explicit conversion. Its documentation says `Boole[expr]` returns 1/0 for `True`/`False` and remains unchanged if the argument is neither. [Wolfram Logic & Boolean Algebra](https://reference.wolfram.com/language/guide/LogicAndBooleanAlgebra.html.en), [Wolfram Boole](https://reference.wolfram.com/language/ref/Boole.html).

## Matrices: preserve the type from parsed entries

Every matrix element is parsed and evaluated, so matrix construction should inspect the *evaluated types*:

- If every element is Boolean (a Boolean literal, comparison, or logical expression), build a Boolean matrix.
- If every element is numeric, build the existing numeric matrix. A matrix written as `mat(0, 1)` is numeric; Kalt cannot infer that the author intended a Boolean matrix from the values alone.
- If a matrix mixes Boolean and numeric entries, use numeric promotion: convert Boolean entries to 0/1 and build a numeric matrix. This allows expressions such as `mat(1 < 2, 3)` to remain useful instead of raising a new class of matrix errors.
- For a Boolean matrix made from numeric 0/1 data, require an explicit constructor/conversion such as `boolmat(...)` or `as_bool(A)`. This marks intent unambiguously.

This makes the distinction visible: `mat(1 < 2, 3 < 4)` is Boolean, while `mat(1 < 2, 3)` promotes to numeric. Keep the existing `Array2<Complex<f64>>` storage for numeric matrices and introduce Boolean storage separately.

## Dot products: dispatch by the actual operand types

Two operations are mathematically legitimate, and they must not be confused.

### Numeric dot product

Ordinary arithmetic multiplication and summation. For `[1, 1] dot [1, 1]`, the result is `2`. This counts matching true entries if the numbers happen to be 0/1.

### Boolean dot product (recommended for two Boolean operands)

Use the Boolean semiring: pairwise AND followed by OR reduction.

`A ⊙ B = (a₁ ∧ b₁) ∨ ... ∨ (aₙ ∧ bₙ)`

For two Boolean vectors `[true, true]` and `[true, true]`, the result is `true`, because at least one pair overlaps. For matrices, each output cell is true iff there is at least one compatible index where both entries are true. This is the standard Boolean matrix product used in graph and relation calculations. See [Boolean matrices](https://ww2.coastal.edu/dweathers/edm/boolean-matrices.html) and [Boolean product](https://www.csd.uwo.ca/~abrandt5/teaching/DiscreteStructures/Chapter3/matrices.html).

**Dispatch rule:** if both dot operands are Boolean tensors, use Boolean-semiring multiplication and return Boolean values. If at least one operand is numeric, use ordinary numeric dot product, promoting Boolean entries to 0/1 as needed. Thus, a pure Boolean dot can return Boolean while an explicitly numeric 0/1 dot returns a count. The result depends on actual types, not guessing from matrix contents.

### Not the same as modulo 2

Modulo-2 dot product computes the parity of matching positions: two matches yield 0, one match yields 1. That's linear algebra over GF(2), where addition is XOR and multiplication is AND. Boolean-semiring multiplication uses OR instead of XOR: any number of matches greater than zero yields true. Do not silently use modulo 2 for Boolean dot product.

## Other operator behavior

- Arithmetic operators (`+`, `-`, `*`, `/`, `^`, numeric functions) accept Boolean operands by numeric promotion to 0/1 and return numeric values.
- Comparisons return Boolean values. Ordering is defined only for real-valued numeric operands; complex values with nonzero imaginary part have no ordering. Equality compares two Booleans logically; when one side is numeric, promote the Boolean to 0/1. Exact versus approximate numeric equality remains a separate decision.
- Logical `∧`, `∨`, `¬` accept Boolean values and return Boolean values. On Boolean matrices they operate element-wise and preserve shape. Numeric operands are not automatically interpreted as truthy/falsy; use comparisons such as `x != 0`.
- Ordinary arithmetic on two Boolean matrices promotes them to numeric matrices; only logical operations and Boolean dot operate in the Boolean domain. Numeric dot remains standard matrix multiplication.
- If support for GF(2) or bitwise operators is needed later, give it explicit names; do not conflate it with Boolean logic. Keep `&` and `|` out of context-sensitive overloading, particularly because vertical bars already denote absolute value in Kalt.

## Implementation plan

1. Validate this type/promotion model and the decision that `¬5` is an error rather than a truthiness test.
2. Introduce an evaluator-level value enum that can represent numeric tensors and Boolean scalar/tensor values. Keep existing numeric `Tensor` and numeric matrix operations largely unchanged.
3. Update matrix construction to infer Boolean matrices from all-Boolean evaluated entries, and promote mixed Boolean/numeric entries to numeric.
4. Add comparison operators and a dedicated parser for chained comparisons such as `1 < 2 < 3`.
5. Add Boolean logic and element-wise Boolean matrix operations.
6. Dispatch dot by operand domain: Boolean-semiring for Boolean × Boolean; ordinary arithmetic when either side is numeric, promoting Boolean operands to 0/1.
7. Specify Typst output behavior, then test expressions through the public Typst interface: `¬5`, arithmetic promotion, Boolean/numeric equality, all matrix-construction cases, Boolean dot vs numeric dot vs GF(2), shapes, complex ordering, precedence, and existing-operation regressions.

## Research basis

- [Wolfram Logic & Boolean Algebra](https://reference.wolfram.com/language/guide/LogicAndBooleanAlgebra.html.en): distinct symbolic truth values and logical operators.
- [Wolfram Boole](https://reference.wolfram.com/language/ref/Boole.html): explicit conversion of `True`/`False` to 1/0.
- [Wolfram Dot](https://reference.wolfram.com/language/ref/Dot.html): ordinary numeric/tensor dot products.
- [Boolean matrices](https://ww2.coastal.edu/dweathers/edm/boolean-matrices.html): Boolean dot defined as OR of pairwise ANDs.
- [Boolean product](https://www.csd.uwo.ca/~abrandt5/teaching/DiscreteStructures/Chapter3/matrices.html): Boolean matrix multiplication contrasted with ordinary matrix multiplication.
- [GF(2)](https://graphsearch.epfl.ch/en/concept/11137263): addition modulo 2 corresponds to XOR, distinct from OR in the Boolean semiring.
- [Kalt numeric Tensor](https://github.com/Uhrendoktor/kalt/blob/main/rust/src/parser/atoms/tensor.rs): currently represents complex scalars and numeric matrices.
- [Kalt numeric Matrix](https://github.com/Uhrendoktor/kalt/blob/main/rust/src/parser/atoms/matrix.rs): numeric matrix storage is `Array2<Complex<f64>>`.
