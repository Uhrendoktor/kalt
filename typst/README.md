**Evaluate complex, vector, and matrix expressions.**

Kalt adds numerical evaluation to mathematical writing in Typst. Wrap an expression in `comp(...)` to calculate it where it appears. Real scalar results become Typst floats that you can reuse in code and layout; complex numbers, vectors, and matrices remain math content that can be typeset directly.

## Your first calculation

Import `comp` from Typst Universe, then wrap the expression you want evaluated:

```typst
#import "@preview/kalt:0.1.1": comp

$comp(e^(i pi))$ // -1
$comp(sqrt((3 + 4i) (3 - 4i)))$ // 5

#let hypotenuse = comp($sqrt(3^2 + 4^2)$)
The hypotenuse is #hypotenuse.
#rect(width: hypotenuse * 1cm, height: 2mm)
```

The first two expressions are evaluated and displayed as mathematics. The last expression returns a regular Typst float, so it can be used in further calculations, text, or layout.

## What `comp` accepts

The `comp(body)` function accepts existing math content as well as common Typst values:

| Input | Interpretation |
| --- | --- |
| Boolean | Converts `false` to `0` and `true` to `1`. |
| Integer, float, or Decimal | Evaluates the value as a numeric scalar. |
| String | Parses the string as a mathematical expression. Radix prefixes such as `0xFF` are supported. |
| Math content | Evaluates the existing mathematical expression, including matrices and vectors. |

```typst
#comp(7)                          // 7.0
#comp(2.5)                        // 2.5
#comp(decimal("3.14159"))         // 3.14159
#comp(true)                       // 1.0
#comp("0xFF")                     // 255.0
#comp("2 + 3 * 4")                // 14.0

#let matrix = math.mat(..((1, 2), (3, 4)))
#let vector = math.vec($1$, $2$, $3$)

#comp(matrix)
#comp(vector)
```

A real scalar result is returned as a Typst `float`. Complex scalars and matrix or vector results are returned as math content. A real `1 × 1` matrix is unwrapped to its scalar value.

## Arithmetic and functions

The basic arithmetic operators are:

| Operation | Notation |
| --- | --- |
| Addition | `+` |
| Subtraction | `-` |
| Multiplication | `*` |
| Division | `/` |
| Modulo | `%` or `mod` |
| Power | `^` |
| Factorial | `!` |

For matrices, these arithmetic operations work element by element. Use `dot` when you need matrix multiplication.

Kalt also supports these mathematical functions:

| Category | Functions |
| --- | --- |
| Roots and combinations | `sqrt(x)`, `root(n, x)`, `binom(n, k)` |
| Trigonometry | `sin(x)`, `cos(x)`, `tan(x)` |
| Inverse trigonometry | `arcsin(x)`, `arccos(x)`, `arctan(x)` |
| Hyperbolic functions | `sinh(x)`, `cosh(x)`, `tanh(x)` |
| Inverse hyperbolic functions | `arcsinh(x)`, `arccosh(x)`, `arctanh(x)` |
| Logarithms | `ln(x)`, `log_a(x)` |
| Scalar and tensor utilities | `abs(x)`, `floor(x)`, `ceil(x)` |

## Complex numbers

Use `i` for the imaginary unit. Complex conjugation uses a superscript star, `x^*`. The real and imaginary parts are available through `Re(x)` and `Im(x)`:

```typst
$comp(e^(i pi))$                  // -1
$comp((2 + 3i)^*)$                // 2 - 3i
$comp(Re(2 + 3i))$                // 2
$comp(Im(2 + 3i))$                // 3
```

![A complex-valued expression and its evaluated result](assets/evaluation.png)

## Matrices and vectors

Create a matrix with `mat(...)` and a column vector with `vec(...)`:

```typst
$comp(mat(1, 2; 3, 4))$
$comp(vec(1, 2, 3))$
```

| Notation | Operation |
| --- | --- |
| `dot` | Matrix multiplication or a vector dot product. |
| `×` | Cross product of three-dimensional vectors. |
| `^T` | Transpose. |
| `abs(x)` | Absolute value of a scalar, vector magnitude, or determinant of a square matrix. |

Matrix arithmetic is element-wise when dimensions match. A scalar can also be applied to every matrix entry.

