# Typst Theme Module

The theming system for Atlas documentation.

## Structure

- `_settings/` - Page layout, typography, defaults
- `_colors/` - Color palette
- `_components/` - Reusable UI elements (badges, callouts, code blocks, etc)
- `init.typ` - Main entry point, imports everything

## Usage

Import in your documentation files:

```typst
#import "path/to/documentation.typ": *
```

This gives you access to all theme functions and colors.