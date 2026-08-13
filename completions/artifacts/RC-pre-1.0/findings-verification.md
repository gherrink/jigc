# The pre-1.0.0 trial — adversarial verification of every claim

**Method.** Every claim from the three blind feedback reports and the operator walk was re-driven
against the installed `jigc 1.0.0-rc.10`, on the trial corpora or a scratch task, before being
written down. **CONFIRMED and REFUTED alike carry repro blocks**
([milestone-completion-workflow.md](../../../implementation/milestone-completion-workflow.md) →
Audit; block shape per [pinning.md](../../../implementation/pinning.md) §3). Where a claim was
adjudicated from committed evidence rather than a fresh run, the evidence is named.

**Two claims died in verification and are recorded as such**, because a trial that only confirms is
not verifying: a first reading had `jigc validate` at exit 0 contradicting its own trailer (that was
`$?` after a pipe reporting `tail`; it exits 1), and an authored changelog item appeared missing from
`doc show` (that was `head -8` truncating the render). Neither is a defect.

---

## The through-line

**The capability exists; the surface the agent is standing on does not name it.** Sixth consecutive
trial with this signature — and this time it landed on **the capability that was built in response to
this exact complaint three trials running** (M43 fork 1, the staged read, recorded as *"demanded three
trials running"*).

All three blind sessions, independently, in separate corpora, went to the filesystem to read their
own in-flight work:

- **G1** ran `cat .jigc/tasks/**/*ingest-overflow*` to show the ADR for sign-off, and named the cause
  precisely: *"nothing in the composed workflow ever mentions reading a doc back. Every verb it
  teaches is create/set-slot/set-field/author."*
- **G2** did something worse and said so: *"I never read back a single managed doc I wrote"* —
  ~15KB of authored prose, verified only by `doc author`'s one-line ack and a clean `validate`. It
  states plainly that it cannot attest the grounding research rendered into the vision at all.
- **G3** `cat`-ed staged files four times and **found both of its defects that way**, concluding:
  *"the read surface is where it leaked — `doc show` being committed-only meant that every time I
  wanted to verify my own work in progress, the supported answer was 'finalize and find out'."*

`jigc doc show <addr> --task <id>` has shipped since M43 and serves exactly that. It is documented in
the second sentence of `doc show --help` and in **line 3 of the `AGENT.md` preload** every session
starts with. It was still not found by any of the three.

**The located, mechanically checkable gap** (F1): of **69 pack step files, exactly one** mentions
`doc show` — `locate-from-spec.yaml`, which is about locating against a spec, not reading back your
own writes. No authoring step names it. The agent is told once at session start, then walked through
six write verbs that never mention it again.

---

## CONFIRMED

### F1 · HIGH — the read-back verb is absent from the composed authoring surface

The write verbs are taught step by step; the read-back verb is taught nowhere in the flow that
solicits the writes. Consequence, measured across three sessions: two agents bypassed the channel the
adapter claims sole ownership of, and one authored 15KB unverified.

```yaml
claim: "the composed authoring steps never name the staged read, so agents cat the working area"
verdict: CONFIRMED
setup:
  - fixture: any repo after `jigc setup`
repro:
  - ["sh","-c","grep -rl 'doc show' crates/cli/pack/steps/ packs/methodology/steps/ | wc -l"]
  - ["sh","-c","ls crates/cli/pack/steps/ packs/methodology/steps/ | wc -l"]
expect:
  observed: "1 of 69 step files; the one is locate-from-spec.yaml, not an authoring step"
  wanted: "the authoring steps that solicit writes also name `jigc doc show <addr> --task <id>`"
pinned-by: UNPINNED — a pack-load fence (authoring steps that solicit a write must state the read-back) is the fix's own red test
```

### F2 · HIGH — `doc author` silently ignores a corrected `title:` and acks success

Re-authoring an already-staged doc with a corrected title applies the slot writes, leaves the H1 and
slug at their original values, and prints the **old** slug — indistinguishable from success. `doc
author --help` says a re-authored doc-level leaf *"overwrites in place"* and never exempts `title`.
G3 caught it only by dumping the staged file; the recovery was `task discard` plus re-minting the
whole migration.

```yaml
claim: "doc author ignores a changed title: on re-author over a staged doc, and acks as if it applied"
verdict: CONFIRMED
setup:
  - ["jigc","start","--workflow","record-decision","probe the title reauthor"]
  - ["jigc","doc","author","adr","--from-file","-","--task","<t>"]   # title: Keep the sample store in-memory
repro:
  - ["jigc","doc","author","adr","--from-file","-","--task","<t>"]   # title: Keep the sample store in memory
  - ["jigc","doc","show","adr:keep-the-sample-store","--task","<t>"]
expect:
  ack: "adr:keep-the-sample-store"           # the OLD slug, no mention that title was dropped
  h1: "# Keep the sample store in-memory"    # unchanged
  slots: "updated — the rest of the payload applied"
  wanted: "reject the payload, or ack `title ignored — create-only`; never a silent no-op under a success ack"
pinned-by: UNPINNED — the fix's red test pins it
```

### F3 · HIGH — `milestone provision` destroys a non-registered leftover's uncommitted work

Full detail and repro in [v1-walk.md](v1-walk.md) → Arm 3 (V1-F1). Staged, unstaged **and** untracked
work destroyed at exit 0 under the ordinary success line; the untracked file unrecoverable. The
decisive framing is the contrast inside the same walk: `milestone discard` and `jigc uninstall` both
**refuse** over a single untracked file. M46 entry 11's trigger — *"an adopter reports real sub-agent
work lost"* — is now met with the loss shown rather than inferred.

### F4 · MEDIUM — `setup` lists the pre-commit hook as installed; its install commit omits it

With `core.hooksPath` pointing in-repo, `setup` splices its managed block into the tracked foreign
hook (correctly, preserving it verbatim) but does not commit that file. Both the summary and
QUICKSTART (*"commits its own install… only the files above"*) imply otherwise. Found in the operator
rehearsal, then independently reported by G1 with the consequence named: *"a clone gets no drift
backstop, and nothing says so."*

```yaml
claim: "setup's install commit does not carry the pre-commit hook it lists as installed (in-repo core.hooksPath)"
verdict: CONFIRMED
setup:
  - ["git","config","core.hooksPath",".githooks"]
  - ["sh","-c","install a tracked foreign .githooks/pre-commit and commit it"]
repro:
  - ["jigc","setup"]
  - ["git","show","--stat","<install-commit>"]
  - ["git","status","--short",".githooks/"]
expect:
  observed: "install commit e80e0c2 carries 7 files, none of them .githooks/pre-commit; hook left ` M` at session end"
  wanted: "commit it with the install, or say it is left for the operator to commit"
pinned-by: UNPINNED
```

### F5 · MEDIUM — `jigc config set` leaves config uncommitted, and it rides an unrelated feature commit

`config set` never commits (by design — `config.rs`: *"committed by the operator's next commit, never
here"*) and the ack says nothing. Three independent instances: the operator walk (swept into a
`docs(adr)` migrate commit), G1 (swept into its `docs(ingest)` ADR commit), and the fresh clone in
walk arm 5, which **inherited no config and produced zero invocation-log records**. G1 names the
sharpest form: `setup` goes out of its way to keep its install off your first feature commit, and the
very next documented command breaks that principle.

G1 is also precise about what is *not* a violation: `--dry-run` disclosed it, and the carryover gate
correctly did not fire (it keys on staged-before-mint; this was untracked). The unstated rule is the
one that pulled it in — the workflow says finalize commits *"the staged set plus the docs it
manages"*, and config is not a doc.

```yaml
claim: "config set leaves .jigc/config untracked; a later unrelated finalize commits it, and a clone taken first loses the setting"
verdict: CONFIRMED
repro:
  - ["jigc","config","set","invocation-log","true"]
  - ["git","status","--short",".jigc/config/manifest.yaml"]
  - ["sh","-c","git clone . ../clone && ls ../clone/.jigc/config/"]
expect:
  observed: "untracked; ack silent; clone has no manifest.yaml and logs nothing"
  wanted: "commit it, or say it is uncommitted until your next commit"
pinned-by: UNPINNED
```

### F6 · MEDIUM — an idempotent `rename` misreports git's empty commit as a hook rejection

[v1-walk.md](v1-walk.md) → Arm 6 (V1-F2), with its repro block. Same-slug **and** same-H1 is the
un-swept point of an axis whose other point (`same slug, different H1`) is fenced by
`flow37_rename::placement_same_slug_retitle_succeeds`.

### F7 · MEDIUM — an omitted optional slot still emits its heading, into a committed doc

The migrate template instructs *"omit this entry if unused"*; omitting it renders an empty
`## Options` heading, which shipped into tidepool's committed ADR at `ac08936` (lines 16–19: heading,
then blank). Either omission drops the heading or the guidance must stop saying "omit".

