// Instruction and reference formatting
#import "../_colors/base.typ": *
#import "badges.typ": *

#let instruction(name, args, description, ext: none) = block(
  inset: 1em,
  stroke: (bottom: 1pt + rgb("#f3f4f6")),
  [
    #text(weight: "bold", size: 1.1em, fill: color-base)[#name] #if ext != none [#h(0.5em) #extension(ext)]
    #par[Args: `#args`]
    #par[#description]
  ]
)
