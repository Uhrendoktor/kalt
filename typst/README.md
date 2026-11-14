**Keep the equation. Get the answer.**

Kalt evaluates native Typst math content in place. Write an expression once and use its computed value in the same equation or elsewhere in your document. It works with real and complex numbers, vectors, matrices, nested operators, and slices—without translating your math into a second expression language.

## Get started

Import `comp`, then pass it the math content you want evaluated:

```typst
#import "@preview/kalt:0.1.1": comp

$ (2 + 3i)^2 = #comp($(2 + 3i)^2$) $
```

A real scalar result becomes a Typst number. Complex, vector, and matrix results remain native mathematical content, ready to typeset.

## See it in action

The examples below are complete Typst documents. The page settings only size the source and its matching preview; remove or adjust them when placing the expressions in an existing document.

### Nested expressions and operator precedence

Combine powers, functions, grouping, arithmetic, comparisons, and bitwise operators in one expression. Kalt parses the nested expression according to operator precedence.

<table width="100%" cellspacing="0" cellpadding="0">
<tr>
<td width="50%" valign="middle">

````typst
#import "@preview/kalt:0.1.1": comp

#set page(width: 72mm, height: 30mm, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

$ ((2 + 3 * (4^2 - 5)) / (6 - 2)) + log_2(2^3 + 8) - ln(e^2) = #comp($((2 + 3 * (4^2 - 5)) / (6 - 2)) + log_2(2^3 + 8) - ln(e^2)$) $

$ ((12 + 3 * 4) > (2^3 + 2)) ∧ 6 ⊕ 3 ∨ 2 = #comp($((12 + 3 * 4) > (2^3 + 2)) ∧ 6 ⊕ 3 ∨ 2$) $
````

</td>
<td width="50%" valign="middle">
<img src="assets/readme/scalar-operations.svg" alt="Nested arithmetic, logarithms, powers, comparisons, and bitwise operators evaluated from Typst math." width="100%">
</td>
</tr>
</table>

### Matrix algebra

Compatible matrices support element-wise arithmetic. Use `dot` when you want matrix multiplication.

<table width="100%" cellspacing="0" cellpadding="0">
<tr>
<td width="50%" valign="middle">

````typst
#import "@preview/kalt:0.1.1": comp

#set page(width: 50mm, height: 24mm, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

#let a = $mat(1, 2; 3, 4)$
#let b = $mat(5, 6; 7, 8)$

$ "Element-wise" quad A * B = #comp($#a * #b$) $

$ "Matrix product" quad A dot B = #comp($#a dot #b$) $
````

</td>
<td width="50%" valign="middle">
<img src="assets/readme/matrix-operations.svg" alt="Element-wise matrix multiplication and matrix dot product, rendered from Typst math." width="100%">
</td>
</tr>
</table>

### Vector products

Vectors are column vectors. Transpose the left operand for a dot product; use `times` for the three-dimensional cross product.

<table width="100%" cellspacing="0" cellpadding="0">
<tr>
<td width="50%" valign="middle">

````typst
#import "@preview/kalt:0.1.1": comp

#set page(width: 45mm, height: 22mm, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

#let a = $vec(1, 2, 3)$
#let b = $vec(4, 5, 6)$

$ "Dot product" quad a^T dot b = #comp($#a^T dot #b$) $

$ "Cross product" quad a times b = #comp($#a times #b$) $
````

</td>
<td width="50%" valign="middle">
<img src="assets/readme/vector-operations.svg" alt="Vector dot product and three-dimensional cross product, rendered from Typst math." width="100%">
</td>
</tr>
</table>

### Complex arithmetic

Complex values work throughout expressions, including division, powers, conjugation, and magnitude.

<table width="100%" cellspacing="0" cellpadding="0">
<tr>
<td width="50%" valign="middle">

````typst
#import "@preview/kalt:0.1.1": comp

#set page(width: 50mm, height: 30mm, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

#let voltage = $100 + 0i$
#let impedance = $3 + 4i$
#let current = comp($#voltage / #impedance$)
#let conjugate = comp($#current^*$)
#let power = comp($#voltage * #conjugate$)

$ V = #voltage " V" $

$ Z = #impedance Ω $

$ I = V / Z = #current " A" $

$ I^* = #conjugate " A" $

$ S = V I^* = #power " VA" $

$ abs(S) = #comp($abs(#power)$) " VA" $
````

