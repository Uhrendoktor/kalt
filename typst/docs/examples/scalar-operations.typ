#import "@preview/kalt:0.1.1": comp

#set page(width: 56mm, height: auto, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

$ 12 + 3 * 4 = #comp($12 + 3 * 4$) $
$ (2 + 3)^2 = #comp($(2 + 3)^2$) $
$ sqrt(81) = #comp($sqrt(81)$) $
$ abs(-7) = #comp($abs(-7)$) $
