---
cites: [adr:compose-the-parser-as-discrete-stages]
---

# parser-subsystem

## Overview

The parser subsystem turns canonical source bytes into a structured document and back: section parsing builds the in-memory instance, the writer renders it to byte-stable canonical form.

## Components

### Section parser  {#section-parser}

Scans a document's source against its schema into the in-memory section/slot tree the rest of the pipeline operates on.

<!-- fields -->
- implemented-by: src/parser.rs#parse_sections

### Canonical writer  {#canonical-writer}

Renders a parsed instance back to canonical bytes, the round-trip-stable serialization the byte-stability invariant rests on.

<!-- fields -->
- implemented-by: src/writer.rs#render
