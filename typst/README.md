# Kalt

**Evaluate mathematics directly in your Typst document.**

Kalt keeps calculations alongside the equations and explanations in your document. It is useful for more than simple arithmetic: evaluate nested expressions, work with complex numbers, and calculate with vectors and matrices. The results can be reused elsewhere in Typst instead of being copied from a separate calculator.

## Start calculating

Import `comp` and wrap the expression you want evaluated:

```typst
#import "@preview/kalt:0.1.1": comp

$comp(sqrt((3 + 4i) (3 - 4i)))$ // 5
$comp(e^(i pi))$                // -1
$comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8))$
```

A real-number result is a regular Typst value. It can be used in further calculations or to control document content and layout:

```typst
#let side = comp($sqrt(3^2 + 4^2)$)
#rect(width: side * 1cm, height: 0.2cm)
```

You can pass scalar values, strings, and existing math content directly from Typst code. `comp` interprets each input according to its type:

| Input | Interpretation | Result |
| --- | --- | --- |
| Boolean | Converts `false` to `0` and `true` to `1`. | Typst float |
| Integer, float, or Decimal | Evaluates the value as a numeric scalar. | Typst float |
| String | Parses the text as a mathematical expression, including radix-prefixed numbers. | Typst float for a real scalar; math content otherwise |
| Math content | Parses the existing mathematical content. | Typst float for a real scalar; math content otherwise |

```typst
#comp(7)                      // 7.0
#comp(2.5)                    // 2.5
#comp(true)                   // 1.0
#comp(false)                  // 0.0
#comp(decimal("3.14159"))     // 3.14159

#comp("0xFF")                 // 255.0
#comp("2 + 3 * 4")             // 14.0

#let matrix = math.mat(..((1, 2), (3, 4)))
#let vector = math.vec($1$, $2$, $3$)

#comp(matrix)
#comp(vector)
```

Complex numbers, vectors, and matrices stay in mathematical notation so they can be shown as part of your equations.

## Arithmetic

Kalt supports the following arithmetic operations:

| Operation | Notation |
| --- | --- |
| Addition | `+` |
| Subtraction | `-` |
| Multiplication | `*` |
| Division | `/` |
| Modulo | `%` or `mod` |
| Power | `^` |
| Factorial | `!` |

For matrices, these arithmetic operations work entry by entry. A scalar can be applied to every entry, while two matrices are combined entry by entry when their dimensions match. Use `dot` when you mean matrix multiplication:

```typst
$comp(2 * mat(1, 2; 3, 4))$              // mat(2, 4; 6, 8)
$comp(mat(7, -7) mod 4)$                // mat(3, 1)
$comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8))$
```

## A complete mathematical expression

Kalt can evaluate a whole expression, even when it combines several kinds of mathematics. This example brings together complex powers, a fraction, a matrix product, and a vector:

```typst
#import "@preview/kalt:0.1.1": comp

$comp(
  frac(
    binom(2, 5) (1 + 3i)^4.3 (2 - 3i)^(2-5) 5!,
    sqrt((3 + 4i) (3 - 4i))
  )
  dot mat(
    1+i, 2, 0;
    0, 3-i, 0;
    0, 5, 7+i;
  )
  dot vec(1, 2, 3)
)$
```

![The expression written in Typst](assets/complex_equation.png)

![The evaluated vector result](assets/complex_equation_result.png)

## Complex numbers and functions

Use `i` for the imaginary unit. Kalt supports complex arithmetic and the usual ways of working with the real and imaginary parts of a value:

```typst
$comp(e^(i pi))$                  // -1
$comp((2 + 3i)^*)$                // 2 - 3i
$comp(Re(2 + 3i) + i Im(2 + 3i))$ // 2 + 3i
```

Write complex conjugation with a superscript star: `x^*`.

The supported functions are grouped below:

| Category | Functions |
| --- | --- |
| Roots and combinations | `sqrt(x)`, `root(n, x)`, `binom(n, k)` |
| Trigonometry | `sin(x)`, `cos(x)`, `tan(x)` |
| Inverse trigonometry | `arcsin(x)`, `arccos(x)`, `arctan(x)` |
| Hyperbolic functions | `sinh(x)`, `cosh(x)`, `tanh(x)` |
| Inverse hyperbolic functions | `arcsinh(x)`, `arccosh(x)`, `arctanh(x)` |
| Logarithms | `ln(x)`, `log_a(x)` |
| Scalar utilities | `abs(x)`, `floor(x)`, `ceil(x)` |

