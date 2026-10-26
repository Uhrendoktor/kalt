# Kalt

Kalt evaluates arbitrary nested equations and removes the need to transcribe
your equations into the default `calc` system. It supports scalars, complex
numbers, matrices, vectors, and matrix indexing. Real scalar results from
`comp` are returned as Typst floats; complex scalars and matrices remain
math content.

Complex equation?

![complex equation](assets/complex_equation.png)

```typst
$comp(frac(binom(2, 5) (1 + 3 i)^4.3 (2 - 3 i)^(2-5) 5!, sqrt((3+4i)(3-4i))) dot mat(
    1+i, 2, 0;
    0, 3-i, 0;
    0, 5, 7+i;
) dot vec(1, 2, 3))$
```

![complex equation result](assets/complex_equation_result.png)

## Import and start evaluating

```typst
#import "@preview/kalt:0.1.1": comp;
```

`comp` evaluates the supplied math content and returns Typst content. This means
the result can still be assigned, formatted, indexed, or combined with other
Typst expressions.

## Supported Operations

The following operations are currently supported by the Rust backend and
available through `comp`.

### Arithmetic

- `+` — addition. Scalars can be combined with matrices, and matrices are added
  element-wise when they have the same shape.
- `-` — subtraction with the same element-wise matrix rules.
- `*` — element-wise multiplication. Scalar-matrix multiplication is also
  applied element-wise.
- `/` — element-wise division.
- `%` or `mod` — modulo for real or complex operands, applied element-wise to matrices. For real operands, the remainder has the sign of the divisor. For complex operands, Kalt follows Wolfram Language: `a % b = a - b Floor[a / b]`, where `Floor` is applied separately to the real and imaginary parts.
- `^` — exponentiation.
- `!` — factorial.
- `binom(a, b)` — binomial coefficient. For matrices, both arguments must have
  the same shape and the operation is applied element-wise.
- `root(x, n)` — nth root.
- `sqrt(x)` — square root.
- `abs(x)` — absolute value for scalars, Euclidean norm for column vectors, and
  determinant for square matrices.
- `ceil(x)` — ceiling, applied element-wise to matrices.
- `floor(x)` — floor, applied element-wise to matrices.
- `sign(x)` — sign operation, applied element-wise to matrices.

```typst
$comp(sqrt((3+i)!)) dot pi/(e + 2.3^2)$
```

![evaluation of equation](assets/evaluation.png)

### Complex Numbers

Complex numbers are supported at any position in an equation.

- `conjugate(x)` — complex conjugate.
- `Re(x)` — real part.
- `Im(x)` — imaginary part.

```typst
$comp(e^(i pi))$ // => -1
$comp(conjugate(2 + 3i))$ // => 2 - 3i
$comp(Re(2 + 3i) + i Im(2 + 3i))$ // => 2 + 3i
```

### Matrices and Vectors

- `dot` — dot product and matrix multiplication for supported vector/matrix
  shapes.
- `cross` — cross product for three-dimensional vectors.
- `T` / `transpose` — matrix transpose.
- indexing — matrix element access and slicing.

```typst
$comp(mat(1/i+2, 0)^T dot mat(1, 2))$
```

### Indexing

Matrices can be indexed using Typst-style indices and slices. Ranges can use
steps and negative steps, making it possible to reverse or subsample a matrix.

```typst
$comp(mat(1,2;4,5)[::, 1])$ // => mat(2; 5)
$comp(mat(1,2,3;4,5,6)[1:2:, [2,1]])$ // => mat(6, 5)
```

### Trigonometric and Hyperbolic Functions

- `sin(x)` — complex sine; `sin^(-1)(x)`, `arcsin(x)`, and the short operator alias `op("asin")(x)` — principal inverse sine.
- `cos(x)` — complex cosine; `cos^(-1)(x)`, `arccos(x)`, and the short operator alias `op("acos")(x)` — principal inverse cosine.
- `tan(x)` — complex tangent; `tan^(-1)(x)`, `arctan(x)`, and the short operator alias `op("atan")(x)` — principal inverse tangent.
- `sinh(x)`, `cosh(x)`, and `tanh(x)` — hyperbolic sine, cosine, and tangent.

All these operations are applied element-wise to matrices and vectors.

### Other Built-in Functions

- `ln(x)` — natural logarithm.
- `log_a(x)` — logarithm of `x` with base `a`.

Real-valued scalar results are returned as Typst floats, so they can be used
directly in calculations outside math mode.

```typst
#let sine = comp($sin(pi / 2)$) // 1.0
$comp(ln(2^4))$ // => 4.0
$comp(log_2(16))$ // => 4.0
```

## Number Formats

Decimal, binary, and hexadecimal number literals can be used inside evaluated
expressions.

```typst
$comp("2.1e3"+1)$ // => 2101.0
$comp("0b010"*2)$ // => 4.0
$comp("0XFF"i/2)$ // => 127.5i
```

## Element-wise Operations and Mapping

Element-wise operations are supported by default for operations where the
backend defines an element-wise matrix implementation:

```typst
#let m = $mat(1,2,3;4,5,6)$
$comp(#m compose #m)$ // => mat(1, 4, 9; 16, 25, 36)
```

You can also apply custom mapping functions to each element in Typst.
The callback is the first argument, followed by one or more matrices. It
receives one element from each matrix, followed by its zero-based column
coordinate `x` and row coordinate `y`. Numeric matrix cells are passed to the
callback as Typst floats when possible; non-numeric content is preserved. The
callback's return value is kept as returned, including arbitrary content such
as highlights.

```typst
#import "@preview/kalt:0.1.1": comp, map;

$map(#{ (a, x, y) => $#a + #x + #y + 3$ }, mat(2, 2; 2, 6))$
```

The helpers `any` and `all` take a predicate followed by one or more
matrices or vectors. The predicate receives one element from each input at
the current coordinate; if shapes differ, missing cells are passed as `none`.
It must return a boolean. `any` returns true when at least one coordinate
satisfies the predicate, while `all` returns true only when every coordinate
satisfies it.

```typst
#import "@preview/kalt:0.1.1": any, all, comp;
#let m = $mat(1, 2; 3, 4)$
#any(value => comp(value) > 3, m) // => true
#any(value => comp(value) < 0, m) // => false
#all(value => comp(value) > 0, m) // => true
#all(value => comp(value) > 1, m) // => false
```

Or merge multiple matrices together:

```typst
#import "@preview/kalt:0.1.1": comp, merge;

$merge(#{ (a, b) => $log_#a (#b)$ }, mat(2, 2; 2, 2), mat(2, 2; 2, 8))$ // => mat(1, 1; 1, 3)
```

## Error Handling

Invalid operations and incompatible matrix shapes are returned as inlined Typst
error messages rather than causing the whole document to fail silently.

![error reporting](assets/error_reporting.png)

## Development

The heavy computation is implemented in Rust and exposed to Typst through Wasm.
The backend libraries for both `sertyp` and `kalt` are available as standalone
libraries and can be used independently.

The Typst-side tests are organized by operation under `tests/ops`. Shared test
helpers live in `tests/utils.typ`.

## Contributions

If an operation is missing, the usual contribution consists of implementing it
in the Rust backend, adding a Typst operation test, and updating the supported
operation list in this README. Improvements to error messages, matrix shape
handling, and number formatting are also useful contributions.
