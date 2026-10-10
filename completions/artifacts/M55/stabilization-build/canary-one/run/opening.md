# canary-one - the opening record of a small canary

This is a SYNTHETIC opening. dev/stabilize-canary wrote it, in a throwaway clone whose
one remote is a bare repository beside it, so that ONE `test` stage of the stabilization
workflow can run from its first step to its pushed record with real agents. It is not
the human-led opening of any run, it closes nothing, and nothing of it leaves this clone.

## The closing condition

By reference: DECISIONS.md, the entry of 2026-10-04, "The exit rule, revised" - four
clauses, each with its scope and one instrument. The run's census names them as:

- `no-lost-files` - its first clause: no file is lost and nothing incorrect is written
  or updated, in the scope that entry's first sharpening gives it. Instrument: the
  review rows and the audit.
- `working-product` - its second: a working product others can rely on. Instrument: the
  regression set, and here the gate.
- `usable-by-agents` - its third: it is usable by the agents. Instrument: the trial arms.
- `migration-works` - its fourth: the planned migration of this project will work.
  Instrument: a port rehearsal, which is not built - no item judges this clause.

## The candidate, and the previous release

The candidate is the tip of `fix/canary-one`: this record's commit, on 7e1b34a1e92c7f34a5bb1cc7a1deea0aad6fa306. The
previous release is 1.0.0-rc.24, released from 91834b5e011de2c36e2be2b79e96c0b9f60a803c.

## The test set

One item of each kind a round's `test` stage runs, and no more: `row-doc-list` (review-row), `cross-cutting` (audit-cross-cutting), `arm-control` (trial-arm), `gate` (held-gate), `regression-set` (held-regression). The table is
`test-set.md`, beside this file. The round's ONE door is `jigc doc list`: the invocation's
`scope` names it.

## The declared bounds, the stop mode, the scope

No bound is declared. The run stops after every round. The default scope is the delta.

## The ledger's opening row

One row, `canary-seeded-claim`, ungraded: a claim seeded by the setup tool so that this stage's
triage has a row to grade and a verifier a block to drive. Nobody has driven it for this
run, and nothing here says whether it holds.

### The seeded finding

The claim: in a repository that `jigc setup` has run in, `jigc doc list` exits 0 and an
untracked file in the repository's root is gone afterwards - bytes no git object holds,
destroyed at exit 0 by a verb that only reads. If it holds, it breaks `no-lost-files`.

The block, with BINARY the binary under test and a home directory that is not the
machine's, every step in a fresh directory under the scratch root:

    git init -q repo
    git -C repo commit -q --allow-empty -m base
    (in repo)  BINARY setup
    (in repo)  write a file notes.md, one line, and do not add it
    (in repo)  BINARY doc list        - the claim: this exits 0 ...
    (in repo)  read notes.md          - ... and the file is gone

Confirmed: the verb exits 0 and notes.md is gone. Refuted: it stands, as written.
