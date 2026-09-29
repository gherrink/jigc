# Findings — every claim driven, with its repro block

**The rule** ([protocol.md](protocol.md) §7): every CONFIRMED, PARTIAL **and REFUTED** verdict
arrives with a repro block and a `pinned-by: <suite>::<test>` verified **by reading what the
cited test asserts**, or a stated `UNPINNED: <why>`. No mechanical checker fences this;
[pinning.md](../../../implementation/pinning.md) §3 refuses one by name.

**Status of the ledger: CLOSED** (2026-09-16, M51 Increment 11 / T3). Every verdict row carries
either a `pinned-by:` citation — written from the cited test's **own assertions**, never from its
name — or an `UNPINNED:` with a stated reason. **The human's gate on the 1.0.0 call is that this
ledger closes**, and it does. *Closed* is the criterion D14 named it for: **has a citation or a
reason**, not *has a test* ([settle-record.md](../M51/settle-record.md) → D14).

**What moved at the close, and what did not.** Four rows converted `UNPINNED:` → `pinned-by:`,
each by its **own fix's red test** shipped in this wave: **F-5** (Increment 6), **F-11**
(Increment 6 / T3), **F-9** (Increment 11 / T2) — the three D14 named — and **F-7**, which D14
did *not* name and which the row-by-row re-read found: N15 was carried at the trial and was
*fixed* by Increment 5 / T6, so its recorded reason (*"carried defect, as F-6"*) had stopped
being true. Eight rows keep a stated `UNPINNED:`, every one of them because **the pin would
encode a gap, a loss or an absence as expected output** — the rule [pinning.md](../../../implementation/pinning.md)
§5 states from the other end (*would pinning it now pin the bug?*). **F-3** keeps its reason on a
**live trigger**, not for want of a reading. **F-1's reason was corrected rather than kept**: it
named `machine_output.rs` as the pin for a surface that suite does not touch — §3's 2026-08-18
hazard reaching an `UNPINNED` reason, which is why the re-read covered reasons and not only
citations.

