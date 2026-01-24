// Atlas Documentation Module
// Provides unified styling, functions, and utilities for all Atlas documentation

// Configure page and document settings
#set document(
  title: "Atlas Documentation",
  author: "Atlas Project"
)

#set page(
  numbering: "1",
  margin: (top: 0.75in, bottom: 0.75in, left: 1in, right: 1in),
  header: context [
    #set text(size: 0.85em, fill: rgb("#6b7280"))
    #v(-0.3em)
    Atlas-8
    #h(1fr)
    #counter(page).display()
  ]
)

#set heading(
  numbering: "1.1"
)

#set text(
  font: "New Computer Modern",
  size: 11pt,
  fill: rgb("#1f2937")
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

// Color scheme
#let color-base = rgb("#1e40af")      // Dark blue for base ISA
#let color-ext = rgb("#2563eb")       // Medium blue for extensions
#let color-note = rgb("#dc2626")      // Red for important notes
#let color-accent = rgb("#059669")    // Green for examples
#let color-warning = rgb("#ea580c")   // Orange for warnings
#let color-info = rgb("#0369a1")      // Cyan for info

// Extension badge function
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

// Base ISA marker
#let base-isa() = extension("Base ISA", color: color-base)

// Note box function
#let note(title, content) = block(
  fill: rgb("#fef2f2"),
  stroke: (left: 3pt + color-note),
  inset: 1em,
  [
    #text(weight: "bold", fill: color-note)[#title]
    #content
  ]
)

// Callout function
#let callout(type, content, color: color-info) = block(
  fill: color.lighten(85%),
  stroke: (left: 3pt + color),
  inset: 1em,
  [
    #text(weight: "bold", size: 0.9em, fill: color)[#type]
    #content
  ]
)

// Code block with styling and syntax highlighting
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

// Instruction reference function
#let instruction(name, args, description, ext: none) = block(
  inset: 1em,
  stroke: (bottom: 1pt + rgb("#f3f4f6")),
  [
    #text(weight: "bold", size: 1.1em, fill: color-base)[#name] #if ext != none [#h(0.5em) #extension(ext)]
    #par[Args: `#args`]
    #par[#description]
  ]
)

// Table styling function
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

// Document title function
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

// Section markers
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
