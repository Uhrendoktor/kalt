#import "@preview/kalt:0.1.1": comp

#set page(width: 46mm, height: 18mm, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

#let a = $mat(1, 2; 3, 4)$
#let b = $mat(5, 6; 7, 8)$

$ "Element-wise" quad A * B = #comp($#a * #b$) $

$ "Matrix product" quad A dot B = #comp($#a dot #b$) $