</td>
<td width="50%" valign="middle">
<img src="assets/readme/complex-arithmetic.svg" alt="Complex voltage, impedance, current, conjugate, and power calculations." width="100%">
</td>
</tr>
</table>

## What Kalt evaluates

### Scalars and complex numbers

Use Typst's native math syntax for real and complex expressions. Kalt supports nested arithmetic, implicit multiplication, powers, factorials, roots, binomial coefficients, and comparison and bitwise operators.

| Group | Syntax |
| --- | --- |
| Arithmetic | `+`, `-`, `*`, `/`, implicit multiplication, `%` |
| Powers and products | `^`, `dot`, `times`, factorial `!` |
| Roots and combinations | `sqrt(x)`, `root(n, x)`, `binom(n, k)` |
| Logarithms | `ln(x)`, `log_a(x)` |
| Trigonometry | `sin`, `cos`, `tan`, and inverse functions |
| Hyperbolic | `sinh`, `cosh`, `tanh`, and inverse functions |
| Other functions | `abs(x)`, `floor(x)`, `ceil(x)`, `re(x)`, `im(x)` |
| Comparisons | `=`, `≠`, `<`, `≤`, `>`, `≥` |
| Bitwise | `∧` AND, `∨` OR, `⊕` XOR, `~` NOT |

Comparisons evaluate to numeric indicators: false is `0` and true is `1`. Bitwise operations require integer values. Boolean NOT is written `¬`; `⊙` is the boolean matrix product.

### Matrices, vectors, and indexing

Create matrices with `mat(...)` and column vectors with `vec(...)`:

```typst
#import "@preview/kalt:0.1.1": comp

$ comp(mat(1, 2; 3, 4)^T) $
$ comp(vec(1, 2, 3)^T dot vec(4, 5, 6)) $
```

For compatible matrices, `+`, `-`, `*`, and `/` operate element-wise. Use `dot` for matrix multiplication. Transpose is `^T`; complex conjugation is `^*`. The `abs` function returns the vector length for vectors and the determinant for square matrices.

Matrix indices are zero-based. The stop index in a slice is excluded, and negative indices count from the end.

| Expression | Selection |
| --- | --- |
| `M[1, 2]` | One entry at row 1, column 2 |
| `M[:, 0]` | First column |
| `M[2, :]` | Third row |
| `M[1:3, 1:3]` | A 2×2 interior block |
| `M[::2, ::2]` | Every second row and column |
| `M[::-1, ::-1]` | Reverse both axes |
| `M[[0, -1], :]` | First and last rows |

Slices support `start:stop:step`, omitted boundaries, negative steps, and explicit index lists.

### Transform and inspect matrix data

Kalt also provides helpers for cell-wise transformations and aggregation.

| Function | Purpose |
| --- | --- |
| `map(fn, ..mats)` | Transform aligned cells and return a matrix. The callback receives each cell followed by its zero-based column `x` and row `y`. |
| `reduce(fn, initial, ..mats)` | Fold aligned cells into an accumulator. Use `comp(value)` when the callback needs a number from a math-content cell. |
| `any(fn, ..mats)` | Return true when at least one aligned cell satisfies the predicate. |
| `all(fn, ..mats)` | Return true only when every aligned cell satisfies the predicate. |
| `tensor-variant(value)` | Return `"scalar"` or `"matrix"` for mathematical content. |
| `to-float-if-possible(value)` | Evaluate numeric math content to a float; otherwise return the original value. |

Example: convert a matrix of measurements to percentage deviations.

```typst
#import "@preview/kalt:0.1.1": map

#let measurements = $mat(1.02, 1.05; 0.98, 1.01)$
#let deviations = map((value, x, y) => (value - 1) * 100, measurements)
```

The aggregation helpers accept multiple matrices or vectors. Inputs are aligned by coordinate; a missing cell from a smaller input is passed as `none`.

## Errors and compatibility

Kalt reports evaluation and shape errors as formatted Typst errors, so an invalid operation is visible at the expression that caused it. Matrix and vector operations check compatible dimensions instead of silently discarding mismatched data.

## Documentation and source

- [Source code and issue tracker](https://github.com/Uhrendoktor/kalt)
- [Development examples](https://github.com/Uhrendoktor/kalt/tree/main/typst/docs/examples)
- [Typst package documentation guidelines](https://github.com/typst/packages/blob/main/docs/documentation.md)

Kalt is released under the [MIT License](LICENSE).
