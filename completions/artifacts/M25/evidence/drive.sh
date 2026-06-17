#!/usr/bin/env bash
# M25 inc-7 T2 — the FULL generalized real-corpus live-migration driver (the done-bar).
# Generalizes the M24 changelog driver to the location-bearing, multi-instance doctypes
# adr / spec / prd, ONE FILE AT A TIME (auto-migration.md → Migration model: NOT a sweep).
#
# Each foreign file runs the doctype-general spine through the re-pinned HEAD `jigc`
# (from PATH, EMBEDDED dev pack — no JIGC_PACK_DIR) on a THROWAWAY clone / scratch repo:
#   setup -> migrate -> ONE `doc author --from` batch -> review-gate block(bare) ->
#   --approve -> retire+adopt -> ingest (round-trip) -> ingest again (idempotent).
#
# Framing A: the heredoc payloads ARE the agent rewriting the foreign prose into one
# declarative payload; this script places nothing — the CLI applies every leaf and owns
# placement, the slug, the retire, and the commit. The payload never touches {{source}}.
#
# ADR arm  : thomvaill/log4brains  docs/adr/   — REAL MADR/Nygard set, TWO supersession
#            chains, migrated target-first (the dependency-ordering contract).
# spec arm : semver/semver  semver.md          — REAL, idiosyncratic numbered-clause spec.
# prd arm  : SYNTHESIZED, LABELED-SYNTHETIC    — prd has no real foreign-corpus standard
#            (auto-migration.md honest bound); the foreign file is fabricated here.
set -u

EVID="$(cd "$(dirname "$0")" && pwd)"
ADR_SRC=/tmp/cand-thomvaill-log4brains
SEMVER_SRC=/tmp/cand-semver

# run_file <slug-tag> <log> <foreign-rel> <doctype> <payload-file> -- one file, full spine.
run_file() {
  local tag="$1" log="$2" rel="$3" doctype="$4" payload="$5"
  {
    echo "=================================================================="
    echo "### FILE: $rel   (--as $doctype)   tag=$tag"
    echo "foreign git-blob: $(git hash-object "$rel" 2>/dev/null)"
    echo "foreign line count: $(wc -l < "$rel")"
    echo
    echo "--- jigc migrate $rel --as $doctype (composed view) ---"
  } >> "$log"
  # ONE migrate; capture the task id from its own output (the guidance prints it).
  local mig_out task
  mig_out=$(jigc migrate "$rel" --as "$doctype" 2>&1)
  echo "$mig_out" >> "$log"
  task=$(printf '%s\n' "$mig_out" | grep -oE 'migrate-'"$doctype"'-[a-z0-9-]+' | head -1)
  echo "MIGRATION_TASK=$task" >> "$log"

  {
    echo
    echo "--- jigc doc author $doctype --from <payload> (ONE batch) ---"
  } >> "$log"
  local minted
  minted=$(jigc doc author "$doctype" --from - --task "$task" < "$payload" 2>>"$log")
  echo "AUTHOR_MINTED=$minted" >> "$log"

  {
    echo
    echo "--- jigc task finalize $task   (review gate, NO --approve) ---"
  } >> "$log"
  jigc task finalize "$task" >> "$log" 2>&1
  local block_exit=$?
  echo "BLOCK_EXIT=$block_exit" >> "$log"

  {
    echo
    echo "--- jigc task finalize $task --approve ---"
  } >> "$log"
  jigc task finalize "$task" --approve >> "$log" 2>&1
  local approve_exit=$?
  echo "APPROVE_EXIT=$approve_exit" >> "$log"

  local canon_dir
  case "$doctype" in
    adr) canon_dir=decisions ;;
    spec) canon_dir=specs ;;
    prd) canon_dir=prds ;;
  esac
  local slug="${minted#*:}"
  {
    echo
    echo "--- foreign original present after approve? ---"
    if [ -f "$rel" ]; then echo "PRESENT (NOT retired)"; else echo "GONE (retired)"; fi
    echo
    echo "--- canonical $canon_dir/$slug.md ---"
  } >> "$log"
  if [ -f "$canon_dir/$slug.md" ]; then
    cp "$canon_dir/$slug.md" "$EVID/$tag.canonical.md"
    {
      cat "$canon_dir/$slug.md"
      echo
      echo "canonical line count:     $(wc -l < "$canon_dir/$slug.md")"
      echo "canonical 'date:' count:  $(grep -c 'date:' "$canon_dir/$slug.md")"
      echo "canonical '## ' count:    $(grep -c '^## ' "$canon_dir/$slug.md")"
      echo "canonical '### ' count:   $(grep -c '^### ' "$canon_dir/$slug.md")"
    } >> "$log"
  else
    echo "(no canonical doc produced)" >> "$log"
  fi

  {
    echo
    echo "--- jigc ingest (round-trip conformance: adopted?) ---"
    jigc ingest 2>&1 | grep -iE "$canon_dir|adopt|reconcile|drift" | head -20
    echo
    echo "--- jigc ingest AGAIN (idempotent — file-state hash stable) ---"
    jigc ingest 2>&1 | grep -iE "$canon_dir|adopt|reconcile|drift" | head -20
    echo
    echo "--- HEAD commit subject + name-status (the migration commit) ---"
    git show --name-status --format='%H%n%s%n%b' HEAD
    echo
    echo "RESULT $tag: BLOCK_EXIT=$block_exit APPROVE_EXIT=$approve_exit MINTED=$minted"
  } >> "$log"
  echo "$tag: BLOCK_EXIT=$block_exit APPROVE_EXIT=$approve_exit MINTED=$minted"
}

