#import "@preview/kalt:0.1.1": comp

#set page(width: 40mm, height: 30mm, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

#let a = $vec(1, 2, 3)$
#let b = $vec(4, 5, 6)$

$ "Dot product" quad a^T dot b = #comp($#a^T dot #b$) $

$ "Cross product" quad a times b = #comp($#a times #b$) $
