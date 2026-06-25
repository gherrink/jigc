#!/usr/bin/env bash
# structural-drive.sh — M34 Increment 4: the v1->v2 STRUCTURAL corpus migration
# (the reconstructed M25 `prd.requirements` fixed-slot -> repeatable reshape),
# observed on the ACTUALLY-PINNED `jigc` binary (NOT `cargo test`), over a real
# dev-pack corpus in a scratch git repo. The runtime sibling of
# crates/cli/tests/flow36_corpus_structural.rs, run on the pinned binary with the
# PRODUCTION probe resolution (NO JIGC_DOC_CODE_PROBE — `doc-code` is resolved as
# the sibling beside the installed `jigc`). The prior shapes are sourced from a
# `FilesystemPack` fixture (JIGC_PACK_DIR) carrying the versioned
# `schema-snapshots/<type>.v<N>.yaml` store. Logs land alongside this script.
# Design: corpus-migration.md → Prior-schema sourcing / Acceptance flows;
# worked-examples.md → flow 36.
#
# The headline structural loop — one v1 corpus, detect -> migrate -> re-validate:
#   1 · DETECT     — two committed v1-stamped FIXED-SLOT prd docs (below the bumped
#                    manifest version 2) are reported by `jigc validate` as
#                    `schema-conformance` breaks routed `migrate` (report-only,
#                    exit 0). The v1 adr stays conformant under the WIDENED card,
#                    so it is not flagged here — the verb still migrates it (the
#                    value-bump), proven in step 2.
#   2 · MIGRATE    — `jigc migrate-corpus` sources each prior shape from the
#                    snapshot store keyed on the doc's stamp, applies the
#                    fixed-slot -> repeatable-with-default splice (the old slot
#                    prose preserved as the default first item), VALUE-BUMPS the
#                    stamp 1->2, gates each doc on conformance, writes back
#                    byte-stable (exit 0). The widened-cardinality adr migrates
#                    byte-identical-EXCEPT-stamp.
#   3 · RE-VALIDATE — the migrated corpus is v2-conformant: no `schema-conformance`
#                    finding, no `migrate` route, exit 0.
#   4 · IDEMPOTENT — a re-run is a byte-untouched no-op (the now-v2 docs read
#                    at-version and are skipped).
#   5 · DETERMINISM — the same two-prd corpus committed in id-order and in REVERSE
#                    migrates to BYTE-IDENTICAL output (increment-workflow #7).
set -u

HERE="$(cd "$(dirname "$0")" && pwd)"
SRC="$(cd "$HERE/../../../.." && pwd)"   # repo root (completions/artifacts/M34/evidence -> root)
EMBEDDED_PACK="$SRC/crates/cli/pack"
SNAPS="$SRC/crates/cli/tests/fixtures/corpus-structural/schema-snapshots"

# Resolve the PINNED binary from PATH; print its identity for the record.
JIGC="$(command -v jigc)"
echo "jigc on PATH: $JIGC"
sha256sum "$JIGC"

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

# ---- Build the v1->v2 FIXTURE PACK ------------------------------------------
# Clone the shipped dev pack (so setup + every command path behaves exactly like
# the real pack), overlay the two prior-schema snapshots, and bump prd + adr to
# manifest version 2 so their committed v1-stamped docs read as below-version.
PACK="$scratch/pack"
cp -r "$EMBEDDED_PACK" "$PACK"
cp -r "$SNAPS" "$PACK/schema-snapshots"
manifest="$PACK/config/schema-manifest.yaml"
perl -0pi -e 's/  - type: prd\n    schema-version: 1/  - type: prd\n    schema-version: 2/' "$manifest"
perl -0pi -e 's/  - type: adr\n    schema-version: 1/  - type: adr\n    schema-version: 2/' "$manifest"
grep -q "  - type: prd"$'\n'"    schema-version: 2" "$manifest" \
  && grep -q "  - type: adr"$'\n'"    schema-version: 2" "$manifest" \
  && echo "fixture manifest: prd + adr bumped to schema-version 2" \
  || { echo "FATAL: manifest bump failed"; exit 1; }

# A throwaway git repo with identity + one commit, then `jigc setup` over it with
# the fixture pack (the production probe sibling is resolved beside the pinned
# binary — no override).
mk_repo() {
  local r="$1" h="$2"
  mkdir -p "$r" "$h"
  git -C "$r" init -q
  git -C "$r" config user.email t@e.com
  git -C "$r" config user.name T
  echo hello > "$r/README.md"
  git -C "$r" add .
  git -C "$r" commit -qm init
  ( cd "$r" && env -u JIGC_DOC_CODE_PROBE HOME="$h" JIGC_PACK_DIR="$PACK" "$JIGC" setup ) >/dev/null 2>&1
}

# Commit a v1-stamped FIXED-SLOT prd at docs/prds/<slug>.md (the byte form a
# committed v1 prd had before the M25 repeatable reshape).
commit_prd() {
  local r="$1" slug="$2" title="$3" req="$4"
  mkdir -p "$r/docs/prds"
  {
    printf -- '---\n'
    printf 'schema-version: 1\n'
    printf -- '---\n\n'
    printf '# %s\n\n' "$title"
    printf '## Vision\n\nA fast, predictable system.\n\n'
    printf '## Requirements\n\n%s\n\n' "$req"
    printf '## Context\n\nLatency budgets are tight and the team is small.\n'
  } > "$r/docs/prds/${slug}.md"
  git -C "$r" add .
  git -C "$r" commit -qm "seed prd" >/dev/null
}

