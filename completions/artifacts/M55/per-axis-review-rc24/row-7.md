<!-- Reconciled ROW 7 file (pinned read contracts), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis7` in the body means ROW 7 of this run, not numbered axis 7. -->

<!-- RECONCILED ROW FILE — rc.24 partial per-axis re-review, ROW 7 · pinned read contracts.
     Part 1 is the Opus driver's table, UNCHANGED EXCEPT ONE DEMOTION (row G1, marked in place); its §1 and §6 counts are
     the driver's as written (213 numbered rows; 212 after the demotion). Part 2 — "Reconciliation ledger" and
     "Doors covered (reconciled)" at the end of the file — is the reconciler's. This row HAS a source pass
     (the Codex pass exited 0 and is non-empty). The reconciler authored neither the driver table nor the code. -->

<!-- rc.24 PARTIAL per-axis re-review — ROW 7 · pinned read contracts (= numbered axis 5, scoped) — THE OPUS DRIVER.
     Every row driven on the installed `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`. Release posture. -->

# rc.24 partial per-axis re-review — ROW 7 · pinned read contracts — THE OPUS DRIVER

**Binary.** `~/.local/bin/jigc` → **`jigc 1.0.0-rc.24`**, asserted before anything else ran (and again
inside every rig: `$JIGC --version` → `jigc 1.0.0-rc.24`). **Release posture** — the debug-only
route/span fences do not exist in this binary, so a fence violation would show as a bad emitted
byte, never a panic. Repository read at `bffa6667` (`work/rc24-gate`; the instrument was read at
`aa6666cb`, and the one commit between them touches `completions/trial-driver/` only).

**Scope.** Three doors — `jigc doc show` · `jigc doc list` · `jigc doc schema` — over the ten cells
of `axis7-driver-scope.md`, plus the baseline rows that scope names. **The other 45 `VERB_KINDS`
leaves are the numbered axis's door set and are not re-driven by this row**; two of them
(`milestone create`, `task amend`) are doors of a driven row only because the baseline's one
tier-1 row lives there.

**Instrument discipline.** Rigs: `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit;
eval "$rig"; [ -n "$REPO" ] || exit` — stdout only, two steps, the `$REPO` guard before every git
command (the assignments were saved to a scratch file and re-sourced per shell, with a second guard
that the cwd is a rig root). No teardown, no `rm -rf`; every scratch root is a `mktemp -d`. No exit
code was read through a pipe: every drive is `cmd > out 2> err; rc=$?`, then `jq` over the file.
Under `.jigc/` every claim uses `command grep` / `shasum` with a before-control. **`CLAUDECODE` is
set** in this session and was held constant; no key this row reads carries a commit message.
The working repository was checked after the last drive: `git status --porcelain` empty, HEAD
`bffa6667` unmoved.

**Fixtures.** Built by driving the binary, with three stated exceptions, each the condition under
test: (1) hand edits of **committed** managed docs followed by a plain `git commit` (the
out-of-band path the scope asks for: a deleted `status:` line, a deleted `# H1`, deleted section
headings); (2) plain foreign `.md` files written at a managed home; (3) one `mv` of a pack file out
of a `--pack-from-dev` throwaway pack copy into a `mktemp -d` stash, restored immediately. Nothing
was written into the gitignored `.jigc/` workbench; staged copies were read (`command grep`,
`shasum`) but never written by hand.

**Route kind.** The findings envelope carries no route-kind key, so the column below is my reading
of the emitted route text: **Mechanical** = the route carries a copy-runnable command; **Human** =
prose only; **Informational** = an advisory `note:` on stderr beside a served read; **none** = a
served read.

I did **not** read `axis7-prompt.md` or anything under `codex/`.

---

## 0 · The door set, derived from the code

Counts read **by symbol** at `bffa6667`. `ENVELOPE_ARMS` and `VERB_KINDS` were extracted
mechanically (a script over the const blocks), not transcribed.

