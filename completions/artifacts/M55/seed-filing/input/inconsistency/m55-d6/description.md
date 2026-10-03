`design/methodology-docs.md`'s doctype table shows a deferral-ledger entry's `kind` as `enum D/I`. The members have been `Decision` and `Idea` since M41 renamed them, which bumped the schema to version 2. The docs gap-detector found it.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a7a742d3`): **still open**. The table row moved from `:31` to `:30` and is unchanged, and `jigc doc schema deferral-ledger` prints `kind: enum [Decision|Idea]`.