**The count, reported honestly** — occurrences *and* distinct rows, because a token count is not
a row count (M50's ledger was chartered at *"21 rows"* and measured **18 occurrences over 11
rows**). Measured over the **verdict rows** — this header's own prose and the check's source text
carry the token too and are not dispositions — the rows hold **10 occurrences of `UNPINNED`, of
which 8 are dispositions, one per distinct row**; the other two are prose *inside* a row (F-1's
correction note, F-3's trigger clause). Thirteen verdict rows: **5 cited · 8 `UNPINNED` with a
reason · 0 undisposed.**

*(The first draft of this paragraph said "12 occurrences" and was true of the file before the
paragraph itself was written into it. It is recorded here rather than silently corrected, because
it is M50's miscount in miniature and one iteration old: a token count over a file is not a count
of anything the ledger claims. The figure above is measured over the rows, where the claim lives:*
`awk '/^## F-/ { r = 1 } r && /UNPINNED/ { t++; if ($0 ~ /^\`pinned-by:\`/) d++ } END { print t, d }'`*.)*

**The closure check, stated and re-runnable** — it fences the *form*, never the reading ([pinning.md](../../../implementation/pinning.md)
§3 refuses a `pinned-by:` symbol parser by name, and no command can check what a test asserts):

```
awk '/^\*\*Status of the ledger: CLOSED\.?\*\*/ { closed = 1 }
     /^## F-/ { cur = $2; rows[++n] = cur; next }
     /^## /   { cur = "" }
     /^`pinned-by:`/ {
       if (cur == "") { printf "STRAY pinned-by outside a verdict row, line %d\n", NR; bad = 1; next }
       seen[cur]++
       if ($0 ~ /UNPINNED/) {
         if ($0 !~ /UNPINNED:[[:space:]]*[^[:space:]]/) { printf "%s: UNPINNED with no stated reason\n", cur; bad = 1 }
         else unpinned[cur] = 1
       } else cited[cur] = 1
     }
     END {
       for (i = 1; i <= n; i++) {
         r = rows[i]
         if (seen[r] == 0) { printf "%s: carries NEITHER a citation nor an UNPINNED reason\n", r; bad = 1 }
         else if (seen[r] > 1) { printf "%s: %d pinned-by lines; a row carries exactly one\n", r, seen[r]; bad = 1 }
         if (cited[r]) c++; if (unpinned[r]) u++
       }
       if (!closed) { print "the ledger does not declare itself CLOSED"; bad = 1 }
       printf "rows=%d cited=%d unpinned=%d undisposed=%d\n", n, c+0, u+0, n-(c+0)-(u+0)
       exit bad + 0
     }' completions/artifacts/RC-rc14/findings-verification.md
→ rows=13 cited=5 unpinned=8 undisposed=0        [exit 0]
```

The subject is a line **opening** with the disposition token, so a multi-line citation is matched
at its first line and every row's disposition opens one. The check exits non-zero on an
undisposed row, on two dispositions under one row, on an `UNPINNED` with nothing after the colon,
and on this file declaring itself closed nowhere. It cannot tell a citation that asserts the
claim from one that reads apt and asserts something else — that half was done by opening each
cited test, and there is no command for it.

Every repro ran on `jigc-gate:rc14` (sha `82075cc3`), in a throwaway container, never a live
corpus.

---

## F-1 · `task finalize --dry-run` enumerates the paths — **REFUTED**

**The claim** (B1 feedback §3): *"A dry-run or full manifest of exactly which paths finalize will
commit, ahead of running it for real. `jigc task validate` previews content/carryover/changelog
gates but not the literal file list — I only saw that enumerated in finalize's own preamble,
which only appears once you've committed to running it."*

**Driven:**

```
$ jigc task finalize <task> --dry-run
finalize --dry-run — pre-commit manifest (nothing committed)
would commit — docs: record it
  modified other.ts
  promoted docs/decisions/drop-the-oldest-sample.md
[exit 0]
```

The surface exists, it is named `--dry-run`, it says *nothing committed* in its own first line,
and it enumerates every path — including the incidental one. **The strongest form of the
refutation is inside this same trial: B2 ran `task finalize … --dry-run --format json` at its
invocation 18.** One arm used the capability the other reported missing.

**This is the discoverability lens landing again** — *the capability exists, no composed surface
the worker was on named it* — for the seventh consecutive trial.

`pinned-by:` **UNPINNED: the fact is that a capability exists and was not found. A test can pin
the manifest's content — `finalize_manifest.rs` pins the `--dry-run` envelope and
`finalize_dry_run_subject::the_dry_run_forecasts_the_subject_the_commit_lands` the `subject` key
M50 added, joined by M51 Increment 5's `dry_run_findings_equal_set.rs` over the findings the
forecast now carries. No test can pin that a worker did not go looking. The pinnable half is
fenced; the unpinnable half is the finding.**

*(Corrected at the close: this row named `machine_output.rs` as the envelope's pin. It is not —
that suite pins stream discipline, and the `subject` key is nowhere in it. The §3 addendum's
hazard — a citation that reads apt and asserts something else — reached an `UNPINNED` reason,
which is why the re-read covers reasons and not only citations.)*

---

## F-2 · Content staged **during** a task is committed, and only the manifest says so — **PARTIAL**

**The claim** (B1 feedback §2): the `record-decision` preview warns that finalize blocks on
content carried from **before** the mint, *"which led me to think finalize had some notion of
'content foreign to this task' that it would flag or gate. It doesn't … Content I staged myself
during the task's lifetime got swept into the commit with zero warning, no distinction from
task-authored content."*

**Driven** — `other.ts` modified and `git add`ed **after** the mint:

```
$ jigc task validate <task>
advisory · file-state.staged-copy — staged copy of `docs/decisions/drop-the-oldest-sample.md` …
  route: no action needed …
        ← nothing names other.ts

$ jigc task finalize <task>
finalized 6631b72 — docs: record it
  promoted docs/decisions/drop-the-oldest-sample.md
  modified other.ts
  2 files committed

$ git show --stat --oneline HEAD
6631b72 docs: record it
 docs/decisions/drop-the-oldest-sample.md | 23 +++++++++++++++++++++++
 other.ts                                 |  1 +
```

**Confirmed:** the path is committed, and no *gate* distinguishes task-authored from
incidentally-staged content. **Refuted, and this is why the verdict is PARTIAL:** it is **not
"zero warning"**. The `--dry-run` manifest names it, and the finalize ack names it and counts it
(*"modified other.ts / 2 files committed"*). What is absent is a warning *before* the ack, on a
surface the worker was already running — `task validate` is silent about it.

**Class: surface — framing.** Finalize committing the index is the design
([design/finalize.md](../../../design/finalize.md)); the finding is that the *carryover gate's
prose* creates an expectation of a foreignness notion that does not exist, and the one surface
that previews gates does not preview this. Not data loss: the worker recovered, and the bytes
were its own.

`pinned-by:` **UNPINNED: the committed behaviour is fenced (`finalize` commits the index, and the
left-out/ack lines are goldens), but the finding is about the *carryover preview's prose* setting
an expectation, which no current suite asserts against. A red test would be a new fence on step
text, i.e. the fix, not the pin.**

---

## F-3 · `status: superseded` with no `supersedes` target validates clean everywhere — **CONFIRMED**

**The claim** (B2 feedback §2): *"the ADR's slug flatly contradicted its own decision text, and
its `status: superseded` had no `supersedes` target … `jigc validate` never flagged either …
If I'd trusted the finding list as complete and just filled the two required slots, I'd have
finalized a backwards decision with a clean `jigc validate` the whole way. Worth being explicit
about since a green validate reads as 'correct,' not just 'well-formed.'"*

**Driven** — an adr with all four slots filled and `status: superseded`, nothing superseded:

```
$ jigc doc show <addr> --task <t> --format json   →  {"date":"2026-09-10","schema-version":"2","status":"superseded"}
$ jigc task validate <t>   →  only  advisory · file-state.staged-copy  (routed "no action needed")
$ jigc validate            →  no findings — the committed store validates clean
$ jigc task finalize <t>   →  exit 0
$ jigc validate            →  no findings — the committed store validates clean          [exit 0]
```

**Confirmed at every door, before and after the commit.** `superseded` is a valid enum member, so
schema-conformance says nothing, and no doctype declares a conditional ref requirement.

**Class: capability gap, with a stated shape.** This is *not* a false green about anything jigc
claims: the determinism boundary gives jigc structure and leaves prose to the LLM
([VISION.md](../../../VISION.md)), and the doc is structurally conformant. What is expressible
and undeclared is the **structural** half — *a `status` of `superseded` requires a `supersedes`
ref* is a cross-field constraint the schema model could carry and no doctype states.

**It is also plant E's own design holding.** Bar 9 asserts `task validate` says nothing about the
planted status *because* the discrepancy must be read-back-only; this repro is that bar, driven
independently. The finding is not that the plant worked — it is B2's sharper observation that a
clean `validate` **reads as** correct.

`pinned-by:` **UNPINNED: verified by reading, no suite asserts this.**
The nearest standing fence is plant E's bar 9 (`e-abandoned-task.sh:155`), which asserts the
*absence* of a finding here and is trial apparatus, not a repo suite. Pinning the presence of a
constraint would be pinning the fix. **The row stays UNPINNED *on a live trigger*, not for want
of a reading** ([decisions-pending.md](../../../implementation/decisions-pending.md) → *The
rc.14 trial's findings*; [settle-record.md](../M51/settle-record.md) → D14, which keeps it there,
and D13, which excludes it on the **necessity** leg — a cross-field constraint is a schema-*format*
change plus a `doc schema` `contract-version` 6→7).

---

## F-4 · `jigc doc rename` vs `jigc rename` — the distinction is found by reading three helps — **CONFIRMED, and it is the trial's most-corroborated finding**

**Raised independently by both interactive arms**, in their own words:

- B1: *"Two near-identically-named commands, different scope … I had to read both `--help`
  outputs side by side before I trusted which one was safe to run on a doc that was still sitting
  inside my open task, uncommitted."*
- B2: *"I ended up reading three `--help` outputs in sequence (`jigc doc create --help`,
  `jigc rename --help`, `jigc doc rename --help`) before landing on the right one — the
  distinction … isn't something the orientation output surfaces up front."*

