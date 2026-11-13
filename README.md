# Kalt

**Make the equations in your Typst document calculate themselves.**

Kalt evaluates mathematical expressions directly inside Typst. Write a formula once, get its result in the document, and reuse that result elsewhere. It is especially useful when your work goes beyond everyday arithmetic: complex numbers, vectors, matrices, and larger expressions.

## Try it

Import Kalt and wrap an expression in `comp(...)`:

```typst
#import "@preview/kalt:0.1.1": comp

$comp(sqrt((3 + 4i) (3 - 4i)))$ // 5
$comp(e^(i pi))$                // -1

$comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8))$
```

You can also use a result as an ordinary Typst value:

```typst
#let result = comp($sin(pi / 2)$)

The result is #result.
```

A real-number result can be reused in other Typst expressions. Complex numbers, vectors, and matrices remain beautifully typeset mathematics.

![A complex expression in a Typst document](typst/assets/complex_equation.png)

![The evaluated mathematical result](typst/assets/complex_equation_result.png)

## Why use Kalt?

- **Keep calculations with the explanation.** No switching to a separate calculator and copying numbers back into your document.
- **Go beyond basic arithmetic.** Work with complex numbers, matrices, vectors, and a broad range of mathematical functions.
- **Reuse what you calculate.** Feed numeric results into further calculations, text, tables, and other parts of your Typst document.
- **See problems where they happen.** Invalid expressions are reported directly in the document.

## What can you calculate?

| Area | Examples |
| --- | --- |
| Arithmetic | Addition, subtraction, multiplication, division, powers, modulo, factorials |
| Complex numbers | Complex arithmetic, conjugates, real and imaginary parts |
| Matrices and vectors | Matrix multiplication, dot and cross products, transpose, determinants, norms |
| Mathematical functions | Roots, trigonometric and hyperbolic functions, logarithms, rounding |
| Comparisons and logic | Equality, inequalities, and logical or bitwise operations |
| Indexing | Select individual matrix entries, rows, columns, or ranges |
| Typst helpers | Map, reduce, and test values across matrices and vectors |

For more examples and the available operations, see the **[Typst usage guide](typst/README.md)**.

