// Page layout and header configuration

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
