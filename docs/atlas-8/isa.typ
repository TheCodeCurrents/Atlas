// Import unified documentation module
#import "../typst/init.typ": *

// Apply theme settings (page/header/typography)
#atlas-setup()

// Document configuration
#let title = "Atlas-8 ISA Specification"
#let version = "1.0"
#let date = "January 2026"
#let author = "Atlas Project"
#let overview = "Complete instruction set architecture specification for the Atlas-8 processor, including the base instruction set and optional ISA extensions for caching and memory management."

#doc-title(title, version: version, date: date, author: author, overview: overview)

#outline()


= Overview

The Atlas-8 instruction set architecture (ISA) defines a machine model and the set of instructions that the Atlas-8 processor can execute.
This document provides a comprehensive specification of the ISA.

The Atlas-8 is a 8-bit RISC architecture designed for efficiency and simplicity, while supporting a variety of complex features like caching and memory management through optional extensions.
It uses fixed-length 16-bit instructions and a load/store architecture.

== Machine Model

The Atlas-8 processor has the following key components:
- 10 general-purpose registers (R0-R9), each 8 bits wide.