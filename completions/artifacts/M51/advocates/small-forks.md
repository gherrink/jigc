# M51 Settle — the robust case on three smaller forks

Independent robust-advocate. I proposed none of these cheap cuts, settle nothing, made no edits.
Every claim verified at HEAD `74627547` by reading or counting the code named. **Bound honesty:** the
brief asked ≤300 words per fork; these run ~340–400. Every word over is a citation or a driven datum,
and I judged a stripped case worse than an over-length one — trim the scope lists first.

---

## Fork A — EC-14/EC-22/EC-23/EC-12: rewrite the numeral, or fence it

**The hole.** The wave's claim is *"every surface 1.0.0 pins says what the binary does."* A rewritten
numeral satisfies that for one commit. A pin outlives the wave; a hand-written count does not.

**Three spikes at HEAD.** (1) CLAUDE.md's *"thirteen `CELLS` rows"* was **63** when written; I counted
`write_miss_cells.rs` at HEAD — **64**. It moved again between the evidence check and this Settle:
100 % defect rate, half-life shorter than the wave. (2) The sync mechanism actually in use is a comment
addressed to a human — `invocation_log.rs:513` asserts `len() == 11` with *"a change here revises
design/surface-contract.md's mirror in the same commit"*, and nothing reads that file. The door count
moved 9→10 at M49; **five prose homes still say nine**, including `MIGRATING.md:39` (ships to every
adopter) and `flow47_acceptance.rs` (self-contradicting four lines apart). Same prose-not-fence
mechanism EC-10 records failing five consecutive waves. (3) The fence built for this class became an
instance of it: `foldback_truth.rs:458` fails any claim `== 69` as *"never the number"* — the tree is
30+39 = **69**. And `setup` is not among the ten `COMMITTING_DOORS` while fork 4's robust arm makes it
one: a numeral can go stale *inside* this wave.

