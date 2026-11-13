# Kalt

**Mathematics that calculates inside your Typst document.**

Kalt turns expressions in your Typst math into evaluated results. Use it for calculations that are awkward to do by hand or with basic arithmetic alone—especially when your document includes complex numbers, matrices, or vectors.

Write the equation where you want to show it. Kalt calculates it there, and you can reuse the result elsewhere in the document.

## Try it

Import `comp` and wrap the expression you want to evaluate:

```typst
#import "@preview/kalt:0.1.1": comp

$comp(sqrt((3 + 4i) (3 - 4i)))$ // 5
$comp(e^(i pi))$                // -1

$comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8))$ // mat(19, 22; 43, 50)
```

A calculated real number can also be used like any other Typst value:

```typst
#let result = comp($sin(pi / 2)$)

The result is #result.
```

![A complex equation written in Typst](assets/complex_equation.png)

![The evaluated equation in the document](assets/complex_equation_result.png)

## Why Kalt?

- **Calculations stay in your document.** Keep formulas, explanations, and results together instead of moving between Typst and a separate calculator.
- **More than real-number arithmetic.** Calculate with complex numbers, matrices, and vectors.
- **Results stay useful.** Reuse real-number results in Typst code while keeping complex, vector, and matrix results in mathematical notation.
- **Errors are visible.** When an expression cannot be evaluated, Kalt reports the problem in the document.

## What you can do

| Category | Operations and examples |
| --- | --- |
| Arithmetic | `+`, `-`, `*`, `/`, `%`, powers, factorials |
| Complex numbers | `i`, `conjugate(x)`, `Re(x)`, `Im(x)` |
| Matrices and vectors | `mat(...)`, `vec(...)`, `dot`, cross product, transpose, determinant, vector norm |
| Mathematical functions | Roots, binomial coefficients, trigonometric and hyperbolic functions, logarithms, absolute value, rounding |
| Comparisons | Less than, greater than, equality, and inequality |
| Logical and bitwise operations | AND, OR, XOR, logical NOT, bitwise NOT |
| Indexing and slicing | Select entries, rows, columns, and ranges from matrices |
| Matrix helpers | `map`, `reduce`, `any`, and `all` |

### A few examples

**Complex numbers**

```typst
$comp(e^(i pi))$                  // -1
$comp(conjugate(2 + 3i))$         // 2 - 3i
$comp(Re(2 + 3i) + i Im(2 + 3i))$ // 2 + 3i
```

**Matrices and vectors**

```typst
$comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8))$
$comp(vec(1, 0, 0) × vec(0, 1, 0))$ // vec(0, 0, 1)
$comp(mat(1, 2; 3, 4)^T)$
```

**Functions**

```typst
$comp(sqrt(81))$       // 9
$comp(ln(e^2))$        // 2
$comp(sin(pi / 2))$    // 1
$comp(abs(3 + 4i))$    // 5
```

## Work with matrix values in Typst

Kalt also provides helpers for using matrix and vector entries in Typst callbacks. Import the helpers you need:

```typst
#import "@preview/kalt:0.1.1": comp, map, any, all

#let values = $mat(1, 2; 3, 4)$

#any(value => comp(value) > 3, values) // true
#all(value => comp(value) > 0, values) // true
```

- `map` applies a callback to corresponding entries and returns a matrix.
- `reduce` combines entries into one accumulated result.
- `any` checks whether a condition is true for at least one entry.
- `all` checks whether a condition is true for every entry.

