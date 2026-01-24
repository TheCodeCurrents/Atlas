// Typography configuration: fonts, sizes, and heading styles

#set text(
  font: "New Computer Modern",
  size: 11pt,
  fill: rgb("#1f2937")
)

#set heading(
  numbering: "1.1"
)

#show heading: it => {
  set text(weight: "semibold", fill: rgb("#111827"))
  it
  v(0.4em)
}

#show heading.where(level: 1): it => {
  set text(size: 1.6em, weight: "bold", fill: rgb("#000000"))
  v(0.5em)
  it
  v(0.3em)
  line(length: 100%, stroke: 1.5pt + rgb("#d1d5db"))
  v(0.4em)
}
