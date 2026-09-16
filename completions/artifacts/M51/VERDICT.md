# M51 — the evidence-check wave (the unadjudicated-axis wave) · VERDICT

**Status: complete.** Built + audited — code review **1 HIGH · 1 MEDIUM · 3 LOW**, e2e **19 of 22
scenarios** green through the real binary (its three reds: two confirmed defects and one LOW) —
**five audit findings, all confirmed live before any fix and all fixed axis-complete**, re-verified
**3547 passed / 0 failed**, clippy + fmt clean. **`1.0.0-rc.15` is built and installed after the
fixes, not before** — owed to the act that follows this record, not claimed by it.

Planning: [charter](charter.md) · [settle-record](settle-record.md) ·
[baseline-ledger](baseline-ledger.md) · [gap-findings](gap-findings.md) ·
[envelope-key census](envelope-key-census.md) · [acceptance design](acceptance-design.md) ·
[planning-gate-record](planning-gate-record.md) · [gate-halt discharges](gate-halt-discharges.md).
Decomposition: [roadmap](../../../implementation/roadmap.md) → Milestone 51.
Acceptance: [worked-examples](../../../design/worked-examples.md) → flow 52.

---

## What the wave claimed, and whether it is true of what shipped

> **No caller-supplied token and no repository posture reaches a door that destroys, commits or
> moves without that door having adjudicated it — the two exemptions the M50 registries carried
> (`ArgToken::Plain`'s path family; HEAD posture at `COMMITTING_DOORS`) are closed as classes — and
> every surface 1.0.0 pins says what the binary does, so that the pin closes over declared keys,
> true counts and a stated evolution rule.**

**It holds, and one clause of it is *why* the audit's largest class exists.** The e2e half drove the
claim's own subjects and found them working: the path-argument registry refuses all eight chartered
escape shapes at `migrate` and at the destructive sink; the posture family closes over all **12**
acting doors × **3** driven postures with the commit/move partition intact and `setup`'s unborn
exemption real; all **25** `WORK_UNIT_ID_DOORS` rows answer an unknown id with the findings
envelope; the four pre-pin envelope-key deletes are off the wire and the declared adds are on it;
and a 3-worktree fan-out is order-invariant across three deliberately divergent execution orders to
a byte-identical committed index (`git ls-files -s` sha `32470dee…`).

**The correction the audit forced is inside the claim's second half, not its first.** *"Every surface
1.0.0 pins says what the binary does"* is the clause the wave's own new `schema-conformance
.orphaned-instance` violated: it called an ordinary team document's front-matter key *"a jigc
schema-version stamp"*, which is a law-1 overclaim about bytes jigc did not write, and flipped
`jigc validate` to exit 1 over it. The fix does not narrow the claim; it makes the surface true.

**Two symmetric honest bounds rode the claim into the build and are untouched by the audit:** the
`Plain` family ships as a **classified registry with one stated rule — or one stated no-rule-and-why
— per member**, not one shared predicate; and the posture family closes over **three of four** driven
members, with the `GIT_DIR` redirect declared out with its rationale and its reopening condition.

---

## The audit — five findings, five fixed, nothing deferred that is not triggered

**Every one was a larger class than the finding reported.** That is now **five consecutive waves**
(M46, M48, M49, M50, M51) and it is recorded here as a property of the instrument, not a
coincidence: an auditor finds a defect by *hitting* it — one repro, one site — and bounding the
class is a different act, performed by whoever drives the axis afterwards.

| # | severity | reported as | what it actually was | commit |
|---|---|---|---|---|
| F1 | HIGH | one false-positive predicate | an **unbounded subject**: every committed `.md` in the repository, over an un-namespaced stamp key — closed by bounding the *home* | `b5ccd818` |
| F2 | MEDIUM | two paths the guard cannot see | the guard's **shape**: a `before ∩ after` conjunction, structurally blind — class **ten install paths, not eleven** (`.claude/settings.json` is one path with three writers) | `0fc80bab` |
| F3 | LOW + **both** e2e defects | 3 route sites | **22 producers over four carriers**, plus a tenth axis cell and two new fences | `8a42fbbd` |
| F4 | LOW | one roadmap overclaim | confirmed, bracketed at **three** homes; no completeness fence built, and said so | `dc508994` |
| F5 | LOW | two roadmap sentences | **three** roadmap sentences, plus a third LOW **made true in the binary** instead | `dc508994` |

### F1 (HIGH) — the orphan sweep speaks for jigc's homes, not the repository

**Reported evidence, verbatim from the code review** (driven on `dev/jigc-rig committed-singletons`
with the built debug binary):

```
printf -- '---\ntitle: API notes\nschema-version: 3\n---\n\n# API notes\n\nOur own doc convention.\n' > api-notes.md
git add api-notes.md && git commit -m 'team notes'
jigc validate   ->  exit 1

blocking · schema-conformance.orphaned-instance — committed doc `api-notes.md` carries a jigc
  schema-version stamp but sits at no resolved doctype's home ...
  route: restore what claims it — re-add the pack that defines its type, or move it to that
  doctype's home — or take it out of jigc's world: `jigc unmanage api-notes.md`, then delete
  the file or its `schema-version:` stamp ...
a stamped committed doc is claimed by no resolved doctype ... exits non-zero
```

The same repo is **exit 0** without that file. Root cause, read in the source: `orphaned_instances`
(`orphan.rs:305`) took its subject from `committed_markdown` (`orphan.rs:143` — `git ls-files -z`
filtered to `.md`, i.e. **every** committed markdown file in the repository), subtracted only the
resolved doctypes' `committed_instances` and the strand set, and kept everything `carries_stamp`
(`orphan.rs:330`) answered true for — a bare test for a `schema-version:` key in a `---`-fenced
preamble. **Nothing in that predicate is jigc-specific.**

**Three of the wave's own statements were falsified by it**, and the finding named all three: the
producer's doc-comment (*"A file jigc never stamped … is not this condition's subject and is never
named by it"*), Increment 8's declared reachability bound (*"an orphaned instance needs
`JIGC_PACK_DIR` or a PB-1 project pack"* — driven, it needs neither), and the message's own
*"a jigc schema-version stamp"*.

**How the fixer derived the class.** Four readers in the codebase ask *"is this jigc's stamp?"*.
Three — `classify_provenance`, the fifth store family, `file_state`'s baseline arm — take a
`&Schema` and are therefore **bounded to a doctype's home by construction**. The fourth, this
sweep, was unbounded; it is the whole class, and it is the defect. Driven at HEAD, `api-notes.md`,
`notes/deep/api.md` and `docs/gone.md` all fired.

**The fix.** Since the stamp cannot discriminate, the **home** does: the new `orphan::Territory`
bounds the subject to the trees an adopter's own configuration hands jigc — the resolved `docs-root`
tree ∪ a non-repo-root `placement-root` ∪ every resolved doctype's home directory, each read
**through the cascade** (never a raw `schema.location`, placement branch handled), at **both**
consumers. The finding stays **blocking** and stays a `STORE_EXIT_FLIPS` member; only what it may
speak about narrows. Red first: an out-of-territory arm in `orphaned_instance.rs` (repo root + an
arbitrary nested dir), a territory-derivation unit test, and a cell pinning this repo's own
committed stamped fixture silent. `doc_list.rs`'s orphan arm had planted its fixture **inside** the
false-positive population and was moved.

**The human's adjudication, against an independent robust-advocate (2026-09-16).** The options and
the refutation are in this session's record. The advocate's cheaper alternative — key the
discriminator on **jigc's own file-state baseline** rather than on the home — was **REFUTED on a
datum**: `.jigc/state/` is gitignored and is not rebuilt on clone, so a fresh clone would see zero
baseline and every managed doc would read as un-stamped-by-jigc. The home is the only discriminator
available today that survives a clone.

**Two residuals, written rather than hidden** (producer doc-comment + `design/validation.md`'s
inventory row): a foreign stamped `.md` **inside** the docs home still fires — that is the directory
the adopter handed jigc, and the route names `jigc unmanage <path>` — and an orphan at a
**root-level placement home** (`VISION.md`) goes unflagged at exit 0, that cell's pre-M51 status
quo, since a repo-root home declares no directory that is not the whole repository. Both are
asserted in the suites rather than dropped.

**The real fix is deferred with a trigger, not waved off.** A **namespaced stamp key** (jigc already
namespaces in the one artifact it owns outright — `SKILL.md`'s `jigc-version:` /
`jigc-body-blake3:`) would let the subject widen back to the whole repository with zero false
positives and both residuals closed. It is not built here because M51's hard bound is *zero
schema-hash movement, zero `schema-version` bumps, zero corpus migrations*, and a stamp-key change
is all three at once. *Trigger:* **the next `schema-version` bump of any doctype**
([decisions-pending.md](../../../implementation/decisions-pending.md) → *Deferred at the M51
completion audit*).

### F2 (MEDIUM) — the dirty-install guard asks once, before the first write

**Reported evidence, verbatim** (driven on `dev/jigc-rig bare` after a clean `jigc setup`):

```
printf '\n## TEAM RULES\n\nNEVER deploy on Friday.\n' >> .jigc/AGENT.md
git status --short          ->   M .jigc/AGENT.md
jigc setup                  ->  exit 0
git status --short          ->  (empty)
grep -c 'NEVER deploy on Friday' .jigc/AGENT.md   ->  0
git log --all -S 'NEVER deploy on Friday'         ->  (empty)
```

The bytes existed in no git object and are gone; `git status` reads clean, so nothing prompts
recovery — **the exact signature Increment 3 was chartered on, one path over.** A second member,
same shape: `printf '# my note\n' >> .jigc/config/packs.yaml; jigc setup` — exit 0, status empty,
the comment gone through the parse-mutate-serialize round-trip. Positive controls fired green in the
same repo: an edit to `CLAUDE.md` and to `.claude/settings.json` each raised
`setup.dirty-install-path` at exit 1 with the bytes intact.

**Mechanism.** `InstallSubject::probe` took the pre-write dirty set and `commit_install`
(`setup.rs:2014-2023`) kept only paths present in **both** `before` and `after`.
`adapter::write_bootstrap_file` (`adapter.rs:587-593`) is an unconditional `fs::write`, so after the
install `.jigc/AGENT.md` equals `HEAD` again, `after` does not contain it, and the conjunction is
false. The refusal's own sentence — *"every path listed above still has its pre-run bytes there"* —
would have been false for such a path had it ever been listed.

**The design doc's enumeration of the carve-out was itself wrong.** `design/assistant-adapter.md:52`,
rewritten in this wave, said the whole-rewrite artifacts *"(`.jigc/AGENT.md`, `.jigc/version`, the
guide)"* do not survive. Driven, that is **false for the guide** — M48's refuse-to-clobber holds,
`adapter-guide.user-modified` fires and the edit survives — and the enumeration **omitted**
`.jigc/config/packs.yaml`. The sentence was wrong about one of the three artifacts it named, and the
one artifact that *is* guarded is the one it cited as unguarded.

**How the fixer derived the class.** `install_tracked_paths` (`setup.rs:1491-1524`) yields 8 fixed
plus 3 conditional **occurrences**; the fixer corrected that to **ten paths, not eleven** —
`.claude/settings.json` is one path with three writers. Two of the ten silently lost authorable
bytes; both driven.

**The fix — shape (a), the Settle's.** The door now asks the **pre-write predicate alone** and acts
on it **before the first write**, so the refusal installs nothing and its own sentence is true at a
regenerated path as much as at a merged one. The commit-time ask survives as the backstop for the
hook, the one pathspec member a pre-write enumeration cannot name. **The whole-rewrite carve-out is
withdrawn and the default inverted**: an install path refuses unless an oracle can read the file and
say the bytes are jigc's own (the guide's recorded body digest, `.jigc/version`'s one-line shape) —
so forgetting a new install path now costs a **loud false alarm `--force` clears, never a loss** —
fenced by `InstallPathDisposition` against `install_tracked_paths` itself, so a twelfth path reddens
until dispositioned. **`--force` consents, it does not preserve**: the install runs over those bytes
and the new advisory **`setup.forced-install-path`** names every path the consent was spent on.

Two test fixtures had encoded the falsified rule and were corrected rather than kept:
`flow52_acceptance.rs` arm 3 drove the carve-out and asserted exit 0 over destroyed bytes, and
`gitignore_amend_union`'s setup arms planted an uncommitted `.jigc/.gitignore`. Docs corrected with
the datum: `assistant-adapter.md:52`, `validation.md`, `surface-contract.md`, `finalize.md`,
`QUICKSTART.md`, `MIGRATING.md`, the worked-examples transcript, and the roadmap (dated bracket).

**Stated, not done:** `--force` **does not probe the hook path** — recorded in
`install_candidate_paths`' doc-comment.

*Guide-byte note:* the adopter-facing bytes therefore move **once more** relative to Increment 9,
which is still **one** installed-guide hash move relative to rc.14.

### F3 (LOW, and both e2e-confirmed defects) — a route's command span is bytes that run

The code review reported this as one `Route::mechanical` site in `ingest`; the e2e half independently
confirmed **two** more, one of them a **release-binary dead end** and one a **debug-binary panic**.
All three are one class, and the fix took the class.

**e2e defect 1 — the route dead-ends in the RELEASE binary for an ordinary spaced filename:**

```
rig=$(dev/jigc-rig fresh --binary $PWD/target/release/jigc) || exit; eval "$rig"
printf '# doc\n' > 'my notes.md'
$JIGC migrate 'my notes.md' --as adr
# exit 1:
#   blocking · migrate.source-untracked — `my notes.md` is in neither this repository's index nor its HEAD ...
#     route: stage it with `git add -- my notes.md`, then re-run `jigc migrate my notes.md --as adr`
# Following that route VERBATIM:
$ git add -- my notes.md              -> fatal: pathspec 'my' did not match any files   (exit 128)
$ $JIGC migrate my notes.md --as adr  -> error: unexpected argument 'notes.md' found    (exit 2)
$ git status --short                  -> ?? "my notes.md"      (state unchanged)
```

**e2e defect 2 — `task finalize <migration-task> --approve` panics the DEBUG binary at exit 101:**

```
thread 'main' panicked at crates/engine/src/finding.rs:741:13:
a `Route::mechanical` argv must parse against the real CLI: token `my notes.md` is not shell-safe
as emitted — a route's text is `argv.join(" ")`, i.e. bytes an agent pastes into a shell, so an
author-owned prose token (a title, an intent) must be rendered through `crate::task::shell_token`
— argv ["jigc", "unmanage", "my notes.md"] (design/surface-contract.md → The route fence)
```

Site: `crates/engine/src/file_state.rs:356` — `ConflictBlock::task` eagerly constructs that route for
**every** migration task, so the fence fires on construction whether or not the conflict renders.
Release posture **measured, not assumed**: the panic is `#[cfg(debug_assertions)]`, so release exits
0 and lands the migration — and would emit the same unsafe bytes if the conflict arm rendered.
Control (passes, so the gap is the path family specifically): `jigc start` with an intent carrying
`$dollars`, a `;` and embedded quotes, and `jigc doc create adr --title 'A $title; touch PWNED'`,
both exit 0 with no `PWNED` file — prose tokens already went through `shell_token`.

**The counter-example was inside the same wave and the same door:** Increment 9's new
`ingest.unaddressable-identity` *does* quote — `route: rename it to a doc id — `git mv 'docs/decisions/My
Notes.md' docs/decisions/my-notes.md`` — while the non-conformant arm one branch over did not.

**How the fixer derived the class.** Not the three sites: **an emitted command line whose operand is
a path, a git ref or author prose, rendered raw** — a `Route::human` naming a command in prose, a
`Mechanical` route's argv, its prose tail, and the non-`Finding` remedy spans (`pack.rs`'s
`rm <shadow>` precedent). **22 producers converted across both crates**; the ones grammar already
governs (ids, addresses, short shas) excluded with reasons, and the `git <args>` failure diagnostics
excluded as the *"quoting an invocation"* case `surface-contract.md` already disposes.