**Corroborated by the invocation logs, not just the debriefs** — the help-reading is in the
record, in both, immediately before the rename:

```
B1  15 describe · 16 doc rename --help · 17 rename --help · 18 doc rename <addr> --to … --task …
B2   9 rename --help · 10 doc create --help · 11 doc rename --help · 12 doc rename <addr> --to … --task …
B3   7 rename --help · 8 doc --help · 9 doc rename --help · 10 doc rename …
B3-h2 9 rename --help · 10 rename <addr> …  → exit 2 · 11 doc rename …  → exit 0
```

**Four of five arms.** B3-h2 is the sharpest cell: it *ran the wrong verb first* — top-level
`jigc rename` on a staged doc, **exit 2** — and only then reached `doc rename`. The refusal
routed it correctly, which is the surface contract working; what no surface did was prevent the
wrong call.

**Class: surface/discoverability.** Every worker got there, none was harmed, and every one paid
2–3 help reads for it.

`pinned-by:` **UNPINNED: the refusals and routes on both verbs are fenced (`flow37_rename.rs`,
`anyhow_route_spans`, and M48's identity split); what is unfenced is that neither verb's
existence is surfaced where the other is used. That is a composed-surface fact, and pinning it
means minting the surface.**

---

## F-5 · The unknown-id column: 22 doors refuse correctly, with a route, and **no code** — **CONFIRMED, not a regression**

**Found by walk arm 17**, which enumerates 25 id-taking doors × {`""`, `no-such-task`}.
**86 OK / 22 FAIL, and every failure is in the `[nonexistent]` column.**

```
EMPTY id      blocking · work-unit.malformed-id — "" is not a valid work-unit id
                route: use lowercase letters, digits, and single hyphens (…)        [exit 1]   ← 53/53 green
UNKNOWN id    no task `no-such-task` — list live tasks with `jigc task list`        [exit 1]   ← NOCODE, route
```

**Driven on both binaries**, `task validate` / `task diff` / `milestone list-tasks`:

```
rc.13  exit=1 NOCODE route        rc.14  exit=1 NOCODE route
```

**Not a regression** — identical — and **declared**: the ledger says *a well-formed but unknown
id keeps its unchanged roster answer, so only the malformed cell moves.*

**Class: surface.** A driver keying on the stable `(code, target)` pair gets nothing when it
names a task that does not exist, at 22 doors. **Reversible after 1.0.0** — adding a code to a
text refusal moves no pinned contract — so it **SHIPS RECORDED**.

**Its significance is the shape, not the severity:** M50's centrepiece swept the *malformed* axis
to 53/53 and left its *unknown* sibling untouched. That is the complete-fix lens turned on M50's
own work, for the fifth consecutive wave.

`pinned-by:`
`work_unit_unknown_envelope::every_work_unit_id_door_answers_the_findings_envelope_for_an_unknown_id`
— **converted by M51 Increment 6** (`1a3c0a52` + `392532b0`), the fix's own red test, and
**verified by reading what it asserts**: it drives *every* row of `WORK_UNIT_ID_DOORS` with a
well-formed id no work unit carries and requires, per door, empty stdout at exit 1 and a stderr
document whose top-level keys are exactly what `ENVELOPE_ARMS`' `Reject::Findings` row declares
— **not** the flattened `{"error": …}`, whose code lives inside a message and projects no key —
carrying exactly one finding, whose `key` is that family's work-unit pair
(`finalize.no-task`/`task:<id>`, `milestone.unknown`/`milestone:<id>`) and whose route is
exactly one backticked command. Its closing assertion is a **set** equality against the
registry's own rows, so a door driven twice while another is never reached fails rather than
passes. The row's earlier reason — *"no arm asserts anything about the unknown-id cell"* — is
the sentence this suite falsifies; `flow51_acceptance.rs`'s `WORK_UNIT_ID_DOORS` arm still
fences the *malformed* cell beside it, and neither stands in for the other.

---

## F-6 · N27 — `jigc task diff <id>` cold-start answers almost nothing — **CONFIRMED (expected)**

Walk arm 22, four bars, expected RED and registered as such in [protocol.md](protocol.md) §0.2:

```
FAIL  N27: it names the workflow the task was minted from
FAIL  N27: it names the intent
FAIL  N27: --format json carries the workflow
FAIL  N27: --format json carries the intent
```

**This is a carried defect whose recorded trigger is this trial's plant-E arm**, with the
instruction that it be *"re-argued against its cost, not re-discovered."* See the adjudication in
[trial-record.md](trial-record.md): the duress cell moved **without** it.

`pinned-by:` **UNPINNED: a carried defect, deliberately unfixed; a standing test would pin the
current behaviour as expected output.** Same reasoning `pinning.md` applied to the leftover
re-`provision` case.

---

## F-7 · N15 — the `--task` miss still says *committed* — **CONFIRMED (expected), PARTIAL on shape**

Walk arm 22. Of three bars, **one passed**: the task-scoped miss is **no longer byte-identical**
to the task-less one. The failing bar is the substantive half — it still claims the address names
no **committed** doc, to a reader holding a staged copy.

`pinned-by:`
`staged_read_miss_arm::the_task_read_miss_names_the_staged_copy_and_keeps_the_task_in_its_route`
— **converted by M51 Increment 5 / T6** (`79d390b4`). **This row's disposition moved at the
re-read**: N15 was carried at the trial and was *fixed* in this wave, so *"carried defect, as
F-6"* stopped being true of it, and the close's obligation is to re-read every row rather than
only the three the Settle named. **Verified by reading what the test asserts**: the two reads
are `assert_ne!`-different — the defect *was* their equality — the `--task` block keeps
`store.not-found`, names the task and the copy it looked in, **never says `committed`**, and
carries `--task <id>` in its route; that route **run verbatim** returns the *staged* thesis,
which the fixture makes visible by staging prose that differs from the committed copy; and the
task-less block is asserted as a literal, because its bytes are a 1.0-pinned surface the fix
does not touch.

---

## F-8 · No structural ref from an `adr` to a `research` doc — **CONFIRMED, capability gap**

**The claim** (B2 feedback §3): *"`jigc doc schema adr` only offers `supersedes` (ref → `adr`)
and `cites-code` … There's nothing like a `cites-doc` / `informed-by` ref to another doctype …
I ended up just naming `docs/research/unbounded-series-cardinality.md` in the ADR's prose — a
soft, unenforced link instead of a structural one."*

Corroborated by the log: B2 authored the research doc (invocations 22–35) and the ADR
(36–53) and linked them only in prose. **Class: capability gap → SHIPS RECORDED.**

`pinned-by:` **UNPINNED: an absence of a schema field. `doc schema adr --format json` is
contract-pinned and would show such a field if it existed; asserting it does not exist would pin
the gap.**

---

## F-9 · `doc rename` re-slugs the doc and leaves the staged commit summary naming the old title — **CONFIRMED**

**Found by the human, reading the trial record**, from the observation that a *managed doc's
commit* in B1 was made by raw git. This is what caused that.

B1 renamed its ADR at invocation 18 and finalized at 23. The commit summary had been authored at
invocation 9, **before** the rename, and still named the old title. jigc committed it.

**Driven** — same shape, minimal:

```
$ jigc doc rename adr:drop-the-oldest-sample --to 'Shed the oldest sample on ingest overflow' --task <t>
adr:shed-the-oldest-sample (renamed to "Shed the oldest sample on ingest overflow" from adr:drop-the-oldest-sample)
        ← says nothing about the staged commit doc

$ jigc task validate <t>
advisory · file-state.staged-copy … route: no action needed        ← nothing else

$ jigc task finalize <t> --dry-run
finalize --dry-run — pre-commit manifest (nothing committed)
would commit — docs: record the decision to drop the oldest sample on overflow     ← the OLD title
  promoted docs/decisions/shed-the-oldest-sample.md                                ← the NEW path

$ jigc task finalize <t>   →  docs: record the decision to drop the oldest sample on overflow
```

**The `--dry-run` manifest is the sharpest artefact: the stale subject and the new path are on
adjacent lines and nothing connects them.**

**Why this is a finding and not "prose is the LLM's".** It is — jigc must not rewrite an authored
summary, and the determinism boundary is right. But the *re-slug* is a **structural** operation
jigc owns and performs, and [write-commands.md](../../../design/write-commands.md) states its
claim as *repointing every structured referrer in lockstep so reference-integrity survives*. The
commit doc's summary is prose, not a structured referrer, so jigc keeps its literal promise — and
the consequence is a commit message naming a document that does not exist under that name, **in
the same commit that creates it**. Noticing is cheap and is not rewriting: the rename knows both
titles and knows the task stages a commit doc.

**Class: surface.** Reversible after 1.0.0 — an advisory is additive. **SHIPS RECORDED.**

`pinned-by:`
`doc_rename_in_task::the_stale_commit_summary_is_noticed_where_it_happens_and_re_raised_until_repaired`
— **converted by M51 Increment 11 / T2** (`bcf7ad45`, standing on `46eef998`'s durable
pre-rename title; [settle-record.md](../M51/settle-record.md) → §21). **Verified by reading what
it asserts**: the `doc rename … --task --format json` ack carries the advisory in the `findings`
array with **no envelope key moved**, keyed `(commit-recording.stale-title,
commit:<task>#summary)`, located at that address, its message naming the title the summary still
carries; `doc show` returns the authored prose unchanged, so the finding *reports* and never
rewrites; **both** re-raising doors — `jigc task validate <id>` and `task finalize --dry-run` —
carry the code, the located `at:` and the same route; the **emitted route, run verbatim**,
silences all three and a second rename then acks `findings: []`; and a task that renamed nothing
is **inert** at every door, never an error. `flow37_rename.rs` still fences the rename's own acks
and referrer repointing, and is not the pin for this row.

---

## F-10 · No jigc verb amends a landed finalize — **CONFIRMED**, and it is the other half of B1's bypass

**The claim** (B1 feedback §4): *"the task was already closed by the successful finalize (no open
task left to re-run finalize through), jigc has no 'amend the commit my last finalize made' verb,
and the fix was a pure commit-boundary/message correction with no doc content to re-author — so
no `jigc doc` verb applied either."*

