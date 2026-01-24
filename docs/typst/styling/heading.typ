// Document titles and heading utilities
#import "./colors.typ": *

#let doc-title(title, version: "", date: "", author: "", overview: "") = [
  #set align(center)
  #v(3em)
  #text(
    size: 2.8em,
    weight: "bold",
    fill: rgb("#000000"),
    title
  )
  #v(2em)
  
  #if overview != "" [
    #set align(left)
    #text(size: 0.95em, fill: rgb("#374151"))[#overview]
    #set align(center)
    #v(2em)
  ]
  
  #if author != "" [
    #text(size: 1em, fill: rgb("#6b7280"))[#author]
    #v(0.5em)
  ]
  
  #if version != "" [
    #text(size: 0.95em, fill: rgb("#6b7280"))[Version #version]
  ]
  
  #if date != "" [
    #v(0.3em)
    #text(size: 0.9em, fill: rgb("#9ca3af"))[#date]
  ]
  
  #v(3em)
  #line(length: 100%, stroke: 1.5pt + rgb("#d1d5db"))
  #v(2em)
  #set align(left)
  #pagebreak()
]

#let toc() = [
  #heading(level: 1, "Table of Contents")
  #outline(indent: 1.5em)
  #pagebreak()
]

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
