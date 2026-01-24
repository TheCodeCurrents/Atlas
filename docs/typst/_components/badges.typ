// Extension badges and ISA markers
#import "../_colors/base.typ": *

#let extension(name, color: color-ext) = text(
  size: 0.85em,
  weight: "bold",
  fill: white,
  box(
    fill: color,
    inset: (x: 0.4em, y: 0.2em),
    radius: 3pt,
    name
  )
)

#let base-isa() = extension("Base ISA", color: color-base)