# Commit a v1-stamped adr (the supersedes 0..1 prior shape — carries no
# supersedes, so valid under both 0..1 and the widened 0..*).
commit_adr() {
  local r="$1" slug="$2" title="$3"
  mkdir -p "$r/docs/decisions"
  {
    printf -- '---\n'
    printf 'status: accepted\n'
    printf 'date: 2026-06-25\n'
    printf 'schema-version: 1\n'
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
  ( cd "$r" && env -u JIGC_DOC_CODE_PROBE HOME="$h" JIGC_PACK_DIR="$PACK" "$JIGC" "$@" ) > "$log" 2>&1
  echo "exit: $?" | tee -a "$log"
}

# ---- The structural loop: one v1 corpus, detect -> migrate -> re-validate ----
repo="$scratch/structural"; home="$scratch/home"; mk_repo "$repo" "$home"
commit_prd "$repo" cache-prd "Cache Prd" "The cache must answer in under a millisecond."
commit_prd "$repo" queue-prd "Queue Prd" "The queue must never drop an enqueued job."
commit_adr "$repo" alpha-decision "Alpha Decision"

echo "--- prd (cache-prd) before migration ---" > "$HERE/structural-prd-before.log"
cat "$repo/docs/prds/cache-prd.md" >> "$HERE/structural-prd-before.log"
echo "--- adr (alpha-decision) before migration ---" > "$HERE/structural-adr-before.log"
cat "$repo/docs/decisions/alpha-decision.md" >> "$HERE/structural-adr-before.log"

run "$repo" "$home" "$HERE/structural-detect.log"  validate
run "$repo" "$home" "$HERE/structural-migrate.log" migrate-corpus

echo "--- prd (cache-prd) after migration ---" > "$HERE/structural-prd-after.log"
cat "$repo/docs/prds/cache-prd.md" >> "$HERE/structural-prd-after.log"
echo "--- adr (alpha-decision) after migration ---" > "$HERE/structural-adr-after.log"
cat "$repo/docs/decisions/alpha-decision.md" >> "$HERE/structural-adr-after.log"

run "$repo" "$home" "$HERE/structural-revalidate.log" validate

# ---- Idempotence guard: a re-run is a byte-untouched no-op ------------------
sha_prd_before="$(sha256sum "$repo/docs/prds/cache-prd.md" | cut -d' ' -f1)"
sha_adr_before="$(sha256sum "$repo/docs/decisions/alpha-decision.md" | cut -d' ' -f1)"
run "$repo" "$home" "$HERE/structural-idempotent.log" migrate-corpus
sha_prd_after="$(sha256sum "$repo/docs/prds/cache-prd.md" | cut -d' ' -f1)"
sha_adr_after="$(sha256sum "$repo/docs/decisions/alpha-decision.md" | cut -d' ' -f1)"
{
  echo "cache-prd.md      sha256 before re-run: $sha_prd_before"
  echo "cache-prd.md      sha256 after  re-run: $sha_prd_after"
  echo "alpha-decision.md sha256 before re-run: $sha_adr_before"
  echo "alpha-decision.md sha256 after  re-run: $sha_adr_after"
  [ "$sha_prd_before" = "$sha_prd_after" ] && [ "$sha_adr_before" = "$sha_adr_after" ] \
    && echo "BYTE-IDENTICAL: re-run is a no-op (idempotent)" \
    || echo "MISMATCH: re-run rewrote a migrated doc"
} | tee -a "$HERE/structural-idempotent.log"

# ---- Determinism guard: id-order vs REVERSE commit order -------------------
# Migrate two fresh corpora seeded in opposite commit orders; the migrated bytes
# must be identical (the verb keys output on the path-sorted corpus, not commit
# order; increment-workflow #7).
migrate_order() { # migrate_order <tag> <slug1> <slug2> -> writes the migrated cache-prd to stdout path
  local tag="$1" s1="$2" s2="$3"
  local r="$scratch/order-$tag" h="$scratch/home-$tag"; mk_repo "$r" "$h"
  for s in "$s1" "$s2"; do
    case "$s" in
      cache-prd) commit_prd "$r" cache-prd "Cache Prd" "The cache must answer in under a millisecond." ;;
      queue-prd) commit_prd "$r" queue-prd "Queue Prd" "The queue must never drop an enqueued job." ;;
    esac
  done
  ( cd "$r" && env -u JIGC_DOC_CODE_PROBE HOME="$h" JIGC_PACK_DIR="$PACK" "$JIGC" migrate-corpus ) >/dev/null 2>&1
  echo "$r"
}
fwd="$(migrate_order fwd cache-prd queue-prd)"
rev="$(migrate_order rev queue-prd cache-prd)"
{
  echo "--- determinism: id-order vs reverse commit order ---"
  cf="$(sha256sum "$fwd/docs/prds/cache-prd.md" | cut -d' ' -f1)"
  cr="$(sha256sum "$rev/docs/prds/cache-prd.md" | cut -d' ' -f1)"
  qf="$(sha256sum "$fwd/docs/prds/queue-prd.md" | cut -d' ' -f1)"
  qr="$(sha256sum "$rev/docs/prds/queue-prd.md" | cut -d' ' -f1)"
  echo "cache-prd id-order sha: $cf"
  echo "cache-prd reverse  sha: $cr"
  echo "queue-prd id-order sha: $qf"
  echo "queue-prd reverse  sha: $qr"
  [ "$cf" = "$cr" ] && [ "$qf" = "$qr" ] \
    && echo "BYTE-IDENTICAL across commit orders (deterministic)" \
    || echo "MISMATCH: output depends on commit order"
  [ "$cf" != "$qf" ] && echo "GUARD: the two docs migrate to genuinely distinct content" \
    || echo "GUARD FAIL: the two docs migrated identical (fixture is degenerate)"
} | tee "$HERE/structural-determinism.log"