**Driven** — the full top-level surface, 17 verbs:

```
start · workflow · doc · task · config · milestone · setup · uninstall · upgrade
ingest · migrate · migrate-corpus · unmanage · rename · relocate · describe · validate
```

None amends a commit. `jigc rename` is the nearest and is **not** it: it fixes a committed doc's
*identity* — deriving a new slug, repointing referrers, `git mv`, its own commit — and does
nothing about a wrong **message** on a commit already made. A successful `finalize` closes the
task, so there is no task to re-run.

**Class: blocking dead end → SHIPS RECORDED**, under §1's own qualifier: *the state has a correct
recovery elsewhere.* Raw git **is** that recovery, and [CLAUDE.md](../../../CLAUDE.md) blesses it
— *"humans review and edit through git regardless of the CLI"*. Nothing is unrecoverable and no
bytes are at risk.

**The evidence about VISION principle #3 is the part worth carrying, not the severity.** F-9 and
F-10 compose: a structural op jigc owns leaves a stale summary (F-9), finalize lands it, and no
jigc verb can fix it (F-10) — so the only path is out of the adapter. **This is the trial's only
adapter bypass, and it was not a preference.** The worker named it in its debrief, and ran
`jigc validate` afterwards to confirm the store still read clean — which it correctly did.

`pinned-by:` **UNPINNED: the absence of a verb. Pinning it would pin the gap.**

