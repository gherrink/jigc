# M30 — the declared change-set contract (finalize commits only what's staged, surfaces the rest)

**Run 2026-06-20.** The M30 done-bar mandated by [worked-examples.md](../../../design/worked-examples.md)
→ flow 32, [finalize.md](../../../design/finalize.md) → Dirty-tree policy (revised M30),
[DECISIONS.md](../../../DECISIONS.md) → 2026-06-20 M30 Increment 4 planning (G1–G6), and
[roadmap.md](../../../implementation/roadmap.md) → M30 Increment 4 (T4). M30 makes per-task
`finalize` commit **exactly the declared change-set** — the agent's git **index** plus jigc's own
promoted/config files — and **surface everything else** as a named left-out set, never sweeping
unrelated dirty paths into the commit. The narrowing itself shipped across Inc 1–3
(`StagePolicy::IndexHonoring` + the left-out manifest + the **index-validated** `doc-code` gate); Inc 4
closes the loop — G5 instructs the agent to `git add` its edits and lets the Claude-Code adapter permit
it, and flow 32 proves the closed loop end-to-end. This artifact records the **three measured facts of
flow 32**, each **observed by running the ACTUALLY-PINNED `cargo install`-built binary** (NOT
`cargo test`) over real git repos with **no environment overrides** — the production path a real install
hits, now with the G5 pack embedded.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `43f06ab131aa5a7fec171825042adb09dcf91b700aabf9728ed03492031bc8b5` |
| `doc-code` probe sha256 | `530dd89f32d7c6a6afbb617b7672744897316d9ecb243a6cd06710ec22302398` |
| HEAD commit | Built at `427475f` (M30 milestone-audit fix — the manifest renders a `git add`-ed new file as `added`, not the retired `swept (was untracked)`; the cold-start install-footprint fix landed `f38d2e1`, the G5 agent-stages pack Inc-4 T1, the narrowing logic Inc 1–3). Both audit fixes changed the `jigc` binary, so this run **re-pins** the binary and **refreshes** the captured facts (the old `aa5bda9…` sha and the stale `swept (was untracked)` evidence are retired). This re-pin commit adds **only** `completions/artifacts/M30/` (this file + `evidence/`) — no Rust source change — so the pinned binary stays **byte-identical to a fresh `cargo install` at final HEAD**. |
| Built | `cargo install --path crates/cli --force` (release; `build.rs` builds the eight-grammar `doc-code` probe — Rust + TypeScript/TSX + JavaScript/JSX + Python + PHP + bash + CSS + YAML — and embeds it via `OUT_DIR/doc-code`, plus the now-G5 dev pack); `rustc 1.95.0` |
| Pinned to | `jigc` **and** its `doc-code` probe sibling to **both** `~/.local/bin/` and `~/.cargo/bin/`. `cargo install` placed `jigc` at `~/.cargo/bin/jigc`; it was copied to `~/.local/bin/jigc`; a `jigc setup` run from **each** bin dir confirmed that dir's `doc-code` sibling (the extract is a **no-op** when byte-identical — the audit fixes changed only `jigc` source, not the `doc-code` probe's grammar set, so the sibling stays the M29 eight-grammar probe). **All four sha256 identical** — `jigc` identical across both dirs (`43f06ab…`), `doc-code` identical across both dirs (`530dd89…`). `which jigc` → `~/.local/bin/jigc`, `which doc-code` → `~/.local/bin/doc-code` (a pinned dir). |
| Fresh-build cross-check | The relocated `jigc` is **byte-identical** to the freshly-`cargo install`-built `target/release/jigc` (same `43f06ab…`); the `doc-code` siblings are byte-identical to the `build.rs`-embedded `OUT_DIR/doc-code` and to `target/release/doc-code` (`530dd89…`) — i.e. the pinned binaries match a fresh `cargo install` at this HEAD. |
| Size | release `jigc` **13.69 MB** (13 688 808 B), `doc-code` **8.49 MB** (8 486 360 B). The two audit fixes (the cold-start install footprint + the `added`-manifest tag) left `jigc` essentially unchanged from M30 Inc-4's 13.69 MB; `doc-code` is unchanged from M29 (the probe binary didn't change). The size guard's **90 MiB** ceiling ([`crates/cli/tests/cargo_install_probe.rs`](../../../crates/cli/tests/cargo_install_probe.rs)) protects the **debug** `CARGO_BIN_EXE_jigc` (**71.87 MiB**, 75 358 152 B); still well under. |
| Invocation | `jigc` from `PATH`, with the **embedded** dev pack (**no** `JIGC_PACK_DIR`) and the `doc-code` probe resolved **as the sibling beside the installed binary** (**no** `JIGC_DOC_CODE_PROBE` — the driver `unset`s both) — the production probe-resolution path (the M20 embed/sibling). |
| Driver | [`evidence/drive.sh`](evidence/drive.sh) — two detached scratch repos + isolated `$HOME` per arm; logs in [`evidence/`](evidence/). |

