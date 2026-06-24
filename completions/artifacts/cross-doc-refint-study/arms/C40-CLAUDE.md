# Project working agreement

This repository keeps its architectural **decision records** (ADRs) under
`docs/decisions/`, one per file, plus an architecture overview under
`docs/architecture/` that ties them together. Read this file before changing
any decision record.

## Cross-reference integrity discipline

This project keeps a set of **decision records** under `docs/decisions/` (one Architecture
Decision Record per file) and a **`docs/architecture/`** overview that ties them together.
These documents **reference each other**, and the references must always resolve:

- An ADR may record that it **supersedes** an earlier decision, via a
  `supersedes: adr:<slug>` field in its front-matter.
- The architecture overview **cites** the decisions that govern the system, via a
  `cites: [adr:<slug>, …]` field in its front-matter.

Each `adr:<slug>` names another decision file (`docs/decisions/<slug>.md`). A reference that
names a decision which no longer exists is a defect — treat a dangling cross-reference with
the same seriousness as a failing test.

### The rule

Whenever you **delete, rename, split, or otherwise restructure any decision record**, keep
**every cross-reference across all documents valid** — no `supersedes:` and no `cites:`
entry, in any file, may point at a decision slug that no longer exists. **This includes
references in documents you were not asked to touch.**

### How to be sure

After any such change, **search the whole `docs/` tree for the old slug** and confirm no
`supersedes:` or `cites:` field anywhere still names a decision that is now absent. Update or
remove every reference you find. A cross-reference that does not resolve to an existing
decision is the failure mode this discipline exists to prevent.