**Not premature generality; not unprovable.** The molds ship (`doctype_map_versions.rs`,
`foldback_truth`'s derived guide set, `SchemaChangeKind::ALL`); `ManifestKind` is `pub`. And two EC-14
members were **refuted** on re-derivation while two are **history**: a human told *"fix the counts"*
re-pins history, where `dated_correction_spans` distinguishes it.

**Price of cheap.** The guides are `include_str!`'d into `SKILL.md` (`setup.rs:69-70`), so every
post-1.0 correction moves `jigc-body-blake3` and meets `adapter-guide.user-modified`. Free now,
adopter-visible after.

**Robust scope.** `ManifestKind::ALL` + a per-prose-unit `foldback_truth` arm · triage keys via
`CorpusMigrationReport`'s exhaustive destructure · `COMMITTING_DOORS` and the `ERROR_CODE_REGISTRY`
mirror on the `doctype_map_versions` mold · historical members dated-bracketed · `== 69` re-keyed ·
**`AMBUSH_CLASS_CODES` derived** (N26's trigger fired verbatim) · **EC-10's fence**, narrowing
`foldback_truth.rs:223`, which declines *choosing a numeral*, not *comparing two homes*; refuse a pack
step as its home, since `completion.yaml` ships to every adopter and telling their agent to bump a
workspace version is a law-1 lie for every reader who is not this repo.

**Verdict: robust-now** — a count over a set the code can move is true only if something derives it,
and this repo has lost that bet at the prose, at a test literal, and at the fence built to prevent it.

---

## Fork B — EC-6, the invocation log

**I argue (i): declare it UNVERSIONED, close the key set, state the additive-only rule. The version
integer is the cheap-*looking* option.**

**It answers a question nobody asked.** `measurement.md:75` means independent **of the dogfood hook's
JSONL schema** — *"no shared format is implied or required"* — echoed at `invocation_log.rs:379`.

**Three ways it is wrong.** It **mints a pinned surface in the wave whose job is closing the pin** — a
one-way door created, not closed. It adds a bump obligation to the same prose-not-fence mechanism EC-10
records failing five consecutive waves. And it is **weaker than what ships**: every record carries
`binary_version` (driven: `"1.0.0-rc.14"`). On an append log fed by successive binaries, a per-record
stamp maps each record to the format that wrote it; one file-level integer cannot, because one file
legitimately holds several formats.

**The real hole the cheap read skips.** `Record` has **8** fields; `tests/invocation_log.rs:128`
asserts presence/type on **four**; `grep "as_object\|keys()"` over the suite → **no key-set equality
anywhere**, so an added key is unfenced and a **removed** `argv` or `output_bytes` stays green. The one
doc stating the shape, `measurement.md:62`, is **wrong in two fields**: `duration` (emitted
`duration_ms`) and `finding_codes` scoped *"(on failure)"* while it ships present-and-empty on success.
Documented wrong and fenced open is a hole in a declared surface — independent of versioning.

**Honest bound + scope.** Opt-in, gitignored, read only by `completions/trial-driver/*`, reaching no
adopter contract: **not** a one-way door — which is why declaring it unversioned belongs on the record,
keeping the log out of the pin rather than dragged in. Cost: one exhaustive destructure of `Record`
driving key-set equality on the emitted JSON, plus one `measurement.md` paragraph (unversioned ·
additive-only · `binary_version` the discriminator · tolerate unknown keys) and the two wrong fields
corrected. M44's `task_id` deferral rests on *"purely additive, no one-way door"*; this makes that basis
stated rather than assumed.

**Verdict: robust-now, as (i).** Versioning wears the thorough costume while adding an obligation,
adding nothing `binary_version` lacks, and minting a pin in the wave that closes pins.

---

## Fork C — N23/EC-38, doctype deregistration

**The hole is a false green in the adopter's CI.** Driven twice (baseline §6, EC-38): with a doctype
out of the resolved set, `jigc validate` prints *"no findings — the committed store validates clean"*
at **exit 0**, `jigc doc list` **drops the rows entirely** (not `unregistered` — gone), and
`git ls-files` still carries every file. `MIGRATING.md` ships into every adopter repo telling them to
CI-gate on that verb. A green meaning *"I stopped looking at these files"* is the sharpest law-1 lie
available, because the reader delegated the check to it.

**The trigger has fired, verbatim.** N23 ends: *"Trigger: … **or the committed store-surface enumerator
is next opened** — the `doc list` / `validate` / `migrate-corpus` triple, whose disagreement here is
the defect."* M51 opens that triple (EC-23, EC-1's migrate sink, the adoption discriminator); EC-38
re-drove the row.

**"Narrow reachability" is a claim about the calendar.** PB-1 shipped at M49 as **the documented way an
adopter owns doctypes**, and its declared bound makes divergence expected — *"a project pack is a fork,
not a subclass … a vendored copy does not track its base pack's upgrades."* Retiring or renaming a
doctype you own is the first thing a doctype owner does. The population is narrow because there are no
adopters yet — the population 1.0.0 exists to create.

**Reversibility, honestly.** A code plus a route is additive, so this is **not** a one-way door; it
wins on leg 0's second clause — a hole in a **declared** surface. The cost is still asymmetric: fixing
it post-1.0 is a **behaviour change to a green CI gate**, a red build adopters did not cause.

**Buildable now, verified.** `crates/cli/src/orphan.rs` already enumerates committed markdown from
`git ls-files` and mints `file-state.orphaned-doc` with a route; its strand key needs the doctype to
still exist — that is the missing arm. `engine::validate::schema_version_from_front_matter`
(`validate.rs:1383`) reads a doc's stamp **with no parse and no schema**, so *"a committed `.md`
carrying a jigc stamp at a home no resolved doctype claims"* is computable at HEAD. (`is_unadopted_foreign`
cannot be reused as-is: it takes a `&Schema` that by construction no longer exists.) Scope: one arm
beside `orphaned_docs`, one finding code, a `Human` route (re-add the pack defining `<ty>`, or
`jigc unmanage`), `doc list` printing the row instead of dropping it.

**Verdict: robust-now** — a false green on the one verb we ask adopters to CI on, a deferral trigger
fired by the record's own terms, and one arm over machinery that already ships.
