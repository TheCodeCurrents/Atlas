// Table styling and formatting

#let styled-table(columns, rows) = table(
  columns: columns,
  stroke: (x, y) => (
    top: if y == 0 { 1.5pt } else { 1pt },
    bottom: if y == rows.len() - 1 { 1.5pt } else { 1pt },
    left: none,
    right: none
  ) + rgb("#d1d5db"),
  fill: (x, y) => if y == 0 { rgb("#f3f4f6") } else if calc.odd(y) { rgb("#ffffff") } else { rgb("#fafbfc") },
  ..rows
)
