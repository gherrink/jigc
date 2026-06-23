---
---

# Core Public API

## Overview

The public entry points of @ui-doc/core: the documentation engine, the parsers that turn source comments into a documentation context, and the error types they raise.

## Components

### UIDoc  {#uidoc}

The main documentation engine: ingests sources and emits the rendered documentation.

<!-- fields -->
- implemented-by: packages/core/src/UIDoc.ts#UIDoc

### CommentBlockParser  {#commentblockparser}

The default block parser: extracts comment blocks from source and builds the context via tag transformers.

<!-- fields -->
- implemented-by: packages/core/src/CommentBlockParser.ts#CommentBlockParser

### MarkdownDescriptionParser  {#markdowndescriptionparser}

Parses block descriptions written in Markdown into HTML.

<!-- fields -->
- implemented-by: packages/core/src/MarkdownDescriptionParser.ts#MarkdownDescriptionParser

### BlockParseError  {#blockparseerror}

Raised when a comment block cannot be parsed.

<!-- fields -->
- implemented-by: packages/core/src/errors/BlockParseError.ts#BlockParseError

### CSSParseError  {#cssparseerror}

Raised when CSS input fails to parse.

<!-- fields -->
- implemented-by: packages/core/src/errors/CSSParseError.ts#CSSParseError

### ColorParseError  {#colorparseerror}

Raised when a color value cannot be parsed; a specialization of CSSParseError.

<!-- fields -->
- implemented-by: packages/core/src/errors/ColorParseError.ts#ColorParseError

### TagTransformerError  {#tagtransformererror}

Raised when a tag transformer rejects its input.

<!-- fields -->
- implemented-by: packages/core/src/errors/TagTransformerError.ts#TagTransformerError

### BlockParser  {#blockparser}

The interface every block parser implements.

<!-- fields -->
- implemented-by: packages/core/src/BlockParser.types.ts#BlockParser
