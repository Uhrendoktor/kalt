# Kalt for Typst

Kalt evaluates Typst math expressions containing real and complex scalars, vectors, and matrices. Its `comp(...)` function evaluates expressions with matrix indexing and slicing. Compatible matrix arithmetic is element-wise by default; use `dot` for matrix multiplication. The collection helpers `map`, `reduce`, `any`, and `all` provide callbacks and aggregation when a calculation needs them.

The [repository README](../README.md) pairs Markdown-fenced Typst source code with the titleless SVG output compiled directly from each matching `.typ` example file.

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

## Reproducible example renders

The four titleless SVG outputs in the repository README are compiled directly from the matching Typst documents in [`docs/examples/`](docs/examples/). The Markdown code blocks are the readable source listing; the SVGs contain only the rendered output from those source files. Run [`docs/render-examples.sh`](docs/render-examples.sh) with Typst 0.15.1 to regenerate the images.
