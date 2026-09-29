#!/usr/bin/env bash
# M26 inc-3 T2 — the FULL hybrid arch-doc real-corpus live-migration driver (the done-bar).
# Closes the migration arc on its last + hardest doctype (`arch-doc`): repeatable `components`
# carrying BOTH a `description` slot AND an `implemented-by` code anchor, plus a `cites → adr`
# header relation, over the RE-PINNED HEAD `jigc` found on PATH with the EMBEDDED dev pack
# (NO JIGC_PACK_DIR) and the `doc-code` probe sibling resolved beside the installed binary
# (NO JIGC_DOC_CODE_PROBE). ONE FILE AT A TIME (auto-migration.md → Migration model: NOT a sweep).
#
# Framing A: the payload-*.yaml files ARE the agent rewriting the foreign prose into ONE
# declarative batch each; this script places nothing — the CLI applies every leaf and owns
# placement, the slug, the retire, the commit. The payload never touches {{source}}.
#
# HYBRID corpus (C1 — auto-migration.md → Acceptance corpus):
#   REAL-FOREIGN arm   : project-beta/project-beta ARCHITECTURE.md — a REAL in-the-wild ~700-line non-English
#                        architecture document; anchors FILE-ONLY over real .py/.ts
#                        (non-Rust → symbol-exists degrades to file-exists). The fidelity claim.
#   SYNTHETIC-over-Rust: LABELED SYNTHETIC arch-doc whose ≥2 components anchor at REAL,
#                        independently-resolving jigc Rust symbols (clones of jigc's own
#                        parse.rs / write.rs in the scratch repo — fixture/clone, never a live
#                        jigc dep), + a `cites → adr` over a committed precondition adr
#                        (committed-FIRST). The C2 mandate corpus.
set -u

EVID="$(cd "$(dirname "$0")" && pwd)"
JIGC_REPO=/home/maurice/Projects/gherrink-jigc
PROJECT_BETA=/home/maurice/Projects/project-beta          # READ-ONLY source repo — never mutated.

# run_file <tag> <log> <foreign-rel> <doctype> <payload> -- one file, full spine.
# Captures: migrate (composed view) -> ONE doc author batch -> bare finalize (review gate
# block) -> finalize --approve (promote+retire+adopt) -> ingest x2 (round-trip + idempotent)
# -> the migration commit. Returns BLOCK_EXIT / APPROVE_EXIT / MINTED.
run_file() {
  local tag="$1" log="$2" rel="$3" doctype="$4" payload="$5"
  {
    echo "=================================================================="
    echo "### FILE: $rel   (--as $doctype)   tag=$tag"
    echo "foreign git-blob: $(git hash-object "$rel" 2>/dev/null)"
    echo "foreign line count: $(wc -l < "$rel")"
    echo
    echo "--- jigc migrate $rel --as $doctype (composed view, head) ---"
  } >> "$log"
  local mig_out task
  mig_out=$(jigc migrate "$rel" --as "$doctype" 2>&1)
  printf '%s\n' "$mig_out" | head -40 >> "$log"
  task=$(printf '%s\n' "$mig_out" | grep -oE 'migrate-'"$doctype"'-[a-z0-9-]+' | head -1)
  echo "MIGRATION_TASK=$task" >> "$log"

  { echo; echo "--- jigc doc author $doctype --from <payload> (ONE batch) ---"; } >> "$log"
  local minted
  minted=$(jigc doc author "$doctype" --from - --task "$task" < "$payload" 2>>"$log")
  echo "AUTHOR_MINTED=$minted" >> "$log"

  { echo; echo "--- jigc task finalize $task   (review gate, NO --approve) ---"; } >> "$log"
  jigc task finalize "$task" >> "$log" 2>&1
  local block_exit=$?
  echo "BLOCK_EXIT=$block_exit" >> "$log"

  { echo; echo "--- jigc task finalize $task --approve ---"; } >> "$log"
  jigc task finalize "$task" --approve >> "$log" 2>&1
  local approve_exit=$?
  echo "APPROVE_EXIT=$approve_exit" >> "$log"

  local canon_dir
  case "$doctype" in
    adr) canon_dir=decisions ;;
    arch-doc) canon_dir=architecture ;;
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
      echo "canonical line count:        $(wc -l < "$canon_dir/$slug.md")"
      echo "canonical '## ' count:       $(grep -c '^## ' "$canon_dir/$slug.md")"
      echo "canonical '### ' count:      $(grep -c '^### ' "$canon_dir/$slug.md")"
      echo "canonical implemented-by:    $(grep -c 'implemented-by' "$canon_dir/$slug.md")"
      echo "canonical cites:             $(grep -c '^cites:' "$canon_dir/$slug.md")"
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

