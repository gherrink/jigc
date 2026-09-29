# M50 — the input from the pre-v1 trial

The human's criterion for M50 ([handover.md](handover.md)): *everything that improves usability,
routing, bug-fixes — everything that makes the product better and more acceptable at v1, and
above all everything that becomes impossible or expensive to change once people start using the
product.* This file hands M50 the trial's yield **by §1 row**, each item keyed to its verified
row in [findings-verification.md](findings-verification.md), plus the items the handover carried.
It settles nothing; the Settle is M50's.

## Tier 0 — blocks the 1.0.0 call

- **W-13 · `jigc task discard ""` destroys `.jigc/tasks/`** at exit 0. The axis is the
  **empty-id class over 25 id-taking doors** (walk 17's table, [W-16]); the seam is
  `TaskArea::resolve` (`task.rs:763`), which every `task`/`doc --task`/`workflow --task`/`start
  --task` door passes through. The fix is owed over the class with a test that iterates the table
  — not a guard at one door. Three doors ack `""` today (`task validate` false-green, `doc list
  --task`, `task discard`); four `""` cells have no route at all (*"could not read the base pin"*).
  Whether `task discard` also joins `DESTROYING_DOORS` (it destroys staged, uncommitted prose by
  design) is a Settle question: the M48 guard's subject is *work no commit holds*, and a task's
  staged prose is exactly that.

## Tier 1 — cheap now, expensive after the pin

- **W-1 · the milestone door's text render** drops `blocking · <code> —` (`milestone.rs:4106`
  bypasses `render.rs:2642`). One renderer; a text-scraping driver keyed on the prefix.
- **W-15 · the section-only `set-field` miss** is the last bare cell of M49 T3 — mints a
  `(code, target)` key where none exists today; `write_miss_shape_axis.rs` gains the row.
- **W-14 · `placement-root .jigc`** — refuse it as a home (the `.git` guard's sibling, one
  predicate) *or* teach `uninstall` to narrate tracked children of `.jigc/`. Whichever, an adopter
  who chose `.jigc` before the change has a corpus the guard then refuses — cheaper before 1.0.0.
- **W-5 · `read_pack`'s literal** blames the embedded pack for a listed pack's missing resource,
  at four sites; the closure needs the pack identity. The first door a pack author meets.
- **F-5 · `jigc start` and `jigc start "<intent>"` do not name an open task** (B2). The two doors an agent
  meets first are silent about the one state that changes what it should do next; M43's resume line
  binds only the post-mint composition. One block on the orientation and the router — and the
  strongest single lead on the trial's escalated headline: three of four workers went to the
  filesystem to *orient over someone else's task*, and two of them said why (*one command for the
  task's full state*). Whether a read verb answers that — `doc list --task` is the index, `doc show
  --task` one doc — is M50's to decide under *we do not bend the CLI to fit the project*; here the
  project is three adopters' workers, not this repo.
- **The handover's item 2 · `AddedNestedRepeatable`** — re-adjudicate under the criterion: a
  shipped nested block (`changelog.releases/changes`) that no migration can reshape, with a route
  into a file no adopter can edit, is the definition of *expensive after*.

## Tier 2 — verified wording and shape, ships either way

- **W-2** the probe-error refusal at `uninstall` over a file-shaped leftover: route fits a
  directory, sibling leftover not enumerated, `--force` not named (class pinned by
  `leftover_probe_fail_closed.rs`; the three sub-claims not).
- **W-6** `house/vfs-local` on `--explain`. **W-7** `describe` carries no origin pack (capability,
  cheap to add as an additive key — but the additive-key window is closed, so it is a
  `contract-version` question; decide, do not drift).
- **F-1 / F-2 (B1's feedback)** — the `code-anchor` grammar is stated nowhere a worker looks (`doc schema`
  names the type, `set-field --help` and SKILL.md show no form), and the `file:line` miss says *"resolves
  to no file"* instead of naming the `path#Symbol` grammar. A `hint:` on the field type plus one message.
- **F-6 / F-7 (B2)** — no `ref` field from `adr` to `research` (a frozen-doctype one-way door; the razor's
  call), and `--dry-run` shows the manifest but not the composed subject line.
- **W-8** `setup` at 0/4 over a shape-changing shadow — declared bound; M50 may let the bootstrap
  door *say* what the next door will refuse without refusing itself.

## Tier 3 — the conversion rows the wave owes

Every `UNPINNED` row in the ledger is pinned by the wave that fixes it, and two rows are owed
regardless of a fix: **plant E's `doc rename` preserving header fields and slot bodies** (reached
by R3 and B3, asserted by no test — `doc_rename_in_task.rs` asserts identity movement only), and
**the milestone door's finding codes** (`HANDLED_COMMIT_LEAVES` pins which leaves gate, not which
code each raises).

## Carried from the handover, disposed

| item | disposition |
|---|---|
| 1 · `dev/gate` end-to-end fence asserting step + exit code | build infrastructure; the harness-surface wave's, not M50's — stated boundary holds |
| 2 · `AddedNestedRepeatable` | **Tier 1 above** |
| 3 · the fan-out Fix phase | cut and waiting; its prerequisite (`d854e25`) shipped and **held under the migration pair** (walk 14/21). M50 is the wave it belongs to if M50 runs a completion audit with ≥2 independent findings — which W-13 + W-15 + W-1 already are |
| 4 · measure whether the dev tooling took | M50's *build* transcripts — not this trial's |
| 5 · the shell-guard `rm` rule | the human's session; unchanged |
| 6 · T1-a | **re-described**: release = false green, debug = panic; **and it is the same class as W-13** — one entry, Tier 0 |

## What this trial did not reach, stated

Both interactive arms ran; every feedback claim held; the REFUTED set is empty. The headline's 1/3 is
escalated to the human in the record, not adjudicated here or there.
B4's seeded re-run is scored in [trial-record.md](trial-record.md). The §6 *neither* set is
four rows, each explained in [coverage.md](coverage.md).
