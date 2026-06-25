#!/usr/bin/env bash
# drive.sh — M34 corpus-migration detect→block→migrate loop, observed on the
# ACTUALLY-PINNED `jigc` binary (NOT `cargo test`), over a real dev-pack corpus in
# a scratch git repo. The runtime sibling of
# crates/cli/tests/corpus_migration.rs, run on the pinned binary with the
# PRODUCTION probe resolution (NO JIGC_DOC_CODE_PROBE — the `doc-code` probe is
# resolved as the sibling beside the installed `jigc`). Logs land alongside this
# script. Design: corpus-migration.md → Acceptance flows; worked-examples.md →
# flow 35.
#
# The headline dogfood — one corpus, three sequential steps (the loop):
#   1 · DETECT     — a committed v0 ADR (no schema-version stamp) is reported by
#                    `jigc validate` as a `schema-conformance` break routed
#                    `migrate` (below the current schema-version), report-only at
#                    store scope (exit 0).
#   2 · MIGRATE    — `jigc migrate-corpus` stamps the corpus byte-stable via the
#                    real added-optional-field transform (the live add-field e2e:
#                    `schema-version: 1` spliced into the existing header, prior
#                    fields + body prose preserved), exit 0.
#   3 · RE-DETECT  — `jigc validate` now finds the corpus conformant + stamped v1:
#                    no `schema-conformance` finding, no `migrate` route, exit 0.
#
# Plus the false-positive guard:
#   4 · NO-OP      — a corpus already stamped at the current version is left
#                    byte-identical by `jigc migrate-corpus` (0 migrated, the doc
#                    reported already current).
set -u

HERE="$(cd "$(dirname "$0")" && pwd)"

# Resolve the PINNED binary from PATH; print its identity for the record.
JIGC="$(command -v jigc)"
echo "jigc on PATH: $JIGC"
sha256sum "$JIGC"

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

# A throwaway git repo with identity + one commit, then `jigc setup` over it (the
# production probe sibling is resolved beside the pinned binary — no override).
mk_repo() {
  local r="$1" h="$2"
  mkdir -p "$r" "$h"
  git -C "$r" init -q
  git -C "$r" config user.email t@e.com
  git -C "$r" config user.name T
  echo hello > "$r/README.md"
  git -C "$r" add .
  git -C "$r" commit -qm init
  ( cd "$r" && env -u JIGC_DOC_CODE_PROBE HOME="$h" "$JIGC" setup ) >/dev/null 2>&1
}

# Commit an `adr` at its canonical docs/decisions/<slug>.md. $4 = stamp version,
# empty = the unstamped v0 state.
commit_adr() {
  local r="$1" slug="$2" title="$3" stamp="${4:-}"
  local stamp_line=""
  [ -n "$stamp" ] && stamp_line="schema-version: ${stamp}"$'\n'
  mkdir -p "$r/docs/decisions"
  {
    printf -- '---\n'
    printf 'status: accepted\n'
    printf 'date: 2026-06-25\n'
    printf '%s' "$stamp_line"
    printf -- '---\n\n'
    printf '# %s\n\n' "$title"
    printf '## Context\n\nSession lookups must stay sub-millisecond.\n\n'
    printf '## Decision\n\nKeep sessions in a single in-memory node.\n\n'
    printf '## Consequences\n\nA cold node loses its sessions.\n'
  } > "$r/docs/decisions/${slug}.md"
  git -C "$r" add .
  git -C "$r" commit -qm "seed adr" >/dev/null
}

run() { # run <repo> <home> <logfile> <jigc args...>
  local r="$1" h="$2" log="$3"; shift 3
  ( cd "$r" && env -u JIGC_DOC_CODE_PROBE HOME="$h" "$JIGC" "$@" ) > "$log" 2>&1
  echo "exit: $?" | tee -a "$log"
}

# ---- The dogfood loop: one v0 corpus, detect → migrate → re-detect ----------
repo="$scratch/dogfood"; home="$scratch/home"; mk_repo "$repo" "$home"
commit_adr "$repo" alpha-decision "Alpha decision"   # v0: no stamp

echo "--- ADR before migration ---"  > "$HERE/adr-before.log"
cat "$repo/docs/decisions/alpha-decision.md" >> "$HERE/adr-before.log"

run "$repo" "$home" "$HERE/detect.log"     validate
run "$repo" "$home" "$HERE/migrate.log"    migrate-corpus

echo "--- ADR after migration ---"   > "$HERE/adr-after.log"
cat "$repo/docs/decisions/alpha-decision.md" >> "$HERE/adr-after.log"

run "$repo" "$home" "$HERE/revalidate.log" validate

# ---- The no-op guard: a corpus already at the current version ---------------
repoN="$scratch/current"; homeN="$scratch/homeN"; mk_repo "$repoN" "$homeN"
commit_adr "$repoN" beta-decision "Beta decision" 1   # already v1
sha_before="$(sha256sum "$repoN/docs/decisions/beta-decision.md" | cut -d' ' -f1)"
run "$repoN" "$homeN" "$HERE/noop.log" migrate-corpus
sha_after="$(sha256sum "$repoN/docs/decisions/beta-decision.md" | cut -d' ' -f1)"
echo "beta-decision.md sha256 before: $sha_before" | tee -a "$HERE/noop.log"
echo "beta-decision.md sha256 after:  $sha_after"  | tee -a "$HERE/noop.log"
[ "$sha_before" = "$sha_after" ] \
  && echo "BYTE-IDENTICAL: current corpus left untouched" | tee -a "$HERE/noop.log" \
  || echo "MISMATCH: current corpus was rewritten" | tee -a "$HERE/noop.log"