```yaml
claim: "omitting an optional slot per the template's instruction still renders its heading into the committed doc"
verdict: CONFIRMED
repro:
  - ["sh","-c","migrate a foreign ADR --as adr, omitting the options entry, then --approve"]
  - ["sed","-n","14,20p","docs/decisions/keep-the-sample-store.md"]
expect:
  observed: "## Options followed by blank lines, committed at ac08936"
  wanted: "no heading for an omitted optional section, or template guidance that matches"
pinned-by: UNPINNED
```

### F8 · MEDIUM — no config read verb, and read intents are routed to write verbs

`jigc config get` and `jigc config list` do not exist; the config surface is write-only. G1 hit it
concretely — hardcoding `docs/decisions/…` into a source comment with no way to confirm `docs-root`,
falling back to QUICKSTART's documented default.

**One cause, two doors**, which makes it one axis rather than two papercuts: clap's similarity tip
answers a *read* intent with a *write* verb. `config get` → *"a similar subcommand exists: 'set'"*;
`doc read` → *"a similar subcommand exists: 'create'"* (G3), where `show` was wanted.

```yaml
claim: "there is no config read verb, and near-miss read verbs are routed to write verbs"
verdict: CONFIRMED
repro:
  - ["jigc","config","get","docs-root"]
  - ["jigc","doc","read","adr:x"]
expect:
  observed: "unrecognized subcommand 'get' → tip: 'set';  unrecognized subcommand 'read' → tip: 'create'"
  wanted: "a config read verb; and near-miss tips that prefer a read verb for a read-shaped miss"
pinned-by: UNPINNED
```