**The fix, two fences because one alone is blind.** The quoting rule moved to
`engine::finding::shell_token` — it had lived in `cli::task` while the **engine** mints routes of its
own, so it was enforced in exactly the half of the codebase that did not need it. Then: a **span**
fence on all three `Route` constructors, over the **composed text**, so a `Human` route and a prose
tail are checked and not just an argv; and a **subject** fence at `Finding::graded`, because an
unquoted path with a space is not one bad token but **two inert ones** — the token check passes and
the command exits 128 — with the word boundary taken from the finding's own located address.
`shell_operand` is the already-inert sibling for addresses and paths, because `shell_token` quotes a
`#` that `shell_safe` reads bare; **the asymmetry is stated and pinned** rather than left to be
rediscovered.

**The axis that could not see this** was `path_arg_occurrence_axis.rs`, whose nine escape cells were
all shell-safe **by accident of spelling**. It gains a tenth, `ShellUnsafeName`, and now asserts of
**every** cell that the command spans a door prints can be run.

**Two brief corrections the fixer made against its own brief:** `git switch <branch>` is **not**
grammar-safe and was converted, and the **milestone record path resolves through user-settable
knobs**, so it is not the constant the brief assumed.

**Declared bound, stated and not half-taken:** a **control byte** in a path is a *rendering* question,
not a route-safety one — `'two\nlines.md'` is correct bytes that wrap on screen, and escaping it in
the route would name a **different file**. Recorded in `design/surface-contract.md`.