## Honest posture — protocol-observed, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23/M24/M25/M26/M27/M28/M29):
this is an **owner-artifact recording** (the recorded-alongside measure run on the actually-pinned
binary), **not** a TDD red→green test — its done-criterion is this committed artifact plus the clean
end-to-end runs on the installed binary. The narrowing, the left-out manifest, and the index-validated
`doc-code` gate are independently **binary-proven** by
[`crates/cli/tests/flow32_acceptance.rs`](../../../crates/cli/tests/flow32_acceptance.rs) (Inc-4 T3)
against this same binary's source. This artifact's job is to show those facts *on the binary a real
install actually resolves*, with the production sibling probe and no overrides.

## Corpus — two real git repos (mirrors flow 32 / `flow32_acceptance.rs`)

Two throwaway `git init` repos, each with one initial commit + the `.jigc/config/` project layer (the
embedded pack provides the workflows — no `jigc setup` of the corpus repo):

- **scope arm** — `jigc start --workflow single-task`, then a *staged* task edit (`feature.rs`, `git
  add`ed), an unrelated *untracked* file (`scratch.txt`), and an unrelated *unstaged-modified tracked*
  file (`README.md`), then `jigc task finalize scope-the-set`.
- **block arm** — `jigc start --workflow architecture-documentation`, an `arch-doc` whose one component
  anchors `widget.rs#render_widget`; the agent appends `render_widget` to the tracked `widget.rs` but
  does **not** `git add` it (an unrelated `other.rs` *is* staged so the narrowed set is non-empty), then
  `jigc --format json task finalize document-the-gateway`.

Full logs per arm in [`evidence/`](evidence/).

## The three measured facts — each observed on the installed binary

All three observed by running `~/.local/bin/jigc` (sha256 `43f06ab…`) from `PATH`, **no
`JIGC_PACK_DIR`, no `JIGC_DOC_CODE_PROBE`**, resolving the `doc-code` sibling (`530dd89…`) beside it.

### Fact 1 — finalize commits only the declared change-set ([`evidence/scope.log`](evidence/scope.log))