### F9 · MEDIUM — the generated slug statement omits the literal hyphen as a word separator

The seam-generated rule (`slug::mint_statement`) enumerates the characters that become word
boundaries — *space, `_`, `/` and `.`* — and does **not** list a literal `-`, while its next clause
says an edge filler word is kept *"unless a hyphen glues it to its neighbour"*, which implies hyphens
bind. G3 titled an ADR *"Keep the sample store in-memory"* expecting `in-memory` to survive as one
glued token; the hyphen split it for counting, the 5-word cap cut after `in`, and the trailing `in`
dropped → `keep-the-sample-store`.

The behaviour is correct by design (`glue_before` affects edge-stopword dropping, not the word cap).
The **statement** under-describes it — and this is the surface M47 Increment 1 made a one-way door and
deliberately seam-generated so it could not drift.

```yaml
claim: "the rendered slug rule does not state that a literal hyphen splits words for the cap"
verdict: CONFIRMED
repro:
  - ["jigc","doc","create","adr","--title","Keep the sample store in-memory","--task","<t>"]
expect:
  observed: "adr:keep-the-sample-store  (in-memory split; cap at 5; trailing `in` dropped)"
  stated_rule_omits: "that a literal `-` is also a word boundary"
  wanted: "the generated statement names the literal hyphen alongside space/_/./ as a boundary"
pinned-by: UNPINNED — the statement is generated, so the fix and its fence are the same change
```

