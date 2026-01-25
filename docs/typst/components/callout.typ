// Callout boxes: notes, warnings, info callouts
#import "../styling/colors.typ": *

#let note(title, content) = block(
  fill: rgb("#fef2f2"),
  stroke: (left: 3pt + color-note),
  inset: 1em,
  [
    #text(weight: "bold", fill: color-note)[#title]
    #content
  ]
)

#let callout(type, content, color: color-info) = block(
  fill: color.lighten(85%),
  stroke: (left: 3pt + color),
  inset: 1em,
  [
    #text(weight: "bold", size: 0.9em, fill: color)[#type]
    #content
  ]
)

#let warning(title, content) = callout(title, content, color: color-warning)

#let info(title, content) = callout(title, content, color: color-info)