# ─────────────────────── REAL-FOREIGN arm (the fidelity claim) ───────────────────────
# This arm cannot be re-run from this repository: its source is a real client repository,
# and its log, payload and canonical output were removed before publication
# (implementation/public-hygiene.md -> rule 2). The SYNTHETIC arm below is complete.
real_arm() {
  local repo=/tmp/jigc-m26-real log="$EVID/real-arch-doc.log"
  : > "$log"
  rm -rf "$repo"; mkdir -p "$repo/docs"
  cd "$repo" || return 1
  git init -q; git config user.email t@example.com; git config user.name Tester
  # The REAL foreign architecture doc, copied verbatim from the live project-beta repo (read-only).
  cp "$PROJECT_BETA/ARCHITECTURE.md" docs/ARCHITECTURE.md
  # The real NON-Rust anchor target files (file-only anchors resolve to file-exists).
  mkdir -p service-a service-b service-c service-d
  cp "$PROJECT_BETA/service-a/models.py"      service-a/models.py
  cp "$PROJECT_BETA/service-b/models.py"   service-b/models.py
  cp "$PROJECT_BETA/service-c/models.py" service-c/models.py
  cp "$PROJECT_BETA/service-d/middleware.ts"           service-d/middleware.ts
  git add .; git commit -q -m "import foreign architecture doc + anchor targets"
  {
    echo "### REAL-FOREIGN ARM — project-beta/project-beta docs/ARCHITECTURE.md"
    echo "scratch repo: $repo   (live project-beta repo at $PROJECT_BETA untouched)"
    echo "real foreign H1: $(head -1 docs/ARCHITECTURE.md)"
    echo "anchor targets (real, non-Rust): service-a/b/c models.py + service-d middleware.ts"
    echo "binary under test: $(command -v jigc)  sha256=$(sha256sum "$(command -v jigc)" | cut -d' ' -f1)"
    echo "doc-code sibling:  $(command -v doc-code)  sha256=$(sha256sum "$(command -v doc-code)" | cut -d' ' -f1)"
    echo
  } >> "$log"
  jigc setup >> "$log" 2>&1
  run_file real-project-beta "$log" "docs/ARCHITECTURE.md" arch-doc "$EVID/payload-real-arch-doc.yaml"
  cd / || true
}

# ─────────── SYNTHETIC-over-jigc-Rust arm (LABELED SYNTHETIC, the C2 mandate) ───────────
synthetic_arm() {
  local repo=/tmp/jigc-m26-synthetic log="$EVID/synthetic-arch-doc.log"
  : > "$log"
  rm -rf "$repo"; mkdir -p "$repo/src" "$repo/fixtures"
  cd "$repo" || return 1
  git init -q; git config user.email t@example.com; git config user.name Tester
  # Clones of jigc's OWN Rust (fixture/clone targets — never a live jigc dep) so the
  # implemented-by anchors resolve REAL jigc symbols via tree-sitter (symbol-exists, not degraded).
  cp "$JIGC_REPO/crates/engine/src/parse.rs" src/parser.rs
  cp "$JIGC_REPO/crates/engine/src/write.rs" src/writer.rs
  # The committed-adr precondition source + the synthetic foreign arch-doc source.
  mkdir -p docs/adr
  cp "$EVID/foreign-precondition-adr.md" docs/adr/0001-parser-stages.md
  cat > docs/parser-subsystem.md <<'FOREIGN'
# Parser subsystem

> SYNTHETIC fixture — an architecture sketch over jigc's own parser/writer, authored for
> the M26 measured corpus (arch-doc has no real foreign-corpus standard over jigc's subsystem).

## Overview

Turns source bytes into a structured document and renders them back.

## Components

### Section parser

Parses sections.

### Canonical writer

Renders canonical bytes.
FOREIGN
  git add .; git commit -q -m "import synthetic arch-doc source + jigc Rust clones + foreign adr"
  {
    echo "### SYNTHETIC-over-jigc-Rust ARM — LABELED SYNTHETIC (the C2 mandate)"
    echo "scratch repo: $repo"
    echo "Rust anchor targets: src/parser.rs (clone of crates/engine/src/parse.rs #parse_sections)"
    echo "                     src/writer.rs (clone of crates/engine/src/write.rs  #render)"
    echo "binary under test: $(command -v jigc)  sha256=$(sha256sum "$(command -v jigc)" | cut -d' ' -f1)"
    echo "doc-code sibling:  $(command -v doc-code)  sha256=$(sha256sum "$(command -v doc-code)" | cut -d' ' -f1)"
    echo
  } >> "$log"
  jigc setup >> "$log" 2>&1
  # Committed-FIRST: provision the cited adr BEFORE the citing arch-doc.
  echo ">>> PRECONDITION: migrate the cited adr first (committed-first ordering)" >> "$log"
  run_file synth-precond-adr "$log" "docs/adr/0001-parser-stages.md" adr "$EVID/payload-precondition-adr.yaml"
  echo ">>> ARCH-DOC: now migrate the citing arch-doc" >> "$log"
  run_file synth-parser "$log" "docs/parser-subsystem.md" arch-doc "$EVID/payload-synthetic-arch-doc.yaml"
  cd / || true
}

real_arm
synthetic_arm
echo "ALL DONE"
