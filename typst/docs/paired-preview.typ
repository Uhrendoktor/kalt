#let example_path = "examples/" + sys.inputs.example + ".typ"
#let example_source = read(example_path)
#let example_result = include(example_path)

#set page(width: 280mm, height: auto, margin: 7mm)
#set text(size: 9pt)

#grid(
  columns: (1fr, 1fr),
  gutter: 8mm,
  block(
    width: 100%,
    fill: rgb("#f5f7fa"),
    stroke: 0.6pt + rgb("#e1e7ef"),
    radius: 4pt,
    inset: 7mm,
  )[
    #text(size: 9pt, weight: "bold", fill: rgb("#475467"))[TYPST SOURCE]
    #v(4mm)
    #raw(example_source, lang: "typ")
  ],
  block(
    width: 100%,
    fill: rgb("#ffffff"),
    stroke: 0.6pt + rgb("#e1e7ef"),
    radius: 4pt,
    inset: 7mm,
  )[
    #text(size: 9pt, weight: "bold", fill: rgb("#475467"))[RENDERED RESULT]
    #v(4mm)
    #example_result
  ],
)