### F4 (LOW) — the ambush owe-set is not derived, and the roadmap said it was

The roadmap's Increment 8 Deliverable read *"A new blocking ambush-class contract **cannot stay
silently off a hand-list**"* and its Grouped scope *"`AMBUSH_CLASS_CODES` becomes **derived**"*. What
ships is a hand-written table, `pack.rs:823 AMBUSH_CONTRACTS`, filtered by `ambush_class_codes()`.
**The source itself already said so** (`pack.rs:815-823`): *"The source set is the rule each row must
satisfy, not a producer scan — and that is a bound, stated rather than discovered later."*

The reviewer verified there is **no completeness fence in the other direction**: the only consumers
outside `pack.rs` are `stated_at_fence.rs:1251` (each row's named site really mints its code) and
`flow52_acceptance.rs:2256/:2327`. Nothing scans production for a blocking code minted at a
`CommitsOnBehalf` door and asserts membership.

**Fixed as a record repair, bracketed at three homes** — the roadmap (with the doc-comment as the
datum), `worked-examples.md`'s flow-52 arm 8 (both the arm-kind line and the numbered claim), and
`validation.md:733`. **Not done, and named as such:** no completeness fence is built — a membership
scan over production is new scope, named in the bracket as a **retrospective candidate** rather than
smuggled into a record repair. `cli.rs:1831` and `flow52_acceptance.rs:2232` still say *"derivation"*
in a non-completeness sense and were left alone.

### F5 (LOW) — the `orphaned` row's `id` is `null`, and one LOW was made true in the binary

The roadmap said twice that the orphan row's `id` is *"read from the stamp"*. The shipped code emits
`id: None` (`doc.rs:4298-4303`, and the same arm at `cli.rs:1364`), `design/doc-read-surface.md`
already carried the correction with its datum (*a jigc stamp is `---\nschema-version: <n>\n---` and
**names no type***), and [settle-record](settle-record.md) §20 fork 1 decided it. **Swept rather than
taken on report:** the fixer found **three** roadmap sentences, not the two reported — Increment 5's
Grouped scope, Increment 8's Grouped scope, and Increment 8's *Proves* bullet — each struck with the
datum.

**The third LOW was made true in the binary instead of explained away.** `migrate`'s read fault
composed `repo_root.join(path).display()`, so an operator who typed `adir` was answered
`/private/var/.../repo/adir` — a host path on a surface law 1 binds, unpasteable into the route
printed beside it, and flatly contradicted by the source comment two lines above. The e2e half found
the same thing from the other end: `UNSWEPT_PRODUCERS`' exempting reason for `migrate.rs` (*"quoted
back as given"*) was **falsified by the binary**, and M51 Increment 1's own new comment repeated it.

Printing the token as given is the smaller diff and the only one that keeps law 1 honest: the comment
then describes the code, and the fence row becomes true **by deletion**. With its one production
`.display()` gone, `migrate.rs` **leaves `UNSWEPT_PRODUCERS` and joins `GUARDED_SRC`** — the fence's
own prescribed move — so the next absolute render there reddens rather than ships. Driven after:
``could not read the foreign `adr` source at `adir` ``.

---

## The e2e half — 22 scenarios, 19 green

| # | scenario | verdict |
|---|---|---|
| 1 | Inc 1 — the path-argument registry refuses every chartered escape shape at `jigc migrate` | PASS |
| 2 | Inc 1 — untracked vs tracked source, and the exit-4 hold names the file `--approve` deletes | PASS |
| 3 | Inc 1 — the destructive sink re-validates a rewritten `source-path` and rolls the promote back | PASS |
| 4 | Inc 1 — the registry's other occurrences behave per their stated rule or stated no-rule | PASS |
| 5 | Inc 1 — `migrate.source-untracked`'s route dead-ends on shell-significant bytes (release) | **CONFIRMED DEFECT** → `8a42fbbd` |
| 6 | Inc 1 — `task finalize <migration-task> --approve` panics the debug binary at exit 101 | **CONFIRMED DEFECT** → `8a42fbbd` |
| 7 | Inc 2 — the posture family × the commit-on-behalf class: 12 acting doors × 3 driven postures | PASS |
| 8 | Inc 2 — `jigc setup` is exempt from the unborn member; a fan-out worktree is typed, not sniffed | PASS |
| 9 | Inc 3 — `setup` refuses its install commit over bytes it did not write; five controls clean | PASS |
| 10 | Inc 4 — `gitignore::ensure` is amend-to-union and byte-idempotent across its callers | PASS |
| 11 | Inc 4 — the config-layer CAS pre-image: restore on a rejected finalize, `finalize.rollback-conflict` on the raced cell | PASS |
| 12 | Inc 5 — the `EnvelopeArm` registry: 60 arms, four deletes off the wire, declared adds present | PASS |
| 13 | Inc 6 — every work-unit-id door answers an unknown id with the findings envelope (25 doors) | PASS |
| 14 | Inc 7 — the count fences and the key-set fences hold against the sets they name | PASS |
| 15 | Inc 8 — the two store-surface causes and the derived ambush owe-set | PASS |
| 16 | Inc 9 — the law-1 surface batch drives clean | PASS |
| 17 | Inc 9 — `UNSWEPT_PRODUCERS`' stated reason for `migrate.rs` is falsified by the binary | **LOW** → `dc508994` |
| 18 | Inc 11 / F-9 — `commit-recording.stale-title` produced at `doc rename --task` and re-raised | PASS |
| 19 | Fan-out determinism — three divergent execution orders, byte-identical committed output | PASS |
| 20 | Adapter spawn template rendered through the binary and its command executed verbatim | PASS |
| 21 | Misuse probes off the acceptance path | PASS |
| 22 | Full gate at HEAD `befdbf93` — `passed=3532 failed=0`, GATE: PASS | PASS |

Scenario 19's assertion is the strong one: three deliberately divergent execution orders over a
3-worktree fan-out yield a **byte-identical committed index** (`git ls-files -s` sha `32470dee…`) and
an identical join listing **by task id**, the only cross-run delta being the repo-identity `base:`
sha. Scenario 21 re-confirmed the M40 undeclared-nested-field class stays closed and that
`doc show 'adr:/etc/passwd'` / `'adr:../../etc/passwd'` refuse with `store.malformed-slug`.

---

## The instrument, honestly

**The build halted twice, and both halts were adjudicated rather than guessed** — each is an
instrument finding in its own right, and each is on the record rather than in a summary
([settle-record](settle-record.md) §20 at commit `d5e0508c`, §21 at `5ebafaeb`; cited, not restated).
§20's fork 1 turned on a settle sentence — *"`id` is read from the stamp"* — that **one `cat` of a
committed doc's front matter falsifies**, which is this project's signature failure again: a claim
about the composed product reached by reading the files it is composed from. §21's is the mirror
shape one layer out: **F-9 was in scope on four records and absent from the fifth** (the roadmap's
Increment 9 Grouped scope), lost in the gate-record→roadmap transcription with **nothing fencing the
two against each other** — so the close's planner halted rather than absorb an unsettled fix or
reduce the human's own gate.

**Three of the four fixers correctly narrowed or widened the finding they were handed**, which is
the *brief the fixer with the finding, never with the finding's boundary* rule working: F2 corrected
eleven occurrences to **ten paths**; F3 corrected **3 reported sites to 22 producers** and corrected
two premises in its own brief; F5 corrected **two roadmap sentences to three**. And one red test
**surfaced an unreported partial write** while it was being written.

**The e2e's own bound, flagged by the agent that ran it:** it is **headless**, so the genuine
concurrent Task-tool spawn is the orchestrator's main-session artifact it did **not** run. What it
drove is the **N-process binary sim** plus the rendered adapter spawn template **executed verbatim**
as its own process — honest about the CLI payload and the join, and **not** proof that the real
assistant primitive reaches the CLI. The M39 honest bound's live confirmation (2026-07-22) is what
that leans on; this wave adds nothing to it.

**One instrument cost is measured and owed as work, not as prose.** A full `dev/gate` run during
this wave was taking **~20 minutes**; `target/debug/deps` held **1.8 M** split-debuginfo `.rcgu.o`
files, and deleting them mid-run took the same full gate to **~9.5 minutes** (measured by the
orchestrator, 2026-09-14). The durable fix — `split-debuginfo = "packed"` in the workspace profile,
a `cargo clean`, and an after-milestone cleanup step in the build loop — is **owed post-wave** and
minted with a trigger in [decisions-pending.md](../../../implementation/decisions-pending.md) →
*Deferred at the M51 completion audit*, because changing a workspace profile inside a close
increment is a full-recompile change against a gate this record has to quote.

---

## Declared bounds, carried in writing

1. **A control byte in a path is not escaped in a route.** `'two\nlines.md'` is correct bytes that
   wrap on screen; escaping them would name a different file. Stated in
   [surface-contract.md](../../../design/surface-contract.md), not half-taken (F3).
2. **No completeness fence guards the ambush owe-set.** `AMBUSH_CONTRACTS` stays a hand-written
   table whose two checkable legs are fenced and whose completeness leg is a **stated judgment**. A
   membership scan over production is named as a retrospective candidate (F4).
3. **`jigc setup --force` does not probe the hook path**, stated in `install_candidate_paths`'
   doc-comment (F2).
4. **The orphan sweep's two residuals** — a foreign stamped `.md` *inside* the docs home still
   fires; a root-level placement orphan (`VISION.md`) is silent at exit 0 — are asserted in the
   suites, and the namespaced-key fix carries a trigger (F1).
5. **The genuine concurrent Task-tool spawn was not run by this audit.** The e2e half is headless;
   the sim and the verbatim-executed spawn template are not that proof.
6. **The wave's own build bounds stand unchanged** and are not re-stated here: zero schema-hashes,
   zero `schema-version`s and zero corpora moved; the descriptor-held traversal refusal at the path
   sink; the pre-pin window's admissibility of the 25-door wire change; and **no mechanical checker
   fences the conversion ledger or the disposition ledger** —
   [pinning.md](../../../implementation/pinning.md) §3 refuses a `pinned-by:` symbol parser by name,
   so each citation is verified by reading what its cited test asserts.

---

## What is next

**`1.0.0-rc.15` is owed as the act immediately following this record** — built and installed **after**
the fixes, not before, with the ten version-bearing goldens regenerated in the same motion and their
diff verified to carry nothing but the version string. The version-stamp confirmation M47's audit
added has caught the owed bump unshipped in five consecutive waves; it did not need to here.

Then the **per-axis review on the installed rc.15** (D15's acceptance binary — naming an acceptance
binary is not the 1.0.0 call), and then **the 1.0.0 call, which is the human's.** Also owed
post-wave: the gate-speed profile change and its cleanup step, on the trigger *before the next
milestone build*.