Pre-finalize `git status --porcelain`: `A  feature.rs` (staged), ` M README.md` (unstaged-modified
tracked), `?? scratch.txt` (untracked), `?? .jigc/` (jigc's own). `jigc task finalize scope-the-set`:

```
finalized 28bb9d8 — feat(cache): scope the declared change set
  added .jigc/.gitignore
  added feature.rs
  2 files committed
```

`FINALIZE_EXIT=0`, HEAD **delta +1** (exactly one commit). `git show --name-only HEAD` carries
**only** `feature.rs` (the staged task edit) and `.jigc/.gitignore` (jigc's own config file) — **not**
`scratch.txt`, **not** `README.md`. `git show HEAD:README.md` is the unedited baseline (`hello`): the
unrelated local edit never landed. The staged index plus jigc's own files committed; everything else
stayed in the working tree. The manifest now tags both included files **`added`** (the post-audit
`427475f` wording — a `git add`-ed new file, porcelain `A`, is an *included* member, not the retired,
inaccurate `swept (was untracked)`).

### Fact 2 — finalize names the left-out set ([`evidence/scope.log`](evidence/scope.log))

The same finalize output carries a **left-out** section naming exactly the two unrelated dirty paths:

```
  left-out (unstaged/untracked — git add to include):
    README.md
    scratch.txt
```

Post-finalize `git status --porcelain` confirms both **remain** uncommitted (` M README.md`,
`?? scratch.txt`) — the agent's unstaged-modified tracked file and unrelated untracked file are
surfaced, not swept and not forbidden (the "surfaced, not prevented" B1 policy).

### Fact 3 — a dangling citation blocks finalize ([`evidence/block.log`](evidence/block.log))

The agent appended `render_widget` to the working tree's `widget.rs` but did **not** `git add` it, so
the **index** copy (`git show :widget.rs`) still carries only `placeholder`. The finalize-scope
`doc-code` probe validates the **materialized git index** (M30 Inc 3, G4), so the cited symbol is
absent. `jigc --format json task finalize document-the-gateway`:

```json
"severity": "blocking",
"probe": "doc-code",
"check": "symbol-exists",
"code": "doc-code.symbol-exists",
"message": "anchor `widget.rs#render_widget` resolves to no symbol (`render_widget` is absent from `widget.rs`)",
"location": { "address": "arch-doc:gateway#components/widget/implemented-by", "line": 1, "col": 1 }
```

`FINALIZE_EXIT=3`, HEAD unchanged (delta 0), `gateway.md` **ABSENT** (nothing promoted). The block names
the citing item's anchor address and the dangling anchor target. No `pack-probe-integrity` meta-finding
surfaced — the installed `jigc` found a runnable `doc-code` sibling beside itself
(`~/.local/bin/doc-code`) through the production default (`current_exe().parent()/doc-code`, no override)
and ran the probe over the **index**, not the working tree: validated reality == committed reality.

## Bounds (carried in honestly)

- **n = 2 arms (scope, block)** over hand-built fixtures — a done-bar shape check, not a distribution.
  The narrowing, the left-out manifest, and the index-validated `doc-code` block are binary-proven by
  `flow32_acceptance.rs` against this same binary; this artifact is the *observed-on-the-installed-binary*
  shadow.
- **Owner-artifact recording**, not a CLI-emitted metric — the recorded-alongside posture. Only the
  finalize exit / landed change-set / promotion is the binary's own yes/no; the left-out reading is the
  owner's inspection of the emitted output.
- **Per-task `IndexHonoring`, not the milestone `Sweep`** — M30 narrows the *per-task* `finalize` to the
  declared change-set; the milestone-level `Sweep` policy is the M31 seam (the M30/M31 divergence
  recorded in `finalize.md`). This artifact measures only the per-task path.
- **The scratch repos + `$HOME`s under `/tmp` are detached and discarded**; no developer repo was
  mutated. jigc's own source was not modified.

## Verdict — done-bar met

Per-task `finalize` commits **exactly the declared change-set** and **surfaces the rest** —
**measured on the actually-pinned, `cargo install`-built binary** (`jigc` `43f06ab…`, `doc-code`
`530dd89…`, both identical across `~/.local/bin` + `~/.cargo/bin` and matching a fresh build at HEAD),
resolving the probe through the **production sibling path** with **no env overrides**. (1) The commit
carries **only** the staged task edit + jigc's own files, never the unrelated untracked or
unstaged-modified tracked paths (`HEAD:README.md` is the unedited baseline); (2) the emitted output
**names the left-out set** (`README.md`, `scratch.txt`), which **remain** uncommitted post-finalize; and
(3) a citation the agent wrote but did **not** stage **blocks** finalize (exit 3, HEAD unchanged, nothing
promoted) on `doc-code.symbol-exists` — the probe validates the materialized index, so validated reality
equals committed reality. **The closed loop ships. The M30 done-bar is met.**