| registry | file:line | what I read | what the instrument's author read | difference |
|---|---|---|---|---|
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs:6647` | **66** arms; 49 distinct `path` values = **48 leaves + 1 cross-cutting empty path** (the two `Reject::*` rows); **59 `Pinned` · 7 `Unpinned`**; outcome **61 `Success` · 3 `Adjudicated` · 2 `Reject`**; `doc show` **8** arms · `doc schema` **1** · `doc list` **1** | 66 arms over 48 leaves, 59 · 7, 8 · 1 · 1 | **none** against the instrument. **A datum against the rc.20 record**, which wrote `Pinned 60 / Unpinned 6` and `60 Success / 3 / 2` (which sums to 65): `git diff 4d3175c3..HEAD -- crates/cli/src/render.rs` moves **no** `ArmStatus::` and **no** `ArmOutcome::` line, so 59 · 7 and 61 · 3 · 2 were the registry's content on rc.20 too — the rc.20 figures were a miscount, not a later move. What the range did move in the table: `"title"` joins both `WholeDoc::*` shapes |
| `VERB_KINDS` | `crates/cli/src/cli.rs:1960` | **48** leaves (12 `Read` · 36 `Write`); the three doors are `Read` | 48 | none |
| `DOC_READ_VERBS` | `crates/cli/src/doc.rs:39` | **3**: `show` · `schema` · `list` | 3 | none |
| `WHOLE_DOC_KEYS` | `crates/cli/src/doc.rs:5710` | **7**: `type` · `slug` · `title` · `item-count` · `schema-version` · `fields` · `sections` (+ `STAGED_KEY = "staged"`) | 7 (+ `staged`) | none |
| `DocRow` | `crates/cli/src/doc.rs:4938` | **6** keys, in serialization order: `id` · `path` · `state` · `item-count` · `title` · `fields` | 6 | none |
| `SchemaContract.contract_version` | `crates/cli/src/doc.rs:5285` | **7** | 7 | none |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs:6032` | **4**: `store.not-found` · `store.no-such-leaf` · `store.unknown-type` · `store.fixed-identity` (each asked of its minting crate's const) | 4 | none |

**Shipped doctypes, counted by driving `doc schema` over every schema file of both packs: 18**
(dev 6 · methodology 12), every one `contract-version: 7`, every one the registry's seven keys.
**Three declare a `default:`**, all on `status`: `adr` (`proposed`), `inconsistency` (`open`),
`jigc-feedback` (`open`).

---

## 1 · Summary

| | |
|---|---|
| rows driven | **213** numbered rows (tables A – K below; each has a repro block in §4) |
| rows not driven | **13** entries (§5), each with its reason |
| **tier-1 findings** | **0** |
| tier-2 findings | 0 |
| tier-3 findings | **3** new — `(R7, D-1)`, `(R7, D-2)`, `(R7, D-3)` (§3) |
| baseline rows re-driven | 6 keys: **1 CLOSED (the tier-1 row, stays closed)** · **3 STILL-OPEN(expected)** · the M51 hostile-cwd family **CLOSED** on the three read verbs · the M52 lead **dispositioned** (§2) |
| declared == driven key set | **every** whole-doc serve (20 / 20), **every** `doc list` row (6 keys on every row of every state), `doc schema` on 18 / 18 doctypes |
| `doc show` arms driven | **8 / 8** — including `CompoundFieldSlice`, which the rc.20 record marked unreachable (§4 block B) |
| the read is a read | holds on every batch (§4 block G), with two write-controls that move the snapshot |

**No row of this run is an exit-0 loss or a repository harm.** The three doors are `Read` leaves;
nothing they did moved `git status --porcelain`, HEAD, `.jigc/state/file-state.json`,
`.jigc/index/edges.json` or any other byte under `.jigc/`.

---

## 2 · Baseline rows: CLOSED / STILL-OPEN

Baseline: `completions/artifacts/M53/per-axis-review-rc20/README.md` (numbered axis 5). Keys kept
verbatim.

| key | tier there | rc.24 status | the datum |
|---|---|---|---|
| **`(5, DEFECT 1)`** (rc.17) — a mint door commits a record at a fabricated identity | **1** | **CLOSED (stays closed)**, at both mint doors it names | block K: `milestone create ""` and `task amend ""` (and `"!!!"`, `"   "`) all exit **1** `write.unslugable-title`; HEAD unmoved, `git status` empty, no `.jigc/tasks`, no `.jigc/milestones`, no `docs/`, `task list` → *no active tasks* |
| **`(5, DEFECT 4)`** (rc.17) — `doc show` blocks a declared but unpopulated optional leaf (`store.no-such-leaf`) | 3 | **STILL-OPEN (expected — triaged to 1.x)** | block H rows H16, D18 – D24: `doc schema vision` still advertises `vision:<slug>#meta/grounded-in` `required: false`; `doc show 'vision:vision#meta/grounded-in'` → exit 1, key `{store.no-such-leaf, vision:vision#meta/grounded-in}`, *"names no leaf `grounded-in` in section `meta`"*. **What M55 changed beside it, precisely:** the **whole-doc** `fields` map and the **`doc list`** row now project an absent **defaulted** field at its default. **What did not change:** the `#section` fields-only slice still omits an absent field and the `#section/<leaf>` slice still blocks it — for an unpopulated optional leaf (`grounded-in`, `supersedes`, `about`) exactly as on rc.20, **and now also for a defaulted one**: on one doc, `doc show <doc>` says `fields.status = "open"` and `doc show <doc>#meta/status` says *names no leaf `status`* (rows D1 vs D18). That second half is **declared** (`design/doc-read-surface.md` → *The projection is the whole-doc serve's alone*; `design/findings-channel.md` §10), so it is not a new finding — but it widens the population this row's sentence is true of, and the message still calls a schema-declared leaf *no leaf* |
| **rc.17 `DEFECT A`** — `store.unknown-type` emits a URI-shaped target at `doc show`, the bare id at `doc schema` | 3 | **STILL-OPEN (expected)** | rows H1 – H3: `doc show 'nosuchtype:x'` → key target `nosuchtype:x`; `doc schema nosuchtype` and `doc list nosuchtype` → target `nosuchtype`. One code, two target shapes. (Also driven: `doc schema 'vision:vision'` / `doc list 'vision:vision'` → `store.unknown-type`, target `vision:vision` — the bare-id doors echo whatever token they were handed.) |
| **`(5, DEFECT 3)`** (rc.17) — the colon-less-address bail carries no code — **the `doc show` half only** | 3 | **STILL-OPEN (expected)** | row H6: `doc show nosuchtype` → exit 1, `Reject::Error`, `{"error":"malformed address `nosuchtype`: missing ':' between type and slug — …"}`; no `blocking · <code>`, no key. Same on `--task` (H28). The `rename` half was **not** re-driven (outside the subject) |
| M51 **`DEFECT A · B · C · D`** (the hostile-cwd sweeps) — the three read verbs' cells only | — | **still CLOSED** on `doc show` · `doc schema` · `doc list` | rows H48 – H59: outside any git repository, 5 / 5 `--format json` drives land in `Reject::Error` (`{"error":"not inside a git repository …"}`), exit 1, stdout 0 bytes; cwd deleted under the process, 3 / 3 land in `Reject::Error` with the one identical `{"error":"cannot determine the current directory: …"}`; plain arms carry the same sentence |
| §D lead (M52, axis 7): **`store.no-such-leaf`, the 2nd `ENVELOPE_OWED_CODES` member** (not driven on rc.16) | lead | **CLOSED as a lead — driven; the envelope the registry owes is paid** | the code was driven at **four producers** on both arms: an undeclared leaf in a non-repeatable section (H12, H31), a declared-but-unpopulated optional leaf (H16), an absent defaulted leaf (D18 – D21), an undeclared leaf of an item (H15, H30). Every one: exit 1, `Reject::Findings` (`findings` · `schema_version: 3`), stable key `{code, target}` with the address as typed, stdout 0 bytes, the JSON on stderr. The **lead** closes; **`(5, DEFECT 4)`**, which rides the same code, stays open — they are two questions and one drive answered both |

**Listed as NOT RE-DRIVEN (outside this row's subject) — they stay where rc.20 left them and their
absence here is not closure:** `(5, DEFECT 2)` · `(5, C1)` · `(5, D1)` · `(5, DEFECT 1 · rc.20)` =
`(2, A2-2)` · `(5, DEFECT 2 · rc.20)` · the `rename` half of `(5, DEFECT 3)`.
**`(5, C-13)`** and **`(5, C-18)`** remain **open leads, not drivable in this posture** — the first
is the wording of a doc-comment no invocation emits, the second is four `#[test]` targets on the
debug build (`crates/cli/tests/format_json_success_axis.rs`); neither was run to "confirm" a row.

---

## 3 · Defects (new on this run)

### `(R7, D-1)` · proposed **tier 3** — the two triage steps say `jigc doc list <type>` lists each record *"by its slug and its `status`"*; the command as printed lists no status

*Tier 3 because a composed surface states what a command prints and the binary prints something
else; nothing is lost and the route is not dead (`--format json` carries the value).*

- **Door / cell:** `doc list` · cell 6 (the plain arm against the JSON arm). Surface: the composed
  text of `triage-inconsistency` and `triage-jigc-feedback` (pack steps
  `author-inconsistency-triage`, `author-jigc-feedback-triage`) — **row 8 owns the composed
  surface; this row owns the door it names**, so it is recorded here once and flagged for row 8.
- **The contract contradicted:** the step's own sentence — *"The committed records, each by its
  slug and its `status`:"* followed by `jigc doc list inconsistency` (resp. *"The committed rows,
  each by its slug and its `status`:"* / `jigc doc list jigc-feedback`).
- **Observed:** the command, exactly as printed (default `agent` format), prints three columns —
  `id  path  state` — and `state` is the **registration** state (`managed`), not the doc's
  `status`. `status` reaches a reader only as `.docs[].fields.status` on `--format json`. A reader
  taking the step at its word reads `managed` in a column named `state` as the answer.

```
rig: fresh; one inconsistency landed through `report-inconsistency`, its status then hand-deleted
     and committed (so the effective status is the projected default `open`)
$ jigc start --workflow triage-inconsistency "triage the retry inconsistency"        -> 0
  … The committed records, each by
  its slug and its `status`:

  jigc doc list inconsistency
$ jigc doc list inconsistency                                                        -> 0
  id  path  state
  inconsistency:retry-budget-differs-between-readme  docs/inconsistencies/retry-budget-differs-between-readme.md  managed
$ jigc doc list inconsistency --format json   -> 0   .docs[0].fields.status = "open"   (the only arm that carries it)
$ jigc start --workflow triage-jigc-feedback "triage the staged note feedback"       -> 0
  … The committed rows, each by its
  slug and its `status`:

  jigc doc list jigc-feedback
```

### `(R7, D-2)` · proposed **tier 3** — on `--task`, the fragment-miss routes tell the reader to look at the **committed** doc, including for a doc that has no committed copy

*Tier 3 because the route names a copy the read did not serve; the code, key and exit are right and
the reader is not stranded.*

- **Door / cell:** `doc show --task` · cell 8 (an undeclared leaf / a missing item on the staged
  arm). Exit 1, codes `store.no-such-leaf` (two producers) and `store.no-such-item`.
- **The contract contradicted:** `design/doc-read-surface.md` → *What it reads*: a read-side block
  routes *"on **both** arms"*, and the staged arm's route must not claim the committed copy — the
  doc states the rule for `store.unparseable` in exactly these words: *"the shared tail claiming a
  staged doc is committed would be a lie"*. The three fragment-miss routes still carry the
  committed-arm tail on the staged arm.
- **Observed:** on a doc that exists **only** as a staged working copy (its task-less read answers
  `store.not-found`), all three routes say *committed*:

```
rig: fresh (second), task `second-open-task` (report-inconsistency):
     `jigc doc create inconsistency --title "Two docs disagree" --task second-open-task`  -> 0
     `jigc doc add-item 'inconsistency:two-docs-disagree#sides' --title docs/a.md --slug a --task second-open-task` -> 0
$ jigc --format json doc show 'inconsistency:two-docs-disagree#meta/nope'               -> 1
  key {store.not-found, inconsistency:two-docs-disagree}          <- control: nothing is committed
$ jigc --format json doc show 'inconsistency:two-docs-disagree#sides/nope' --task second-open-task      -> 1
  key {store.no-such-item, inconsistency:two-docs-disagree#sides/nope}
  route: "name an item that exists in the committed doc"
$ jigc --format json doc show 'inconsistency:two-docs-disagree#sides/a/nope' --task second-open-task    -> 1
  key {store.no-such-leaf, inconsistency:two-docs-disagree#sides/a/nope}
  route: "name a leaf that exists in the committed item"
$ jigc --format json doc show 'inconsistency:two-docs-disagree#meta/nope' --task second-open-task       -> 1
  key {store.no-such-leaf, inconsistency:two-docs-disagree#meta/nope}
  route: "name a field the committed section carries (a section's prose slot is the section itself — address it as `#<section>`)"
control (the same arm, a code whose route is arm-neutral):
$ jigc --format json doc show 'inconsistency:two-docs-disagree#nope' --task second-open-task            -> 1
  key {store.no-such-section, …}  route: "address one of the sections `inconsistency:two-docs-disagree` carries — `meta`, `sides`, …"
```

The same tail was driven where the two copies **differ** (rig `refs-post-hoc`: the staged vision
carries `grounded-in`, the committed one does not): `doc show 'vision:vision#meta/nope' --task
<task>` → *"name a field the committed section carries"*, while the field set that read actually
consulted is the staged one. Provenance not established — rc.20 did not drive these codes on
`--task`, so this may predate the range.

### `(R7, D-3)` · proposed **tier 3** — a `doc list` row prints an `id` that `doc show` refuses, and the refusal routes back to `doc list`

*Tier 3 because the row's `id` is not the address the surface says it is; the row's `state`
(`unregistered`) still points at adoption, so it is not a dead end.* **Outside the M55 delta** — the
identity leg is M50's — and recorded because it was driven while building cell 3's fixture; the
reconciler may read it as a declared bound of the M50 leg and demote it.

- **Door / cell:** `doc list` · cell 3 (`unregistered`, bytes that parse).
- **The contract contradicted:** `jigc doc list --help` — *"Every instance … carries its
  `<type>:<slug>` identity"*; `DocRow.id`'s own statement — *"the address every `doc` verb takes"*;
  and the rule `design/doc-read-surface.md` states for the orphan row — *"never a synthesized
  `<type>:<slug>`, which would be an address `doc show` refuses"*.
- **Observed:**

```
rig: fresh; a stamped, schema-conformant inconsistency written by hand at
     docs/inconsistencies/Not_A_Slug.md and committed with plain git
$ jigc doc list --format json                                                  -> 0
  {"id":"inconsistency:Not_A_Slug","path":"docs/inconsistencies/Not_A_Slug.md","state":"unregistered",
   "item-count":0,"title":"Stamped but not a slug","fields":null}
$ jigc --format json doc show 'inconsistency:Not_A_Slug'                       -> 1
  {"error":"blocking · store.malformed-slug — \"Not_A_Slug\" is not a valid doc slug — the `<slug>` head of
            address `inconsistency:Not_A_Slug`\n  route: `jigc doc list` lists the committed docs and the
            identity each one carries; use lowercase letters, digits, and single hyphens …"}
```

The `title` / `fields` values on that row are **as declared** (`fields` is `null` on an
`unregistered` file whose bytes parse) — the defect is the `id` cell alone.

---

## 4 · The (door, cell) table and its repro blocks

Row schema: `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted, verdict}`.
`…` in an argv is the rig's task id or slug, spelled out in the block. Every row's argv ran on the
installed binary. **M** = matches contract.

### A · cell 1 — `doc show`, whole doc: the top-level key set

Declared (`WHOLE_DOC_KEYS`): `fields, item-count, schema-version, sections, slug, title, type`
(sorted), `+ staged` on a staged serve. All rows: door `doc show`, `--format json`, exit 0, code
none, route none, stdout one JSON document.

| # | cell (doctype · shape class · arm) | rig | driven key set | `title` | verdict |
|---|---|---|---|---|---|
| A1 | `inconsistency` · per-instance `id-from: title`, repeatable `sides` · committed | fresh | the 7 | the H1 | M |
| A2 | `jigc-feedback` · per-instance `id-from: title` · committed | fresh | the 7 | the H1 | M |
| A3 | `inconsistency` · staged (`--task`, never committed) | fresh | the 7 + `staged` | the H1 | M |
| A4 | `jigc-feedback` · staged | fresh | the 7 + `staged` | the H1 | M |
| A5 | `commit:<task>` · transient · staged | fresh | the 7 + `staged`; `schema-version: null` | the staged doc's H1 (the task id) | M |
| A6 | `vision` · singleton, placement at the repo root · committed | committed-singletons | the 7 | `Vision` | M |
| A7 | `roadmap` · singleton, placement under `docs/`, multi-slot repeatable · committed | committed-singletons | the 7 | `Roadmap` | M |
| A8 | `decisions-log` · singleton, placement · committed | committed-singletons | the 7 | `Decisions Log` | M |
| A9 | `changelog` · singleton, placement, nested repeatable · committed | committed-singletons | the 7 (`item-count: 2`, `schema-version: 2`) | `Changelog` | M |
| A10 | `vision` · staged copy of a committed doc | refs-post-hoc | the 7 + `staged` | `Vision` | M |
| A11 | `research` · per-instance · committed | refs-post-hoc | the 7 | `Context Loss` | M |
| A12 | `adr` · per-instance, pre-M55 · committed | fresh (second) | the 7 | the H1 | M |
| A13 | `adr` · staged at create | fresh (second) | the 7 + `staged` | the H1 | M |
| A14 | `milestone-record` · per-instance, compound `base` · committed | migrated | the 7; `fields.base` an object `{sha, short}` | `alpha-wave` | M |
| A15 | `vision` landed through `migrate … --approve` · committed; bare `doc show vision` byte-identical to `vision:vision` | migrated | the 7 | `Vision` | M |
| A16 | `jigc-feedback` · committed, **H1 removed by hand** | fresh | the 7; **`title: null`**, key present | `null` | M |
| A17 | the same doc on `--task` (staged copy predates the removal) | fresh | the 7 + `staged` | the H1, from the staged bytes | M |
| A18 | `adr` · committed, no H1, a fenced `# …` line in a slot | fresh (second) | the 7; `title: null` (the fence is skipped) | `null` | M |
| A19 | `inconsistency` · committed, conformant bytes with **no stamp** (v0-era by the discriminator) | fresh | the 7; **`schema-version: null`**, and `fields` carries no `schema-version` (never synthesized) | the H1 | M |
| A20 | `vision` named bare (`doc show vision`) | committed-singletons | the 7 | `Vision` | M |

```
rig: fresh
$ jigc start --workflow report-inconsistency "the retry doc and the retry code disagree"   -> 0  task minted: retry-doc-and
$ jigc doc create inconsistency --title "Retry budget differs between README and client" --task retry-doc-and --format json
  -> 0 {copied_in:false, existed:false, findings:[], op:"create", target:{doctype:"inconsistency", slug:"retry-budget-differs-between-readme"}}
$ jigc doc set-field '…#meta/kind' --value code-doc --task retry-doc-and                   -> 0
$ jigc doc add-item '…#sides' --title README.md --slug readme --task retry-doc-and         -> 0   (and `client`, `adr`: three sides)
$ jigc doc set-slot '…#sides/readme/says' | '…#sides/client/says' | '…#description' --from-file - --task retry-doc-and   -> 0 each
A3 $ jigc doc show inconsistency:retry-budget-differs-between-readme --task retry-doc-and --format json   -> 0
   keys ["fields","item-count","schema-version","sections","slug","staged","title","type"]
   title "Retry budget differs between README and client"  item-count 3  schema-version 1  staged "retry-doc-and"
   fields {"date":"2026-10-03","kind":"code-doc","schema-version":"1","status":"open"}
A5 $ jigc doc show commit:retry-doc-and --task retry-doc-and --format json                 -> 0
   {"fields":{"scope":"","type":""},"item-count":0,"schema-version":null,"sections":{"body":"","summary":"","trailers":[]},
    "slug":"retry-doc-and","staged":"retry-doc-and","title":"retry-doc-and","type":"commit"}
$ (commit doc filled) jigc task finalize retry-doc-and   -> 0  finalized a13a1b2 … promoted docs/inconsistencies/retry-budget-differs-between-readme.md
A1 $ jigc doc show inconsistency:retry-budget-differs-between-readme --format json         -> 0
   keys ["fields","item-count","schema-version","sections","slug","title","type"]   (no `staged`)
   sections.sides [{"id":"readme","says":"Three retries.","title":"README.md"},{"id":"client",…},{"id":"adr","says":"","title":"adr:retry-policy#decision"}]
$ jigc start --workflow report-jigc-feedback "doc list note names a task with no doc of the type"   -> 0  task minted: doc-list-note-names
$ jigc doc create jigc-feedback --title "Staged note names the wrong task" --task doc-list-note-names   -> 0
$ jigc doc set-field '…#meta/{kind,found-in,jigc-version,about}' …                         -> 0 each
A4 $ jigc doc show jigc-feedback:staged-note-names-the-wrong --task doc-list-note-names --format json   -> 0   the 7 + staged
$ jigc task finalize doc-list-note-names                                                   -> 0  finalized 2fd0526
A2 $ jigc doc show jigc-feedback:staged-note-names-the-wrong --format json                 -> 0   the 7
   fields {"about":"jigc doc list","date":"2026-10-03","found-in":"review:rc24-row7/driver","jigc-version":"1.0.0-rc.24","kind":"bug","schema-version":"1","status":"open"}

A16/A17 (after the triage task of block D had copied the doc in):
$ command grep -c '^# ' docs/jigc-feedback/staged-note-names-the-wrong.md   -> 1   (before-control)
$ sed -i '' '/^# Staged note/d' <that file>; git add; git commit -m "chore: hand-remove H1 (out-of-band)"   -> 0
A16 $ jigc doc show jigc-feedback:staged-note-names-the-wrong --format json                -> 0
    keys = the 7   {"title":null, …, "fields":{… "status":"open"}}     stderr: the stale-read `note:` only
A17 $ jigc doc show jigc-feedback:staged-note-names-the-wrong --task triage-the-staged-note-feedback --format json   -> 0
    {"title":"Staged note names the wrong task","staged":"triage-the-staged-note-feedback", …}
$ jigc validate --format json   -> 0   one advisory `file-state.hash-matches` on that path

rig: committed-singletons
A6–A9 $ jigc doc show {vision:vision | roadmap:roadmap | decisions-log:decisions-log | changelog:changelog} --format json   -> 0 each, the 7
      vision  {title:"Vision", item-count:0, schema-version:1, fields:{"schema-version":"1"}}
      roadmap {title:"Roadmap", item-count:1, schema-version:1}   decisions-log {title:"Decisions Log", item-count:0}
      changelog {title:"Changelog", item-count:2, schema-version:2, fields:{"schema-version":"2"}}
A20 $ jigc --format json doc show vision   -> 0, the 7

rig: refs-post-hoc   ($RIG_TASK = ground-the-vision-in-research)
A10 $ jigc doc show vision:vision --task ground-the-vision-in-research --format json   -> 0   the 7 + staged
    fields {"grounded-in":["research:context-loss"],"schema-version":"1"}
A11 $ jigc doc show research:context-loss --format json   -> 0   the 7   {title:"Context Loss", fields:{"date":"2026-10-03","schema-version":"1"}}

rig: migrated
A15 $ jigc doc show vision:vision --format json   -> 0  the 7 ;  `jigc doc show vision --format json` -> 0, `cmp` identical
$ jigc milestone create "Alpha wave" --format json   -> 0  {hook_output:"", text:"minted milestone:alpha-wave …"}   (HEAD 005ef39)
A14 $ jigc doc show milestone-record:alpha-wave --format json   -> 0  the 7
    {title:"alpha-wave", fields:{"base":{"sha":"<sha40>","short":"2f37a34"},"schema-version":"3","status":"active"}, item-count:0, schema-version:3}

rig: fresh (second)   — an ADR landed through `single-task` (`doc create adr --title "Single node cache"`, three slots, finalize -> 4c5ef3c)
A13 $ jigc doc show adr:single-node-cache --task choose-a-cache-strategy --format json   -> 0  the 7 + staged   fields {date, schema-version:"2", status:"proposed"}
A12 $ jigc doc show adr:single-node-cache --format json   -> 0  the 7     (first driven after block D's deletion)
A18: the committed ADR rewritten by hand with no H1, a ```sh fence holding `# a comment inside a fence, not a title`, all four section headings; plain `git commit`
    $ jigc doc show adr:single-node-cache --format json   -> 0   {"title":null, "fields":{…,"status":"proposed"}, sections.context: "```sh\n# a comment inside a fence, not a title\n```"}

A19: rig fresh — docs/inconsistencies/conformant-but-unstamped.md written by hand: front matter `kind`/`status`/`date`, **no** `schema-version`, an H1, the four section headings
    $ jigc doc show inconsistency:conformant-but-unstamped --format json   -> 0
      {"title":"Conformant but unstamped","schema-version":null,"fields":{"date":"2026-10-01","kind":"doc-doc","status":"open"}}
```

### B · cell 2 — no `title` (and no `staged`, no `item-count`) on a slice

All rows: door `doc show`, `--format json`, exit 0, code none, route none. The root is the bare
value the registry declares; **no row's root carries `title`, `staged`, `item-count` or
`schema-version` as an added key** (a fields-group slice carries the *field* `schema-version`,
which is a stored leaf).

| # | registry arm | argv (`doc show … --format json`) | driven root | verdict |
|---|---|---|---|---|
| B1 | `FieldsGroupSlice` | `inconsistency:…#meta` | object `{date, kind, schema-version, status}` | M |
| B2 | `FieldsGroupSlice` | `jigc-feedback:…#meta` | object, 7 stored leaves | M |
| B3 | `FieldsGroupSlice` | `changelog:changelog#meta` | `{"schema-version":"2"}` | M |
| B4 | `FieldsGroupSlice` (staged) | `vision:vision#meta --task …` | `{"grounded-in":["research:context-loss"],"schema-version":"1"}` — no `staged` | M |
| B5 | `FieldsGroupSlice` | `milestone-record:alpha-wave#meta` | `{base:{sha,short}, schema-version, status}` | M |
| B6 | `FieldsGroupSlice` | `adr:single-node-cache#status` (status line deleted) | `{"date":…,"schema-version":"2"}` | M (declared: the slice serves stored leaves) |
| B7 | `SlotSlice` | `inconsistency:…#description` | string | M |
| B8 | `SlotSlice` | `vision:vision#thesis` | string | M |
| B9 | `SlotSlice` (staged) | `vision:vision#thesis --task …` | string | M |
| B10 | `ItemArraySlice` | `inconsistency:…#sides` | array of 3 `{id, says, title}` | M |
| B11 | `ItemArraySlice` | `changelog:changelog#releases` | array of `{changes, date, id, link, title}` | M |
| B12 | `ItemArraySlice` | `roadmap:roadmap#milestones` | array of `{decomposition, id, proves, title}` | M |
| B13 | `ItemArraySlice` (nested) | `changelog:changelog#releases/1-0-0/changes` | array of `{category, id, notes}` | M |
| B14 | `ItemSlice` | `inconsistency:…#sides/readme` | `{id, says, title}` | M |
| B15 | `ItemSlice` | `changelog:changelog#releases/1-0-0` | `{changes, date, id, link, title}` | M |
| B16 | `SlotSlice` (item leaf) | `inconsistency:…#sides/readme/says` | `"Three retries."` | M |
| B17 | scalar (the `id-from` leaf) | `inconsistency:…#sides/adr/title` | `"adr:retry-policy#decision"` | M |
| B18 | scalar (item field) | `changelog:changelog#releases/1-0-0/date` | `"2026-10-03"` | M |
| B19 | scalar (field leaf, non-repeatable) | `inconsistency:…#meta/status` (stored) | `"open"` | M |
| B20 | scalar (field leaf) | `jigc-feedback:…#meta/found-in` | `"review:rc24-row7/driver"` | M |
| B21 | `ListFieldSlice` (staged) | `vision:vision#meta/grounded-in --task …` | `["research:context-loss"]` | M |
| B22 | **`CompoundFieldSlice`** | `milestone-record:alpha-wave#meta/base` | object, keys `["sha","short"]` | **M — and reachable** |
| B23 | scalar (staged, after a `set-field`) | `inconsistency:…#meta/status --task …` | `"intended"` | M |
| B24 | scalar (a present-but-empty stored value) | `adr:single-node-cache#status/status` | `""` | M (stored served as stored) |

```
rig: fresh, after block A's finalize (a13a1b2 / 2fd0526)
B1  $ jigc doc show 'inconsistency:retry-budget-differs-between-readme#meta' --format json   -> 0  {"date":"2026-10-03","kind":"code-doc","schema-version":"1","status":"open"}
B7  $ … '#description' --format json   -> 0  "The README promises three retries; the client performs five."
B10 $ … '#sides' --format json         -> 0  [{"id":"readme","says":"Three retries.","title":"README.md"},{"id":"client",…},{"id":"adr","says":"","title":"adr:retry-policy#decision"}]
B14 $ … '#sides/readme' --format json  -> 0  {"id":"readme","says":"Three retries.","title":"README.md"}
B16 $ … '#sides/readme/says'           -> 0  "Three retries."        B17 $ … '#sides/adr/title' -> 0 "adr:retry-policy#decision"
B19 $ … '#meta/status'                 -> 0  "open"                  B20 $ jigc doc show 'jigc-feedback:…#meta/found-in' --format json -> 0
rig: committed-singletons
B3/B11/B13/B15/B18/B8/B12  as tabulated; `changelog:changelog#releases` -> [{"changes":[{"category":"added","id":"added","notes":"- the trial-shaped fixture builder"}],"date":"2026-10-03","id":"1-0-0","link":"https://example.com/compare/0.9.0...1.0.0","title":"1.0.0"}]
rig: refs-post-hoc
B4/B9/B21  as tabulated
rig: migrated, after `jigc milestone create "Alpha wave"`
$ jigc doc schema milestone-record --format json | fields[] with json-shape
  {"id":"base","type":"string","required":true,"author-required":false,"set":"on-create","section":"meta","json-shape":{"sha":"string","short":"string"}}
B22 $ jigc doc show 'milestone-record:alpha-wave#meta/base' --format json   -> 0   keys ["sha","short"]
    $ jigc doc show 'milestone-record:alpha-wave#meta/base'                 -> 0   "<sha40> 2f37a34"   (the stored space-joined scalar)
B5  $ jigc doc show 'milestone-record:alpha-wave#meta' --format json        -> 0   {"base":{"sha":"<sha40>","short":"2f37a34"},"schema-version":"3","status":"active"}
```

**Datum against the rc.20 record.** Its row 33 and §9.1 mark `doc show | CompoundFieldSlice` *NOT
DRIVEN — unreachable*, on the ground that *"no shipped doctype declares a field of that shape"*.
`milestone-record.base` does (`doc schema` carries its `json-shape`), and one `jigc milestone
create` builds an instance. **All eight `doc show` arms of `ENVELOPE_ARMS` are driven on this run.**

### C · cell 3 — `doc list` rows × row state

All rows: door `doc list`, `--format json`, exit 0, code none. **Every row of every listing carries
exactly `id, path, state, item-count, title, fields`** (`jq '[.docs[]|keys_unsorted]|unique'` → one
key list on every listing below). Route: none, or Informational where a `note:` rode stderr.

| # | row state · argv | `title` | `fields` | other keys | verdict |
|---|---|---|---|---|---|
| C1 | managed, parses · `doc list` (fresh, two finding docs) | the H1 | the header map, = `doc show`'s | `item-count` 3 / 0 | M |
| C2 | managed, parses · `doc list inconsistency` | H1 | map | | M |
| C3 | managed, parses · `doc list jigc-feedback` | H1 | map | | M |
| C4 | managed, parses · `doc list` (4 singleton / placement docs) | `Changelog` · `Decisions Log` · `Roadmap` · `Vision` | `{"schema-version": …}` | paths `CHANGELOG.md`, `docs/decisions-log.md`, `docs/roadmap.md`, `VISION.md` | M |
| C5 | managed, parses · `doc list` (refs-post-hoc, 5 docs) | H1s | maps | stderr `note:` (a task stages docs) | M |
| C6 | managed, parses · `doc list` (migrated) | `Vision` | map | | M |
| C7 | managed, parses, compound field · `doc list milestone-record` | `alpha-wave` | `{"base":{"sha":…,"short":…},"schema-version":"3","status":"active"}` | | M |
| C8 | **managed, does not parse**, H1 present · `doc list` | the H1 | **`null`** | `item-count: 0` | M |
| C9 | same · `doc list inconsistency` | the H1 | `null` | | M |
| C10 | **managed, does not parse, no H1** · `doc list` | **`null`** | `null` | `item-count: 0` | M |
| C11 | same · `doc list jigc-feedback` | `null` | `null` | | M |
| C12 | **unregistered**, H1 (untracked worktree file, then committed) · `doc list` | the H1 | `null` | `item-count: 0` | M |
| C13 | unregistered, no H1 · `doc list` | `null` | `null` | | M |
| C14 | unregistered, **bytes parse** (stamped, conformant, non-slug file name) · `doc list` | the H1 | **`null`** (never the map) | `id: "inconsistency:Not_A_Slug"` | M on `title`/`fields`; the `id` cell is **`(R7, D-3)`** |
| C15 | same · `doc list inconsistency` | the H1 | `null` | | as C14 |
| C16 | **orphaned**, H1 · `doc list` | the H1 | `null` | `id: null`, `item-count: null`, sorted after every resolved row | M |
| C17 | orphaned, no H1 · `doc list` | `null` | `null` | `id: null`, `item-count: null` | M |
| C18 | orphaned × a doctype filter · `doc list inconsistency` | — | — | **no orphan row** in the filtered listing | M |
| C19 | orphaned, **untracked** · `doc list` | — | — | no orphan row until the file is committed (the enumerator is `git ls-files`); unregistered rows at a home list while untracked | n/a — recorded datum (the orphan arm reads the committed set by design) |
| C20 | managed, v0-era (conformant, no stamp) · `doc list` | the H1 | the map, **without** `schema-version` | `state: managed` | M (the discriminator's declared bound: an unstamped file that parses reads managed) |
| C21 | `--task`, unfiltered (a `commit` row + an `inconsistency` row) | read from the staged copies | maps from the staged copies | `state: managed` on both; `commit` row `path` = its identity | M |
| C22 | `--task` · `doc list inconsistency --task …` | H1 | map | `path` = the promote home | M |
| C23 | `--task` · `doc list commit --task …` | the task id | `{"scope":"","type":""}` | `path: "commit:<task>"` | M |
| C24 | `--task`, unfiltered (refs-post-hoc) | `Vision` | `{"grounded-in":[…],"schema-version":"1"}` | | M |
| C25 | `--task` · `doc list vision --task …` | `Vision` | staged map | | M |
| C26 | `--task` · `doc list research --task …` (nothing of that type staged) | — | — | `{"docs":[]}`, no stderr | M |
| C27 | `--task` · `doc list jigc-feedback --task …` (staged at create) | H1 | 7-leaf map | | M |
| C28 | `--task` after the **committed** copy lost its H1 and then its parse | the H1, from the staged copy | the map, from the staged copy | committed row for the same id: `title: null`, `fields: null` | M |
| C29 | managed, does not parse, no H1, a fenced `# …` · `doc list adr` | `null` | `null` | | M |
| C30 | managed, does not parse, a **late** `# …` line after the sections · `doc list adr` | that late heading | `null` | | n/a — recorded datum (the reader returns the first H1 outside front matter and fences) |
| C31 | **agreement sweep** — every listed row with an `id`, against `doc show` of that id on the same arm: `{title, fields, item-count}` | | | **21 / 21 AGREE** (13 committed rows over 5 rigs, 8 staged rows over 4 tasks); the 5 listed rows whose `doc show` blocks are those of C8, C10, C12, C13, C14, and each is `fields: null` | M |

```
rig: fresh, after blocks A and D.  Fixtures (hand-written, then `git add docs; git commit -m "chore: row-state fixtures (out-of-band)"`):
  docs/inconsistencies/archive/old.md        = "---\nschema-version: 1\n---\n\n# An orphan\n\nStamped, claimed by nothing.\n"     (crates/cli/tests/doc_list.rs's own orphan shape, under a declared home's subdirectory)
  docs/inconsistencies/archive/untitled.md   = "---\nschema-version: 1\n---\n\nNo heading at all.\n"
  docs/jigc-feedback/foreign-note.md         = "# Foreign note\n\nSomebody wrote this by hand.\n"
  docs/jigc-feedback/foreign-untitled.md     = "Just prose, no heading.\n"
  docs/inconsistencies/conformant-but-unstamped.md = conformant front matter + sections, no stamp
  docs/inconsistencies/Not_A_Slug.md         = conformant, stamped, a file name that is not a slug
C19 (before the commit, files untracked)  $ jigc doc list --format json   -> 0   5 rows: the two `unregistered`, the unstamped `managed`, the two findings — **no orphan row**
C12–C17, C20 (after the commit)           $ jigc doc list --format json   -> 0
  {"id":"inconsistency:Not_A_Slug","path":"docs/inconsistencies/Not_A_Slug.md","state":"unregistered","item-count":0,"title":"Stamped but not a slug","fields":null}
  {"id":"inconsistency:conformant-but-unstamped",…,"state":"managed","item-count":0,"title":"Conformant but unstamped","fields":{"date":"2026-10-01","kind":"doc-doc","status":"open"}}
  {"id":"inconsistency:retry-budget-differs-between-readme",…,"state":"managed","item-count":3,…}
  {"id":"jigc-feedback:foreign-note","path":"docs/jigc-feedback/foreign-note.md","state":"unregistered","item-count":0,"title":"Foreign note","fields":null}
  {"id":"jigc-feedback:foreign-untitled",…,"state":"unregistered","item-count":0,"title":null,"fields":null}
  {"id":"jigc-feedback:staged-note-names-the-wrong",…,"state":"managed","item-count":0,"title":null,"fields":{…}}
  {"id":null,"path":"docs/inconsistencies/archive/old.md","state":"orphaned","item-count":null,"title":"An orphan","fields":null}
  {"id":null,"path":"docs/inconsistencies/archive/untitled.md","state":"orphaned","item-count":null,"title":null,"fields":null}
  jq '[.docs[]|keys_unsorted]|unique'  -> [["id","path","state","item-count","title","fields"]]
C15/C18 $ jigc doc list inconsistency --format json   -> 0   3 rows (Not_A_Slug, conformant-but-unstamped, retry-…); no orphan row

"managed, does not parse":  an appended undeclared `## Bogus section` did NOT break the parse (the doc still served at exit 0, `fields` a map) — a recorded non-fixture.
  The fixture that does: `sed -i '' '/^## Description$/d; /^## Sides$/d'` on the inconsistency, `'/^## Description$/d'` on the feedback doc (which already had no H1); plain `git commit`.
C8/C10 $ jigc doc list --format json   -> 0
  {"id":"inconsistency:retry-budget-differs-between-readme",…,"state":"managed","item-count":0,"title":"Retry budget differs between README and client","fields":null}
  {"id":"jigc-feedback:staged-note-names-the-wrong",…,"state":"managed","item-count":0,"title":null,"fields":null}
C9  $ jigc doc list inconsistency --format json   -> 0   the same row      C11 $ jigc doc list jigc-feedback --format json -> 0  the same row
control: $ jigc doc show inconsistency:retry-budget-differs-between-readme --format json   -> 1  key {store.unparseable, inconsistency:retry-budget-differs-between-readme}

C21–C23: rig fresh, task retry-doc-and, before its finalize
$ jigc doc list --task retry-doc-and --format json   -> 0
  {"id":"commit:retry-doc-and","path":"commit:retry-doc-and","state":"managed","item-count":0,"title":"retry-doc-and","fields":{"scope":"","type":""}}
  {"id":"inconsistency:retry-budget-differs-between-readme","path":"docs/inconsistencies/retry-budget-differs-between-readme.md","state":"managed","item-count":3,"title":"Retry budget differs between README and client","fields":{"date":"2026-10-03","kind":"code-doc","schema-version":"1","status":"open"}}
C24–C26: rig refs-post-hoc — as tabulated.     C7: rig migrated — as tabulated.
C29/C30: rig fresh (second), the hand-written ADR of A18 before its `## Options` heading was restored (does not parse: `store.unparseable`), then with a `# A late heading in the consequences prose` line appended
  $ jigc doc list adr --format json   -> 0  {"id":"adr:single-node-cache","state":"managed","title":null,"fields":null}
  $ jigc doc list adr --format json   -> 0  {"id":"adr:single-node-cache","state":"managed","title":"A late heading in the consequences prose","fields":null}
C31: for each rig in {fresh, fresh (second), committed-singletons, refs-post-hoc, migrated}: `doc list --format json`, then per row `doc show <id> --format json`,
     comparing `{title, fields, item-count}`; repeated with `--task <id>` for every task `jigc task list --format json` names.  21 AGREE, 0 DIFFER.
```

### D · cell 4 — the default projection

Doors `doc show` and `doc list`. Exit 0 unless stated. In every block the stored bytes were
checksummed before and after the reads (`shasum -a 256`), with a `command grep -c '^status:'`
before-control that finds the line and an after-count of 0.

| # | door · arm · argv | surface asserted | exit · code | verdict |
|---|---|---|---|---|
| D1 | `doc show` · committed · `inconsistency:… --format json` (no `status:` line stored) | `fields.status == "open"` | 0 · none | M |
| D2 | `doc show` · committed · `jigc-feedback:… --format json` | `fields.status == "open"` | 0 · none | M |
| D3 | `doc list` · committed · unfiltered | both rows `fields.status == "open"` | 0 · none | M |
| D4 | `doc list` · committed · `jigc-feedback` | `fields.status == "open"` | 0 · none | M |
| D5 | `doc show` · **staged** · `inconsistency:… --task …` (a copied-in staged copy with no `status:` line) | `fields.status == "open"` | 0 · none | M |
| D6 | `doc list` · staged · `inconsistency --task …` | `fields.status == "open"` | 0 · none | M |
| D7 | `doc list` · staged · unfiltered `--task …` | the same | 0 · none | M |
| D8 | `doc show` · staged · `jigc-feedback:… --task …` | `fields.status == "open"` | 0 · none | M |
| D9 | `doc list` · staged · `jigc-feedback --task …` | `fields.status == "open"` | 0 · none | M |
| D10 | `doc show` · committed · `adr:single-node-cache` (pre-M55 doctype) | `fields.status == "proposed"` | 0 · none | M |
| D11 | `doc list` · committed · unfiltered | adr row `fields.status == "proposed"` | 0 · none | M |
| D12 | `doc list` · committed · `adr` | the same | 0 · none | M |
| D13 | `doc show` · staged · `adr:… --task …` | `fields.status == "proposed"` | 0 · none | M |
| D14 | `doc list` · staged · `adr --task …` | the same | 0 · none | M |
| D15 | `doc show` · committed · `inconsistency:…#meta` | the slice **omits** `status` | 0 · none | M (declared: the projection is the whole-doc serve's alone) |
| D16 | `doc show` · staged · `inconsistency:…#meta --task …` | omits `status` | 0 · none | M |
| D17 | `doc show` · committed · `adr:…#status` | `{date, schema-version}` only | 0 · none | M |
| D18 | `doc show` · committed · `inconsistency:…#meta/status --format json` | blocks | 1 · `store.no-such-leaf`, target the address · Human | M as declared — and the datum of `(5, DEFECT 4)` |
| D19 | `doc show` · committed · `jigc-feedback:…#meta/status` | blocks | 1 · `store.no-such-leaf` · Human | as D18 |
| D20 | `doc show` · staged · `inconsistency:…#meta/status --task …` (json and plain) | blocks; the route says *committed* | 1 · `store.no-such-leaf` · Human | block as declared; the route wording is **`(R7, D-2)`** |
| D21 | `doc show` · committed · `adr:…#status/status` | blocks | 1 · `store.no-such-leaf` · Human | as D18 |
| D22 | `doc show` · committed · plain `inconsistency:…#meta/status` | `blocking · store.no-such-leaf — … names no leaf `status` in section `meta`` + `at:` + `route:` | 1 | as D18 |
| D23 | **control — absent, no default:** `jigc-feedback`'s `about:` line deleted; `doc show` whole, `doc list`, and `…#meta/about` | `about` **absent** from both `fields` maps (not `""`, not `null`); the leaf blocks `store.no-such-leaf` | 0 · 0 · 1 | M |
| D24 | control — absent, no default, never populated: `adr:…#status/supersedes` | blocks `store.no-such-leaf` | 1 | M / `(5, DEFECT 4)` |
| D25 | **control — present, ≠ default (staged):** `set-field …#meta/status --value intended --task …`; `doc show --task`, `doc list --task`, `…#meta/status --task` | staged `"intended"` on all three; the committed arms still `"open"` | 0 | M |
| D26 | control — present, ≠ default (committed): after the triage finalize | `doc show` and `doc list` `"intended"`; the file carries `status: intended` | 0 | M |
| D27 | control — `adr` staged `accepted` vs committed (projected) `proposed` | each arm its own value | 0 | M |
| D28 | a **present-but-empty** stored `status:` (hand edit) · `doc show`, `doc list`, `…#status/status` | `""` on all three — the default is **not** projected | 0 | M (*a stored value is always served as stored*); recorded as a datum: a `status == "proposed"` filter misses this row |
| D29 | an out-of-enum stored `status: bogus` · `doc show`, `doc list` | `"bogus"` | 0 | M (stored served as stored; conformance is `validate`'s) |
| D30 | plain whole-doc, committed and staged, and plain `#meta` | the doc **as stored**: no `status:` line | 0 | M (declared: the plain render is the doc as stored) |
| D31 | **bytes untouched** — committed files ×3 doctypes and staged copies ×3 | `shasum` before == after on every one; `git status --porcelain` empty | — | M |

```
rig: fresh, after block A.   IF = docs/inconsistencies/retry-budget-differs-between-readme.md   FF = docs/jigc-feedback/staged-note-names-the-wrong.md
$ command grep -c '^status:' IF FF        -> 1, 1        (before-control: the line is there)
$ sed -i '' '/^status: /d' IF FF ; sed -i '' '/^about: /d' FF
$ command grep -c '^status:' IF FF        -> 0, 0
$ git add IF FF ; git commit -q -m "chore: hand-delete status (out-of-band)"   -> 0   (b621966);  git status --porcelain -> empty
$ shasum -a 256 IF FF  -> c1 ;  snapshot of .jigc + git status + HEAD -> B
D1  $ jigc doc show inconsistency:retry-budget-differs-between-readme --format json   -> 0   fields {"date":"2026-10-03","kind":"code-doc","schema-version":"1","status":"open"}
D2  $ jigc doc show jigc-feedback:staged-note-names-the-wrong --format json           -> 0   fields {"date":…,"found-in":…,"jigc-version":…,"kind":"bug","schema-version":"1","status":"open"}     <- no `about` (D23)
D3  $ jigc doc list --format json            -> 0   both rows' fields as D1 / D2
D4  $ jigc doc list jigc-feedback --format json   -> 0
D15 $ jigc doc show '…inconsistency…#meta' --format json   -> 0   {"date":"2026-10-03","kind":"code-doc","schema-version":"1"}
D18 $ jigc doc show '…inconsistency…#meta/status' --format json   -> 1   stdout 0 B
    key {"code":"store.no-such-leaf","target":"inconsistency:retry-budget-differs-between-readme#meta/status"}
    route "name a field the committed section carries (a section's prose slot is the section itself — address it as `#<section>`)"
D19 $ jigc doc show '…jigc-feedback…#meta/status' --format json   -> 1   same code
D23 $ jigc doc show '…jigc-feedback…#meta/about' --format json    -> 1   key {store.no-such-leaf, …#meta/about}
D30 $ jigc doc show inconsistency:…             -> 0   "---\nkind: code-doc\ndate: 2026-10-03\nschema-version: 1\n---\n…"
    $ jigc doc show '…#meta'                    -> 0   "kind: code-doc\ndate: 2026-10-03\nschema-version: 1"
D22 $ jigc doc show '…#meta/status'             -> 1   blocking · store.no-such-leaf — `…#meta/status` names no leaf `status` in section `meta`
$ shasum -a 256 IF FF == c1   -> BYTES untouched ;  snapshot == B   -> identical ;  git status --porcelain -> empty

staged copy (the binary makes it — nothing was written into .jigc/):
$ jigc start --workflow triage-inconsistency "triage the retry inconsistency"   -> 0   task minted: triage-the-retry-inconsistency
$ jigc doc set-slot 'inconsistency:…#resolution' --from-file - --task triage-the-retry-inconsistency --format json   -> 0  {"chars":9,"copied_in":true,…}
$ command grep -c '^status:' .jigc/tasks/triage-the-retry-inconsistency/docs/inconsistency:retry-budget-differs-between-readme.md   -> 0
  command grep -c '^kind:'   <same file>                                                                                             -> 1   (control: the file is there and grep finds a line in it)
D5  $ jigc doc show inconsistency:… --task triage-the-retry-inconsistency --format json   -> 0   fields {…,"status":"open"}   staged "triage-the-retry-inconsistency"
D6  $ jigc doc list inconsistency --task triage-the-retry-inconsistency --format json     -> 0   row fields {…,"status":"open"}
D7  $ jigc doc list --task triage-the-retry-inconsistency --format json                   -> 0   the same row + the `commit` row
D16 $ jigc doc show '…#meta' --task … --format json            -> 0   {"date":…,"kind":"code-doc","schema-version":"1"}
D20 $ jigc doc show '…#meta/status' --task … --format json     -> 1   key {store.no-such-leaf, …#meta/status}   route "name a field the committed section carries …"
$ shasum of the staged copy before == after   -> STAGED BYTES untouched
D25 $ jigc doc set-field '…#meta/status' --value intended --task … --format json   -> 0
    $ jigc doc show … --task … --format json   -> fields.status "intended" ;  doc list inconsistency --task …  -> "intended" ;  '…#meta/status' --task … -> "intended"
    $ jigc doc show … --format json            -> fields.status "open"  (committed, projected) ;  doc list inconsistency --format json -> "open"
D26 $ (commit doc filled) jigc task finalize triage-the-retry-inconsistency   -> 0  finalized b2dd849
    $ jigc doc show … --format json -> "intended" ;  jigc doc list --format json -> "intended" ;  head -3 IF -> "status: intended"
D8/D9: $ jigc start --workflow triage-jigc-feedback "triage the staged note feedback" -> 0 ; set-slot '…#resolution' --task … -> 0 copied_in:true
    staged copy: `command grep -c '^status:'` -> 0, `'^about:'` -> 0, `'^kind:'` -> 1 (control)
    $ jigc doc show jigc-feedback:… --task triage-the-staged-note-feedback --format json   -> 0   the 7 + staged   fields {…,"status":"open"}  (no `about`)
    $ jigc doc list jigc-feedback --task … --format json                                   -> 0   the same map

rig: fresh (second).   AF = docs/decisions/single-node-cache.md  (landed by `single-task`, finalize 4c5ef3c; front matter status/date/schema-version)
$ command grep -c '^status:' AF -> 1 ; sed -i '' '/^status: /d' AF ; command grep -c '^status:' AF -> 0 ; '^date:' -> 1 (control)
$ git add AF ; git commit -q -m "chore: hand-delete adr status (out-of-band)"   -> 0
D10 $ jigc doc show adr:single-node-cache --format json   -> 0   fields {"date":"2026-10-03","schema-version":"2","status":"proposed"}
D11 $ jigc doc list --format json                          -> 0   row fields the same        D12 $ jigc doc list adr --format json -> 0
D17 $ jigc doc show 'adr:single-node-cache#status' --format json          -> 0   {"date":"2026-10-03","schema-version":"2"}
D21 $ jigc doc show 'adr:single-node-cache#status/status' --format json   -> 1   key {store.no-such-leaf, adr:single-node-cache#status/status}
D24 $ jigc doc show 'adr:single-node-cache#status/supersedes' --format json -> 1  key {store.no-such-leaf, …/supersedes}
$ shasum AF before == after -> BYTES untouched ; snapshot identical
$ jigc start --workflow single-task "revisit the cache decision" -> 0 ;  set-slot 'adr:…#options' --task revisit-the-cache-decision -> 0 copied_in:true
  staged copy: `command grep -c '^status:'` -> 0 ; `'^date:'` -> 1 (control)
D13 $ jigc doc show adr:single-node-cache --task revisit-the-cache-decision --format json   -> 0   fields {…,"status":"proposed"}
D14 $ jigc doc list adr --task revisit-the-cache-decision --format json                     -> 0   the same ;  staged copy shasum unchanged
D27 $ jigc doc set-field 'adr:…#status/status' --value accepted --task … -> 0 ;  show --task -> "accepted" ;  show (committed) -> "proposed"
D28 $ (hand edit: a `status:` line with an empty value; plain commit)  show -> fields.status "" ;  list adr -> "" ;  '#status/status' -> 0, `""`
D29 $ (hand edit: `status: bogus`; plain commit)  show -> "bogus" ;  list adr -> "bogus"
```

### E · cell 5 — `doc schema`

| # | argv | exit | surface asserted | verdict |
|---|---|---|---|---|
| E1 | `doc schema jigc-feedback --format json` | 0 | keys `contract-version, fields, home, identity, schema-version, sections, type` = the registry's row; `contract-version: 7`; `schema-version: 1`; `home {kind: location, path: docs/jigc-feedback/<slug>.md}`; `identity {kind: slugged, address: jigc-feedback:<slug>}`; 10 fields; `status` carries `"default":"open"` and the five members; `duplicate-of` `type: ref, to: jigc-feedback`; three slot sections | M (against `design/findings-channel.md` §1.1) |
| E2 | `doc schema jigc-feedback` | 0 | the same facts as text; `(default: open)` printed on `status`; the `* = author-required` legend line present, and markers land (`kind`, `found-in`, `jigc-version`) | M |
| E3 | `doc schema jigc-feedback --format human` | 0 | byte-identical to E2 (`cmp`) | M |
| E4 | `doc schema inconsistency --format json` | 0 | the 7 keys; `contract-version: 7`; `home docs/inconsistencies/<slug>.md`; `identity slugged`; 4 fields, `status` `"default":"open"`; the **`sides` item block**: `{id: sides, kind: repeatable, add-item, item: {fields: [{id: title, …, write-key: "--title", retitle-item}], slots: [{id: says, optional: true, set-slot: …#sides/<id>/says}], nested: []}}` | M (against §1.2) |
| E5 | `doc schema inconsistency` | 0 | the same as text, the `sides` block nested under `sections:`; `(default: open)` | M |
| E6 | `doc schema inconsistency --format human` | 0 | byte-identical to E5 | M |
| E7 | `doc schema <ty> --format json` over **all 18** shipped doctypes | 0 ×18 | `contract-version: 7` ×18 (unmoved); the 7 keys ×18; `home.kind` ∈ `location` (12) · `placement` (5) · `transient` (1, `path: null`); `identity.kind` `slugged` (13) · `fixed` (5); defaults on `adr`, `inconsistency`, `jigc-feedback` only | M |
| E8 | `doc schema commit --format json` (the transient) | 0 | the 7 keys; `home {kind: transient, path: null}` | M |
| E9 | `doc schema milestone-record --format json` | 0 | `base` carries `"json-shape":{"sha":"string","short":"string"}` — the member shape `doc show` serves (B22) | M |

```
rig: committed-singletons
E1 $ jigc doc schema jigc-feedback --format json   -> 0
   {"type":"jigc-feedback","contract-version":7,"schema-version":1,"home":{"kind":"location","path":"docs/jigc-feedback/<slug>.md"},"identity":{"kind":"slugged","address":"jigc-feedback:<slug>"}}
   {"id":"status","type":"enum","of":["open","resolved","declined","duplicate","refuted"],"required":true,"author-required":false,"default":"open","section":"meta","set-field":"jigc-feedback:<slug>#meta/status"}
   {"id":"schema-version","type":"int","required":true,"author-required":false,"set":"schema-version","section":"meta"}            (the injected stamp; no default)
E4 $ jigc doc schema inconsistency --format json   -> 0
   {"id":"sides","kind":"repeatable","add-item":"inconsistency:<slug>#sides","item":{"fields":[{"id":"title","type":"string","required":true,"author-required":true,"add-item":"inconsistency:<slug>#sides","retitle-item":"inconsistency:<slug>#sides/<id>","write-key":"--title"}],"slots":[{"id":"says","optional":true,"set-slot":"inconsistency:<slug>#sides/<id>/says"}],"nested":[]}}
E2/E5 the plain listings, first lines:  "doctype: jigc-feedback (schema-version 1)" / "* = author-required" / "fields:" …
E3/E6 `cmp agent.out human.out` -> identical, both doctypes
E7 for ty in adr arch-doc changelog commit completion-record decisions-log deferral-ledger dogfood-record idea inconsistency jigc-feedback milestone-record planning-record prd research roadmap spec vision:
   jigc doc schema $ty --format json -> rc=0, contract-version 7, the 7 keys            n = 18
```

### F · cell 6 — the plain and human arms

Graded against law 1 only. **Recorded, as the scope asks:** the plain `doc show` whole-doc arm
shows the title (it is the stored `# H1`) and shows **no** projected default (it is the doc as
stored); the plain `doc list` arm shows **neither** `title` nor `fields` — three columns, `id  path
 state`.

| # | door · argv | exit | surface asserted | verdict |
|---|---|---|---|---|
| F1 | `doc show inconsistency:…` | 0 | stdout = the stored file's bytes + one `\n` (366 B against 365 B stored — the declared doubled terminator); no footer | M |
| F2 | `doc show inconsistency:… --format human` | 0 | byte count identical to F1 | M |
| F3 | `doc show jigc-feedback:…` | 0 | stored bytes + `\n` (281 / 280) | M |
| F4 | `doc list` | 0 | header `id  path  state`, one row per doc, no title, no fields | M — and the surface `(R7, D-1)`'s step misdescribes |
| F5 | `doc list --format human` | 0 | byte-identical to F4 | M |
| F6 | `doc show '…#meta'` | 0 | the field lines as stored | M |
| F7 | `doc show '…#sides'` | 0 | item headings with their `{#id}` anchors, re-rooted | M |
| F8 | `doc show 'milestone-record:alpha-wave#meta/base'` | 0 | `<sha40> <short>` — the stored scalar, where the JSON arm serves the object | M (declared: json-projection-only) |
| F9 | `doc list --task …` | 0 | the same three columns; the `commit` row's path cell is its identity | M |
| F10 | `doc list` with the two orphans | 0 | `(none)  docs/inconsistencies/archive/old.md  orphaned` — the unpasteable identity cell | M |
| F11 | `doc schema <ty>` agent vs human | 0 | identical bytes (E3, E6) | M |

```
rig: fresh
F1 $ jigc doc show inconsistency:retry-budget-differs-between-readme > out ; wc -c out -> 366 ; wc -c <stored> -> 365 ; `cmp` reports EOF on the stored file only
F4 $ jigc doc list   -> 0
   id  path  state
   inconsistency:retry-budget-differs-between-readme  docs/inconsistencies/retry-budget-differs-between-readme.md  managed
   jigc-feedback:staged-note-names-the-wrong  docs/jigc-feedback/staged-note-names-the-wrong.md  managed
F10 $ jigc doc list  -> 0  … "(none)  docs/inconsistencies/archive/old.md  orphaned" / "(none)  docs/inconsistencies/archive/untitled.md  orphaned"
F7 $ jigc doc show '…#sides' -> "### README.md  {#readme}\n\nThree retries.\n\n### src/client.rs  {#client}\n\nFive retries.\n\n### adr:retry-policy#decision  {#adr}"
```

### G · cell 7 — the read is a read

**The witness.** One digest over: `git status --porcelain`, `git rev-parse HEAD`, and the SHA-256
of **every file under `.jigc/`** (so `state/file-state.json`, `index/edges.json`, every task area
and every staged copy are inside it). Taken before and after each batch below.

| # | batch | result | verdict |
|---|---|---|---|
| G1 | every drive of blocks A – F, H and J, digested per batch in each rig | digest **identical** on every batch | ~~M~~ — **DEMOTED BY THE RECONCILER: NOT DRIVEN.** The row carries no repro block (no rig, no argv list, no digest is recorded for any of its batches); the property it asserts stands only as far as G2's block and the reconciler's three digested batches carry it (ledger §R.5) |
| G2 | the explicit sweep: 16 argv × 3 formats = **48 invocations** over the three doors (served reads, `--task` reads, slices, and four rejects), rig refs-post-hoc **with `edges.json` materialized** (a prior `jigc task validate` writes it; 73 B) | `file-state.json` byte-identical · `edges.json` byte-identical · `git status` identical · whole-tree digest identical | M |
| G3 | **controls** — a write through the binary between two digests (`doc set-field … --task …` in fresh; `doc set-slot 'vision:vision#thesis' --task …` in refs-post-hoc) | the digest **moves** both times | the witness is not blind |

```
rig: refs-post-hoc.  `.jigc/index/` held only `edges.json.lock` in every rig as built — the edge index is not on disk until a gate writes it —
so the first pass could show only "absent before, absent after". `jigc task validate $RIG_TASK` (exit 3) materialized `edges.json` (73 B); then:
$ shasum -a 256 < .jigc/state/file-state.json -> b_fs ;  < .jigc/index/edges.json -> b_ed ;  git status --porcelain | shasum -> b_gs ;  digest -> B
$ for a in "doc show vision:vision" "doc show vision:vision --task T" "doc show vision:vision#meta --task T" "doc show vision:vision#meta/grounded-in --task T"
           "doc show research:context-loss" "doc list" "doc list vision" "doc list --task T" "doc list research --task T" "doc schema vision"
           "doc schema jigc-feedback" "doc schema inconsistency" "doc show nosuchtype:x" "doc show vision:vision#meta/grounded-in" "doc list --task nope" "doc list nosuchtype";
    for f in agent json human: jigc $a --format $f
$ b_fs == a_fs · b_ed == a_ed · b_gs == a_gs · B == A                      -> all four identical
G3 $ jigc doc set-slot 'vision:vision#thesis' --from-file - --task T   -> 0 ;  digest != B   -> "control: a write moves the snapshot"
```

### H · cell 8 — reject funnels on the three verbs

`--format json` unless the row says plain. On every reject row: **stdout 0 bytes**, the document on
stderr, exactly one JSON document. `Findings` = `{findings, schema_version: 3}`; `Error` =
`{error}`.

| # | door · cell · argv | exit | envelope · `(code, target)` | route kind | verdict |
|---|---|---|---|---|---|
| H1 | `doc show` · unknown type · `nosuchtype:x` | 1 | Findings · `(store.unknown-type, nosuchtype:x)` | Mechanical (`jigc describe`) | M on the envelope; target shape = rc.17 `DEFECT A`, STILL-OPEN |
| H2 | `doc schema` · unknown type · `nosuchtype` | 1 | Findings · `(store.unknown-type, nosuchtype)` | Mechanical | as H1 |
| H3 | `doc list` · unknown type · `nosuchtype` | 1 | Findings · `(store.unknown-type, nosuchtype)` | Mechanical | as H1 |
| H4 | `doc show` · unknown slug, per-instance · `adr:nope` | 1 | Findings · `(store.not-found, adr:nope)`; the route names the staged read `jigc doc show adr:nope --task <task-id>` and `jigc task list` | Mechanical | M |
| H5 | `doc show` · unknown slug, singleton · `vision:nope` | 1 | Findings · `(store.fixed-identity, vision:nope)`; route `jigc doc show vision:vision` | Mechanical | M |
| H6 | `doc show` · malformed, colon-less · `nosuchtype` | 1 | **Error** · no code, no key | Mechanical (`jigc describe`) | `(5, DEFECT 3)` STILL-OPEN |
| H7 | `doc show` · malformed · `vision:` (empty slug) | 1 | Error · no code | Mechanical | same family as H6 |
| H8 | `doc show` · malformed · `:x` (empty type) | 1 | Error · no code | Mechanical | same family |
| H9 | `doc show` · malformed slug · `vision:Bad_Slug` | 1 | Error carrying `blocking · store.malformed-slug` (the registry's declared exception to the owed envelope) | Mechanical (`jigc doc list`) | M |
| H10 | `doc show` · malformed · `vision:vision#` (empty fragment) | 1 | Error · no code | Mechanical | same family as H6 |
| H11 | `doc show` · over-deep fragment · `vision:vision#a/b/c/d/e/f` | 1 | Findings · `(store.no-such-section, <address as typed>)`; route lists the four real sections | Human | M |
| H12 | `doc show` · undeclared leaf · `vision:vision#meta/nope` | 1 | Findings · `(store.no-such-leaf, …#meta/nope)` | Human | M |
| H13 | `doc show` · undeclared section · `vision:vision#nope` | 1 | Findings · `(store.no-such-section, …#nope)` | Human | M |
| H14 | `doc show` · missing item · `changelog:changelog#releases/9-9-9` | 1 | Findings · `(store.no-such-item, …)` | Human | M |
| H15 | `doc show` · undeclared item leaf · `…#releases/1-0-0/nope` | 1 | Findings · `(store.no-such-leaf, …)` | Human | M |
| H16 | `doc show` · **declared-but-unpopulated optional leaf** · `vision:vision#meta/grounded-in` | 1 | Findings · `(store.no-such-leaf, vision:vision#meta/grounded-in)`, *"names no leaf `grounded-in` in section `meta`"* | Human | `(5, DEFECT 4)` STILL-OPEN |
| H17 | `doc show` · `--task` naming no task · `vision:vision --task nope` | 1 | Findings · `(finalize.no-task, task:nope)`; route `jigc task list` | Mechanical | M |
| H18 | `doc list` · `--task nope` | 1 | Findings · `(finalize.no-task, task:nope)` | Mechanical | M |
| H19 | `doc list` · `vision --task nope` | 1 | Findings · `(finalize.no-task, task:nope)` | Mechanical | M |
| H20 | `doc show` · transient, task-less · `commit:x` | 1 | Findings · `(store.transient-type, commit:x)`; route the staged read | Mechanical | M |
| H21 | `doc schema` · an address where a doctype is expected · `vision:vision` | 1 | Findings · `(store.unknown-type, vision:vision)` | Mechanical | M (the token is echoed as typed) |
| H22 | `doc list` · the same · `vision:vision` | 1 | Findings · `(store.unknown-type, vision:vision)` | Mechanical | M |
| H23 | `doc show --task` · not staged, a committed sibling exists · `research:context-loss --task T` | 1 | Findings · `(store.not-staged, research:context-loss)`; route `jigc doc show research:context-loss` | Mechanical | M |
| H24 | `doc show --task` · not staged, nothing anywhere · `adr:nope --task T` | 1 | Findings · `(store.not-staged, adr:nope)`; route *create or author the doc in this task first* | Human | M |
| H25 | `doc show --task` · unknown type | 1 | Findings · `(store.unknown-type, nosuchtype:x)` | Mechanical | M |
| H26 | `doc show --task` · undeclared leaf · `vision:vision#meta/nope --task T` | 1 | Findings · `(store.no-such-leaf, …)`; route says *committed section* | Human | code/key M; wording = `(R7, D-2)` |
| H27 | `doc list --task` · unknown type | 1 | Findings · `(store.unknown-type, nosuchtype)` | Mechanical | M |
| H28 | `doc show --task` · colon-less | 1 | Error · no code | Mechanical | `(5, DEFECT 3)` on the staged arm too |
| H29 | `doc show --task` · missing item, **a doc with no committed copy** | 1 | Findings · `(store.no-such-item, …#sides/nope)`; route *"… exists in the committed doc"* | Human | **`(R7, D-2)`** |
| H30 | `doc show --task` · undeclared item leaf, same doc | 1 | Findings · `(store.no-such-leaf, …#sides/a/nope)`; route *"… exists in the committed item"* | Human | **`(R7, D-2)`** |
| H31 | `doc show --task` · undeclared leaf, same doc | 1 | Findings · `(store.no-such-leaf, …#meta/nope)`; route *"… the committed section carries"* | Human | **`(R7, D-2)`** |
| H32 | `doc show --task` · undeclared section, same doc | 1 | Findings · `(store.no-such-section, …#nope)`; arm-neutral route | Human | M |
| H33 | `doc show` · the same doc task-less | 1 | Findings · `(store.not-found, inconsistency:two-docs-disagree)` — the key drops the fragment; route the staged read | Mechanical | M |
| H34 | `doc show` · managed, does not parse | 1 | Findings · `(store.unparseable, inconsistency:retry-…)`; route *fix the committed file so it conforms to its schema* | Human | M |
| H35 | the same, plain | 1 | `blocking · store.unparseable — … does not parse: section heading "Evidence" does not match required section `sides`` | Human | M |
| H36 | `doc show` · **unregistered** (foreign bytes) · `jigc-feedback:foreign-note` | 1 | Findings · `(store.unparseable, jigc-feedback:foreign-note)`; route *adopt — run `jigc ingest` to route it; it is a foreign file, not an unmigrated managed doc* | Mechanical | M (the adoption reroute) |
| H37 | `doc show` · unregistered, unaddressable id · `inconsistency:Not_A_Slug` | 1 | Error carrying `store.malformed-slug`; route `jigc doc list` | Mechanical | **`(R7, D-3)`** |
| H38 – H47 | **plain arms** of H1, H2, H3, H6, H16, H17, H18, H4, H5, H20 | 1 ×10 | stdout 0 B; `blocking · <code> — …` + `at:` + `route:` + the footer on the eight coded ones; the bare sentence + `route:` on H6's | as the JSON row | M (text == JSON on code, locus and route in all ten) |
| H48 | `doc show` · **outside any repository** · `vision:vision` | 1 | Error · `not inside a git repository (no `.git` found from <tmp>) — run jigc from inside the target git repository; …` | Human | M — M51 family CLOSED |
| H49 | `doc show --task x` · outside | 1 | Error, identical | Human | M |
| H50 | `doc schema vision` · outside | 1 | Error, identical | Human | M |
| H51 | `doc list` · outside | 1 | Error, identical | Human | M |
| H52 | `doc list vision --task x` · outside | 1 | Error, identical | Human | M |
| H53 – H55 | plain arms of H48, H50, H51 | 1 ×3 | the same sentence, stdout 0 B | Human | M |
| H56 | `doc show` · **cwd deleted under the process** | 1 | Error · `cannot determine the current directory: No such file or directory (os error 2)` | none | M — M51 family CLOSED |
| H57 | `doc schema` · cwd deleted | 1 | Error, identical | none | M |
| H58 | `doc list` · cwd deleted | 1 | Error, identical | none | M |
| H59 | plain `doc list` · cwd deleted | 1 | the same sentence | none | M |

```
rig: committed-singletons   (H1 – H22)
$ jigc --format json doc show 'nosuchtype:x'   -> 1  stdout 0 B  stderr:
  {"schema_version":3,"findings":[{"severity":"blocking","probe":"store","check":"unknown-type","code":"store.unknown-type",
    "key":{"code":"store.unknown-type","target":"nosuchtype:x"},"message":"unknown doctype `nosuchtype` for `nosuchtype:x`",
    "location":{"address":"nosuchtype:x","line":1,"col":1},"route":"list the available doctypes with `jigc describe`"}]}
$ jigc --format json doc schema nosuchtype     -> 1  key {store.unknown-type, nosuchtype}
$ jigc --format json doc list nosuchtype       -> 1  key {store.unknown-type, nosuchtype}
$ jigc --format json doc show nosuchtype       -> 1  {"error":"malformed address `nosuchtype`: missing ':' between type and slug — a doc is addressed as `<type>:<slug>`, e.g. `adr:single-node-cache` (a singleton doctype like `changelog` or `vision` may be named bare)\n  route: run `jigc describe` for the doctype surface"}
$ jigc --format json doc show 'vision:vision#meta/grounded-in'   -> 1  key {store.no-such-leaf, vision:vision#meta/grounded-in}
    message "`vision:vision#meta/grounded-in` names no leaf `grounded-in` in section `meta`"
    control: `jigc doc schema vision --format json` still lists `grounded-in` (`type: ref`, `required: false`, `set-field: vision:<slug>#meta/grounded-in`)
$ jigc --format json doc show vision:vision --task nope   -> 1  key {finalize.no-task, task:nope}   route "`jigc task list` lists the live tasks"
  (the remaining rows of the table: argv, exit and key exactly as tabulated)
rig: refs-post-hoc, T = ground-the-vision-in-research   (H23 – H28)
rig: fresh (second), task second-open-task              (H29 – H33; block in §3 `(R7, D-2)`)
rig: fresh, after the row-state fixtures                (H34 – H37)
H38 – H47 plain, rig refs-post-hoc, e.g.:
$ jigc doc show nosuchtype:x   -> 1
  blocking · store.unknown-type — unknown doctype `nosuchtype` for `nosuchtype:x`
    at: nosuchtype:x
    route: list the available doctypes with `jigc describe`
  — jigc · run `jigc start` for orientation; all writes through `jigc`.
H48 – H55: W=$(mktemp -d …); cd "$W"   (`git rev-parse --show-toplevel` -> fatal: not a git repository); HOME = the rig's
H56 – H59: D=$(mktemp -d …); cd "$D"; rmdir "$D"   (`ls -d "$D"` -> No such file or directory)
```

### I · cell 9 — `--help`

| # | argv | exit | surface asserted | verdict |
|---|---|---|---|---|
| I1 | `doc show --help` | 0 | *"a whole-doc object keyed by `type`, `slug`, `title`, `item-count`, `schema-version`, `fields`, `sections` — a staged serve adds the one `staged` key carrying the task id"* — equal, as a set and in `WHOLE_DOC_KEYS`' order, to the driven key set of A1 – A20 | M |
| I2 | `doc list --help` | 0 | *"the pinned shape `{"docs":[{id, path, state, item-count, title, fields}]}`"* = the driven row keys in their serialized order; *"`title` is the doc's `# H1` or null, and `fields` its header fields as `doc show` serves them on a managed row that parses, else null"* = C1 – C31; the three states and the orphan's *null identity and no item count* = C16 | M on the key list and the per-state values. (Its opening sentence *"Every instance … carries its `<type>:<slug>` identity"* is the statement `(R7, D-3)` is filed against.) |
| I3 | `doc schema --help` | 0 | *"`contract-version: 7`"* = E7; *"Task-less"* (the verb has no `--task` flag) | M |

### J · cell 10 — the staged-elsewhere note on `doc list` (O24's surface), and its `doc show` sibling

All rows exit 0; the note rides **stderr**; route kind Informational.

| # | state · argv | stderr | verdict |
|---|---|---|---|
| J1 | one open task staging an `inconsistency` + its `commit` · `doc list --format json` | `note: docs are also staged in open task retry-doc-and — … `jigc doc list --task retry-doc-and`` | M |
| J2 | same · `doc list inconsistency --format json` | `note: `inconsistency` docs are also staged in open task retry-doc-and — … `jigc doc list inconsistency --task retry-doc-and`` | M (the reader's scope survives into the route) |
| J3 | same · `doc list adr --format json` — **the task stages no `adr`** | **empty** | **M — O24 holds** |
| J4 | same · `doc list commit --format json` | the typed note for `commit` (the task does stage one; the handed-over listing is non-empty — C23) | M |
| J5 | one open task staging only a `jigc-feedback` · `doc list inconsistency --format json` (one committed row) | **empty** | **M — O24 holds** |
| J6 | same · `doc list jigc-feedback --format json` (empty committed set) | the typed note | M (the empty listing routes too) |
| J7 | same · plain `doc list jigc-feedback` | stdout `jigc doc list — no committed `jigc-feedback` docs`; the note on stderr | M |
| J8 | same · `doc list --format json` | the untyped note | M |
| J9 | same · plain `doc list adr` | stdout the empty-set line; stderr **empty** | M |
| J10 | **two** open tasks — one stages an `adr`, the other only an `inconsistency` · `doc list adr` | names **only** `revisit-the-cache-decision` | M |
| J11 | same · `doc list inconsistency` | names **only** `second-open-task` | M |
| J12 | same · `doc list jigc-feedback` | **empty** | M |
| J13 | same · `doc list` | plural: *"open tasks revisit-the-cache-decision, second-open-task — … if one of them is yours … `jigc doc list --task <task-id>`"* | M |
| J14 | same · `doc list commit` | plural, typed | M |
| J15 | stdout with vs without the open tasks · `doc list adr --format json` | `cmp` of the normalized documents: identical | M (no second key reaches the pin) |
| J16 | refs-post-hoc · `doc list --format json` | the untyped note naming `$RIG_TASK` | M |
| J17 | `doc show vision:vision --format json`, the doc also staged | `note: `vision:vision` is also staged in open task … — this read served the committed copy, so any edits staged there are not shown; if that task is yours, read your staged work: `jigc doc show vision:vision --task …``; stdout the 7 keys | M |
| J18 | `doc show inconsistency:… --format json`, the doc also staged in the triage task | the same note; stdout unchanged | M |

```
rig: fresh, task retry-doc-and open (J1 – J4):
$ jigc doc list adr --format json   -> 0   stdout {"docs":[]}   stderr 0 B
rig: fresh, task doc-list-note-names open, the inconsistency already committed (J5 – J9):
$ jigc doc list inconsistency --format json   -> 0   1 row    stderr 0 B
$ jigc doc list jigc-feedback --format json   -> 0   0 rows   stderr: note: `jigc-feedback` docs are also staged in open task doc-list-note-names — this listing is the committed store; if that task is yours, list what it stages: `jigc doc list jigc-feedback --task doc-list-note-names`
rig: fresh (second), tasks revisit-the-cache-decision (stages adr + commit) and second-open-task (stages inconsistency + commit) (J10 – J15):
$ jigc doc list --format json   -> 0  stderr: note: docs are also staged in open tasks revisit-the-cache-decision, second-open-task — this listing is the committed store; if one of them is yours, list what it stages: `jigc doc list --task <task-id>`
```

### K · the baseline tier-1 row — `(5, DEFECT 1)`, both mint doors

| # | door · argv | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|
| K1 | `milestone create ""` (`--format json`) | 1 | `write.unslugable-title` | Mechanical | `{"error":"blocking · write.unslugable-title — cannot mint a milestone: … this title slugs to nothing …"}`; nothing minted | **CLOSED** |
| K2 | `task amend ""` | 1 | `write.unslugable-title` | Mechanical | `{"error":"blocking · write.unslugable-title — cannot mint a task: …"}`; nothing minted | **CLOSED** |
| K3 | `milestone create "!!!"` | 1 | `write.unslugable-title` | Mechanical | the same | CLOSED |
| K4 | `task amend "   "` | 1 | `write.unslugable-title` | Mechanical | the same | CLOSED |

```
rig: fresh (second, as built — before any task)      cwd = $REPO
before:  H0 = git rev-parse HEAD ;  ls .jigc/tasks -> No such file or directory ;  ls .jigc/milestones -> No such file or directory ;  ls docs -> No such file or directory
$ jigc --format json milestone create ""    -> 1  {"error":"blocking · write.unslugable-title — cannot mint a milestone: its id is slugged from the title, and this title slugs to nothing — ids are built from ASCII letters and digits, so a title in another script, or of stopwords only, yields none\n  at: milestone\n  route: re-run with a title carrying ASCII letters or digits — the milestone id is slugged from it"}
$ jigc --format json task amend ""          -> 1  {"error":"blocking · write.unslugable-title — cannot mint a task: …\n  at: task\n  route: re-run with a title carrying ASCII letters or digits — the task id is slugged from it, or omit the title — `jigc task amend` names the task after the commit it rewrites (`amend-<sha7>`)"}
$ jigc --format json milestone create "!!!" -> 1  same code      $ jigc --format json task amend "   " -> 1  same code
after:   git rev-parse HEAD == H0 ;  git status --porcelain -> empty ;  .jigc/tasks, .jigc/milestones, docs -> still absent ;  jigc task list -> "no active tasks"
```

---

## 5 · What was NOT driven, and why — stated plainly

1. **The other 45 `VERB_KINDS` leaves and the 56 `ENVELOPE_ARMS` rows that are not `doc show` /
   `doc schema` / `doc list`.** Outside this row by its scope. `start`, `doc create`,
   `doc set-field`, `doc set-slot`, `doc add-item`, `task finalize`, `task list`, `task validate`
   and `validate` were **run as fixture construction or as a control only** — no verdict was
   recorded against them, so they are not doors of a driven row and confer no coverage.
2. **A `--task` staged row whose staged copy does not parse** (cell 3 × the staged arm). The only
   construction is a hand edit inside `.jigc/tasks/<id>/docs/`, which the brief forbids. The staged
   arm was driven only on copies the binary wrote, all of which parse.
3. **`orphaned` by the construction the scope names** — *a doctype removed from the resolved set
   through a pack copy.* **Attempted, and refused by the binary, which is the datum:** on
   `dev/jigc-rig fresh --pack-from-dev`, moving `schemas/adr.yaml` out of the pack copy makes every
   verb exit 1 with `{"error":"pack-load ref-target fence failed: `arch-doc`'s `#meta/cites` field
   declares `to: adr`, which no doctype of the loaded set provides …"}`; moving `prd.yaml` out
   instead trips the same fence through `spec`'s `#meta/derived-from`. Both files were restored
   (the stash is empty; `doc list` clean again). Separately, a doc landed under that fs-local pack
   copy carries **no stamp** (its row's `fields` had no `schema-version`), so it could not have
   been an orphan by definition. The `orphaned` rows C16 – C18 were therefore built by the suite's
   own construction (`crates/cli/tests/doc_list.rs`: a stamped file under a declared home's
   subdirectory that no resolved doctype claims), committed with plain git.
4. **A `managed` row at a doctype's *prior* home** (the M52 subject widening) × `title` / `fields`.
   Needs a schema bump that moves a home; no rig state builds one and none may be built here.
5. **A BOM-prefixed file** through the H1 reader (the contract names it; front matter and the code
   fence were driven — A16, A18).
6. **The whole-doc serve on the eight doctypes for which no instance was built:** `prd`, `spec`,
   `arch-doc`, `idea`, `completion-record`, `planning-record`, `deferral-ledger`, `dogfood-record`.
   Ten of eighteen were driven (A1 – A20). `doc schema` was driven on all eighteen.
7. **`--format human` on the slice arms** and on the reject arms. Driven only where tabulated
   (whole-doc show, list, schema; the 48-invocation sweep of block G ran `human` for exit-and-digest
   only and its bytes were not graded).
8. **The four non-root cwds** the rc.20 run crossed with every arm (a subdirectory, a provisioned
   fan-out worktree, an ordinary linked worktree, a spaced repository path) for the three read
   verbs. This row's hostile-cwd cells are the two the scope names (outside any repository; cwd
   deleted).
9. **`doc list` / `doc show` under a project-layer schema shadow** (a shadowed `default:`). No scope
   grant for a hand-written `.jigc/config/` file on this row.
10. **The four proof fences** of `crates/cli/tests/format_json_success_axis.rs` — `#[test]` targets
    on the debug build, not drivable in this posture: **`(5, C-18)`, unchanged reason.**
11. **`(5, C-13)`** — the wording of a registry doc-comment that no invocation emits. Not drivable;
    unchanged reason.
12. **The `rename` half of `(5, DEFECT 3)`**, and `(5, DEFECT 2)`, `(5, C1)`, `(5, D1)`,
    `(5, DEFECT 1 · rc.20)` = `(2, A2-2)`, `(5, DEFECT 2 · rc.20)` — outside the subject; listed in
    §2 so their absence is not read as closure.
13. **Any comparison against an older binary** (was `(R7, D-2)` or `(R7, D-3)` already true on
    rc.20?). None is installed and none may be built; both findings say *provenance not
    established*.

---

## 6 · Counts

| | |
|---|---|
| rigs built | 6 (`fresh` ×2, `committed-singletons`, `refs-post-hoc`, `migrated`, `fresh --pack-from-dev`) + 2 `mktemp -d` roots for the hostile-cwd cells |
| numbered rows driven | **213** = A 20 · B 24 · C 31 · D 31 · E 9 · F 11 · G 3 · H 59 · I 3 · J 18 · K 4 |
| verdict *matches contract* | **188** |
| rows carrying a baseline STILL-OPEN datum | **14** — rc.17 `DEFECT A`: H1, H2, H3 · `(5, DEFECT 3)` and its code-less malformed-address family: H6, H7, H8, H10, H28 · `(5, DEFECT 4)`: H16, D18, D19, D21, D22, D24 |
| rows carrying a new finding | **9** — `(R7, D-1)`: F4 · `(R7, D-2)`: D20, H26, H29, H30, H31 · `(R7, D-3)`: C14, C15, H37 |
| recorded data, no contract either way | **2** — C19, C30 |
| declared key set == driven key set | whole-doc 20 / 20 · `doc list` rows on every listing · `doc schema` 18 / 18 |
| `doc show` registry arms driven | 8 / 8 |
| rows not driven | 13 entries (§5) |
| tier-1 / tier-2 / tier-3 findings | **0 / 0 / 3** |

Two rows are multi-argv and are counted **once** each: E7 (18 `doc schema` drives) and C31 (21
list-row / `doc show` pairs). G1 is a property asserted over every batch and is counted once.

---

## 7 · Doors covered

`doors_covered` — every clap leaf (`VERB_KINDS` spelling) that is the door of ≥ 1 driven row:

- **`doc show`** — blocks A, B, D, F, H, J (all 8 registry arms; both reject envelopes)
- **`doc list`** — blocks C, D, F, H, J
- **`doc schema`** — blocks E, F, H
- **`milestone create`** — block K (the baseline tier-1 row's first mint door)
- **`task amend`** — block K (its second)

Five leaves. Nothing else is claimed.

---

## 8 · Notes

1. **No tier-1 row on this run.** The three doors under review are `Read` leaves; block G shows
   they wrote nothing, and the baseline's one tier-1 row is closed at both doors (block K).
2. **The M55 additions hold as declared.** `title` is on every whole-doc serve (committed and
   staged), `null` — a present key — when the H1 is gone, and on no slice. `title` and `fields` are
   on every `doc list` row in every state, valued exactly as the per-state table says; `fields` is
   `null` and never `{}` on every row but a managed one that parses. The default projection reports
   the schema default on both doors, on both arms, for all three doctypes that declare one, and the
   stored bytes — committed and staged — are untouched. `doc schema`'s `contract-version` is 7 on
   all eighteen doctypes.
3. **The rc.20 record is wrong in two places this row can show.** (a) Its `ENVELOPE_ARMS`
   partition (`60 Pinned / 6 Unpinned`; `60 / 3 / 2`) does not match the registry, and the range
   moved neither column. (b) Its *unreachable* `CompoundFieldSlice` arm is reachable through
   `milestone-record.base`.
4. **`(5, DEFECT 4)` got a neighbour, not a fix.** After M55 the same doc can answer `status: open`
   on the whole-doc read and *names no leaf `status`* on the leaf read. The design declares that
   split; the message does not say *absent from this doc* — it says the leaf does not exist.
5. **Two things a `status == "<default>"` filter still misses**, both served as the contract says
   (*a stored value is always served as stored*), recorded because the projection's stated purpose
   is that filter: a present-but-empty `status:` line reads `""` (D28), and a managed doc that does
   not parse reads `fields: null` (C8, C10).
6. **An undeclared `## …` section appended to a managed doc does not break its parse**, and its
   prose appears in no `sections` entry of the JSON serve (seen while looking for the *does not
   parse* fixture). Not graded: it is the parser's leniency, not a read-contract statement, and
   conformance is `jigc validate`'s.
7. **The edge index is not on disk in any rig as built** — `.jigc/index/` holds only the lock file
   until a gate runs. The byte-identity claim for `edges.json` in block G is therefore made on the
   one rig where `jigc task validate` had written it; elsewhere the claim is *absent before, absent
   after*.
8. **`(R7, D-1)` sits on the boundary with row 8** (composed surfaces). It is filed here because the
   contradicted statement is about what this row's door prints; the reconciler should not count it
   twice.
9. **Hygiene.** Rig paths are written `$REPO` / `<tmp>`; commit ids are the rigs' own; no
   environment value is recorded (`CLAUDECODE`: set; `JIGC_PACK_DIR`: unset in five rigs, set in
   the `--pack-from-dev` one).

---
---

# Reconciliation ledger — ROW 7 · pinned read contracts (rc.24)

**Who wrote this part.** The reconciler: it did not author the driver table above and did not
build the code. Inputs: the driver table (above), the Codex source pass (`axis7-codex.md`, exit 0,
non-empty — **this row has a source pass**), the row's scope file.

**The rule applied** (`completions/artifacts/M51/acceptance-design.md` → *The reconciliation
rule*): a claim by one that the other cannot reproduce is a lead, not a finding. Every Codex claim
below was entered as `lead(codex, …)` and then **driven** on the installed binary — to a repro
block, to a refutation with its falsifying datum, or left an OPEN lead with the reason. Every
driver defect was re-driven once by the reconciler on rigs of its own.

**Binary.** `~/.local/bin/jigc` → `jigc 1.0.0-rc.24`, asserted before the first drive and again
inside every rig (the rig loader refuses any other answer). Release posture.

**Rigs (the reconciler's own, none shared with the driver).** Six, each
`rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit`
— stdout only, two steps, the `$REPO` guard and a *cwd is a rig root* guard before every git
command: `fresh`, `fresh` (second), `committed-singletons`, `refs-post-hoc`, `migrated`, and
`fresh --pack-from-dev --schema adr <reshaped adr.yaml>` (the one Codex claim needs a doctype shape
no shipped pack carries). Plus two `mktemp -d` roots for the hostile-cwd cells. No teardown, no
`rm`. Nothing was written under `.jigc/` by hand; staged copies were read with `command grep` /
`shasum` only, each with a before-control. Every exit was read from `cmd > out 2> err; rc=$?`,
never through a pipe. `CLAUDECODE`: set. `JIGC_PACK_DIR`: unset in five rigs, set in the
pack-copy one. The working repository after the last drive: `git status --porcelain` empty, HEAD
`bffa6667` unmoved.

---

## R.1 · The door-set count against the registries

Recounted mechanically (a script over the const blocks at `bffa6667`), not transcribed from either
party.

| registry | the driver read | the Codex pass read | the reconciler's count | verdict |
|---|---|---|---|---|
| `ENVELOPE_ARMS` (`crates/cli/src/render.rs`) | 66 arms · 48 leaves + 1 empty path · 59 `Pinned` / 7 `Unpinned` · 61 `Success` / 3 `Adjudicated` / 2 `Reject` · `doc show` 8 · `doc schema` 1 · `doc list` 1 | 66 arms / 48 leaves · 59 / 7 · 8 + 1 + 1 | **66** `EnvelopeArm {` rows · **49** distinct `path` values (48 leaves + the empty path, which carries 2 rows) · **59 / 7** · **61 / 3 / 2** · **8 / 1 / 1** | all three agree |
| `VERB_KINDS` (`crates/cli/src/cli.rs`) | 48 (12 `Read` · 36 `Write`) | 48 leaves | **48** (12 · 36); `doc show`, `doc schema`, `doc list` are `Read` | agree |
| `DOC_READ_VERBS` | 3 | 3 | **3** — `show` · `schema` · `list` | agree |
| `WHOLE_DOC_KEYS` (+ `STAGED_KEY`) | 7 (+ `staged`) | 7 (+ `staged`) | **7** — `type` · `slug` · `title` · `item-count` · `schema-version` · `fields` · `sections`; `STAGED_KEY = "staged"` | agree |
| `DocRow` | 6 keys | `title` and `fields` always serialized | **6** fields, no `skip_serializing_if` on any | agree |
| `SchemaContract.contract_version` | 7 | 7 | **7** | agree |
| `ENVELOPE_OWED_CODES` | 4 | names 3 of them | **4** — `store.not-found` · `store.no-such-leaf` · `store.unknown-type` · `store.fixed-identity` | agree (the Codex pass names three; it does not claim the list is three) |

**The driver's door-set count stands.** Three doors under review (`doc show`, `doc list`,
`doc schema`); two more leaves (`milestone create`, `task amend`) are doors of the one baseline
tier-1 row. The driver's datum against the rc.20 record (59 / 7 and 61 / 3 / 2, where rc.20 wrote
60 / 6 and 60 / 3 / 2) matches the registry as counted here.

---

## R.2 · Every Codex claim — `lead(codex, …)` → CONFIRMED · REFUTED · OPEN LEAD

| # | `lead(codex, <claim>)` | status | the datum |
|---|---|---|---|
| X1 | A **non-header body field** carrying `default:` is projected into `fields` by both `doc show` (whole doc) and `doc list`, committed and `--task`, although `design/findings-channel.md` §5 / §10 scope the projection to *header* fields | **CONFIRMED — a finding, origin codex: `(R7, C-1)`, proposed tier 3** | block R-X1: `fields.visibility == "public"` on four reads of a doc whose body field group stores no `visibility` bullet; bytes untouched |
| X1a | …and `doc show <type>:<slug>#<body-section> --format json` still omits it; the file bytes remain unchanged | **CONFIRMED** | block R-X1: `#context` serves the slot prose alone; `#context/visibility` blocks `store.no-such-leaf`; `shasum` before == after, committed and staged |
| X2 | `(5, DEFECT 1)` — **CLOSED**: the shared mint guard rejects a title whose slug is empty; no bypass in the read verbs | **CONFIRMED** | block R-K: five drives at the two mint doors exit 1 `write.unslugable-title`, nothing minted; the three read doors mint nothing (R-G) |
| X3 | `(5, DEFECT 3)` — **STILL-OPEN**: user-address parsing returns a code-less malformed-address error | **CONFIRMED** (the `doc show` half; the `rename` half not driven) | `doc show nosuchtype` → exit 1, `{"error":"malformed address …"}`, no `findings`, no key — task-less and on `--task` |
| X4 | `(5, DEFECT 4)` — **STILL-OPEN**: an absent optional leaf is not synthesized on a fragment read; only whole-doc `fields` receives defaults | **CONFIRMED** | `doc show 'vision:vision#meta/grounded-in'` → exit 1 `(store.no-such-leaf, vision:vision#meta/grounded-in)` while `doc schema vision` lists the leaf `required: false`; `…#meta/status` blocks on a doc whose whole-doc `fields.status` is the projected `"open"` |
| X5 | rc.17 `DEFECT A` — **STILL-OPEN**: `store.unknown-type` keys to the full address at `doc show`, the bare type at the doctype-only doors | **CONFIRMED** | `doc show 'nosuchtype:x'` → target `nosuchtype:x`; `doc schema nosuchtype`, `doc list nosuchtype` → target `nosuchtype` |
| X6 | `(5, DEFECT 2)`, `(5, C1)`, `(5, D1)` — outside this row; *no disposition change found* | **OPEN LEAD** | Not driven: a `config` write door and two `start` arms, outside this row's three doors by its scope file. A source read is not a drive; they stay where rc.20 left them (STILL-OPEN, 1.x) |
| X7 | `(5, DEFECT 1 · rc.20)` = `(2, A2-2)` and `(5, DEFECT 2 · rc.20)` — outside this row; *no disposition change found* | **OPEN LEAD** | Not driven: fan-out posture previews and `task amend`'s `amend.head-shape` locus, outside this row's doors by its scope file. They stay where rc.20 left them |
| X8 | `DOC_READ_VERBS` is exactly `show`, `schema`, `list` | **CONFIRMED** | R.1's count; `jigc doc --help` → exit 0, eleven subcommands, of which `show` · `schema` · `list` are the three that wrote nothing in R-G's 48-invocation sweep |
| X9 | `WHOLE_DOC_KEYS` is seven keys including `title`; `staged` is inserted on whole-doc staged serves only; fragments receive none | **CONFIRMED** | 15 committed whole-doc serves (distinct docs or shapes, six rigs) → exactly the 7; every staged whole-doc serve whose key set was read (6: `inconsistency`, `commit`, `jigc-feedback` ×2, `vision`, `adr`) → the 7 + `staged`; every slice re-driven (R-C, R-D, R-X1 — committed and `--task`) → a bare value, no `staged`, no `title`, no `item-count` |
| X10 | `DocRow` always serializes `title` and `fields`; a non-qualifying row emits `null`, never an omitted key or `{}`; managed-parsing, unregistered, unparseable, orphaned and staged producers implement the state table | **CONFIRMED** | block R-C: one key list `["id","path","state","item-count","title","fields"]` on every listing; `fields: null` on 3 unregistered, 2 orphaned and 3 managed-unparseable rows; a map on every managed-parsing and every staged row |
| X11 | The staged-elsewhere note uses the doctype-aware predicate the routed listing uses — O24 closed | **CONFIRMED** | block R-J: with a task staging only an `inconsistency`, `doc list adr` → stderr 0 B; with a task staging only a `jigc-feedback`, `doc list inconsistency` → stderr 0 B; with two tasks, each typed note names only the task that stages the type |
| X12 | `read_h1` and the H1 rewrite share `h1_span`, so the reader agrees with the write primitive | **CONFIRMED on the driven arms; one arm OPEN** | `jigc doc rename <addr> --to "Row retitled in task" --task <t>` (exit 0) rewrites the staged `# H1`; `doc list --task` and `doc show --task` both then answer `title: "Row retitled in task"` (`command grep -n '^# '` on the staged copy finds the one line). Front matter and a fenced `# …` are skipped by the reader (A18's shape re-driven: `title: null`). **OPEN:** the BOM arm — no drive wrote a BOM-prefixed file (the driver's §5.5 names the same gap) |
| X13 | Every stdout JSON serialization of the three doors has its registry row: 8 `doc show` arms, 1 `doc schema`, 1 `doc list`; the registry is 66 / 48, 59 / 7 | **CONFIRMED** | R.1's count; all eight `doc show` arms served on re-drive (whole-doc committed · whole-doc staged · fields-group · slot · item-array · item · list-field · compound-field), `doc schema` on 18 / 18 doctypes, `doc list` on every row state |
| X14 | `doc schema` is `contract-version: 7`; both new doctypes are manifest-listed at `schema-version` 1; no other methodology hash moved | **CONFIRMED** for the first two; **OPEN LEAD** for *no other hash moved* | `doc schema <ty> --format json` → exit 0, `contract-version: 7`, the seven keys, on 18 / 18; `jigc-feedback` and `inconsistency` → `schema-version: 1`. **Open:** no surface of the release binary prints a `schema-hash`, and no rc.20 binary is installed to compare against. What a drive does show: pack-load is clean in all six rigs (a hash that moved under an unmoved version would block every verb), so the shipped manifest equals the shipped schemas — that is not the claim *nothing moved since rc.20* |
| X15 | A reject carrying `store.not-found`, `store.no-such-leaf` or `store.unknown-type` is forced through the findings envelope | **CONFIRMED** | block R-H: all three codes (and `store.fixed-identity`, the fourth owed code) → exit 1, `{findings, schema_version: 3}`, one JSON document on stderr, stdout 0 B, key `{code, target}`; the plain arm carries the same code, locus and route |
| X16 | The rc.23 → rc.24 trailer range alters none of these read producers, schemas, `contract-version` or pinned JSON keys; pinned JSON exposes subjects, not full commit messages | **CONFIRMED for the three read doors · one word CORRECTED · one half OPEN** | **Driven** (rig `fresh` (second), whose commits jigc made with `CLAUDECODE` set): 9 argv × 3 formats = 27 invocations of `doc show` / `doc list` / `doc schema` (whole doc, slice, `--task`, the `commit` doc), all exit 0, 13 910 B of stdout + stderr — `Co-Authored`: 0 matches, the landed commit's subject: 0 matches (controls: the doc's slug is found 20 times in the same bytes; `git log -1 --format=%B` on the commit jigc made carries `Co-Authored-By: Claude <noreply@anthropic.com>` — 1 match). **Corrected (a source datum, not a drive):** *schemas* did move in that range by one line — a slot `hint:` reword in `planning-record.yaml`; `doc schema planning-record` on rc.24, all three formats, exit 0, projects no hint text (0 matches of the reworded sentence; control: the section id `claim-driven` is found in each), so no byte of these doors moved with it. **Open:** *pinned JSON exposes subjects* is a statement about the committing doors' acks, which this row does not drive (row 10 owns the trailer) |
| X17 | Bounds: source-only review of tag `jigc-v1.0.0-rc.24`; no binary driven; M55's F21 report-only and outside this row | not a claim to drive | recorded as the pass's own bound |

**No Codex claim is refuted on this row.** Sixteen claims: 1 confirmed as a finding (X1 + X1a),
4 baseline dispositions confirmed (X2 – X5), 7 consistency claims confirmed by drive (X8 – X11,
X13, X15, and the drivable halves of X12, X14, X16), 2 wholly open as outside the row (X6, X7), and
3 partly open (X12's BOM arm, X14's hash-movement half, X16's committing-door half).

**No driver row is contradicted by the source pass**, and the source pass contradicts no driver
defect: it is **silent** on `(R7, D-1)`, `(R7, D-2)` and `(R7, D-3)`. Silence is not refutation;
all three were re-driven (R.3).

### Block R-X1 — `(R7, C-1)`, origin codex: a body-field default is projected

```
rig: fresh --pack-from-dev --schema adr <reshaped>     (JIGC_PACK_DIR: set; the pack copy's freeze manifest is dropped by the rig)
     the reshaped schema = the shipped adr.yaml with two trailing fields added to the BODY section `context`:
       - id: context
         slot: { hint: … }
         fields:
           - { id: visibility, type: enum, of: [public, private], default: public }
           - { id: audience, type: string }
$ jigc doc schema adr --format json                       -> 0
  {"id":"visibility","type":"enum","of":["public","private"],"required":true,"author-required":false,"default":"public","section":"context","set-field":"adr:<slug>#context/visibility"}
$ jigc start --workflow single-task "decide the cache"    -> 0   task minted: decide-the-cache
$ jigc doc create adr --title "Body default probe" --task decide-the-cache              -> 0   adr:body-default-probe
$ jigc doc set-slot 'adr:body-default-probe#{context,decision,consequences}' --from-file - --task decide-the-cache   -> 0 each
$ jigc doc set-field 'adr:body-default-probe#context/audience' --value operators --task decide-the-cache            -> 0
  staged copy, body of `## Context`:   "Some context prose.\n\n<!-- fields -->\n- visibility: public\n- audience: operators"
  (the create wrote the default into the stored bytes — so the absent state is built out of band, below)
$ (commit doc filled) jigc task finalize decide-the-cache -> 0   62c414a   promoted docs/decisions/body-default-probe.md

$ command grep -c '^- visibility:' docs/decisions/body-default-probe.md   -> 1     (before-control)
  command grep -c '^- audience:'   <same file>                            -> 1     (control)
$ sed -i '' '/^- visibility: /d' docs/decisions/body-default-probe.md
$ command grep -c '^- visibility:' <same file> -> 0 ;  '^- audience:' -> 1
$ git -C "$REPO" add …; git -C "$REPO" commit -q -m "chore: hand-delete body visibility (out-of-band)"   -> 0 ; git status --porcelain -> empty
$ shasum -a 256 <that file>  -> c1
$ jigc doc show adr:body-default-probe --format json       -> 0
  keys = the 7     fields {"audience":"operators","date":"2026-10-03","status":"proposed","visibility":"public"}
$ jigc doc list adr --format json                          -> 0
  ["adr:body-default-probe","managed",{"audience":"operators","date":"2026-10-03","status":"proposed","visibility":"public"}]
$ jigc doc show 'adr:body-default-probe#context' --format json             -> 0   "Some context prose."      (the slice omits it — X1a)
$ jigc doc show 'adr:body-default-probe#context'                           -> 0   Some context prose.
$ jigc doc show 'adr:body-default-probe#context/visibility' --format json  -> 1   key {store.no-such-leaf, adr:body-default-probe#context/visibility}
$ jigc doc show 'adr:body-default-probe#context/audience' --format json    -> 0   "operators"                (control: a stored body field serves)
$ shasum -a 256 <that file> == c1   -> BYTES untouched ;  git status --porcelain -> empty

staged arm (the binary makes the copy):
$ jigc start --workflow single-task "revisit the probe"   -> 0   task minted: revisit-the-probe
$ jigc doc set-slot 'adr:body-default-probe#options' --from-file - --task revisit-the-probe --format json   -> 0   copied_in: true
$ command grep -c '^- visibility:' .jigc/tasks/revisit-the-probe/docs/adr:body-default-probe.md  -> 0 ;  '^- audience:' -> 1 (control)
$ jigc doc show adr:body-default-probe --task revisit-the-probe --format json   -> 0
  {"staged":"revisit-the-probe","fields":{"audience":"operators","date":"2026-10-03","status":"proposed","visibility":"public"}}
$ jigc doc list adr --task revisit-the-probe --format json                      -> 0   the same map
$ jigc doc show 'adr:body-default-probe#context' --task revisit-the-probe --format json   -> 0   "Some context prose."
$ shasum of the staged copy before == after   -> STAGED BYTES untouched
```

**Tier — proposed 3, and the weakest kind of 3.** The behaviour is exactly as Codex read it. What
it contradicts is **narrower than the pass says**:

- *Contradicted:* `design/findings-channel.md` §5 (*"`title` + `fields` — the header fields"*) and
  §10's *absent-default projection* row (*"a defaulted **header** field is absent from the stored
  doc"*), and `jigc doc list --help` (*"`fields` its **header** fields as `doc show` serves them"*)
  — driven: the row's `fields` carries the body-group leaves `audience` and `visibility`.
- *Not contradicted:* `design/doc-read-surface.md` — the contract that owns these two doors —
  states the driven behaviour in so many words (*"every simple section's leaves flattened … the
  header front-matter + any body field group"*; *"A declared simple-section field that carries a
  `default:` and is absent … projects its schema default"*). The Codex pass cites this sentence
  itself. So the binary conforms to the owning contract, and the disagreement is **doc ↔ doc**
  (two design docs) plus one help phrase.
- *Reach:* **none of the 18 shipped doctypes declares a body field group at all** (a scan of both
  packs' schema files: 0 non-header sections carry `fields:`), so with the shipped packs the two
  wordings describe the same set and the behaviour is unreachable; it takes a project- or
  pack-authored schema.

**Not tier 1 — both halves are missing:** the doors are `Read` leaves (no committing, destroying
or moving door), and nothing is lost (checksums identical before and after, committed and staged;
`git status` empty). **Not tier 2:** no route is involved. The assembler may equally file it as an
`inconsistency` between the two design docs rather than a defect in the binary; it is kept here as
a confirmed finding because the rule promotes a driven Codex claim, and the claim was driven.

---

## R.3 · Every driver defect — re-driven once by the reconciler

| key | origin | re-drive | status | tier, and why |
|---|---|---|---|---|
| **`(R7, D-1)`** — the two triage steps say `jigc doc list <type>` lists each record *"by its slug and its `status`"*; the command as printed lists no status | driver | block R-D1 — **reproduces** | **CONFIRMED** | **3.** A composed surface states what a command prints and the binary prints something else (`id  path  state`, where `state` is the registration state). **Not tier 2:** the route is not dead — `--format json` carries `.docs[].fields.status`, and `doc show` serves it. **Not tier 1:** exit 0 on a `Read` leaf, no bytes lost, no committing door. Filed once, here; row 8 (composed surfaces) must not count it again |
| **`(R7, D-2)`** — on `--task`, the three fragment-miss routes say *committed*, including for a doc with no committed copy | driver | block R-D2 — **reproduces**, on a staged-only doc and on a doc whose two copies differ | **CONFIRMED** | **3.** The route names a copy the read did not serve; `design/doc-read-surface.md` states the rule for the sibling code (*"the shared tail claiming a staged doc is committed would be a lie"*). Code, key and exit are right. **Not tier 2:** the reader is not stranded — the message names the leaf and section, and `store.no-such-section`'s route on the same arm lists the real sections. **Not tier 1:** exit 1 on a `Read` leaf; nothing written. Provenance not established (no older binary installed) |
| **`(R7, D-3)`** — a `doc list` row prints an `id` that `doc show` refuses, and the refusal routes back to `doc list` | driver | block R-D3 — **reproduces**; and one more datum the driver did not have | **CONFIRMED** | **3.** The row's `id` is not an address; `doc list --help` says every instance *"carries its `<type>:<slug>` identity"*. **New datum:** the binary says so itself at another door — `jigc ingest` classifies the same file `blocking · ingest.unaddressable-identity — … its name is not a doc id, so no `<type>:<slug>` address reaches it`, with a copy-runnable `git … mv` route. That is also why it is **not tier 2**: the row's `state: unregistered` points at adoption, and the adoption door answers with a mechanical route, so the loop `doc show` → `doc list` → `doc show` has an exit. **Not tier 1:** `Read` leaves, nothing lost. Outside the M55 delta (the identity leg is M50's); no design doc declares it as a bound of `doc list` (searched `design/` for the code and the phrase — only `design/validation.md`'s `ingest` / `relocate` rows carry it), so it is **not demoted** |

**Tier-1 adjudication, both directions.** No finding of this row is tier 1: every confirmed
finding sits on a `Read` leaf (`VERB_KINDS`), and for each the reconciler holds a before/after
digest or checksum showing no byte moved — so **both** halves (the exit-0 loss, and the
committing / destroying / moving door) are missing in all four. No finding the driver tiered lower
shows both halves. The one row of this run that touches a committing door is the baseline tier-1
row `(5, DEFECT 1)`, and it is closed at both doors (R-K).

### Block R-D1

```
rig: fresh; one inconsistency landed through `report-inconsistency` (finalize 84160e6), one jigc-feedback through
     `report-jigc-feedback` (finalize 98d437d); both `status:` lines then hand-deleted and committed (block R-D)
$ jigc start --workflow triage-inconsistency "triage the retry inconsistency"     -> 0   task minted: triage-the-retry-inconsistency
  lines 9-12 of the composed text:
    … The committed records, each by
    its slug and its `status`:

    jigc doc list inconsistency
$ jigc doc list inconsistency                                                     -> 0   stderr 0 B
  id  path  state
  inconsistency:retry-budget-differs-between-readme  docs/inconsistencies/retry-budget-differs-between-readme.md  managed
$ jigc doc list inconsistency --format json                                       -> 0   .docs[0].fields.status = "open"
$ jigc start --workflow triage-jigc-feedback "triage the staged note feedback"    -> 0   task minted: triage-the-staged-note-feedback
  lines 9-12:  … The committed rows, each by its
               slug and its `status`:

               jigc doc list jigc-feedback
source of the sentence (a read, for the fix's benefit): the methodology pack steps `author-inconsistency-triage`, `author-jigc-feedback-triage`
```

### Block R-D2

```
rig: fresh (second); task `second-open-task` (report-inconsistency)
$ jigc doc create inconsistency --title "Two docs disagree" --task second-open-task                                    -> 0
$ jigc doc add-item 'inconsistency:two-docs-disagree#sides' --title docs/a.md --slug a --task second-open-task          -> 0
control — nothing is committed:  `ls docs` -> No such file or directory ;  `git ls-files | command grep -c inconsisten` -> 0
$ jigc --format json doc show 'inconsistency:two-docs-disagree#meta/nope'                                -> 1
  key {store.not-found, inconsistency:two-docs-disagree}
$ jigc --format json doc show 'inconsistency:two-docs-disagree#sides/nope' --task second-open-task       -> 1
  key {store.no-such-item, inconsistency:two-docs-disagree#sides/nope}        route "name an item that exists in the committed doc"
$ jigc --format json doc show 'inconsistency:two-docs-disagree#sides/a/nope' --task second-open-task     -> 1
  key {store.no-such-leaf, inconsistency:two-docs-disagree#sides/a/nope}      route "name a leaf that exists in the committed item"
$ jigc --format json doc show 'inconsistency:two-docs-disagree#meta/nope' --task second-open-task        -> 1
  key {store.no-such-leaf, inconsistency:two-docs-disagree#meta/nope}
  route "name a field the committed section carries (a section's prose slot is the section itself — address it as `#<section>`)"
$ jigc doc show 'inconsistency:two-docs-disagree#meta/nope' --task second-open-task                       -> 1   (plain: the same route line)
control — an arm-neutral route on the same arm:
$ jigc --format json doc show 'inconsistency:two-docs-disagree#nope' --task second-open-task             -> 1
  key {store.no-such-section, …#nope}   route "address one of the sections `inconsistency:two-docs-disagree` carries — `meta`, `sides`, `description`, `evidence`, `resolution` — …"

rig: refs-post-hoc, T = ground-the-vision-in-research  (the two copies differ)
$ jigc doc show 'vision:vision#meta' --task T --format json    -> 0   {"grounded-in":["research:context-loss"],"schema-version":"1"}
$ jigc doc show 'vision:vision#meta' --format json             -> 0   {"schema-version":"1"}
$ jigc --format json doc show 'vision:vision#meta/nope' --task T   -> 1   key {store.no-such-leaf, vision:vision#meta/nope}
  route "name a field the committed section carries (…)"
```

### Block R-D3

```
rig: fresh; docs/inconsistencies/Not_A_Slug.md written by hand — stamped (`schema-version: 1`), conformant front matter,
     an H1, the four section headings — and committed with plain git ("chore: row-state fixtures (out-of-band)")
$ jigc doc list --format json                           -> 0
  {"id":"inconsistency:Not_A_Slug","path":"docs/inconsistencies/Not_A_Slug.md","state":"unregistered","item-count":0,"title":"Stamped but not a slug","fields":null}
$ jigc --format json doc show 'inconsistency:Not_A_Slug'  -> 1   stdout 0 B
  {"error":"blocking · store.malformed-slug — \"Not_A_Slug\" is not a valid doc slug — the `<slug>` head of address `inconsistency:Not_A_Slug`\n  route: `jigc doc list` lists the committed docs and the identity each one carries; use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)"}
tiering control (run last in this rig; `ingest` is not claimed as a door):
$ jigc ingest                                           -> 0
  needs-reconcile docs/inconsistencies/Not_A_Slug.md → inconsistency
    blocking · ingest.unaddressable-identity — conformant `inconsistency` at `docs/inconsistencies/Not_A_Slug.md` is not adopted: its name is not a doc id, so no `<type>:<slug>` address reaches it — jigc names every doc it writes `<slug>.md`
    route: rename it to a doc id — `git -C $REPO mv docs/inconsistencies/Not_A_Slug.md docs/inconsistencies/not-a-slug.md` — then re-run `jigc ingest`; …
```

---

## R.4 · Baseline rows — CLOSED / STILL-OPEN, as re-driven by the reconciler

| key | tier there | driver | Codex | reconciler's drive | **rc.24 status** |
|---|---|---|---|---|---|
| **`(5, DEFECT 1)`** (rc.17) — a mint door commits a record at a fabricated identity | **1** | CLOSED | CLOSED | block R-K: 5 / 5 refused, nothing minted | **CLOSED (stays closed)** — both mint doors |
| **`(5, DEFECT 4)`** (rc.17) — `doc show` blocks a declared-but-unpopulated optional leaf | 3 | STILL-OPEN | STILL-OPEN | reproduces; and the same block now meets an absent *defaulted* leaf whose whole-doc value is projected (declared in `design/doc-read-surface.md`) | **STILL-OPEN (expected, 1.x)** |
| **rc.17 `DEFECT A`** — `store.unknown-type`: URI-shaped target at `doc show`, bare id at `doc schema` / `doc list` | 3 | STILL-OPEN | STILL-OPEN | reproduces at all three doors | **STILL-OPEN (expected, 1.x)** |
| **`(5, DEFECT 3)`** (rc.17) — the colon-less-address bail carries no code — `doc show` half | 3 | STILL-OPEN | STILL-OPEN | reproduces, both arms; `vision:`, `:x` and `vision:vision#` land in the same code-less `Error` | **STILL-OPEN (expected, 1.x)**; the `rename` half **not re-driven** |
| M51 **`DEFECT A · B · C · D`** — the three read verbs' cells | — | CLOSED | (silent) | outside any repository: 5 / 5 JSON + 3 / 3 plain → exit 1, `{"error":"not inside a git repository …"}`, stdout 0 B; cwd deleted: 3 / 3 JSON + 1 plain → exit 1, one identical sentence | **still CLOSED** on `doc show` · `doc schema` · `doc list` |
| §D lead (M52): **`store.no-such-leaf`**, the 2nd `ENVELOPE_OWED_CODES` member | lead | CLOSED as a lead | envelope forced (X15) | driven at four producers (undeclared leaf · unpopulated optional leaf · absent defaulted leaf · undeclared item leaf), JSON and plain: `Reject::Findings`, key `{code, target}`, stdout 0 B | **CLOSED as a lead** — the owed envelope is paid; `(5, DEFECT 4)`, which rides the code, stays open |
| `(5, DEFECT 2)` (rc.17) · `(5, C1)` · `(5, D1)` (rc.19) | 3 | not re-driven | *no change found* (source) | not driven — outside the row | **where rc.20 left them: STILL-OPEN(1.x)** — absence here is not closure |
| `(5, DEFECT 1 · rc.20)` = `(2, A2-2)` | 2 | not re-driven | *no change found* (source) | not driven — outside the row | **where rc.20 left it: open, tier 2** |
| `(5, DEFECT 2 · rc.20)` | 3 | not re-driven | *no change found* (source) | not driven — outside the row | **where rc.20 left it: open, tier 3** |
| `(5, C-13)` | lead | open | (silent) | not drivable — the wording of a registry doc-comment no invocation emits | **OPEN LEAD, unchanged reason** |
| `(5, C-18)` | lead | open | (silent) | not drivable in release posture — four `#[test]` targets on the debug build | **OPEN LEAD, unchanged reason** |

### Block R-K — `(5, DEFECT 1)`

```
rig: fresh (second), as built — before any task.   cwd = $REPO
before:  H0 = git rev-parse HEAD ;  git status --porcelain -> empty ;  ls .jigc/tasks .jigc/milestones docs -> No such file or directory ×3
$ jigc --format json milestone create ""       -> 1   stdout 0 B
  {"error":"blocking · write.unslugable-title — cannot mint a milestone: its id is slugged from the title, and this title slugs to nothing — …\n  at: milestone\n  route: re-run with a title carrying ASCII letters or digits — the milestone id is slugged from it"}
$ jigc --format json task amend ""             -> 1   stdout 0 B
  {"error":"blocking · write.unslugable-title — cannot mint a task: …\n  at: task\n  route: re-run with a title carrying ASCII letters or digits — the task id is slugged from it, or omit the title — `jigc task amend` names the task after the commit it rewrites (`amend-<sha7>`)"}
$ jigc --format json milestone create "!!!"    -> 1   same code
$ jigc --format json task amend "   "          -> 1   same code
$ jigc --format json milestone create "日本語"  -> 1   same code        (one more than the driver drove: a title in another script)
after:   git rev-parse HEAD == H0 ;  git status --porcelain -> empty ;  the three directories still absent ;  jigc task list -> "no active tasks"
```

### Block R-H — reject funnels (the baseline rows' data, and X15)

```
rig: committed-singletons.   Every row: stdout 0 B, exactly one JSON document on stderr.
$ jigc --format json doc show 'nosuchtype:x'                    -> 1  Findings(3)  (store.unknown-type, nosuchtype:x)     route "list the available doctypes with `jigc describe`"
$ jigc --format json doc schema nosuchtype                      -> 1  Findings(3)  (store.unknown-type, nosuchtype)
$ jigc --format json doc list nosuchtype                        -> 1  Findings(3)  (store.unknown-type, nosuchtype)
$ jigc --format json doc show adr:nope                          -> 1  Findings(3)  (store.not-found, adr:nope)            route names `jigc doc show adr:nope --task <task-id>` and `jigc task list`
$ jigc --format json doc show vision:nope                       -> 1  Findings(3)  (store.fixed-identity, vision:nope)    route "`jigc doc show vision:vision` — …"
$ jigc --format json doc show nosuchtype                        -> 1  Error        "malformed address `nosuchtype`: missing ':' between type and slug — …"
$ jigc --format json doc show 'vision:'                         -> 1  Error        "malformed address `vision:`: empty slug — …"
$ jigc --format json doc show ':x'                              -> 1  Error        "malformed address `:x`: empty type — …"
$ jigc --format json doc show 'vision:Bad_Slug'                 -> 1  Error        "blocking · store.malformed-slug — …"
$ jigc --format json doc show 'vision:vision#'                  -> 1  Error        "malformed address `vision:vision#`: empty fragment — …"
$ jigc --format json doc show 'vision:vision#a/b/c/d/e/f'       -> 1  Findings(3)  (store.no-such-section, vision:vision#a/b/c/d/e/f)
$ jigc --format json doc show 'vision:vision#meta/nope'         -> 1  Findings(3)  (store.no-such-leaf, vision:vision#meta/nope)
$ jigc --format json doc show 'vision:vision#nope'              -> 1  Findings(3)  (store.no-such-section, vision:vision#nope)
$ jigc --format json doc show 'changelog:changelog#releases/9-9-9'        -> 1  Findings(3)  (store.no-such-item, …)
$ jigc --format json doc show 'changelog:changelog#releases/1-0-0/nope'   -> 1  Findings(3)  (store.no-such-leaf, …)
$ jigc --format json doc show 'vision:vision#meta/grounded-in'  -> 1  Findings(3)  (store.no-such-leaf, vision:vision#meta/grounded-in)
    message "`vision:vision#meta/grounded-in` names no leaf `grounded-in` in section `meta`"
    control: `jigc doc schema vision --format json` -> 0, lists {"id":"grounded-in","type":"ref","to":"research","required":false,…,"set-field":"vision:<slug>#meta/grounded-in"}
$ jigc --format json doc show vision:vision --task nope         -> 1  Findings(3)  (finalize.no-task, task:nope)          route "`jigc task list` lists the live tasks"
$ jigc --format json doc list --task nope                       -> 1  Findings(3)  (finalize.no-task, task:nope)
$ jigc --format json doc list vision --task nope                -> 1  Findings(3)  (finalize.no-task, task:nope)
$ jigc --format json doc show commit:x                          -> 1  Findings(3)  (store.transient-type, commit:x)
$ jigc --format json doc schema vision:vision                   -> 1  Findings(3)  (store.unknown-type, vision:vision)
$ jigc --format json doc list vision:vision                     -> 1  Findings(3)  (store.unknown-type, vision:vision)
snapshot (git status + HEAD + every file under .jigc/) before == after this batch

rig: refs-post-hoc, T = ground-the-vision-in-research
$ jigc --format json doc show research:context-loss --task T    -> 1  Findings(3)  (store.not-staged, research:context-loss)   route "`jigc doc show research:context-loss` — …"
$ jigc --format json doc show adr:nope --task T                 -> 1  Findings(3)  (store.not-staged, adr:nope)
$ jigc --format json doc show nosuchtype:x --task T             -> 1  Findings(3)  (store.unknown-type, nosuchtype:x)
$ jigc --format json doc list nosuchtype --task T               -> 1  Findings(3)  (store.unknown-type, nosuchtype)
$ jigc --format json doc show nosuchtype --task T               -> 1  Error        "malformed address `nosuchtype`: …"
plain arms, same rig — ten drives (the argv of H1, H2, H3, H6, H16, H17, H18, H4, H5, H20), each exit 1, stdout 0 B:
  nine print `blocking · <code> — …` / `at: <locus>` / `route: …` / the footer, code == the JSON arm's;
  the colon-less one prints the bare sentence + `route: run `jigc describe` for the doctype surface`, no code.

rig: fresh, after the row-state fixtures
$ jigc --format json doc show inconsistency:retry-budget-differs-between-readme   -> 1  Findings  (store.unparseable, …)  route "fix the committed file so it conforms to its schema"
$ jigc doc show inconsistency:retry-budget-differs-between-readme                -> 1  blocking · store.unparseable — … does not parse: section heading "Evidence" does not match required section `sides`
$ jigc --format json doc show 'jigc-feedback:foreign-note'                        -> 1  Findings  (store.unparseable, jigc-feedback:foreign-note)  route "adopt — run `jigc ingest` to route it; it is a foreign file, not an unmigrated managed doc"

hostile cwd:  X=$(mktemp -d …); cd "$X"    (`git rev-parse --show-toplevel` -> fatal: not a git repository)
$ jigc --format json {doc show vision:vision | doc show vision:vision --task x | doc schema vision | doc list | doc list vision --task x}
  -> 1 ×5, stdout 0 B, {"error":"not inside a git repository (no `.git` found from <tmp>) — run jigc from inside the target git repository; …"}
$ jigc {doc show vision:vision | doc schema vision | doc list}   -> 1 ×3, the same sentence, stdout 0 B
D=$(mktemp -d …); cd "$D"; rmdir "$D"     (`ls -d "$D"` -> No such file or directory)
$ jigc --format json {doc show vision:vision | doc schema vision | doc list}   -> 1 ×3   {"error":"cannot determine the current directory: No such file or directory (os error 2)"}
$ jigc doc list                                                               -> 1      the same sentence
```

---

## R.5 · Driver rows marked driven that carry no repro block

**One demotion.** **G1** asserts a digest *"identical on every batch"* across blocks A – F, H and J
in every rig, and block G records no rig, no argv list and no digest for any of those batches —
only G2's sweep and G3's control are in the block. **G1 is NOT driven; it is demoted in place
above.** The driven-row count is **212**, not 213. The property itself is carried as far as
evidence goes by G2 and by the reconciler's own digests (R-G).

**Rows whose evidence is the table row alone** — the block under their table names the rig and
says *as tabulated*, or shows a neighbour: `B2` · `C1` – `C6` · `C23` · `C27` · `E8` · `F2` ·
`F3` · `F5` · `F6` · `F9` · `H7` – `H15` · `H18` – `H28` · `H34` – `H36` · `H39` – `H47` ·
`H49` – `H55` · `H57` – `H59` · `I1` – `I3` · `J1` · `J2` · `J4` · `J7` – `J12` · `J14` – `J18`.
Each table row does state its argv, its exit and the surface observed, so the reconciler did not
demote on form alone; it **re-drove every one of them** on its own rigs. **All reproduce**, and
the repro is in this ledger (R-X1 · R-C · R-D · R-G · R-H · R-J · R-K and the list below).
**None is demoted.** One small count differs from the driver's prose and is said: of the ten plain-arm rows H38 – H47, **nine** carry `blocking · <code>` + `at:` + `route:` + the footer (the driver wrote *eight*); the tenth is the code-less colon-less bail. One construction differs and is said: `J15` (stdout unaffected by an open
task) was re-driven as `doc list adr --format json` with only a non-`adr` task open (stderr 0 B)
against the same call once a second task stages an `adr` (stderr 212 B) — `cmp` of the two stdouts:
identical.

**Rows with a driver block that the reconciler did not re-drive** (they stand on the driver's
block): `D26` (the committed value after the triage finalize) and the second of G3's two
write-controls. Everything else in tables A – K was re-driven or is subsumed by a re-driven sweep.

### Block R-C — whole-doc keys, slices, `doc list` rows × state (X9, X10, X13)

```
rig: fresh
$ jigc start --workflow report-inconsistency "the retry doc and the retry code disagree"   -> 0   task minted: retry-doc-and
$ jigc doc create inconsistency --title "Retry budget differs between README and client" --task retry-doc-and --format json   -> 0
$ jigc doc set-field '…#meta/kind' --value code-doc ; add-item '…#sides' ×3 (readme, client, adr) ; set-slot ×3            -> 0 each
$ jigc doc show inconsistency:retry-budget-differs-between-readme --task retry-doc-and --format json   -> 0
  keys ["fields","item-count","schema-version","sections","slug","staged","title","type"]   item-count 3  schema-version 1
  fields {"date":"2026-10-03","kind":"code-doc","schema-version":"1","status":"open"}
$ jigc doc show commit:retry-doc-and --task retry-doc-and --format json   -> 0
  {"fields":{"scope":"","type":""},"item-count":0,"schema-version":null,"sections":{"body":"","summary":"","trailers":[]},"slug":"retry-doc-and","staged":"retry-doc-and","title":"retry-doc-and","type":"commit"}
$ jigc doc list --task retry-doc-and --format json   -> 0   two rows, both `managed`, both `fields` a map; the `commit` row's `path` = its identity
$ jigc task finalize retry-doc-and                   -> 0   84160e6
$ jigc doc show inconsistency:retry-budget-differs-between-readme --format json   -> 0   the 7 keys, no `staged`
slices (committed), each exit 0, a bare value, no `title` / `staged` / `item-count`:
  #meta -> {"date":…,"kind":"code-doc","schema-version":"1","status":"open"}      #description -> "The README promises three retries; the client performs five."
  #sides -> [3 × {id, says, title}]      #sides/readme -> {"id":"readme","says":"Three retries.","title":"README.md"}
  #sides/readme/says -> "Three retries."      #sides/adr/title -> "adr:retry-policy#decision"      #meta/status -> "open"
$ (report-jigc-feedback, the same way)  jigc task finalize doc-list-note-names   -> 0   98d437d
$ jigc doc show jigc-feedback:staged-note-names-the-wrong --format json   -> 0   the 7 ;  '#meta' -> an object of 7 stored leaves ;  '#meta/found-in' -> a string
$ jigc doc list --format json        -> 0   jq '[.docs[]|keys_unsorted]|unique' -> [["id","path","state","item-count","title","fields"]]
$ jigc doc list ; jigc doc list --format human   -> 0, byte-identical:  "id  path  state" + one row per doc (no title, no fields)
$ jigc doc show <inconsistency> ; … --format human  -> 0, byte-identical, 366 B against 365 B stored

row states (fixtures hand-written and committed with plain git, as the driver's block C names them):
$ jigc doc list --format json   (fixtures still untracked)   -> 0   6 rows: no orphan row
$ jigc doc list --format json   (after the commit)           -> 0   8 rows, one key list:
  {"id":"inconsistency:Not_A_Slug",…,"state":"unregistered","item-count":0,"title":"Stamped but not a slug","fields":null}
  {"id":"inconsistency:conformant-but-unstamped",…,"state":"managed","item-count":0,"title":"Conformant but unstamped","fields":{"date":"2026-10-01","kind":"doc-doc","status":"open"}}
  {"id":"jigc-feedback:foreign-note",…,"state":"unregistered","item-count":0,"title":"Foreign note","fields":null}
  {"id":"jigc-feedback:foreign-untitled",…,"state":"unregistered","item-count":0,"title":null,"fields":null}
  {"id":"jigc-feedback:staged-note-names-the-wrong",…,"state":"managed","item-count":0,"title":null,"fields":{…,"status":"open"}}      (H1 hand-removed)
  {"id":null,"path":"docs/inconsistencies/archive/old.md","state":"orphaned","item-count":null,"title":"An orphan","fields":null}
  {"id":null,"path":"docs/inconsistencies/archive/untitled.md","state":"orphaned","item-count":null,"title":null,"fields":null}
$ jigc doc list inconsistency --format json   -> 0   3 rows, no orphan row
$ jigc doc show inconsistency:conformant-but-unstamped --format json   -> 0   {"title":"Conformant but unstamped","schema-version":null,"fields":{…no schema-version…}}
managed, does not parse (`sed -i '' '/^## Description$/d; /^## Sides$/d'` on the inconsistency, `'/^## Description$/d'` on the feedback doc; before-counts 1 1 1, after 0 0 0; plain commit):
$ jigc doc list --format json   -> 0
  {"id":"inconsistency:retry-budget-differs-between-readme",…,"state":"managed","item-count":0,"title":"Retry budget differs between README and client","fields":null}
  {"id":"jigc-feedback:staged-note-names-the-wrong",…,"state":"managed","item-count":0,"title":null,"fields":null}
$ jigc doc list --task triage-the-retry-inconsistency --format json   -> 0   the staged rows: title and `fields` from the staged copies
$ jigc doc list commit --task triage-the-retry-inconsistency --format json   -> 0   {"id":"commit:<task>","path":"commit:<task>","state":"managed","item-count":0,"title":"<task>","fields":{"scope":"","type":""}}
$ jigc doc list --task triage-the-retry-inconsistency                 -> 0   "id  path  state" + two rows

rig: committed-singletons
$ jigc doc show {vision:vision | roadmap:roadmap | decisions-log:decisions-log | changelog:changelog} --format json   -> 0 each, the 7 keys
  titles "Vision" · "Roadmap" · "Decisions Log" · "Changelog" ;  item-count 0 · 1 · 0 · 2 ;  schema-version 1 · 1 · 1 · 2
$ jigc --format json doc show vision   -> 0, the 7
$ slices: changelog#meta -> {"schema-version":"2"} · vision#thesis -> string · changelog#releases -> array · roadmap#milestones -> array ·
          changelog#releases/1-0-0/changes -> array · changelog#releases/1-0-0 -> object · changelog#releases/1-0-0/date -> "2026-10-03"
$ jigc doc list --format json   -> 0   4 managed rows (CHANGELOG.md · docs/decisions-log.md · docs/roadmap.md · VISION.md), `fields` {"schema-version": …}
$ for ty in <the 18 shipped doctypes>: jigc doc schema $ty --format json   -> 0 ×18, contract-version 7 ×18, the 7 keys ×18
  home.kind: location 12 · placement 5 · transient 1 (`commit`: {"kind":"transient","path":null}) ;  identity.kind: slugged 13 · fixed 5
  `default` on adr.status (proposed) · inconsistency.status (open) · jigc-feedback.status (open), and nowhere else
  schema-version: adr 2 · changelog 2 · completion-record 2 · deferral-ledger 2 · milestone-record 3 · the other thirteen 1
$ jigc doc schema {jigc-feedback | inconsistency} vs `--format human`   -> `cmp` identical; "(default: open)" printed on `status`
$ jigc doc schema vision --task x   -> 2   "error: unexpected argument '--task' found"       (task-less, as its help says)

rig: refs-post-hoc, T = ground-the-vision-in-research
$ jigc doc show vision:vision --task T --format json   -> 0   the 7 + staged ;  fields {"grounded-in":["research:context-loss"],"schema-version":"1"}
$ jigc doc show research:context-loss --format json    -> 0   the 7
$ slices --task T: #meta -> object (no `staged`) · #thesis -> string · #meta/grounded-in -> ["research:context-loss"]
$ jigc doc list --task T --format json ; doc list vision --task T ; doc list research --task T   -> 0 ; 0 ; 0 {"docs":[]} stderr 0 B

rig: migrated
$ jigc doc show vision:vision --format json ; jigc doc show vision --format json   -> 0 ; 0 ; `cmp` identical
$ jigc milestone create "Alpha wave" --format json   -> 0   (fixture — no verdict recorded against it here)
$ jigc doc show milestone-record:alpha-wave --format json   -> 0   the 7 ;  fields {"base":{"sha":"<sha40>","short":"<short>"},"schema-version":"3","status":"active"}
$ jigc doc show 'milestone-record:alpha-wave#meta/base' --format json   -> 0   keys ["sha","short"]          (the compound-field arm is reachable — the driver's datum against rc.20 holds)
$ jigc doc show 'milestone-record:alpha-wave#meta/base'                 -> 0   "<sha40> <short>"
$ jigc doc list milestone-record --format json   -> 0   one row, `fields.base` an object

rig: fresh (second) — an ADR landed through `single-task` (finalize 7ca09c6)
$ jigc doc show adr:single-node-cache --task choose-a-cache-strategy --format json   -> 0   the 7 + staged   (before the finalize)
$ jigc doc show adr:single-node-cache --format json                                   -> 0   the 7
no-H1 shapes (hand edits, plain commits): without `## Options` -> `doc list adr` row {"state":"managed","title":null,"fields":null}, `doc show` -> 1 store.unparseable ;
  with a late `# …` line appended -> title "A late heading in the consequences prose", fields null ;
  with all four headings and a fenced `# …` line -> `doc show` 0, {"title":null,"fields":{…,"status":"proposed"}}
$ jigc start --workflow report-jigc-feedback "another feedback row" ; jigc doc create jigc-feedback --title "Row staged at create" --task another-feedback-row   -> 0 ; 0
$ jigc doc list jigc-feedback --task another-feedback-row --format json   -> 0   one `managed` row, title the H1, `fields` a 6-leaf map incl. "status":"open"
$ jigc doc rename jigc-feedback:row-staged-at-create --to "Row retitled in task" --task another-feedback-row   -> 0   (X12's control; not claimed as a door)
$ jigc doc list jigc-feedback --task another-feedback-row --format json   -> 0   ["jigc-feedback:row-retitled-in-task","Row retitled in task"]
$ jigc doc show jigc-feedback:row-retitled-in-task --task another-feedback-row --format json   -> 0   title "Row retitled in task"

agreement sweep (C31's method, the reconciler's six rigs): `doc list --format json`, then `doc show <id> --format json` per row with an id,
  comparing {title, fields, item-count}; repeated with `--task <id>` for every task `jigc task list --format json` names.
  -> 28 AGREE · 0 DIFFER · 5 rows whose `doc show` blocks (3 unregistered, 2 managed-unparseable), each `fields: null`

`--help`:
$ jigc doc show --help   -> 0   "a whole-doc object keyed by `type`, `slug`, `title`, `item-count`, `schema-version`, `fields`, `sections` — a staged serve adds the one `staged` key carrying the task id"
$ jigc doc list --help   -> 0   "the pinned shape `{"docs":[{id, path, state, item-count, title, fields}]}` … `title` is the doc's `# H1` or null, and `fields` its header fields as `doc show` serves them on a managed row that parses, else null"
$ jigc doc schema --help -> 0   names `contract-version: 7`
```

### Block R-D — the default projection, shipped doctypes (the neighbour of `(5, DEFECT 4)`)

```
rig: fresh.   IF = docs/inconsistencies/retry-budget-differs-between-readme.md   FF = docs/jigc-feedback/staged-note-names-the-wrong.md
$ command grep -c '^status:' IF FF -> 1, 1 ;  '^about:' FF -> 1          (before-control)
$ sed -i '' '/^status: /d' IF FF ; sed -i '' '/^about: /d' FF
$ command grep -c '^status:' IF FF -> 0, 0 ;  '^about:' FF -> 0 ;  '^kind:' IF -> 1 (control)
$ git -C "$REPO" add IF FF ; git -C "$REPO" commit -q -m "chore: hand-delete status (out-of-band)"   -> 0 ;  git status --porcelain -> empty
$ jigc doc show <inconsistency> --format json   -> 0   fields {"date":"2026-10-03","kind":"code-doc","schema-version":"1","status":"open"}
$ jigc doc show <jigc-feedback> --format json   -> 0   fields {…,"kind":"bug","schema-version":"1","status":"open"}      (no `about`: absent, no default)
$ jigc doc list --format json ; jigc doc list jigc-feedback --format json   -> 0 ; 0   `fields.status == "open"` on both rows, no `about`
$ jigc doc show '<inconsistency>#meta' --format json          -> 0   {"date":…,"kind":"code-doc","schema-version":"1"}      (the slice omits it)
$ jigc doc show '<inconsistency>#meta/status' --format json   -> 1   key {store.no-such-leaf, …#meta/status}   message "… names no leaf `status` in section `meta`"
$ jigc doc show '<jigc-feedback>#meta/status' ; '…#meta/about' --format json   -> 1 ; 1   store.no-such-leaf
$ jigc doc show <inconsistency> ; '…#meta'                    -> 0 ; 0   the doc as stored: no `status:` line
$ shasum IF FF before == after ;  digest of (git status + HEAD + every file under .jigc/) before == after ;  git status --porcelain -> empty
staged (copies made by the binary: `set-slot '…#resolution' --task <triage task>` -> copied_in: true; `command grep -c '^status:'` on each staged copy -> 0, `'^kind:'` -> 1):
$ jigc doc show <inconsistency> --task triage-the-retry-inconsistency --format json   -> 0   fields.status "open", staged "<task>"
$ jigc doc list inconsistency --task … ; jigc doc list --task …                        -> 0 ; 0   "open"
$ jigc doc show '…#meta' --task … --format json   -> 0  omits `status` ;  '…#meta/status' --task …   -> 1  store.no-such-leaf
$ staged-copy shasum before == after
$ jigc doc set-field '…#meta/status' --value intended --task …   -> 0 ;  staged reads -> "intended" ;  committed read -> "open"
$ jigc doc show <jigc-feedback> --task triage-the-staged-note-feedback --format json ; doc list jigc-feedback --task …   -> 0 ; 0   "open"

rig: fresh (second).   AF = docs/decisions/single-node-cache.md
$ command grep -c '^status:' AF -> 1 ; sed -i '' '/^status: /d' AF ; -> 0 ; '^date:' -> 1 (control) ; plain commit -> 0
$ jigc doc show adr:single-node-cache --format json   -> 0   fields {"date":"2026-10-03","schema-version":"2","status":"proposed"}
$ jigc doc list --format json ; jigc doc list adr --format json   -> 0 ; 0   "proposed"
$ jigc doc show 'adr:single-node-cache#status' --format json   -> 0   {"date":…,"schema-version":"2"}
$ jigc doc show 'adr:single-node-cache#status/status' ; '…#status/supersedes' --format json   -> 1 ; 1   store.no-such-leaf
$ staged copy (set-slot '…#options' --task revisit-the-cache-decision, copied_in: true; staged `^status:` count 0): show --task / list adr --task -> "proposed"; bytes untouched
$ set-field '…#status/status' --value accepted --task …  -> 0 ;  staged "accepted", committed "proposed"
$ a present-but-empty `status:` (hand edit, plain commit): show -> "" ; list adr -> "" ; '#status/status' -> 0, ""        (a stored value is served as stored)
$ `status: bogus` (hand edit, plain commit): show -> "bogus" ; list adr -> "bogus"
```

### Block R-G — the read is a read

```
rig: refs-post-hoc, T = ground-the-vision-in-research.   `.jigc/index/` holds only `edges.json.lock` as built;
`jigc task validate T` (exit 3) materializes `edges.json` (73 B).
$ shasum -a 256 < .jigc/state/file-state.json -> b_fs ;  < .jigc/index/edges.json -> b_ed ;  git status --porcelain | shasum -> b_gs ;
  digest of (git status + HEAD + every file under .jigc/) -> B
$ the driver's 16 argv × {agent, json, human} = 48 invocations over the three doors   -> 36 exit 0 · 12 exit 1
$ b_fs == a_fs · b_ed == a_ed · b_gs == a_gs · B == A      -> all four identical
control: $ jigc doc set-slot 'vision:vision#thesis' --from-file - --task T   -> 0 ;  digest != B
also digested, each identical before/after: the `fresh` projection batch (R-D, 11 reads) and the `committed-singletons` batch (R-H, 22 rejects + 13 served reads).
```

### Block R-J — the staged-elsewhere note (X11)

```
rig: fresh, task retry-doc-and open (stages an inconsistency + its commit doc):
$ jigc doc list --format json                 -> 0  {"docs":[]}  stderr: note: docs are also staged in open task retry-doc-and — … `jigc doc list --task retry-doc-and`
$ jigc doc list inconsistency --format json   -> 0  stderr: note: `inconsistency` docs are also staged in open task retry-doc-and — … `jigc doc list inconsistency --task retry-doc-and`
$ jigc doc list adr --format json             -> 0  {"docs":[]}  stderr 0 B
$ jigc doc list commit --format json          -> 0  stderr: the typed note for `commit`
rig: fresh, task doc-list-note-names open (stages a jigc-feedback only), the inconsistency committed:
$ jigc doc list inconsistency --format json   -> 0  1 row   stderr 0 B
$ jigc doc list jigc-feedback --format json   -> 0  0 rows  stderr: the typed note
$ jigc doc list adr --format json             -> 0  0 rows  stderr 0 B
$ jigc doc list --format json                 -> 0  1 row   stderr: the untyped note
$ jigc doc list jigc-feedback                 -> 0  stdout "jigc doc list — no committed `jigc-feedback` docs" ; the note on stderr
$ jigc doc list adr                           -> 0  stdout the empty-set line ; stderr 0 B
rig: fresh (second), tasks revisit-the-cache-decision (adr + commit) and second-open-task (inconsistency + commit):
$ jigc doc list adr            -> 0  note names only revisit-the-cache-decision
$ jigc doc list inconsistency  -> 0  note names only second-open-task
$ jigc doc list jigc-feedback  -> 0  stderr 0 B
$ jigc doc list                -> 0  plural: "open tasks revisit-the-cache-decision, second-open-task — … `jigc doc list --task <task-id>`"
$ jigc doc list commit         -> 0  plural, typed
rig: refs-post-hoc:
$ jigc doc list --format json                -> 0  the untyped note naming the rig's task
$ jigc doc show vision:vision --format json  -> 0  the 7 keys ; stderr: note: `vision:vision` is also staged in open task … `jigc doc show vision:vision --task …`
```

---

## R.6 · Findings of this row after reconciliation

| key | origin | status | tier | door |
|---|---|---|---|---|
| `(R7, D-1)` — the triage steps say `doc list <type>` lists `status`; it lists the registration `state` | driver | CONFIRMED | 3 | `doc list` (the sentence is emitted by `start` composing `triage-inconsistency` / `triage-jigc-feedback`) |
| `(R7, D-2)` — on `--task`, three fragment-miss routes say *committed* | driver | CONFIRMED | 3 | `doc show` |
| `(R7, D-3)` — a `doc list` row's `id` is an address `doc show` refuses | driver | CONFIRMED | 3 | `doc list` |
| `(R7, C-1)` — a body-field `default:` is projected into `fields`; `findings-channel.md` and `doc list --help` say *header* fields | codex | CONFIRMED | 3 (weak — see R-X1) | `doc show`, `doc list` |

**Tier 1: 0. Tier 2: 0. Tier 3: 4** (3 driver, 1 codex). **Refuted: none** — no claim of either
party was contradicted by a drive.

## R.7 · Open leads carried out of this row

1. **`lead(codex, X6)`** — `(5, DEFECT 2)`, `(5, C1)`, `(5, D1)` unchanged. Outside the row's doors; not driven.
2. **`lead(codex, X7)`** — `(5, DEFECT 1 · rc.20)` = `(2, A2-2)` and `(5, DEFECT 2 · rc.20)` unchanged. Outside the row's doors; not driven.
3. **`lead(codex, X12, the BOM arm)`** — the H1 reader skips a BOM as the write primitive does. No BOM-prefixed fixture was driven by either party.
4. **`lead(codex, X14, hash movement)`** — no methodology `schema-hash` other than the two new doctypes' moved since rc.20. No surface of the release binary prints a hash and no rc.20 binary is installed.
5. **`lead(codex, X16, the committing-door half)`** — pinned JSON of the committing doors exposes commit subjects, never full messages. Row 10's subject; not driven here.
6. **`(5, C-13)`** and **`(5, C-18)`** — unchanged reasons (a doc-comment no invocation emits; four `#[test]` targets on the debug build).
7. **`(R7, D-2)` and `(R7, D-3)` provenance** — whether either predates rc.20 is not established; no older binary is installed.
8. **The driver's own not-driven list (§5 above) stands**, minus nothing: a staged copy that does not parse; `orphaned` by doctype removal (refused by the pack-load ref-target fence, as the driver recorded); a managed row at a prior home; a BOM-prefixed file; the whole-doc serve on eight doctypes with no instance; `--format human` on the slice and reject arms; four non-root cwds; a project-layer schema shadow.

---

## Doors covered (reconciled)

Every clap leaf (`VERB_KINDS` spelling) that is the door of ≥ 1 driven row **with a repro block**,
after the demotion of G1 (which removes no door):

- `doc show`
- `doc list`
- `doc schema`
- `milestone create`
- `task amend`

Five leaves — the driver's list, unchanged. `start`, `doc create`, `doc set-field`, `doc set-slot`,
`doc add-item`, `doc rename`, `task finalize`, `task list`, `task validate`, `validate` and
`ingest` were run by one party or both as fixture construction or as a control only; no verdict is
recorded against them and they are **not** claimed.
