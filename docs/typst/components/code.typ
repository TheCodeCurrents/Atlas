// Code block styling with dark theme

#let code-block(content, lang: "asm") = block(
  fill: rgb("#1e293b"),
  stroke: 1pt + rgb("#334155"),
  inset: 1em,
  radius: 4pt,
  text(
    font: "Courier New",
    size: 0.9em,
    fill: rgb("#e2e8f0"),
    content
  )
)
