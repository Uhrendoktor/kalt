# Kalt

**Evaluate mathematics where you write it.**

Kalt brings numerical evaluation into Typst's math notation. It calculates expressions with complex numbers, vectors, and matrices, then returns values you can use in equations and document code. Real scalar results are Typst floats; complex, vector, and matrix results remain mathematical content.

## Try it in a document

Import Kalt from Typst Universe and wrap an expression in `comp`:

```typst
#import "@preview/kalt:0.1.1": comp

$comp(e^(i pi))$ // -1
$comp(sqrt((3 + 4i) (3 - 4i)))$ // 5

#let hypotenuse = comp($sqrt(3^2 + 4^2)$)
The hypotenuse is #hypotenuse.
#rect(width: hypotenuse * 1cm, height: 2mm)
```

The first two calls render evaluated mathematics. The last uses the calculated float as a value in the document's text and layout.

## More than arithmetic

- **Complex numbers:** imaginary unit `i`, conjugation, real and imaginary parts, complex powers, trigonometric and hyperbolic functions, and logarithms.
- **Vectors and matrices:** element-wise arithmetic, matrix multiplication, dot and cross products, transpose, indexing, and slicing.
- **Mathematical functions:** roots, factorials, binomial coefficients, absolute values, floor, and ceiling.
- **Typst integration:** pass booleans, numbers, Decimals, strings, or existing math content; use `map`, `reduce`, `any`, and `all` to work with matrix and vector values.

## One expression, evaluated in place

This example combines complex powers, a fraction, matrix multiplication, and a vector.

![The mathematical expression written in Typst](typst/assets/complex_equation.png)

![The evaluated vector result](typst/assets/complex_equation_result.png)

## Continue

- [First guide and API reference](typst/README.md)
- [Kalt on Typst Universe](https://typst.app/universe/package/kalt/)
- [Report a bug or request a feature](https://github.com/Uhrendoktor/kalt/issues)

Kalt is released under the [MIT License](LICENSE).