Factorial notation `x!` is listed with the arithmetic operators above.

For example, this expression combines a complex factorial, a square root, and a fraction:

```typst
$comp(sqrt((3 + i)!) dot frac(pi, e + 2.3^2))$
```

![A complex-valued expression and its evaluated result](assets/evaluation.png)

## Matrices and vectors

Create matrices with `mat(...)` and column vectors with `vec(...)`:

```typst
$comp(mat(1, 2; 3, 4))$
$comp(vec(1, 2, 3))$
```

Matrix and vector operations include:

| Notation | Operation |
| --- | --- |
| `dot` | Dot products and matrix multiplication. |
| `×` | Cross product of three-dimensional vectors. |
| `^T` | Transpose. |
| `abs(x)` | Absolute value of a scalar, length of a vector, or determinant of a square matrix. |

Scalar operations can also be applied to matrix entries. Element-wise operations can compare or combine matrices of matching dimensions.

## Indexing and slicing

Matrix indices start at zero. The main selection forms are:

```typst
$comp(mat(1, 2; 3, 4)[0, 1])$ // 2
```

| Selection | Syntax |
| --- | --- |
| Single entry | `A[row, column]` |
| Whole row | `A[row, :]` |
| Whole column | `A[:, column]` |
| Range | `A[start:stop:step, :]` |
| Specific indices | `A[[0, 2], :]` |
| Reverse order | `A[::-1, ::-1]` |

Select a whole row or column with `:`. For example, the first column is:

```typst
$comp(mat(1, 2; 3, 4)[:, 0])$
```

Negative indices count from the end, and a negative step reverses the selected order:

```typst
$comp(mat(1, 2, 3; 4, 5, 6)[1:2, 0:2])$
$comp(mat(1, 2, 3; 4, 5, 6)[::-1, ::-1])$
```

A range is written as `start:stop:step`. Leave a part empty to use the beginning or end of that dimension. A list such as `[0, 2]` selects specific rows or columns.

An invalid index is shown directly in the document rather than hidden:

```typst
$comp(mat(1, 2, 3; 4, 5, 6)[1:3:, [2, 1]])$
```

![An out-of-range slice with its inline error message](assets/error_reporting.png)

## Comparisons and Boolean operations

Comparisons use familiar mathematical notation and produce numeric indicators: false is `0` and true is `1`.

| Comparison | Notation |
| --- | --- |
| Less than / greater than | `<`, `>` |
| Less than or equal / greater than or equal | `≤`, `≥` |
| Equal / not equal | `=`, `≠` |

Pass Typst booleans directly to `comp`. In a math call, use `#` to embed the Boolean value:

```typst
#comp(true)    // 1.0
#comp(false)   // 0.0

$comp(#true)$  // 1
$comp(#false)$ // 0
$comp(3 > 2)$  // 1
```

The `#` is Typst's syntax for inserting a code value into a math call; no `op(...)` wrapper is needed.

Logical and bitwise operators are summarized here:

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

## Use matrix values in Typst

Kalt includes helpers for applying Typst callbacks to matrix and vector entries. Import the helpers you need:

```typst
#import "@preview/kalt:0.1.1": comp, map, reduce, any, all

#let values = $mat(1, 2; 3, 4)$

#any(value => comp(value) > 3, values) // true
#all(value => comp(value) > 0, values) // true
```

| Helper | Behavior |
| --- | --- |
| `map` | Applies a callback to every entry and returns the transformed matrix. The callback can also use each entry's column (`x`) and row (`y`) position. |
| `reduce` | Visits the entries and combines them into one accumulated result. |
| `any` | Checks whether a condition matches at least one entry. |
| `all` | Checks whether a condition matches every entry. |

```typst
$map((value, x, y) => $#value + #x + #y$, values)$
#reduce((total, value) => total + comp(value), 0, values) // 10
```