```typst
$comp(2 * mat(1, 2; 3, 4))$ // mat(2, 4; 6, 8)
$comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8))$
$comp(vec(1, 0, 0) × vec(0, 1, 0))$
```

## Indexing and slicing

Matrix indices start at zero. Select a single entry, a complete row or column, or a range:

```typst
$comp(mat(1, 2; 3, 4)[0, 1])$ // 2
$comp(mat(1, 2; 3, 4)[:, 0])$ // first column
```

| Selection | Syntax |
| --- | --- |
| Single entry | `A[row, column]` |
| Whole row | `A[row, :]` |
| Whole column | `A[:, column]` |
| Range | `A[start:stop:step, :]` |
| Specific indices | `A[[0, 2], :]` |
| Reverse order | `A[::-1, ::-1]` |

A range uses `start:stop:step`. Leave a part empty to use the beginning or end of that dimension. Negative indices count from the end, and negative steps reverse the selected order.

```typst
$comp(mat(1, 2, 3; 4, 5, 6)[1:2, 0:2])$
$comp(mat(1, 2, 3; 4, 5, 6)[::-1, ::-1])$
```

An invalid index or slice is reported at the expression:

![An out-of-range slice with its inline error message](assets/error_reporting.png)

## Comparisons and Boolean operations

Comparisons use mathematical notation and produce numeric indicators: false is `0` and true is `1`.

| Comparison | Notation |
| --- | --- |
| Less than / greater than | `<`, `>` |
| Less than or equal / greater than or equal | `≤`, `≥` |
| Equal / not equal | `=`, `≠` |

You can also pass Typst booleans directly to `comp`. In a math expression, use `#` to insert a code value:

```typst
#comp(true)    // 1.0
#comp(false)   // 0.0

$comp(#true)$  // 1
$comp(#false)$ // 0
$comp(3 > 2)$  // 1
```

The logical and bitwise operators are:

| Notation | Meaning |
| --- | --- |
| `∧` | Bitwise AND. |
| `∨` | Bitwise OR. |
| `⊕` | Bitwise XOR. |
| `¬` | Boolean NOT; flips `0` and `1`. |
| `~` | Bitwise NOT for integers. |
| `⊙` | Boolean matrix product; an output entry is `1` when at least one matching pair of entries is `1`. |

```typst
$comp(6 ∧ 3)$ // 2
$comp(6 ∨ 3)$ // 7
$comp(6 ⊕ 3)$ // 5
$comp(¬0)$    // 1
$comp(~2)$    // -3
```

## Work with matrix values in Typst

Kalt provides functions for applying Typst callbacks to matrix and vector entries. Import only the helpers you use:

```typst
#import "@preview/kalt:0.1.1": comp, map, reduce, any, all

#let values = $mat(1, 2; 3, 4)$
#let shifted = map((value, x, y) => $#value + #x + #y$, values)
#let total = reduce((sum, value) => sum + comp(value), 0, values)

#any(value => comp(value) > 3, values) // true
#all(value => comp(value) > 0, values) // true
```

| Helper | Behavior |
| --- | --- |
| `map(fn, ..mats)` | Applies a callback to aligned entries and returns a matrix. The callback receives the entries, then the zero-based column (`x`) and row (`y`) positions. Numeric math content is evaluated with `comp` before it reaches the callback when possible. |
| `reduce(fn, initial, ..mats)` | Folds aligned entries into one value, starting with `initial`. The callback receives the accumulator followed by the entry from each input. |
| `any(fn, ..mats)` | Returns `true` when the predicate matches at least one set of aligned entries. |
| `all(fn, ..mats)` | Returns `true` only when the predicate matches every set of aligned entries. |
| `tensor-variant(tensor)` | Returns `"scalar"` or `"matrix"` for the parsed mathematical tensor. |

When input shapes differ, missing entries are passed to callbacks as `none`. The `map`, `reduce`, `any`, and `all` helpers all work across multiple inputs.

## Feedback and source

For questions, bug reports, or feature requests, use the [Kalt issue tracker](https://github.com/Uhrendoktor/kalt/issues). The source code is available on [GitHub](https://github.com/Uhrendoktor/kalt).
