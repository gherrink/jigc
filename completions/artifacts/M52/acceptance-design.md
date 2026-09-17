# M52 — the acceptance design (flow 53 + the per-axis review re-run)

Adopted at the Settle ([settle-record.md](settle-record.md) → D12). Two instruments, on the mold
every wave since M45 has used: an in-repo arm set whose every arm **iterates its class's axis** from
a set it names the kind of, and the per-axis review re-run on the built and installed binary after
the completion audit's fixes.

**The rule for an arm** (flow 46 → 52): an arm that pins the reported repro is not acceptance for a
rule-shaped fix. Each arm below says which kind of set it iterates — a **code-side registry** (⇔- or
count-fenced against the code), the class's **defining case-set** matched exhaustively, a
**derivation stated as one**, or a **manufactured shape space that says it is manufactured** — and
what it asserts per cell. Every arm drives the **real binary** through the shipped verb's entry
point, never a core function below its assembly gate.

## The arms

| # | class (decision) | the set it iterates, and its kind | per-cell assertion |
|---|---|---|---|
| 1 | the rollback family (D1) | **`ROLLBACK_POPULATIONS`** — a code-side registry minted by the wave, count-fenced by a source scan over the restore functions + the two non-function restores | at the door(s) that reach each population, under a deterministic in-transaction racer (a pre-commit hook that edits the restored path/area and exits 1): `FileCas` rows preserve the racer's bytes and emit `<door>.rollback-conflict` on the reject envelope's `findings`; `MintedSet` rows leave the area with the racer's bytes and name it; `DoorGuard`/`Declared` rows are driven at their guard; the JSON reject stream is **one** document at every cell |
| 2 | the posture family (D2) | **`InProgress::ALL` × `BEHALF_DOORS`'s acting members** — a code-side enum × a code-side registry; the git states manufactured by the **new fixture builder** (D12) | every acting door refuses `repo.operation-in-progress` naming the operation, the route's argv is **run** and git accepts it; `task validate` previews the same refusal; no marker is consumed by any door; the `Neither` class stays silent (control) |
| 3 | the destroying subject (D3, D8; amended §6–§8) | **`TASK_AREA_FILES`'s complement over the tree** — a manufactured shape space over the writer-set registry (a foreign regular file at the root, a non-`.md` under `docs/`, a `docs/*.md` that is no staged id, a `.md` outside `docs/`, a nested dir, a dotfile, symlinks; the milestone-area sibling set) × **`DESTROYING_DOORS`** read through its **`Disposition`** axis — the consenting doors × `{without --force, with --force}`, and the two displacing doors (`task finalize`, `milestone finalize` — §18) × their one mode; the refused `task finalize --force` cell is never enumerated | consenting doors refuse (`<door>.foreign-bytes` naming every path) then narrate; `task finalize` and `milestone finalize` move the complement to `.jigc/displaced/<id>/<relative>` and name it on text + the always-present `displaced` key on each landed envelope; the unreadable-root cell holds at every door; `uninstall` over a plain file at an `ENTRIES` name and over a non-empty `displaced/` refuses; jigc's own files never trip the guard (the zero-false-fire control over a full lifecycle incl. `doc rename --task`) |
| 4 | the corpus walk (D4, D7; amended §5, §15) | **the `{location, placement}²` home-pair set** × `{Relocated-only, Relocated+content}` — the class's defining case-set, matched exhaustively on manufactured packs (`--pack-from-dev --schema … --repin`); and for `home-vacated` **the fixed-identity home set** (placement root file · placement file rerooted through `placement-root` · location singleton · a never-had-history control · a moved-and-adoptable file), a derivation from both packs' schemas stated as one | `migrate-corpus` sees and lands the below-version doc in every cell; a missing snapshot blocks at the enumeration (`migrate-corpus.missing-snapshot`, `<ty>@v<k>`); `validate` flips on the un-migrated doc; the `id-from` `ValueRemapped` cell takes `fold-refused` and re-mints no id; `home-vacated` fires on every vacated exact home on the fresh-clone shape and stays silent on the control and on an emptied collection directory |
| 5 | the fixed identity (D5; amended §9, §10, §15) | **the fixed-identity doctype set** (derived from both packs' schemas: `placement.is_some() ‖ singleton`, stated as a derivation) **plus one manufactured `location:` + `singleton: true` doctype** (the shipped set cannot reach the `singleton`-only disjunct) × **`DOCTYPE_DOORS ▸ Address` + `SLUG_DOORS`' `rename` row** | a well-formed non-canonical head refuses `store.fixed-identity` at every door with the canonical address as route, after each door's own schema resolution and behind `store.unknown-type`; `rename --slug` refuses and its route no longer composes the token; the OS-ceiling head refuses with the ceiling code; `doc schema --format json` at contract-version 7 carries `identity: {kind, address}` and `home: {kind, path}` for every doctype and `base` as `{sha, short}` |
| 6 | the envelope and the funnel (D6; amended §3, §12) | **`PRE_DISPATCH_FAULTS`** (minted by the wave: one row per fault with its phase — before-discovery · after-discovery · after-pack-load — and its fixture constructor) × **`VERB_KINDS`** filtered by each leaf's phase reach, with an expected `(fault, verb) → (arm, exit)` column — a code-side registry × a code-side registry under a stated applicability relation; plus **`ENVELOPE_ARMS`** where membership is the assertion | every reachable cell emits **one** JSON document on its expected arm; a reject that carries a finding is on the `{findings, schema_version}` arm with the operational error as a finding (`finalize.commit-rejected` + `<door>.rollback-conflict` under a racing hook); `setup`/`uninstall` reject on the findings arm; `doc show`'s seven root shapes each match a declared row; the debug-posture seam suite for the `Route` span fence (lead 1) |
| 7 | the composed doors (D9) | **the verb-routed workflow set** — derived from both packs' `suppressed` blocks (a derivation stated as one) × the two compose doors; and the empty-enumeration set from the pack's `{{@…}}` placeholders | `start --workflow`/`workflow --preview` refuse `workflow.verb-routed` with the real door as a runnable route; a legitimately-empty render states the empty case; plain `task finalize` on a source-less migrate-shaped task holds at exit 4 |

**Amended 2026-09-17** at the design review ([settle-record.md](settle-record.md) → Review amendments §7, §12, §15): arms 3–6 rewritten as above — arm 3's product no longer names a cell the record refuses, arm 4 and arm 5 iterate their home and identity sets rather than one repro, arm 6 iterates a registry with an applicability relation rather than an unrealizable product.

**Deliberately unrepresented:** D10's and D11's surface batches and the record corrections mint no
verb, finding or route a done-picture walk needs an arm for beyond the cells above; on the M46 Inc 9
/ M48 Inc 11 / M49 Inc 12 / M51 Inc 9–11 precedent they are not given a manufactured arm.

## The per-axis review re-run

The instrument is preserved at [M51/per-axis-review/instrument/](../M51/per-axis-review/instrument/)
— the eight Codex prompts and the Workflow script. Re-pointed at `completions/artifacts/M52/per-axis-review/`
and the expected `--version` `1.0.0-rc.16`, staffed as before (one Opus driver · one Codex source pass
· one reconciler per axis), driven on the **installed** binary after the completion audit's fixes.
**The comparison is row by row against M51's**: every §A row of M51's ledger is re-driven and
recorded CLOSED (with the argv) or STILL-OPEN (with the datum); every new finding is tiered on the
charter's predicate; the coverage table is diffed (no leaf may lose an axis).

## Spikes owed before the arms are written

The advocates drove the load-bearing claims (the settle record's posture note lists them). Two
remain **relayed** and are spiked at the first increment that touches them: the `unwind_mint`
`ENOTEMPTY` idiom's behaviour on macOS vs Linux (`remove_dir` on a non-empty dir returns
`ENOTEMPTY` on both — verify in CI), and the `OnceLock<Format>` seat's reach for `pack.rs`'s two
warnings (the seat is proven for `refuse_on_posture`; the warnings' call order relative to
`try_parse()` is a read).

## Bounds

The arms are headless by construction: no genuine concurrent process races a population (the hook
is the deterministic racer the design names), and the fan-out's live Task-tool spawn stays the
orchestrator's main-session artifact. `chmod 000` cells declare the CI platform bound (they pass
vacuously as root). The git-state cells are git-2.54.0's on-disk contract.
