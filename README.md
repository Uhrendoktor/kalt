# Kalt

**Evaluate mathematical expressions directly in Typst—and use the results in the rest of your document.**

Kalt is for mathematical writing that goes beyond a few arithmetic operations. It evaluates nested expressions with complex numbers, vectors, and matrices while keeping the equations and their results in your Typst document.

## Start calculating

Import `comp` and place the expression you want evaluated inside it:

```typst
#import "@preview/kalt:0.1.1": comp

$comp(sqrt((3 + 4i) (3 - 4i)))$ // 5
$comp(e^(i pi))$                // -1
$comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8))$
```

Real-number results can be used as regular Typst values. For example, a calculation can determine the size of an element in your document:

```typst
#let side = comp($sqrt(3^2 + 4^2)$)
#rect(width: side * 1cm, height: 0.2cm)
```

Complex numbers, vectors, and matrices remain mathematical content, ready to display in your equations.
You can also pass Typst Booleans, integers, floats, and Decimals directly. Strings are parsed as mathematical expressions; for example, `comp("0xFF")` evaluates to `255.0`.

## From expression to evaluated result

This example combines complex arithmetic, powers, a fraction, a matrix, and a vector in one expression:

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

![The expression written in Typst](typst/assets/complex_equation.png)

![The evaluated vector result](typst/assets/complex_equation_result.png)

## Mathematical tools in your document

Kalt supports the notation you already use in mathematical writing.

### Arithmetic

| Operation | Notation |
| --- | --- |
| Addition | `+` |
| Subtraction | `-` |
| Multiplication | `*` |
| Division | `/` |
| Modulo | `%` or `mod` |
| Power | `^` |
| Factorial | `!` |

Other supported notation and operations include:

- **Complex numbers:** the imaginary unit `i`, conjugation `x^*`, and the real and imaginary parts `Re(x)` and `Im(x)`.
- **Matrices and vectors:** `mat(...)`, `vec(...)`, matrix multiplication with `dot`, cross product `×`, and transpose `^T`.
- **Indexing and slicing:** select an entry with `A[row, column]`, a row or column with `:`, or a range with `start:stop:step`.
- **Mathematical functions:** roots, binomial coefficients, trigonometric and hyperbolic functions, logarithms, absolute value, floor, and ceiling.
- **Comparisons and logic:** `<`, `>`, `≤`, `≥`, `=`, `≠`, Typst booleans `true`/`false`, `∧`, `∨`, `⊕`, `¬`, `~`, and Boolean matrix product `⊙`.
- **Matrix helpers in Typst:** `map`, `reduce`, `any`, and `all`.

The **[Typst usage guide](typst/README.md)** walks through these operations with their notation and examples, including matrix indexing and slicing.