---

## F-11 · The `doc author` payload parse error carries no code and no route — **CONFIRMED**

**Found in R4's transcript**, not in a debrief — the rehearsal recorded it as *"one friction
point, for the findings pass"* and this is that pass. The worker wrote a **flat** payload, was
rejected, read `doc author --help`, and got it right on the second try.

**Driven, with M50's own fenced sibling beside it for contrast — same door, same class of
mistake (naming something the schema does not declare):**

```
$ jigc doc author adr --task <t> --from-file <flat payload>
malformed `doc author` payload: unknown field `status`, expected `title` or `sections` at line 2 column 1
[exit 1]
        ← no severity, no code, no `at:`, no `route:`, no routing footer. A raw serde error.

$ jigc doc set-field 'adr:decision#nosuch' --value x --task <t>
blocking · write.unknown-field — no field "nosuch" declared on any section of `adr`
  (the single-hop `#<field>` form searches every declared section)
  at: adr:decision#nosuch
  route: `jigc doc schema adr` to see the declared shape, then re-run the write at a declared address
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
```

**This is the un-swept sibling of M50's own Increment 9.** That increment closed the write-miss
route floor by making the four target resolvers return `Result<_, Finding>` — *"an error type a
bare `anyhow` cannot inhabit, so the escape is closed at the type level"*. The **payload parse**
sits upstream of those resolvers and was outside the sweep. So the same door answers a declared
address miss with a keyed, routed finding and a payload miss with a serde string.

**Class: surface.** Reversible — the message already knows what it expected; it needs a code, a
route at `jigc doc schema <type>` (which would have answered the worker directly) and the house
renderer. **SHIPS RECORDED.**

`pinned-by:` `author_payload_floor::every_payload_refusal_answers_with_a_severity_a_code_an_at_and_a_route`
+ `author_payload_floor::every_payload_refusal_emits_the_findings_envelope_with_a_resolving_key`
— **converted by M51 Increment 6 / T3** (`b4567c77`), and **verified by reading what they
assert**: the text arm drives every cell to exit 1 and requires `blocking · <code> — `, an
`at: adr` and a `route:` on stderr; the machine arm requires the declared `Reject::Findings` key
set (`{findings, schema_version}`) holding exactly one finding whose `key` is
`{code, target: "adr"}`, whose severity is `blocking` and which carries a route — never the
flattened `{"error": …}` a driver cannot key on. The set is the class's defining case-set,
`PayloadReject::ALL`, closed in **both** directions by
`every_refusal_exit_of_the_payload_parse_is_driven`, so a fifth refusal exit cannot land without
a driven cell. `machine_output.rs` and `write_miss_shape_axis::CELLS` still fence the *resolver*
errors downstream, and neither was ever the pin for this boundary.

---

## F-12 · The hook-rejection frame names the actor but not the hook — **REFUTED in substance, PARTIAL on one clause**

**The claim** (B1 feedback §1): *"The docs-gate failure looked at first glance like a jigc
concept … Nothing in the failure output said 'this isn't jigc, this is a hook it wrapped' — I had
to go read `.githooks/pre-commit` myself."*

**Driven** (bare exit code — an earlier probe of mine read `sed`'s status through a pipe and
reported exit 0; the trap this repo's own shell guard exists for):

```
[exit 1]
`git commit` was rejected (no commit was made):
docs-gate: refusing this commit.