# ─────────────────────────── ADR arm (the headline) ───────────────────────────
adr_arm() {
  local clone=/tmp/jigc-m25-adr log="$EVID/adr.log"
  : > "$log"
  rm -rf "$clone"; git clone -q "$ADR_SRC" "$clone"
  cd "$clone" || return 1
  git config user.email t@example.com; git config user.name Tester
  git checkout -q --detach HEAD
  { echo "### ADR ARM — thomvaill/log4brains docs/adr/ (real MADR set, two supersession chains)"
    echo "clone: $clone"; echo "foreign ADRs:"; ls docs/adr/*.md; echo; } >> "$log"
  jigc setup >> "$log" 2>&1

  # Chain 1, TARGET FIRST: 20200926 (superseded) before 20201016 (supersedes it).
  run_file adr-number "$log" "docs/adr/20200926-use-the-adr-number-as-its-unique-id.md" adr "$EVID/payload-adr-number.yaml"
  run_file adr-slug   "$log" "docs/adr/20201016-use-the-adr-slug-as-its-unique-id.md"   adr "$EVID/payload-adr-slug.yaml"
  # Chain 2, TARGET FIRST: 20240926 (superseded) before 20241217 (supersedes it).
  run_file adr-gitflow "$log" "docs/adr/20240926-transition-to-simplified-git-flow.md" adr "$EVID/payload-adr-gitflow.yaml"
  run_file adr-ghflow  "$log" "docs/adr/20241217-switch-back-to-github-flow.md"        adr "$EVID/payload-adr-ghflow.yaml"

  { echo; echo "--- FINAL: all four adopted? ---"; jigc ingest 2>&1 | grep -iE 'decisions'; } >> "$log"
  cd / || true
}

# ─────────────────────────── spec arm (real, idiosyncratic) ───────────────────────────
spec_arm() {
  local clone=/tmp/jigc-m25-spec log="$EVID/spec.log"
  : > "$log"
  rm -rf "$clone"; git clone -q "$SEMVER_SRC" "$clone"
  cd "$clone" || return 1
  git config user.email t@example.com; git config user.name Tester
  git checkout -q --detach HEAD
  { echo "### SPEC ARM — semver/semver semver.md (real, idiosyncratic numbered-clause spec)"
    echo "clone: $clone"; echo; } >> "$log"
  jigc setup >> "$log" 2>&1
  run_file spec-semver "$log" "semver.md" spec "$EVID/payload-spec-semver.yaml"
  cd / || true
}

# ─────────────────────────── prd arm (SYNTHESIZED, LABELED SYNTHETIC) ───────────────────────────
prd_arm() {
  local repo=/tmp/jigc-m25-prd log="$EVID/prd.log"
  : > "$log"
  rm -rf "$repo"; mkdir -p "$repo"; cd "$repo" || return 1
  git init -q; git config user.email t@example.com; git config user.name Tester
  cp "$EVID/synthetic-prd-source.md" PRD.md
  git add PRD.md; git commit -q -m "track synthetic PRD"
  { echo "### PRD ARM — SYNTHESIZED, LABELED-SYNTHETIC foreign PRD (prd has no real corpus standard)"
    echo "repo: $repo"; echo "foreign PRD: PRD.md (fabricated — see synthetic-prd-source.md)"; echo; } >> "$log"
  jigc setup >> "$log" 2>&1
  run_file prd-habit "$log" "PRD.md" prd "$EVID/payload-prd.yaml"
  cd / || true
}

adr_arm
spec_arm
prd_arm
echo "ALL DONE"
