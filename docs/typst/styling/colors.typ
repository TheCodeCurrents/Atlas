

// Primary color palette for Atlas documentation
#let color-base = rgb("#1e40af")      // Dark blue for base ISA
#let color-ext = rgb("#2563eb")       // Medium blue for extensions
#let color-note = rgb("#dc2626")      // Red for important notes
#let color-accent = rgb("#059669")    // Green for examples
#let color-warning = rgb("#ea580c")   // Orange for warnings
#let color-info = rgb("#0369a1")      // Cyan for info

// Utility function for adjusting colors
#let lighten(color, amount) = color.lighten(amount)
#let darken(color, amount) = color.darken(amount)