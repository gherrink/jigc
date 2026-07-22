# Pinning — how a verified fact stays verified

The foundation for the M45 testing-infrastructure scope (the complete-fix contract's buildable half — [decisions-pending.md](decisions-pending.md) → the rc.9 wave; evidence base [RC-alpha3/findings-verification.md](../completions/artifacts/RC-alpha3/findings-verification.md) → §4). **Definitions and load-bearing decisions only — no code ships with this doc.** The M45 planning session decomposes from here; anything marked *(Settle-dependent)* waits for its fork and must not be pre-built.

**The problem, one line:** facts established by hand — an audit driving the binary, a trial triage, a wave re-verify — were observed once and held by nothing, and later waves moved them silently (the address grammar, the exit-code taxonomy, JSON purity; the commit round-trip exemption is the same hole in fixture form). Three drift modes, three mechanics: **contract drift** → property suites, **surface drift** → compose goldens, **fact drift** → repro blocks. The fourth piece (the fixture builder) is the shared substrate the first two run over.

## 1. The compose-golden suite (surface drift)

**Claim it pins:** the composed surface — everything `start`/`workflow --preview`/`describe`/`doc schema` print — changes only when someone *means* it to, and a pack edit's blast radius is a reviewable diff, not an invisible propagation.

**Why it is cheap:** composition is deterministic by core invariant (same resolved cascade in → same workflow out), so the entire surface is snapshottable. The goldens are generated, never authored.

Load-bearing decisions:

- **Enumeration comes from the registries, never a hand list** (the axis principle; *a grep is not a fence*). The workflow set and doctype set are read at test runtime from the loaded packs (the same enumeration `describe` projects) — a workflow added to any pack, or a whole new pack composed in, joins the sweep with zero test edits.
- **Surfaces swept per member:** `jigc workflow <id> --preview` for every workflow (the mint-free compose — M44); `jigc start --workflow <id>` for the `creates-task` set where minting is part of the surface; `jigc describe` (whole catalog); `jigc doc schema <type> --format json` + human form for every doctype. Each × each fixture state from §4 where the state changes the output (the form-vision trap was state-dependent; a fresh-repo-only sweep re-inherits the create-fresh blind spot).
- **Normalization is minimal and explicit:** absolute repo paths → a `<REPO>` token; nothing else. Task ids are slug-derived (deterministic given the fixture intent), compose embeds no timestamps. If a surface turns out to embed something genuinely nondeterministic, that is a *finding* (reproducibility of structure is the product claim), not something to normalize away silently.
- **Golden layout:** `crates/cli/tests/goldens/compose/<pack>/<surface>--<member>--<state>.txt`. **Regen is one step** — `UPDATE_GOLDENS=1 cargo test -p cli --test compose_goldens` — and the regenerated diff is part of the commit under review. Goldens are for *noticing*, not forbidding: a red golden without regen means "you changed a printed surface without looking at what else changed"; the answer is to look, then regen, never to hand-edit a golden.
- **Byte goldens, not structural assertions.** The judgment tier (prose wording) is exactly what has no other fence — a structural assertion would exempt it again. The style-guide review still governs *quality*; the golden governs *awareness of change*.

**What this would have caught at the introducing commit:** the 3×-repeated batch caveat (M44's census-fix composed three copies into one walk), the degenerate `milestone-execution` walk (zero Spawn lines), the heading-floor statement rendering into the planning workflow.

## 2. The contract property suites (contract drift)

**Rule of shape:** one test file per contract, **named after the contract doc it pins** (`command_output_contract.rs` ↔ `design/command-output-contract.md`), so revising the doc and breaking its fence are visibly the same event. Each suite iterates a *real registry*, so new members join automatically.

- **Address grammar** (`doc_read_surface.rs` + write-side): the registry is `jigc doc schema <type> --format json` itself — since contract-version 3 the projection advertises the settable write addresses, so the suite round-trips **every address the tool's own projection advertises** (set → show → resolve parity), for every doctype in both packs, plus the documented alias forms (bare single-hop `#type` ↔ `#header/type`, bare singleton addresses, `{#id}` anchors). Self-referential on purpose: the projection is the registry, so a projection that advertises a dead address goes red — law 1 fenced at the test layer. *(Pins the rc.5-verified fact that drifted un-fenced, and the refuted `#type` claim from the project-alpha-3.0 trial.)*
- **Exit-code taxonomy** (`exit_codes.rs`): for every verb family, provoke each outcome class (success · usage error · blocking finding at a task gate · store-scope exit-flip · review hold) and assert the severity→exit mapping against **one table constant**, which also becomes the source the AGENT.md preload line is asserted against (statement == constant, and the constant is context-parameterized — the M43-corollary lesson). *(Settle-dependent: fork 3 decides whether the write-verb reject aligns to 3 or the one-liner narrows; the suite pins whichever wins — build it immediately after that fork, not before.)*
- **JSON purity** (`machine_output.rs`): every verb's `--format json` stdout parses as exactly **one** JSON document — verbs enumerated programmatically from the clap `Command` tree (the cli crate links into its own tests; no hand list) — including under the chatty-hook fixture state and at the clap-error surface (the two known cracks). *(Settle-dependent only for the `hook_output`-field fix it verifies; the purity property itself is contract-stable.)*
- **The commit round-trip repair** rides with the trailer fix (tier 1), not here: commit joins the on-disk round-trip fixture suite, and one finalize-path test authors a trailer **through the write verbs** and asserts it in `%(trailers)`. Named in this doc because the *exemption* was the pinning hole.

## 3. The repro-block pipeline (fact drift)

The delivery-format rule is landed convention ([milestone-completion-workflow.md](milestone-completion-workflow.md) → Audit): every CONFIRMED **and REFUTED** verdict arrives with a repro block. This section pins the block's shape and the conversion discipline.

**Block shape** (illustrative — flag: notation not frozen):

```yaml
claim: "set-field commit:<t>#type errors (bare single-hop form)"
verdict: REFUTED
setup:            # throwaway repo state, builder state name or explicit steps
  - fixture: fresh          # a §4 state name, or argv steps
  - ["jigc", "start", "probe intent"]
repro:
  - ["jigc", "doc", "set-field", "commit:<task>#type", "--value", "docs", "--task", "<task>"]
expect:
  exit: 0
  stdout_json: { "findings": [] }
pinned-by: doc_read_surface::bare_single_hop_alias_resolves   # or "UNPINNED: <why>"
```

**Conversion discipline:** confirmed claims' blocks become the fix's red test (the dev workflow already demands this — no change); **refuted claims' blocks are the new obligation** — each becomes a standing test in the wave's fix increments, in a `pinned_facts/` module per domain, doc-comment citing the findings-verification row it pins (provenance walkable both ways). The `pinned-by:` field is filled when the test lands; an audit finds any block still `UNPINNED` without a stated reason. **No script-runner harness:** blocks are converted to Rust tests by hand (they're small), not executed as YAML — a bash/YAML runner is a second test framework to maintain and a platform-fragility source; the block is a *specification for a test*, not a test format.

## 4. The trial-shaped fixture builder (shared substrate)

One test-support builder (`crates/cli/tests/support/trial_corpus.rs` or equivalent) producing named corpus states, **by driving the built binary** — never by writing files directly — so provenance (`edited-from-base`, staged copies, index state) is real, not simulated.

**The initial state set** (each is a state a real trial hit and a synthetic fixture missed):

| state | contents | blind spot it closes |
|---|---|---|
| `fresh` | `jigc setup` only | baseline |
| `committed-singletons` | vision/roadmap/decisions-log created + finalized | the create-over-committed / copy-in paths |
| `migrated` | docs landed via `migrate --approve` (edited-from-base provenance, retired sources) | the form-vision-class traps; fidelity paths |
| `refs-post-hoc` | edges set by `set-field` on committed docs | role-binding, index overlay paths |
| `chatty-hooks` | an active pre-commit emitting stdout on success | JSON purity under the merged-stream reality |
| `vendored` | a gitignored runtime dir + tracked code | worktree provisioning, ingest funnel |

**Growth rule:** when a trial surfaces a new load-bearing corpus state, it is added *here* (one place), upgrading every suite that iterates states — the axis card applied to test substrate. The golden suite and property suites take states by name; a new state auto-joins any suite that iterates "all states."

## Sizing + decomposition hints (for the planning session, non-binding)

The infrastructure is **3–5 increments**, mostly test code: (a) the fixture builder first — everything else consumes it; (b) the golden harness + first full golden generation; (c) the address-grammar + JSON-purity suites; (d) the exit-code suite immediately after Settle fork 3; (e) repro-block conversion rides the fix increments, not its own. The volume risk in M45 is **not here** — it is the four fired-trigger Settle forks; the charter's routing rule (capability-minting fork outcomes go to M46, not M45) is what keeps this wave M42-sized. Sequencing constraint worth honoring at decomposition: the tier-1 defect fixes' axis-iterating acceptance tests want (a) and parts of (b) in place, so the infrastructure increments lead, fixes follow.

**Deliberately out:** a YAML-repro runner (see §3); structural/schema-level golden assertions (see §1); any pre-building of Settle-dependent pieces; the jigc-native fact-ledger doctype (post-1.0 — [ideas/finding-doctype.md](../ideas/finding-doctype.md), 2026-07-22 addendum).
