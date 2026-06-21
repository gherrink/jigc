---
---

# Core Public API

## Overview

The public entry points of @ui-doc/core: the documentation engine and the parsers that turn source comments into a documentation context.

## Components

### UIDoc  {#uidoc}

The main documentation engine: ingests sources and emits the rendered documentation.

<!-- fields -->
- implemented-by: packages/core/src/UIDoc.ts#UIDoc

### CommentBlockParser  {#commentblockparser}

The default block parser: extracts comment blocks from source and builds the context via tag transformers.

<!-- fields -->
- implemented-by: packages/core/src/CommentTagParser.ts#CommentTagParser

### MarkdownDescriptionParser  {#markdowndescriptionparser}

Parses block descriptions from Markdown into HTML.

<!-- fields -->
- implemented-by: packages/core/src/MarkdownDescriptionParser.ts#MarkdownDescriptionParser
