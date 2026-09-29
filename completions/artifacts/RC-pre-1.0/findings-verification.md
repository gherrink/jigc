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

**The located, mechanically checkable gap** (F1): of **66 pack step files, exactly one** mentions
`doc show` — `locate-from-spec.yaml`, which is about locating against a spec, not reading back your
own writes. No authoring step names it.
**[Corrected 2026-08-15 (M48 Increment 12, T1):** the denominator was recorded throughout this file
as **69** — here, in F1's `observed:` line, and in the closing coverage note — and 69 was never the
number of step files. F1's second repro command,
`ls crates/cli/pack/steps/ packs/methodology/steps/ | wc -l`, counts `ls`'s own two directory
headers and the blank line separating them; the tree at the trial's HEAD (`1d4f9bc`) carried **66**
step `*.yaml`, so the command reported 66 + 3. The command is corrected to glob the files it means
to count (`ls crates/cli/pack/steps/*.yaml packs/methodology/steps/*.yaml | wc -l`), and the two
restatements are corrected in place against this bracket. **The verdict does not move**: the
numerator, the named file, and *"no authoring step names it"* were all read directly and are
unaffected. The count's one home, with the binary it was measured on (1.0.0-rc.10), is
[decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.11 wave (M48)*.**]** The agent is told once at session start, then walked through
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
  - ["sh","-c","ls crates/cli/pack/steps/*.yaml packs/methodology/steps/*.yaml | wc -l"]
expect:
  observed: "1 of 66 step files; the one is locate-from-spec.yaml, not an authoring step"
  wanted: "the authoring steps that solicit writes also name `jigc doc show <addr> --task <id>`"
pinned-by: read_back_fence::the_dev_owe_set_is_exactly_its_read_back_declarers · ::the_methodology_owe_set_is_exactly_its_read_back_declarers · ::every_write_soliciting_dev_step_is_fenced_at_pack_load
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
pinned-by: write_title_divergence::cell_a_a_same_slug_reauthor_with_a_dropped_title_blocks_and_routes · doc_rename_in_task::a_committed_doc_is_retitle_only_and_a_reslug_routes_at_jigc_rename
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
pinned-by: setup::setup_commits_the_pre_commit_hook_iff_it_is_a_working_tree_file · setup::setup_json_names_the_core_hookspath_hook
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
pinned-by: config_ack_uncommitted::every_config_ack_states_its_uncommitted_write_in_text_and_on_the_wire · ::config_set_through_the_binary_states_the_uncommitted_write_in_both_formats · ::config_ack_all_bijects_against_the_clap_config_verbs
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
  settled: "the second disjunct — the format is behaving as designed; the guidance is the lying link"
pinned-by: optional_slot_guidance::no_persisted_optional_slot_hint_instructs_an_omission · ::no_step_paragraph_guiding_a_persisted_optional_slot_instructs_an_omission · ::the_transient_exclusion_covers_only_the_commit_sink
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
pinned-by: config_read::config_list_emits_exactly_the_declared_knob_set · ::config_get_names_the_resolved_value_and_its_pack_default_layer · unknown_subcommand_tip::no_read_intent_is_answered_with_a_write_verb
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
pinned-by: engine::slug::tests::mint_statement_states_the_whole_mint_rule
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
pinned-by: doc_schema::doc_schema_json_is_the_pinned_contract · ::doc_schema_plain_listing_surfaces_write_addresses
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
pinned-by: doc_list::doc_list_projects_the_store_surface_with_its_registration_state
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
pinned-by: doc_show_staged::staged_read_serves_plain_json_and_slice  # findability is F1's fence, not this one's
```

### R4 — "`doc show` never rendered my authored changelog item" (operator's own, self-refuted)

Recorded because it was nearly written up as a defect. The item was present; `head -8` cut the render
before it. Verified by reading the whole output.

---

## The conversion ledger

**The gate on the 1.0.0 call: not taken until every row below carries `pinned-by:` or a stated
`UNPINNED: <why>`** ([decisions-pending.md](../../../implementation/decisions-pending.md) →
Acceptance). **Closed 2026-08-15 (M48 Increment 12, T2.)** At the trial's writing every row was
`UNPINNED` **by design** — the trial ran under *no mid-trial fixes*, so no fix and therefore no red
test existed yet. M48 built the fixes; this table is the citation each one earned.

**Every citation below was verified by reading what the test *asserts*, never by matching its name**
— §3's own honesty note (*a symbol-existence parser would be a finder wearing a fence's badge*), and
the discipline the latent-surface sweep ran under two days earlier. The per-suite assertion that
makes each one the pin is recorded in this change's commit message. **The lumped `F10–F16` row is
split**: one row cannot carry seven citations, and a lumped row is exactly where an uncited member
hides. **A citation is not a claim of completeness** — where a finding had two halves and one
shipped, the row says so and the unshipped half carries its own `UNPINNED:`.

| Row | Kind | Disposition — the citation, and what it holds |
|---|---|---|
| F1 | confirmed — **CLOSED** | `pinned-by: read_back_fence::the_dev_owe_set_is_exactly_its_read_back_declarers` · `::the_methodology_owe_set_is_exactly_its_read_back_declarers` · `::every_write_soliciting_dev_step_is_fenced_at_pack_load` — the owe-set **derived from the pack tree itself** equals the set of steps declaring `read.staged-read-back`, and withdrawing any one member's declaration blocks pack load non-zero |
| F2 | confirmed — **CLOSED** | `pinned-by: write_title_divergence::cell_a_a_same_slug_reauthor_with_a_dropped_title_blocks_and_routes` · `doc_rename_in_task::a_committed_doc_is_retitle_only_and_a_reslug_routes_at_jigc_rename` — the silent no-op blocks with its own code and its **emitted** route runs verbatim; the destination that reject needs exists, partitioned over the whole doctype registry |
| F3 | confirmed — **CLOSED** | `pinned-by: provision_leftover_guard::every_verdict_refuses_a_non_empty_leftover_and_leaves_the_planted_bytes_intact` · `milestone_discard::a_non_registered_leftover_at_a_subtask_worktree_path_refuses_the_discard` · `uninstall_worktree_guard::uninstall_refuses_a_non_empty_worktree_path_no_registered_probe_can_see` — every `LeftoverVerdict`, iterated from the code-side table, refuses and leaves the planted bytes **byte-intact**, at all three destroying doors |
| F4 | confirmed — **CLOSED** | `pinned-by: setup::setup_commits_the_pre_commit_hook_iff_it_is_a_working_tree_file` · `::setup_json_names_the_core_hookspath_hook` — over the whole hooks-dir axis the install commit carries the resolved hook **iff** it is a committable working-tree file, and the envelope names where it landed |
| F5 | confirmed — **CLOSED** | `pinned-by: config_ack_uncommitted::every_config_ack_states_its_uncommitted_write_in_text_and_on_the_wire` · `::config_set_through_the_binary_states_the_uncommitted_write_in_both_formats` · `::config_ack_all_bijects_against_the_clap_config_verbs` — every `ConfigAck::ALL` arm states it in both text surfaces and carries `committed: false` on the wire, and the table **bijects against the clap verb tree** so a seventh authoring verb cannot skip it |
| F6 | confirmed — **CLOSED** | `pinned-by: flow37_rename::idempotent_retitle_acks_the_no_op_and_never_claims_a_rejection` · `commit_rejected_axis::no_committing_door_dresses_an_empty_commit_as_a_rejection` — the axis's un-swept point acks the no-op at exit 0, and the class is swept over all nine `COMMITTING_DOORS` |
| F7 | confirmed — **CLOSED** | `pinned-by: optional_slot_guidance::no_persisted_optional_slot_hint_instructs_an_omission` · `::no_step_paragraph_guiding_a_persisted_optional_slot_instructs_an_omission` · `::the_transient_exclusion_covers_only_the_commit_sink` — **arm 2, not arm 1**: no guidance for a persisted optional slot instructs an omission the writer will not honour, subject derived from the loaded schema model. Declared bound, carried from the Settle: the derived-`optional:` half is axis-complete; the step-body half is a bounded omission-vocabulary probe |
| F8 | confirmed — **CLOSED** | `pinned-by: config_read::config_list_emits_exactly_the_declared_knob_set` · `::config_get_names_the_resolved_value_and_its_pack_default_layer` · `unknown_subcommand_tip::no_read_intent_is_answered_with_a_write_verb` — the read rung emits *exactly* the declared knob set and names its winning layer; and over every `(parent, read-shaped guess)` pair the clap tree yields, no emitted tip names a write verb, at least one read verb is named, and the emitted span runs |
| F9 | confirmed — **CLOSED** | `pinned-by: engine::slug::tests::mint_statement_states_the_whole_mint_rule` — the boundary set is **parsed back out of the rendered sentence** and set-compared against the set derived from `renormalize`'s behaviour over every printable ASCII char, so the literal `-` can be neither omitted nor over-claimed |
| F10 | confirmed — **half closed** | `pinned-by: foreign_at_both_doors::a_foreign_file_answers_one_code_and_one_route_at_every_door` · `::the_managed_advisory_routes_on_the_stamp_never_at_adoption` — the lying name is gone: one foreign file answers one code and one route at the store, `task validate` and `finalize` doors, and the managed cell routes on the stamp. **The second half is `UNPINNED:` the advisory still fires on a task that never touched the file** — routed to M46 entry 3 as a counted datum, not fixed, so a standing test over it would pin the advisory-habituation floor as expected output ([pinning.md](../../../implementation/pinning.md) §5) |
| F11 | confirmed — **CLOSED** | `pinned-by: corpus_migration::migrate_corpus_headline_states_its_run_mode_over_the_whole_axis` — the headline is byte-exact and true across three run modes × two corpus states, each landing cell asserting whether `HEAD` moved |
| F12 | confirmed — **CLOSED** | `pinned-by: doc_schema::doc_schema_id_source_names_its_write_key` — the id-source leaf names the key its value is written under, at both nesting depths and both `id-from` types, **fenced against the real clap tree** (the field-id spelling the projection would otherwise imply does not parse), with two omitting contexts |
| F13 | confirmed — **CLOSED** | `pinned-by: finalize_message_truth::both_pre_commit_headers_state_the_intent_and_a_rejected_finalize_never_reads_as_done` — both headers state the intent, the print keeps its pre-commit position, and `HEAD` is asserted unmoved on the rejected run |
| F14 | confirmed — **CLOSED** | `pinned-by: start_orientation::every_selectable_changelog_granting_workflow_names_the_gate_on_both_routing_surfaces` — the member set is derived from the pack's own `allows-create:`, and the assertion runs over the **emitted** catalog line and describe paragraph. Unbundling was refused at the Settle; nothing is owed for it here |
| F15 | confirmed — **CLOSED (the non-schema half; the other was refused)** | `pinned-by: methodology_pack_compose::planning_finalize_names_the_milestone_verbs_and_the_seeding_bound` · `::a_workflow_that_composes_the_bare_finalize_carries_no_milestone_prose` — asserted on the emitted bytes of **both** composing doors, and the `add-from-spec` seeding bound is stated on the surface that creates the adjacency. The structural half (a managed `roadmap-entry → milestone-record` edge) is refused on three independent grounds ([settle-record.md](../M48/settle-record.md) → F15), so it owes no fence |
| F16 | confirmed | `UNPINNED:` routed to **M46 entry 2**, re-counted at seven demands and still deferred — a record where none exists is new domain capability, which the wave's razor refuses ([settle-record.md](../M48/settle-record.md) → F16). No fix exists, so there is no behaviour to cite; `Checkpoint:` remains pure emission, carried against that entry rather than fenced here |
| R1 | **refuted — CLOSED** | `pinned-by: doc_schema::doc_schema_json_is_the_pinned_contract` · `::doc_schema_plain_listing_surfaces_write_addresses` |
| R2 | **refuted — CLOSED** | `pinned-by: doc_list::doc_list_projects_the_store_surface_with_its_registration_state` |
| R3 | **refuted — CLOSED** | `pinned-by: doc_show_staged::staged_read_serves_plain_json_and_slice` |
| R4 | refuted (operator) | `UNPINNED:` the operator's own measurement error (`head -8` truncating the render), not a fact about the product — there is nothing about jigc for a standing test to hold, and the capability it appeared to contradict is R3's, already cited above |

**The refuted set is closed, and closed without minting a test** (2026-08-13). Each of the three
product refutations is a **shipped capability the sessions did not find** — M40's `doc schema`, M42's
`doc list`, M43's `doc show --task` — and each already carries a dedicated contract suite, so the
fact cannot drift silently. §3's obligation is that a refuted fact **has** a standing test, not that a
duplicate is minted (the `pinned_facts.rs` B3 precedent). Every citation above was verified **by test
content, not by test name**, per that module's own honesty note: `doc_schema` drives `doc schema adr`
in both formats; `doc_list` asserts the exact row set *and* the `managed`/`unregistered` split;
`doc_show_staged::staged_read_serves_plain_json_and_slice` drives `doc show <addr> --task <id>` over
uncommitted bytes and asserts the staged bytes serve.

**What that left unpinned was reachability, not capability** — the capabilities were fenced; what
nothing fenced was that an agent can *find* them, F1's countable property (**1 of 66** pack step
files named `doc show`, and it was not an authoring step). **That fence has landed.** At `HEAD`
the tree carries **67** step files, **31** name `doc show`, and **30** of those are held there by
`read.staged-read-back` at pack load — the thirty-first is `locate-from-spec`, the suite's declared
bound (it solicits its writes as literal command lines, so it carries no structural signal to
derive from and states the read-back without joining the fenced set). The property that was
countable-but-unheld is now derived and enforced, which is why F1's row cites a fence rather than a
count.

**The confirmed set is closed, and the gate is discharged.** F1–F16 each carry a citation or a
stated reason: **fourteen closed on a standing test**, one (**F10**) closed on the half that shipped
with its second half explicitly routed and unpinned, and one (**F16**) `UNPINNED` on the Settle's
own refusal. **The two non-fences are not one reason but two**, and saying so is the point of a
ledger: F16 has *no behaviour to fence* — the fix was refused, so there is nothing standing that a
test could hold — while F10's second half **does** stand, and fencing it is the thing that would be
wrong: a standing test over an advisory still firing on an untouched file pins the
advisory-habituation floor as expected output, and a guard over a defect defends the defect against
its own fix ([pinning.md](../../../implementation/pinning.md) §5 addendum). Both are carried against
M46 entries, where their fixes are counted. The 1.0.0 call's precondition is this table; **it no
longer blocks.**

---

## Process changes for the next trial

Recorded here rather than in this trial's `protocol.md`, following the RC-alpha3 precedent
(*process change 7*, which is how the environment-matrix discipline reached this trial): a per-trial
protocol is disposable, so a convention that should outlive it lives with the verdicts and is cited
forward by the next protocol.

1. **Back-date the plant's commit, not just its body.** The G3 foreign-ADR plant carried
   `Date: 2026-08-04` in its body while being committed mid-session, and the worker opened its report
   by naming exactly that mismatch. The catch itself was unweakened — everything that mattered
   concerned the ADR's *content* — but a worker that notices a back-dated file can infer it is being
   tested, which is the contamination the unseeded protocol exists to prevent. Set
   `GIT_AUTHOR_DATE` / `GIT_COMMITTER_DATE` on the plant commit, or drop the body date. Free, and it
   removes the tell.
2. **Keep a contemporaneous `operator-log.md`.** New this trial: every operator utterance into a
   blind session, logged verbatim with its justification *as it happened*, plus the record-side
   observations riding on that exchange. It is what let the record's purity statement be written from
   notes rather than recall — the weakness every prior trial's honesty statement carried. It also
   forced the honest classification of a **pre-registered trap versus an accident** in real time: the
   prose-vs-code contradiction two sessions found independently was the observer's own, and logging
   it at the moment of discovery is what stopped it becoming a "caught trap" in the write-up.
3. **Commit an operator plant with an explicit pathspec.** `git commit -m … -- <path>` rather than
   `git add <path> && git commit`, so a worker's in-flight index can never be swept into the
   operator's commit. Verified before and after on this trial's plant; the bare form would have been
   a silent contamination with no trace.
4. **Stage the shipped guides outside the corpus.** `jigc setup` installs a bootstrap `CLAUDE.md`,
   an allowlist and hooks — but ships **neither `QUICKSTART.md` nor `MIGRATING.md`** into the repo, so
   a blind worker has no path to the docs the charter requires it to read. Copying both to a sibling
   directory named in the prompt keeps them reachable without polluting the ingest funnel (a tracked
   `.md` inside the corpus becomes an ingest candidate). Declare it as operator seeding, and expect
   their in-repo relative links to dangle from that location — if a worker is stranded following one,
   that is a finding, not a prep defect.

   > **Superseded 2026-08-14 — the convention retires (M48 Increment 10, T1).** The rule existed
   > because `setup` shipped no guides; it now writes them itself, as an **adapter-owned,
   > version-stamped artifact** at the profile-declared path (`design/assistant-adapter.md` → The
   > adapter's owned artifacts), committed with the rest of the install. So a trial on rc.11 or later
   > **does not seed a sibling directory** — the guides are in the corpus, matched to the binary, and
   > naming them in a prompt is no longer operator seeding at all. Both halves of the rule's rationale
   > are answered rather than worked around: the artifact lands under the assistant's own skills path
   > (not a doc directory), and its in-repo relative links are **resolved away in the shipped bytes**,
   > so the dangling-link finding this rule pre-registered can no longer be produced by the shipping
   > guides. The note is added, not rewritten: what the rule said was true of the binary the trial ran.
