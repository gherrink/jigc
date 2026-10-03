# M55 — filing the seed

How the M55 seed is filed: every row of the [seed ledger](../seed-ledger.md), re-driven on this build and written as one input directory, then filed through the real binary by **one fan-out into one store** — one milestone, one report sub-task per row, joined by `jigc milestone finalize` ([findings-channel.md](../../../../design/findings-channel.md) → 7 and → 11, flow D; [DECISIONS.md](../../../../DECISIONS.md) → *2026-10-03 — M55 Increment 9 planning*, P4 and P5).

- **`file-seed`** — the driver.
- **`input/<doctype>/<key>/`** — one directory per re-driven row, in the format below. `<doctype>` is `jigc-feedback` or `inconsistency`, and `<key>` is the row's ledger key.

The filed docs are committed under `completions/artifacts/M55/seed/` as `jigc-feedback/` and `inconsistencies/`, the doctypes' own homes, with nothing else beside them, because M56 places that tree under its `docs-root` unchanged.

## Running it

```sh
cargo build    # the rig drives target/debug/jigc
completions/artifacts/M55/seed-filing/file-seed --out "$(mktemp -d)/out" \
    completions/artifacts/M55/seed-filing/input/*/rc16-*
```

`--out` names a directory that does not exist yet. The driver creates it and removes nothing. It needs `bash` (3.2 is enough) and `git`, and it runs from any working directory.

It exits 0 and prints, last of all:

```
filed <N> docs (<a> jigc-feedback, <b> inconsistency) in one commit
```

`--out` then holds `<home>/<slug>.md` for each row, the bytes the join committed. Before that line it prints one `staged …` line per row as each sub-task files it, and one `filed …` line per row naming the doc's path in the rig and in `--out`. Every path it prints is relative, because T7 commits this output as the filing log.

## What it does, and what it asserts

1. **The rig.** It builds `dev/jigc-rig fresh` in two steps (`rig=$(…) || exit; eval "$rig"`), capturing stdout only, and puts `$JIGC`'s directory first on `PATH`, so every `jigc` line it runs is the rig's binary. The rig's construction log is kept in a temp file and shown only if the rig fails.
2. **The fan-out.** It runs `jigc milestone create`, one `jigc milestone add-task <m> <key> --workflow report-<doctype>` per row, then `provision` and `execute`.
3. **Each `Spawn:` line, run verbatim.** The composed text each one prints must:
   - name no `jigc task finalize`;
   - ask for no `commit:<sub>#` write;
   - carry neither omitted step's signature line, *Land this task's docs as exactly one commit. This finalize commits path-scoped:* (`step:finalize-doc-only`) or *finalize renders the commit doc; it does not fill it, so set its header and prose* (`step:author-commit`);
   - end in the `task scope:` trailer naming `` `jigc milestone finalize <m>` is its only commit boundary ``.
4. **Filing, in the sub-task's worktree** (the `Spawn:` line's own `cd` target). Every write is a line the composed author step emitted, with its `<…>` placeholders filled. `doc author` is not used, because its items mint path-coupled ids.
   - The create: `jigc doc create <doctype> --title <TITLE> --task <sub>`, with the `title` file's text. The address it acks names the slug.
   - One `jigc doc set-field <doctype>:<slug>#meta/<leaf> --value <value>` per `fields` line.
   - For each side `n`, in numeric order: `jigc doc add-item …#sides --title "<path or address>" --slug side<n>`, then the `#sides/side<n>/says` set-slot when `says.md` exists.
   - One `jigc doc set-slot <doctype>:<slug>#<slot> --from-file <abs path>` per slot file, in the order `description`, `repro`, `evidence`, `resolution`.
   - The read-back, `jigc doc show <doctype>:<slug> --task <sub>`, must exit 0 and serve the title.

   A leaf or slot the author step does not name gets the same verb line with the address's leaf swapped: the `kind` set-field line for `status`, `tier`, `pinned-by` and `duplicate-of`, and the `description` set-slot line for `resolution`. Any refused write stops the run non-zero, naming the row and printing the binary's own finding. A non-member enum value (`status fixed`) is `write.malformed-value`, and an undeclared leaf is `write.unknown-field`.
5. **The join.** `jigc milestone finalize <m>` must exit 0 and make exactly one new commit whose files are the N docs and the milestone record, and nothing else.
6. **The out tree.** Each doc's committed bytes are copied to `--out/<home>/<slug>.md`.

Before anything is built, every row is checked against the format: its parent names a doctype, its key is a slug that appears only once, it has no file the format does not name, and it has the files the format requires. A row that fails is refused, by name, at exit 2.

## The input format (P4)

One directory per row, `input/<doctype>/<key>/`. Only the binary and the shell parse it.

| File | Doctype | Required | Holds |
|---|---|---|---|
| `title` | both | yes | the doc's title, on one line. The slug is minted from it |
| `fields` | both | yes | one `<leaf> <value>` line per `#meta/<leaf>`. The value is the rest of the line, spaces included |
| `description.md` | both | yes | the `description` slot |
| `repro.md` | `jigc-feedback` | no | the `repro` slot, as a **fenced** block |
| `evidence.md` | `inconsistency` | no | the `evidence` slot |
| `resolution.md` | both | no | the `resolution` slot. A non-`open` status carries one |
| `sides/<n>/title` | `inconsistency` | ≥2 by convention | one side's path or address, filed as item `side<n>` |
| `sides/<n>/says.md` | `inconsistency` | no | what that side says |

**Every input states its `status`** in `fields`, `open` included. `seed_ledger::every_redriven_row_has_an_input_carrying_its_verdict` (`crates/cli/tests/seed_ledger.rs`) holds that line to the ledger's verdict:

- a ledger row with a verdict has an input whose `status` equals it;
- a row without a verdict has no input;
- every input directory is a ledger key under its own doctype.

The field rules — `found-in` at the first source, `jigc-version` the version the row was first seen on, the re-drive's result in `resolution` or `description`, `pinned-by` as `<module>::<test_name>` or `UNPINNED: <why>`, repros rig-relative (`$REPO`, `$JIGC`) — are P3's, stated once in [DECISIONS.md](../../../../DECISIONS.md). `date` is CLI-set when the doc is created and is never an input. After filing, the seed doc is the record and its input is not edited again.

An example `jigc-feedback` row:

```
input/jigc-feedback/rc16-2-defect-c/
  title           Doc show drops the title
  fields          kind bug
                  found-in review:M52-per-axis/(2,DEFECT C)
                  about jigc doc show
                  jigc-version 1.0.0-rc.16
                  tier tier-2
                  status resolved
                  pinned-by doc_show::whole_doc_json_carries_the_title
  description.md
  repro.md
  resolution.md
```

The example is illustrative: its title, fields and test name are not that row's.
