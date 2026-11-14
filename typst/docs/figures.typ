#import "../lib.typ": comp, map, reduce, any, all

#set page(width: 190mm, height: 68mm, margin: 8mm)
#set text(size: 17pt)
#set par(leading: 0.9em)

#let figure = sys.inputs.at("figure", default: "matrix-input")

#let fahrenheit = $mat(68, 77; 86, 95)$
#let celsius = map((value, x, y) => (value - 32) * 5 / 9, fahrenheit)
#let average = reduce((sum, value) => sum + comp(value), 0, celsius) / 4
#let has-hot-reading = any(value => comp(value) >= 30, celsius)
#let all-above-freezing = all(value => comp(value) > 0, celsius)

#align(center)[
  #if figure == "matrix-input" {
    $ A = mat(1, 2; 3, 4) quad B = mat(5, 6; 7, 8) $
  } else if figure == "matrix-expansion" {
    $ A dot B
      &= mat(
        1 dot 5 + 2 dot 7, 1 dot 6 + 2 dot 8;
        3 dot 5 + 4 dot 7, 3 dot 6 + 4 dot 8
      ) $
  } else if figure == "matrix-result" {
    $ A dot B
      &= comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8)) $
  } else if figure == "vector-dot" {
    $ vec(1, 2, 3) dot vec(4, 5, 6)
      &= 1 dot 4 + 2 dot 5 + 3 dot 6 \\
      &= 4 + 10 + 18 \\
      &= comp(vec(1, 2, 3) dot vec(4, 5, 6)) $
  } else if figure == "vector-cross" {
    $ vec(1, 2, 3) × vec(4, 5, 6)
      &= vec(
        2 dot 6 - 3 dot 5,
        3 dot 4 - 1 dot 6,
        1 dot 5 - 2 dot 4
      ) \\
      &= vec(-3, 6, -3) \\
      &= comp(vec(1, 2, 3) × vec(4, 5, 6)) $
  } else if figure == "complex-product" {
    $ (2 + 3i)(4 - i)
      &= 8 - 2i + 12i - 3i^2 \\
      &= 11 + 10i \\
      &= comp((2 + 3i)(4 - i)) $
  } else if figure == "temperature-map" {
    $ F = mat(68, 77; 86, 95) quad
      C = #celsius $
  } else if figure == "temperature-average" {
    $ overline(C) = #average = 27.5 quad °C $
  } else if figure == "temperature-predicates" {
    $ any(C >= 30) = #has-hot-reading quad
      all(C > 0) = #all-above-freezing $
  }
]
