# Kalt for Typst

Kalt evaluates Typst math expressions containing scalars, complex numbers, vectors, and matrices. The main entry point is `comp(...)`. Matrix indexing and slicing make it possible to express calculations over selected regions without manually unpacking every cell.

## First use: evaluate a matrix stencil

This example computes the five-point discrete Laplacian of a sampled 2D field. It extracts four neighbor windows and the matching center window, combines them element-wise, then scales by the squared grid spacing.

~~~typst
#import "@preview/kalt:0.1.1": comp

#let field = $mat(
  10, 12, 15, 14, 11;
  13, 20, 25, 22, 15;
  14, 24, 35, 28, 16;
  12, 19, 27, 23, 14;
   9, 11, 16, 15, 10
)$
#let h2 = 0.25

// Matching 3×3 windows around every interior cell.
#let north = $[0:3, 1:4]$
#let south = $[2:5, 1:4]$
#let west  = $[1:4, 0:3]$
#let east  = $[1:4, 2:5]$
#let center = $[1:4, 1:4]$

#let laplacian = comp($(
  #field#north + #field#south + #field#west + #field#east
  - 4 * #field#center
) / #h2$)

$ nabla_h^2 T = #laplacian $
~~~

![Typst output of the sliced 2D Laplacian calculation](assets/guide/laplacian-slices.svg)

The input is a 5×5 matrix. Every slice is 3×3, so the four neighbor matrices align with the center matrix. The result contains the discrete Laplacian at the nine interior points:

```text
mat(-24, -32, -24;
    -32, -144, -64;
     -8, -60, -32)
```

The calculation is:

```text
ΔₕT = (Tnorth + Tsouth + Twest + Teast − 4 Tcenter) / h²
```

Here `h² = 0.25`. The slices use half-open ranges: `0:3` selects indices 0, 1, and 2, while `2:5` selects 2, 3, and 4.

## Operation reference

### Evaluate an expression with `comp`

`comp(body)` evaluates Typst math content and returns the computed value. Real scalar results can be used as Typst numbers; complex, vector, and matrix results remain mathematical content.

```typst
#import "@preview/kalt:0.1.1": comp

#let value = comp($sqrt(2)^2 + 3$)
```

Use `comp` around the complete expression you want evaluated. It also evaluates expressions containing matrix indexing, slices, and nested operations.

### Matrices and vectors

Write matrices and vectors using Typst's native math notation:

```typst
$mat(1, 2, 3; 4, 5, 6)$
$vec(1, 2, 3)$
```

A semicolon separates matrix rows. Commas separate entries within a row. Kalt represents `vec(...)` as a column vector.

### Indexing and slicing

Matrix indices are zero-based. An index pair selects a row and column:

```typst
#let entry = comp($mat(10, 20, 30; 40, 50, 60)[1, 2]$)
```

A range selects multiple entries. The stop index is excluded:

```typst
#let interior = comp($mat(1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 15, 16)[1:3, 1:3]$)
```

The two axes can be sliced independently:

| Expression | Selection |
| --- | --- |
| `M[1, 2]` | One entry at row 1, column 2 |
| `M[:, 0]` | First column |
| `M[2, :]` | Third row |
| `M[1:3, 1:3]` | A 2×2 interior block |
| `M[::2, ::2]` | Every second row and column |
| `M[::-1, ::-1]` | Reverse both axes |
| `M[[0, -1], :]` | First and last rows |

Ranges use `start:stop:step`. Omit a boundary to use the beginning or end of the axis. Negative indices count from the end; a negative step reverses the selected order. A slice may also select an explicit list of indices, such as `[0, -1]`.

### Matrix and vector operations

Kalt supports arithmetic on scalars and compatible matrices/vectors. Matrix addition and subtraction are element-wise, as is `*` when both operands are matrices of compatible shape. Scalar multiplication and division are also supported.

