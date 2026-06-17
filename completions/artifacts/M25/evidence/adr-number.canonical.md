---
status: superseded
date: 2020-09-26
---

# use-the-adr-number-as-its-unique-id

## Context

An ADR needs a unique identifier — to build its web URL and to name it in CLI commands like edit or preview.

## Decision

Identify each ADR by its number — the numeric filename prefix (e.g. 0001-...). Titles can be duplicated and full filenames are long to type, whereas the number is already what tools like adr-tools use as the unique ID.

## Consequences

Each ADR is addressed by its number; the full filename could later be supported as a secondary CLI identifier.