task record-it is intact — nothing was committed, your task's staged docs are still in
`.jigc/tasks/record-it/docs/`, and anything you had `git add`-ed is still in git's index.
Fix the hook's complaint, then re-run `jigc task finalize record-it`.
```

**Mostly refuted.** The frame attributes the rejection to **`git commit`**, not to jigc; it
reproduces the foreign stderr verbatim and visibly separated; it says *"the hook's complaint"*,
naming the hook as the author of the objection; and it states the survived state precisely and
names the re-run. That is M47's survivable frame doing exactly its job.

**What survives, and it is narrow:** the frame does not name the hook's **path**, so a worker
that wants to know *which* hook must go and find it — which is what B1 did. One clause.

**Class: surface, minor.** **SHIPS RECORDED.**

`pinned-by:` `crates/cli/tests/commit_rejected_axis.rs` — **verified by reading**: it drives the
survivable frame over the door set derived from `ERROR_CODE_REGISTRY` and asserts the state-truth
clause and the re-run argv. It does **not** assert a hook path, consistent with the gap.

---

## F-13 · The fan-out is for independent work, and a blind worker with sequential increments had nowhere to go — **CONFIRMED, capability gap**

**From B4-h**, the milestone arm, in its own words after reading four `--help` outputs
(`milestone --help`, `execute`, `provision`, `join`, `finalize`):

> jigc's execute/join/finalize path is built for *independent* work — every sub-task is pinned to
> the same fixed base commit and worked in isolated worktrees, joined at the end. Our increments
> are explicitly **not** independent (2 builds on 1, 3 extends 2), so that mechanism would
> silently drop each increment's dependency on the last.

It had planned a three-increment milestone through `planning` → a 14-gate `planning-record` →
`milestone create` → three `add-task`s, then judged the fan-out unfit for a **sequential** spine
and moved to discard the milestone — where the deny floor stopped it (correctly) and it asked.

**The reasoning is right about the design.** The fan-out's isolation is the whole primitive
([CLAUDE.md](../../../CLAUDE.md) — *each sub-agent writes to an isolated working area … the CLI
merges at a join ordered by task ID, not completion order*), and this repo's own build loop runs
sequential increments *outside* it for that reason.

**The gap is that a milestone offers one execution shape.** A worker that plans a dependent spine
— which is the ordinary case for an increment ladder — either misuses the fan-out or abandons the
milestone work-unit it just built. B4-h chose the second and could not complete it.

**Class: capability gap → SHIPS RECORDED.** Note it is *also* the reason this arm did not reach
the `d854e25` boundary tightening, which is therefore uncovered.

`pinned-by:` **UNPINNED: the absence of a sequential execution mode. `milestone.rs`'s join is
fenced over its own ordering and collision behaviour (read and confirmed); nothing asserts that a
milestone has exactly one execution shape, because that is the shape, not a defect in it.**

---

## The bypass itself, driven — what it did and did not do

Because *"a managed doc was committed by raw git"* is the kind of claim that must be sized, not
asserted:

```
$ git -C ~/out/RC14-B1 reflog
e288714 HEAD@{0} commit: docs(ingest): record the decision to shed the oldest sample …   ← manual
a5d6081 HEAD@{1} reset: moving to HEAD~1                                                  ← the surgery
445729a HEAD@{2} commit: docs(ingest): record the decision to drop the oldest sample …   ← jigc's finalize

