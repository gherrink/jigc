---
status: accepted
date: 2020-10-16
supersedes: [adr:use-the-adr-number-as-its-unique-id]
---

# use-the-adr-slug-as-its-unique-id

## Context

ADR files were named NNNN-adr-title.md. When two developers each created a new ADR on separate branches, the incremental number collided and caused a git-merge conflict, because an ADR number must be unique.

## Decision

Stop using ADR numbers. An ADR is uniquely identified by its slug (filename without extension), with files named YYYYMMDD-adr-title.md; the date prefix keeps them sorted in the IDE. The core library owns sorting.

## Consequences

No more merge conflicts from colliding numbers; ADRs sort by the Date field, then git creation date, then file creation date, then slug.
