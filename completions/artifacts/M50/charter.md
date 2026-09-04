# M50 — the last wave before 1.0.0 · CHARTER

**Preparation only.** Scope, claim, razor, forks and exclusions are here. **The decomposition is
not** — that is the planning session's Settle and plan, against a baseline that exercises the real
binary rather than this document. Written 2026-09-04 by the session that ran the trial, which is
exactly the position that produces confident wrong premises; every row below is a lead until the
baseline drives it.

**Chartered on** [the pre-v1 trial](../RC-m50/trial-record.md) on `1.0.0-rc.13` (`979baca`):
seven sessions across two transports, a 22-arm walk, the rc.12 → rc.13 migration pair. **One
finding blocks the 1.0.0 call**; twelve more confirmed and recorded; zero corruption, zero
regression; the headline **escalated at 1/3 and read by the human** (below). The human's criterion
for this wave, verbatim from [the handover that preceded the trial](../RC-m50/handover.md):

> everything that improves usability, routing, bug-fixes — everything that makes the product better
> and more acceptable at v1, and above all **everything that becomes impossible or expensive to
> change once people start using the product. Now it's cheap.**

---

## The claim

> **A door that destroys is guarded over every axis its id can take, and the two doors an agent
> meets first tell it the one state that changes what it should do next.**

Two halves, both falsifiable. The first: after this wave, no id-taking door reaches a filesystem
operation with an id the door never validated — the empty-id axis (25 doors, [walk 17](../RC-m50/evidence/walk-record-M50-walk-final.md))
is closed at its one seam, and `task discard` is disposed as what it is: a door that destroys
staged, uncommitted prose. If the baseline finds the seam is not one seam, the first half shrinks
to a list and says so. The second: `jigc start` in both its forms names an open task, and a worker
oriented over someone else's half-finished task has a read verb that answers *what is here* — or
this wave records, with the razor's citation, why it must not. If the next trial's duress cell does
not move, the second half was wrong about the cause.

**The evidence it rests on**, every row driven ([findings-verification.md](../RC-m50/findings-verification.md)):

- **W-13** — `jigc task discard ""` removes `.jigc/tasks/` at exit 0; `TaskArea::resolve`
  (`task.rs:763`) joins the root with `""` and `is_dir()` is true. Not in `DESTROYING_DOORS`.
- **W-16** — the same seam: `task validate ""` false-greens, `doc list --task ""` acks, four `""`
  cells have no route; 42 of 50 cells carry no code.
- **F-5** — `jigc start` and `jigc start "<intent>"` over one live task: zero mentions of it.
- **The duress cell at 1/3** — B2 (interactive) and B3-h2 went to the filesystem on their *first*
  read of the abandoned task's working area and said why in the same words: *one command to see the
  task's full state at once.* Every worker then repaired the doc correctly through jigc.

## Scope

### Tier 0 — blocks the 1.0.0 call

- **The empty-id class over its seam** (W-13 + W-16). The fix is at `TaskArea::resolve` or the
  parse layer above it, its test iterates walk 17's 25-door table over `""` and a wrong id, and
  every cell answers with a code and a route. Whether `task discard` also joins `DESTROYING_DOORS`
  is fork 1.

### Tier 1 — cheap now, expensive after the pin

- **F-5** the orientation and the router name an open task (and, if fork 2 lands it, the read
  verb that shows a task's staged area whole).
- **W-1** the milestone door's text render goes through the house renderer (`milestone.rs:4106`
  → `render.rs:2642`); the blocked-finalize text carries `blocking · <code> —` like every other door.
- **W-15** the section-only `set-field` miss mints `write.unknown-section` (`doc.rs` `Fragment::Unit`
  arm); `write_miss_shape_axis.rs` gains the row.
- **W-14** `placement-root .jigc` — refuse as a home *or* narrate tracked children at `uninstall`
  (fork 3); the axis is *tracked files under `.jigc/`*.
- **W-5** `read_pack`'s literal names the pack it read (four sites: `start.rs:3565`,
  `config.rs:223`, `config.rs:329`, `doc.rs:5643`).
- **`AddedNestedRepeatable`** — re-adjudicated under the criterion (fork 4): a shipped nested
  block no migration can reshape, routed into a file no adopter can edit.
- **F-6** a `ref` from `adr` to `research` — a frozen-doctype one-way door (fork 5).

### Tier 2 — verified wording and shape

- **F-1 / F-2** the `code-anchor` grammar stated where a worker looks (a `hint:` projected by
  `doc schema`, `set-field --help`, one SKILL.md line) and the `file:line` miss naming the grammar.
- **W-2** the probe-error refusal at `uninstall` over a file leftover: a route that fits a file,
  the sibling enumerated, `--force` named.
- **W-6** `house/vfs-local`; **W-7** `describe`'s origin pack (a contract-version question — fork 6);
  **W-8** whether `setup` says what the next door will refuse (fork 7); **F-7** the composed
  subject on `--dry-run`.

### Tier 3 — the conversion rows and the fan-out Fix phase

- Every `UNPINNED` row in the ledger is pinned by the increment that fixes it; two are owed
  regardless: `doc rename --task` preserving header fields and slot bodies; the milestone door's
  finding codes.
- **The fan-out Fix phase** ([decisions-pending.md](../../../implementation/decisions-pending.md)
  → *Chartered and cut*) is cut and waiting for a wave with ≥2 independent confirmed findings.
  M50 has them. Fork 8: take it here, so the completion audit's fixes run through the primitive
  instead of the prose rule that has failed three times.

## The razor

M46's three legs for a defect — **stated** in a locked artifact · **violated** at HEAD ·
**demonstrable** by driving — with M49's fourth for a shape: **necessary**, argued from an adopter's
day rather than from this repo's convenience. The human's criterion supplies the tiebreak for
shapes: *expensive after the pin* admits; *nice* does not. **If the razor cannot refuse, the claim
is wrong** — it must refuse at least the items below, with citations.

## Decided OUT, with the ground

- **The harness-surface wave's items** ([decisions-pending.md](../../../implementation/decisions-pending.md)
  → *The harness-surface wave*): build infrastructure, reaching no adopter. Its boundary paragraph
  already states this; T1-a was the one product item and it is Tier 0 here.
- **Measuring whether the dev tooling took** — M50's *build* transcripts, not a scope item.
- **The corpus template's `IngestQueue`** (PT-D) and the out-dir collision (I-2) — trial tooling.
- **The read-verb question decided by reflex** either way. The human's rule stands — *we do not
  bend the CLI to fit the project* — and here the project is three adopters' workers; the fork is
  argued, with an advocate, not assumed.

## Owed to the planning session

1. **Baseline against the binary, not this charter** — drive W-13 first; it is the cheapest
   premise to falsify and the one the whole first half rests on.
2. **The eight forks**, each with a robust-advocate where a cheap cut is on the table.
3. **The acceptance is the next trial's duress cell as much as flow 51**: the second half of the
   claim is measurable only by a blind worker over an abandoned task, and the protocol that
   measures it inherits everything RC-m50 fixed in its instrument (I-1..I-5).
