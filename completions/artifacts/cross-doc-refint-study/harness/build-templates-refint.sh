#!/usr/bin/env bash
# build-templates-refint.sh — build all 5 arm templates for the cross-doc study as git
# repos seeded with the BYTE-IDENTICAL decision-record graph (build-seed.sh). Run on the
# HOST (arm A needs jigc for setup + ingest). Each template is later bind-mounted rw into a
# fresh container per edit by run-sequence-refint.sh.
#
#   A     : seed + `jigc setup` + `jigc ingest` (managed/baselined) + the BLOCKING
#           cross-doc pre-commit hook (blocking-pre-commit-refint). The differentiator.
#   C40/C160/C550 : seed + a root CLAUDE.md (the dilution ladder). No jigc, no hook.
#   P     : seed only. No CLAUDE.md, no hook. Base-rate anchor.
#
# The docs/ subtree is byte-identical across all arms (ingest is register-only — it never
# rewrites a file); arm A only ADDS .jigc/ + the jigc-bootstrap CLAUDE.md + the hook.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
STUDY="$(cd "$HERE/.." && pwd)"
ROOT="${ROOT:-$HOME/lh-study}"
JIGC="${JIGC:-$HOME/.local/bin/jigc}"
TPL="$ROOT/templates"
ARMS="$STUDY/arms"   # the cross-doc ladder files (build-arms-refint.py output)

[ -x "$JIGC" ] || { echo "no jigc at $JIGC" >&2; exit 2; }
[ -f "$ARMS/C40-CLAUDE.md" ] || { echo "ladder not built — run build-arms-refint.py first ($ARMS)" >&2; exit 2; }

seed_and_init() {  # <dir>
  local dir="$1"
  rm -rf "$dir"; mkdir -p "$dir"
  sh "$HERE/build-seed.sh" "$dir" >/dev/null
  git -C "$dir" init -q
  git -C "$dir" config user.email s@s; git -C "$dir" config user.name s
  git -C "$dir" add -A; git -C "$dir" commit -qm "seed: decision-record graph"
}

# ---- arm A: managed + blocking hook ----
A="$TPL/A"
seed_and_init "$A"
( cd "$A"
  "$JIGC" setup >/dev/null
  "$JIGC" ingest >/dev/null
  git add -A; git commit -qm "jigc setup + ingest (managed, baselined)" >/dev/null 2>&1 || true
  cp "$HERE/blocking-pre-commit-refint" .git/hooks/pre-commit
  chmod +x .git/hooks/pre-commit )
echo "built A (managed + blocking cross-doc hook)"

# ---- static ladder arms ----
for arm in C40 C160 C550; do
  d="$TPL/$arm"
  seed_and_init "$d"
  cp "$ARMS/$arm-CLAUDE.md" "$d/CLAUDE.md"
  git -C "$d" add -A; git -C "$d" commit -qm "add $arm rules file"
  echo "built $arm (static rule, $(wc -l < "$d/CLAUDE.md") lines)"
done

# ---- plain arm ----
seed_and_init "$TPL/P"
echo "built P (plain, no rule)"

# ---- byte-identity of the docs/ subtree across all arms ----
echo "=== docs/ subtree byte-identity across arms ==="
hashes=$(for arm in A C40 C160 C550 P; do
  find "$TPL/$arm/docs" -name '*.md' -exec sha256sum {} \; | sed "s#$TPL/$arm/##" | sort | sha256sum | awk '{print $1}'
done | sort -u | wc -l)
[ "$hashes" -eq 1 ] && echo "OK — docs/ identical across all 5 arms" || echo "MISMATCH ($hashes distinct docs/ trees)"