Use `dot` for matrix multiplication. Because vectors are column vectors, transpose the left operand for a vector dot product:

```typst
#import "@preview/kalt:0.1.1": comp

$ vec(1, 2, 3)^T dot vec(4, 5, 6)
  &= comp(vec(1, 2, 3)^T dot vec(4, 5, 6)) $
```

Use `×` for the three-dimensional vector cross product. Transpose uses `^T`; complex conjugation uses `^*`.

### Transform matrix data with `map`

`map(fn, ..mats)` applies a callback to corresponding cells and returns a matrix. The callback receives one cell from each input, followed by the zero-based column and row coordinates `x` and `y`.

```typst
#import "@preview/kalt:0.1.1": map

#let raw = $mat(1.02, 1.05; 0.98, 1.01)$
#let corrected = map(
  (value, x, y) => (value - 1) * 100,
  raw,
)
```

For real numeric cells, `map` supplies the evaluated number to the callback. Non-numeric content is passed through unchanged.

### Aggregate cells with `reduce`

`reduce(fn, initial, ..mats)` visits corresponding cells and updates an accumulator. Its callback receives the current accumulator followed by one cell from each input matrix. Use `comp(value)` when a callback needs a numeric value from a math-content cell.

```typst
#import "@preview/kalt:0.1.1": comp, reduce

#let total = reduce(
  (sum, value) => sum + comp(value),
  0,
  $mat(1, 2; 3, 4)$,
)
```

### Test cells with `any` and `all`

`any(fn, ..mats)` returns `true` when at least one set of corresponding cells satisfies the predicate. `all(fn, ..mats)` returns `true` only when every set satisfies it.

```typst
#import "@preview/kalt:0.1.1": all, any, comp

#let samples = $mat(12, 18; 24, 31)$
#let over_limit = any(value => comp(value) > 30, samples)
#let all_positive = all(value => comp(value) > 0, samples)
```

These callbacks receive math-content cells; call `comp(value)` to compare their numeric values. All four helpers can accept multiple matrices or vectors. Inputs are aligned by coordinate; a missing cell from a smaller input is passed as `none`.

### Complex numbers and mathematical functions

Kalt supports complex arithmetic using Typst's `i` notation, including conjugation, real and imaginary parts, powers, roots, logarithms, and trigonometric and hyperbolic functions.

| Category | Functions and notation |
| --- | --- |
| Roots and combinations | `sqrt(x)`, `root(n, x)`, `binom(n, k)` |
| Trigonometry | `sin(x)`, `cos(x)`, `tan(x)` |
| Inverse trigonometry | `arcsin(x)`, `arccos(x)`, `arctan(x)` |
| Hyperbolic | `sinh(x)`, `cosh(x)`, `tanh(x)` |
| Inverse hyperbolic | `arcsinh(x)`, `arccosh(x)`, `arctanh(x)` |
| Logarithms | `ln(x)`, `log_a(x)` |
| Other | `abs(x)`, `floor(x)`, `ceil(x)`, factorial `!` |

`abs(x)` returns the absolute value of a scalar, the length of a vector, or the determinant of a square matrix.

### Comparisons and bitwise operations

Comparisons evaluate to numeric indicators: false is `0`, true is `1`. Bitwise operations require integer values.

| Operator | Meaning |
| --- | --- |
| `=`, `≠`, `<`, `≤`, `>`, `≥` | Comparisons |
| `∧`, `∨`, `⊕`, `~` | Bitwise AND, OR, XOR, and NOT |
| `¬` | Boolean NOT |
| `⊙` | Boolean matrix product |

### Tensor type

`tensor-variant(value)` returns `"scalar"` or `"matrix"` for mathematical content.

## Reproducible figure

The opening example's SVG is generated from Typst source in [`typst/docs/figures.typ`](docs/figures.typ). Run [`typst/docs/render-figures.sh`](docs/render-figures.sh) with Typst 0.15.1 to regenerate it.
