# Project working agreement

This repository keeps its architectural **decision records** (ADRs) under
`docs/decisions/`, one per file, plus an architecture overview under
`docs/architecture/` that ties them together. These documents **reference each
other** by slug (`supersedes: adr:<slug>`, `cites: [adr:<slug>, …]`), and every
reference must always resolve to a decision file that exists. Read this file
before renaming or restructuring any decision record.

## Renaming a decision — the one-liner

When you **rename a decision record** (its slug changes — e.g. the file moves
from `docs/decisions/<old>.md` to `docs/decisions/<new>.md`), you must move the
file **and** repoint every cross-reference that named the old slug, **including
references in documents you were not asked to touch**. Do it in one shell motion
rather than hand-editing each referrer:

```sh
# rename adr:<old> -> adr:<new>: move the file, then repoint every referrer
git mv docs/decisions/<old>.md docs/decisions/<new>.md \
  && grep -rl --null -- 'adr:<old>' docs/ \
     | xargs -0 -r sd -- 'adr:<old>' 'adr:<new>'
# `sd` is the modern sed; if it is not installed, fall back to:
#   grep -rl -- 'adr:<old>' docs/ | xargs -r sed -i 's#adr:<old>#adr:<new>#g'
```

After running it, **`grep -r 'adr:<old>' docs/` must return nothing** — confirm no
`supersedes:` or `cites:` field anywhere still names the old slug. A
cross-reference that does not resolve to an existing decision is the failure mode
this discipline exists to prevent; treat a dangling reference as seriously as a
failing test.
