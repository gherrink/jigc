
## Renaming a managed decision — use `jigc rename`

To rename or retitle a managed decision (an ADR under `docs/decisions/`), use the
CLI-owned identity refactor — **never** `git mv` a managed doc or hand-edit its
slug:

```sh
jigc rename adr:<old-slug> --to "<New Title>"
```

One command derives the new slug from the title, rewrites the doc's H1, **repoints
every persisted referrer** (`supersedes:` / `cites:` across the whole store —
including referrers you never opened) old→new, `git mv`s the file, and commits the
whole thing as one atomic transaction. There is nothing left to chase by hand.

A bare `git mv` of a managed doc bypasses this and is **blocked by the pre-commit
hook** as an out-of-band rename — use `jigc rename` instead.
