# Kalt

**Evaluate real, complex, vector, and matrix expressions directly in Typst.**

Kalt evaluates Typst math with scalar and complex arithmetic, vectors, matrices, indexing, and mathematical functions. Each example below keeps its source as a real Markdown fenced Typst code block; the adjacent image is the plain, titleless SVG output compiled directly from that same `.typ` file by Typst. Every rendered example follows one top-to-bottom sequence, matching its Typst source without a multi-column grid.

## Nested scalar expressions and functions

Nested arithmetic, powers, logarithms, complex magnitude, rounding, factorial, and mixed bitwise operations. The expressions deliberately combine operators so precedence and grouping are visible.

<table>
<tr>
<td width="60%" valign="top">

````typst
#import "../../lib.typ": comp

#set page(width: 100mm, height: auto, margin: 3mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

$ ((2 + 3 * (4^2 - 5)) / (6 - 2)) = #calc.round(comp($((2 + 3 * (4^2 - 5)) / (6 - 2))$), digits: 2) $

$ abs((3 - 4i)^2 + 2 * ln(8) / ln(2)) = #calc.round(comp($abs((3 - 4i)^2 + 2 * ln(8) / ln(2))$), digits: 3) $

$ log_2(2^3 + 8) - ln(e^2) = #comp($log_2(2^3 + 8) - ln(e^2)$) $

$ floor((2^3 + 3^2) / 5) + ceil(7 / 3) = #comp($floor((2^3 + 3^2) / 5) + ceil(7 / 3)$) $

$ (3 + 2)! / (2^3 - 1) = #calc.round(comp($(3 + 2)! / (2^3 - 1)$), digits: 2) $

$ ((12 + 3 * 4) > (2^3 + 2)) ∧ 6 ⊕ 3 ∨ 2 = #comp($((12 + 3 * 4) > (2^3 + 2)) ∧ 6 ⊕ 3 ∨ 2$) $
````

</td>
<td width="40%" valign="top">

![Titleless Typst rendering of nested scalar arithmetic, functions, factorial, and bitwise precedence in a single vertical sequence.](typst/assets/readme/scalar-operations.svg)

</td>
</tr>
</table>

## Matrix operations

Element-wise addition, subtraction, multiplication and division, matrix multiplication, transpose, and matrix powers.

<table>
<tr>
<td width="60%" valign="top">

````typst
#import "../../lib.typ": comp

#set page(width: 100mm, height: auto, margin: 3mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

#let a = $mat(1, 2; 3, 4)$
#let b = $mat(5, 8; 9, 12)$

$ A = #a quad B = #b $

$ A + B = #comp($#a + #b$) $

$ A - B = #comp($#a - #b$) $

$ A * B = #comp($#a * #b$) $

$ B / A = #comp($#b / #a$) $

$ A dot B = #comp($#a dot #b$) $

$ A^T = #comp($#a^T$) $

$ A^2 = #comp($#a^2$) $
````

</td>
<td width="40%" valign="top">

![Titleless Typst rendering of matrix arithmetic, transpose, and matrix multiplication in a single vertical sequence.](typst/assets/readme/matrix-operations.svg)

</td>
</tr>
</table>

## Vector products

Column-vector transpose, dot product, cross product, and vector norms.

<table>
<tr>
<td width="60%" valign="top">

````typst
#import "../../lib.typ": comp

#set page(width: 100mm, height: auto, margin: 3mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

#let a = $vec(1, 2, 3)$
#let b = $vec(4, 5, 6)$
#let cross-product = comp($#a times #b$)

$ a = #a $

$ b = #b $

$ a^T dot b = #comp($#a^T dot #b$) $

$ a times b = #cross-product $

$ abs(a) = #calc.round(comp($abs(#a)$), digits: 3) $

$ abs(a times b) = #calc.round(comp($abs(#cross-product)$), digits: 3) $
````

</td>
<td width="40%" valign="top">

![Titleless Typst rendering of vector dot and cross products and their lengths in a single vertical sequence.](typst/assets/readme/vector-operations.svg)

</td>
</tr>
</table>

## Complex arithmetic

Complex division, conjugation, complex power, and magnitude.

<table>
<tr>
<td width="60%" valign="top">

````typst
#import "../../lib.typ": comp

#set page(width: 100mm, height: auto, margin: 3mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

#let voltage = $100 + 0i$
#let impedance = $3 + 4i$
#let current = comp($#voltage / #impedance$)
#let conjugate = comp($#current^*$)
#let power = comp($#voltage * #conjugate$)

$ V = #voltage " V" $

$ Z = #impedance Ω $

$ I = V / Z = #current " A" $

$ I^* = #conjugate " A" $

$ S = V I^* = #power " VA" $

$ abs(S) = #comp($abs(#power)$) " VA" $
````

</td>
<td width="40%" valign="top">

![Titleless Typst rendering of complex current and power calculations in a single vertical sequence.](typst/assets/readme/complex-arithmetic.svg)

</td>
</tr>
</table>

## Documentation

The [Typst operation reference](typst/README.md) documents the full supported syntax, including comparison and bitwise operators. The source files shown here are in [`typst/docs/examples/`](typst/docs/examples/).

## Build

```bash
cargo build --workspace
cargo build --release --target wasm32-unknown-unknown -p kalt_bindings
```

Regenerate the titleless SVGs directly from those Typst files with Typst 0.15.1:

```bash
bash typst/docs/render-examples.sh
```

Kalt is released under the [MIT License](LICENSE).
