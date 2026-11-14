# Kalt

**Evaluate mathematics where you write it.**

Kalt brings numerical evaluation to Typst's math notation. Write a formula, wrap it with <code>comp(...)</code>, and keep its computed result beside the original expression. Work with complex arithmetic, vectors, matrices, and mathematical functions without leaving your document.

## Try it in a document

Import <code>comp</code> from Typst Universe and evaluate expressions directly in an equation:

~~~typst
#import "@preview/kalt:0.1.1": comp

$ sqrt(3^2 + 4^2) &= comp(sqrt(3^2 + 4^2)) \\
  e^(i pi)         &= comp(e^(i pi)) $
~~~

Kalt evaluates the two expressions to <code>5</code> and <code>-1</code>.

A computed value can also drive Typst layout:

~~~typst
#let length = comp($sqrt(3^2 + 4^2)$)
#rect(width: length * 1cm, height: 2mm)
~~~

## More than arithmetic

- **Complex numbers:** conjugation, real and imaginary parts, complex powers, trigonometric and hyperbolic functions, and logarithms.
- **Vectors and matrices:** element-wise arithmetic, matrix multiplication, dot and cross products, transpose, indexing, and slicing.
- **Mathematical functions:** roots, factorials, binomial coefficients, absolute value, floor, and ceiling.
- **Typst integration:** use computed values in document layout, and transform or inspect matrix and vector data with <code>map</code>, <code>reduce</code>, <code>any</code>, and <code>all</code>.

## Worked calculations

The first guide develops matrix multiplication, vector products, complex arithmetic, and matrix transformations as step-by-step calculations, with equations rendered directly from Typst source.

[Read the guide and API reference](typst/README.md).

## Continue

- [Kalt on Typst Universe](https://typst.app/universe/package/kalt/)
- [Report a bug or request a feature](https://github.com/Uhrendoktor/kalt/issues)

Kalt is released under the [MIT License](LICENSE).
