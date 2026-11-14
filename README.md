# Kalt

**Evaluate mathematics where you write it.**

Kalt brings numerical evaluation to Typst's math notation. Write a formula, wrap it with `comp(...)`, and keep its result beside the original mathematics. Use it for complex arithmetic, vectors, matrices, and mathematical functions without moving to a separate calculator.

## Try it in a document

Import `comp` from Typst Universe and evaluate expressions directly in an equation:

```typst
#import "@preview/kalt:0.1.1": comp

$ sqrt(3^2 + 4^2) &= comp(sqrt(3^2 + 4^2)) \\
  e^(i pi)         &= comp(e^(i pi)) $
```

The results are `5` and `-1`.

Calculated numbers can also be used in Typst code. For example, use a computed length to size a shape:

```typst
#let length = comp($sqrt(3^2 + 4^2)$)
#rect(width: length * 1cm, height: 2mm)
```

## More than arithmetic

- **Complex numbers:** imaginary unit `i`, conjugation, real and imaginary parts, complex powers, trigonometric and hyperbolic functions, and logarithms.
- **Vectors and matrices:** element-wise arithmetic, matrix multiplication, dot and cross products, transpose, indexing, and slicing.
- **Mathematical functions:** roots, factorials, binomial coefficients, absolute value, floor, and ceiling.
- **Typst integration:** pass Booleans, numbers, Decimals, strings, or existing math content; use `map`, `reduce`, `any`, and `all` to work with matrix and vector values.

## One expression, evaluated in place

This example brings together complex powers, a fraction, a matrix product, and a vector.

![The mathematical expression written in Typst](typst/assets/complex_equation.png)

![The evaluated vector result](typst/assets/complex_equation_result.png)

## Continue

- [First guide and API reference](typst/README.md)
- [Kalt on Typst Universe](https://typst.app/universe/package/kalt/)
- [Report a bug or request a feature](https://github.com/Uhrendoktor/kalt/issues)

Kalt is released under the [MIT License](LICENSE).