### F10 · LOW — a stale conformance advisory attaches to unrelated tasks, and is named `-block`

`reconciliation.conformance-block` for the unmigrated foreign ADR fired on `validate` **and**
`finalize` of a spec task that never touched that file — 4 firings in tidepool's log. G3 also notes
the name reads as blocking while the severity is advisory.

### F11 · LOW — `migrate-corpus`'s recovery headline says `already current` on a run that landed

[v1-walk.md](v1-walk.md) → Arm 2 (V1-F3). The count describes the scan; the action is only in the
trailing commit line.

### F12 · LOW — an enum repeatable's id-source is projected as `category`, accepted only as `title`

[v1-walk.md](v1-walk.md) → Arm 1 residue (V1-F4). `doc schema` is the contract-pinned surface an agent
is told to consult, which is what makes it law-2 rather than a papercut.

### F13 · LOW — the finalize header asserts the commit before the rejection

A rejected finalize opens `finalize — committing the index; leaving out:` and only then reports the
rejection. G1: *"it reads as done until the next line contradicts it."*

### F14 · LOW — a routing gap manufactures a guaranteed advisory

No workflow fits *"record a decision and point the code at it"* — `record-decision` excludes code,
`decided-task` writes a decisions-log rather than an ADR — so G1 took `single-task`, which grants the
changelog gate, and finalize raised `changelog-recording.gate-granted-unused` (1 firing, logged) on a
change that by construction has no user-facing behaviour. The advisory is correct; the routing made it
inevitable.

### F15 · MEDIUM — "milestone" names two objects with no edge between them

G2: the `planning` workflow authors a roadmap entry and **mints no milestone**; `jigc milestone create`
mints the work-unit. It typed the title twice and got two unrelated ids (the roadmap item id vs
`m1-the-service-runs-and`). *"A reader following the planning workflow to its finalize would stop
believing they'd opened one."* Related: `add-from-spec` seeds sub-tasks from a spec's **criteria**,
while planning produces prose in a decomposition slot — so the two halves cannot meet, and G2
hand-retyped four intents restating prose already committed.

### F16 · LOW (but a counted demand) — a Checkpoint leaves no record

G2: *"There's no verb to record passing it, nothing that blocks finalize if you didn't. I ran the gate
because the prose asked me to."* The halt **behaved** in both G2 settles; what is absent is any record
it was walked. This is the **7th demand** on M46 entry 2, arriving unprompted.

### Smaller confirmed items, recorded without individual blocks

