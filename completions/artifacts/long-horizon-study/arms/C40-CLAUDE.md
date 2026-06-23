# Project working agreement

This is **UI-Doc**, a TypeScript/pnpm monorepo. `@ui-doc/core` turns
tag-comment blocks in source into a documentation context that a renderer emits
as component documentation.

## Documentation-consistency discipline

This project keeps its **documentation honest against the code**. Documentation
that names a code symbol which no longer exists is a defect — treat it with the
same seriousness as a failing test.

The public API is documented in two places that must stay in sync with the code:

- **`packages/core/README.md`** — the human-facing guide: tables, per-symbol
  sections, prose, and code examples.
- **`docs/architecture/core-public-api.md`** — the architecture doc: each
  component records the symbol that implements it as
  `implemented-by: <path>#<Symbol>`.

### The rule

Whenever you **rename, move, or remove a public symbol** (an exported class,
function, type, or interface — e.g. `UIDoc`, `CommentBlockParser`,
`MarkdownDescriptionParser`):

1. Update the **code** (the declaration and every usage, including factory
   functions and re-exports).
2. Update the **tests** that reference it.
3. Update **every documentation reference** to it, so no document names a
   symbol that is absent from the code. This includes:
   - the architecture doc's `implemented-by: …#<Symbol>` lines;
   - the README's section headings, prose mentions, **cross-references** (one
     symbol's docs referring to another), and **code examples**.

### How to be sure

After any such change, **search the whole project for the old name** and
confirm no documentation still references it, and that the architecture doc's
`implemented-by` anchors all point at symbols that exist in the code. A
documented symbol that does not resolve in the code is the failure mode this
discipline exists to prevent.