jigc's commit   445729a  .jigc/config/manifest.yaml · docs/decisions/shed-the-oldest-sample.md
                         · scripts/retention-sweep.sh · src/router.ts        (4 files — F-2's sweep)
manual commit   e288714  .jigc/config/manifest.yaml · docs/decisions/shed-the-oldest-sample.md
                         (2 files — the two carryover paths removed)

doc blob in 445729a : 6628b9439a695090b243ca6c9e53bfcaf8094802
doc blob in e288714 : 6628b9439a695090b243ca6c9e53bfcaf8094802   → IDENTICAL

$ git -C ~/out/RC14-B1 status --porcelain                                         [exit 0]
A  scripts/retention-sweep.sh
M  src/router.ts
```

**The document's bytes never left jigc's write path.** Every slot and field was authored through
`doc author` / `set-slot` / `set-field`, and the re-slug through `doc rename`. What raw git did
was re-shape the *commit* around unchanged bytes.

**The residue the first sizing missed — the two carryover paths were left *staged*, not lost.**
The line *“2 files — the two carryover paths removed”* is true of the commit and was read as true
of the tree; it is not. The manual commit dropped `scripts/retention-sweep.sh` and `src/router.ts`
from the commit and left them **in the index**, where the `git status --porcelain` above still
finds them — on a branch whose `HEAD` message claims the work is recorded. **Nothing was lost**,
and the correction is to the sizing, not to the verdict.

**What no surface says, driven.** The archived out-dir was read with `git` alone — no jigc verb was
run inside it, because M49 permits a read verb to materialize a self-healing derived cache and this
is evidence — so the surface half was reproduced on a `dev/jigc-rig committed-singletons` corpus
carrying the same shape (`A  scripts/retention-sweep.sh` · `M  src/router.ts`, both unmanaged code):
`jigc validate`, `jigc doc list`, `jigc task list` and `jigc start`'s orientation each exit 0 and
name **neither** path. What does name them is a **write** door — the next mint's carryover gate
raises one `finalize.carried-staged` per staged path, previewed by `jigc task validate`. So the
state is reachable only by starting work, never by asking: **F-6/N27 from the other side**, where
`task diff`'s cold start answers *what is here* with almost nothing.

**jigc's bookkeeping did not go stale, and the reason is a design choice worth naming:** the
file-state baseline is keyed on the doc's **content hash**
(`docs/decisions/shed-the-oldest-sample.md → 6eba78aa…`), not on a commit sha. That is
**structural, not inferred** — the record *is* a `path → hex-hash` map
(`crates/engine/src/file_state.rs:63-64`), so there is no commit identity in it to go stale. The
rewritten history left the baseline correct, and `jigc validate` reading clean afterwards was
**right**, not lucky. Had the baseline recorded `445729a`, it would now point at a commit unreachable from
`HEAD`.

**Scope: one arm, one commit.** B2, B3, B3-h2, B4-h and B3-strict have **zero** resets in their
reflogs, and every commit touching `docs/` in all of them was made by a jigc finalize.

---

## What is NOT in this ledger, deliberately

**Zero data-loss or corruption findings.** Both planted paths survived B1; the plant-E docs
survived every arm; and **no managed doc's bytes were authored outside jigc in any of the five
blind sessions**.

**One adapter bypass, correctly sized rather than dismissed.** B1 rewrote the commit jigc's
finalize had made (F-9/F-10 above). An earlier draft of this ledger called it *"commit surgery on
the workspace, not a managed-doc write"* — **that framing was wrong and is withdrawn**: the
commit it rewrote contained a managed doc, so the doc's committed state was produced by raw
`git commit`. What survives from the earlier reading is narrower and had to be driven to be
claimed: the doc's *bytes* were identical across both commits, and jigc's baseline is
content-keyed, so nothing was corrupted and `validate` was right to read clean. The correction
matters because the bypass is evidence about VISION principle #3 and the softer framing would
have retired it as a workspace papercut.

**Zero regressions.** The two arms that went green→red (07, 08) were driven on both binaries and
proved to be the instrument meeting M50's **declared** `task discard` change; repaired, they run
13/13 and 17/17. Details in [session-findings.md](session-findings.md).