`milestone create` takes a positional title while `doc create` takes `--title` (G2, reproduced by the
operator in walk prep) · slug truncation is silent and two differently-titled docs collide easily (G2:
research title and task id both slugged `what-already-exists`) · `set-slot` takes raw prose while
`doc author` requires `<<…>>`, and only `--help` says so · no `doc author --dry-run` for a ~9KB payload
· no `--slug` on `doc create`/`doc author` though `rename` has one · no in-task way to change a
doc-level title (G3's discard-and-redo) · `task diff` has no path filter · `implement-from-spec` prints
an empty criteria list before bind, making an unbound spec look like an empty one · `describe` is one
undifferentiated prose wall with no `--workflows` or `describe <name>`, dominated by hidden `migrate-*`
entries · composed output density (G1: ~200 lines embedding the whole changelog schema for a task with
no changelog; the heading-depth warning verbatim 4×) · the generated hook's own comment concedes
*"the exit code is wrong-way-round"*, after which G1 stopped trusting validate-family exit codes.

---

## REFUTED

Each of these is a shipped capability the session did not find. **Refuted blocks are the conversion
obligation** — the refuting fact is exactly what drifts when nothing tests it.

### R1 — "nothing lets you ask a doctype's schema"

G1 learned the ADR's slots from workflow prose and reported no way to ask. `jigc doc schema adr` is
the contract-pinned projection (contract-version 4), shipped M40.

```yaml
claim: "no way to ask a doctype's schema; adr slots must be learned from workflow prose"
verdict: REFUTED
repro:
  - ["jigc","doc","schema","adr"]
expect:
  exit: 0
  stdout_contains: "doctype: adr (schema-version 2)"
  and: "every field with its set-field address, every section with its set-slot address"
pinned-by: UNPINNED — belongs in pinned_facts/ per pinning.md §3
```

### R2 — "no inventory verb; I used `find docs VISION.md`"

`jigc doc list` is the contract-pinned index read, shipped M42, reporting identity **and**
registration state.

```yaml
claim: "no verb inventories what jigc manages"
verdict: REFUTED
repro:
  - ["jigc","doc","list"]
expect:
  exit: 0
  stdout_contains: "id  path  state"
  and: "one row per managed doc; `unregistered` rows distinguished"
pinned-by: UNPINNED — belongs in pinned_facts/
```

### R3 — "there is no supported way to review in-flight authoring before finalize"

The headline refutation. `jigc doc show <addr> --task <id>` serves the staged working copy through
the identical parse/slice path, shipped M43 — documented in the second sentence of `doc show --help`
*and* in line 3 of the `AGENT.md` preload the session starts with.

```yaml
claim: "doc show is committed-only by design, so in-flight authoring cannot be reviewed before finalize"
verdict: REFUTED
setup:
  - ["jigc","start","--workflow","record-decision","<intent>"]
  - ["jigc","doc","author","adr","--from-file","-","--task","<t>"]
repro:
  - ["jigc","doc","show","adr:<slug>","--task","<t>"]
expect:
  exit: 0
  stdout: "the staged doc rendered through the canonical path, including a transient commit:<task>"
pinned-by: UNPINNED — belongs in pinned_facts/; F1 is the fix that makes it findable
```

### R4 — "`doc show` never rendered my authored changelog item" (operator's own, self-refuted)

Recorded because it was nearly written up as a defect. The item was present; `head -8` cut the render
before it. Verified by reading the whole output.

---

## The conversion ledger

**The gate on the 1.0.0 call: not taken until every row below carries `pinned-by:` or a stated
`UNPINNED: <why>`** ([decisions-pending.md](../../../implementation/decisions-pending.md) →
Acceptance). Every row is currently `UNPINNED` **by design** — the trial ran under *no mid-trial
fixes*, so no fix and therefore no red test exists yet.

| Row | Kind | Conversion owed |
|---|---|---|
| F1 | confirmed | pack-load fence: an authoring step soliciting a write states the read-back |
| F2 | confirmed | red test: re-author with changed title → rejected or acked as ignored |
| F3 | confirmed | red test: provision over a leftover refuses, naming the dirty paths |
| F4 | confirmed | red test: in-repo hooksPath install commit carries the hook |
| F5 | confirmed | red test: `config set` ack states its uncommitted state |
| F6 | confirmed | red test: same-slug **same-H1** rename — the axis's un-swept point |
| F7 | confirmed | red test: omitted optional section renders no heading |
| F8 | confirmed | red test: read-shaped near-miss routes to a read verb |
| F9 | confirmed | the generated statement names the literal hyphen (fix and fence are one change) |
| F10–F16 | confirmed | red tests per item; F15/F16 may route to M46 rather than a fix |
| R1, R2, R3 | **refuted** | `pinned_facts/` standing tests — the refuting fact is what drifts |
| R4 | refuted (operator) | none — a measurement error, not a fact about the product |
