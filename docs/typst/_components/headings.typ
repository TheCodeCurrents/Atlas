// Document titles and heading utilities
#import "../_colors/base.typ": *

#let doc-title(title, version: "", date: "") = {
  set align(center)
  v(2.5em)
  text(
    size: 2.4em,
    weight: "bold",
    fill: rgb("#000000"),
    title
  )
  if version != "" [
    #v(0.8em)
    #text(size: 1em, fill: rgb("#6b7280"))[Version #version]
  ]
  if date != "" [
    #v(0.3em)
    #text(size: 0.9em, fill: rgb("#9ca3af"))[#date]
  ]
  v(3em)
  line(length: 100%, stroke: 1pt + rgb("#e5e7eb"))
  v(2em)
  set align(left)
}

#let section-marker(text-content, color: color-base) = block(
  inset: (x: 0.5em, y: 0.3em),
  fill: color.lighten(80%),
  stroke: (left: 3pt + color),
  text(
    size: 0.9em,
    weight: "bold",
    fill: color,
    text-content
  )
)
