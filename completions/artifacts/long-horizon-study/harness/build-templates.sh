#!/usr/bin/env bash
# build-templates.sh — build the non-A arm templates (C40, C160, C550, P) from the
# baseline @ 542b320 + the BYTE-IDENTICAL arch-doc emitted by arm A (build-arm-A.sh
# must run first). C arms add a root CLAUDE.md (the dilution ladder); P adds nothing.
# No jigc, no .jigc, no hook — plain repos. The arch-doc is committed into the baseline
# so every arm starts from the identical documented state.
set -euo pipefail
SRC="${SRC:-$HOME/Projects/gherrink-ui-doc}"
ROOT="${ROOT:-$HOME/lh-study}"
A="$ROOT/templates/A"
BASELINE=542b320
DOC_REL=docs/architecture/core-public-api.md

[ -f "$A/$DOC_REL" ] || { echo "arm A not built ($A/$DOC_REL missing) — run build-arm-A.sh first" >&2; exit 1; }

build_one() {  # <arm> <claude_md_or_->
  local arm="$1" claude="$2"
  local dir="$ROOT/templates/$arm"
  rm -rf "$dir"; mkdir -p "$dir"
  git -C "$SRC" archive "$BASELINE" | tar -x -C "$dir"
  mkdir -p "$dir/$(dirname "$DOC_REL")"
  cp "$A/$DOC_REL" "$dir/$DOC_REL"          # byte-identical doc
  if [ "$claude" != "-" ]; then cp "$claude" "$dir/CLAUDE.md"; fi
  ( cd "$dir"
    git init -q
    git add -A
    git -c user.email=s@s -c user.name=s commit -qm "baseline @ $BASELINE + core-public-api arch-doc" )
  echo "built $arm  (CLAUDE.md: ${claude##*/})"
}

build_one C40  "$ROOT/arms/C40-CLAUDE.md"
build_one C160 "$ROOT/arms/C160-CLAUDE.md"
build_one C550 "$ROOT/arms/C550-CLAUDE.md"
build_one P    "-"

# Verify the doc is byte-identical across every arm (incl. A).
echo "=== arch-doc byte-identity across arms ==="
( for arm in A C40 C160 C550 P; do sha256sum "$ROOT/templates/$arm/$DOC_REL"; done ) | awk '{print $1}' | sort -u | wc -l \
  | xargs -I{} sh -c '[ {} -eq 1 ] && echo "OK — identical across all 5 arms" || echo "MISMATCH ({} distinct hashes)"'
