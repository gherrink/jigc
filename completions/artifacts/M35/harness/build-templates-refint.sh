#!/usr/bin/env bash
# build-templates-refint.sh — build the M35 rename-cost study's THREE arm templates as git
# repos seeded with the BYTE-IDENTICAL decision-record graph (build-seed.sh). Run on the
# HOST (the jigc arm needs jigc for setup + ingest). Each template is later copied into a
# fresh per-edit container by run-sequence-refint.sh.
#
# The M35 arm set COLLAPSES the cross-doc study's 5-arm dilution ladder (A/C40/C160/C550/P)
# to exactly THREE arms — salience-decay is not what the rename study measures, the cost-win
# is (ideas/cli-owned-rename.md -> Acceptance; DECISIONS.md -> 2026-06-28 M35 planning):
#
#   jigc   : seed + `jigc setup` + `jigc ingest` (managed/baselined) + the `jigc rename`
#            verb AVAILABLE and ADAPTER-ADVERTISED (arms/jigc-CLAUDE.md appended to the
#            setup-written project CLAUDE.md) + the REAL Inc-2 backstop hook. Unlike the
#            cross-doc study (which copied a custom `blocking-pre-commit-refint` because the
#            then-shipped hook was warn-only), M35 installs NO custom hook: `jigc setup`
#            itself now writes the Inc-2 hook that BLOCKS a this-commit out-of-band `git mv`
#            of a managed doc — so a hand-edit in the jigc arm pays the block->recovery cost
#            (the verb-engagement confound, by construction). The differentiator: the agent
#            can PERFORM the rename in one command (the cost-win path).
#   static : seed + a root CLAUDE.md carrying a real SHELL ONE-LINER rename rule
#            (`git mv` + `sd`/`grep` to repoint refs — arms/static-CLAUDE.md). A genuine
#            baseline, NOT a straw manual one. No jigc, no hook.
#   plain  : seed only. No CLAUDE.md, no rule, no jigc, no hook. The omitting case — it
#            builds and runs INERT (honest, never error): the base-rate anchor.
#
# The docs/ subtree is byte-identical across all three arms (ingest is register-only — it
# never rewrites a file; setup/advert/rule files all live OUTSIDE docs/), which the study's
# causal claim rests on: every arm sees the same corpus, only the tooling/instruction differs.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
STUDY="$(cd "$HERE/.." && pwd)"
ROOT="${ROOT:-$HOME/lh-study}"
JIGC="${JIGC:-$HOME/.local/bin/jigc}"
TPL="$ROOT/templates"
ARMS="$STUDY/arms"

[ -x "$JIGC" ] || { echo "no jigc at $JIGC (set JIGC=path/to/jigc)" >&2; exit 2; }
# Precondition: the binary must carry the M35 Inc-1 `rename` verb (the net-new arm
# capability). A stale binary without it cannot build the jigc arm — fail loudly, not late.
"$JIGC" rename --help >/dev/null 2>&1 || {
  echo "jigc at $JIGC lacks the 'rename' subcommand — rebuild/reinstall the M35 binary" >&2; exit 2; }
for f in "$ARMS/static-CLAUDE.md" "$ARMS/jigc-CLAUDE.md"; do
  [ -f "$f" ] || { echo "missing arm file $f" >&2; exit 2; }
done

seed_and_init() {  # <dir>
  local dir="$1"
  rm -rf "$dir"; mkdir -p "$dir"
  sh "$HERE/build-seed.sh" "$dir" >/dev/null
  git -C "$dir" init -q
  git -C "$dir" config user.email s@s; git -C "$dir" config user.name s
  git -C "$dir" add -A; git -C "$dir" commit -qm "seed: decision-record graph"
}

# ---- jigc arm: managed + adapter-advertised rename + the real Inc-2 backstop hook ----
J="$TPL/jigc"
seed_and_init "$J"
( cd "$J"
  "$JIGC" setup  >/dev/null
  "$JIGC" ingest >/dev/null
  # adapter-advertise the rename verb (the verb-engagement confound): append the M35
  # advertisement to the project CLAUDE.md that `jigc setup` wrote (it points at .jigc/AGENT.md).
  cat "$ARMS/jigc-CLAUDE.md" >> CLAUDE.md
  git add -A; git commit -qm "jigc setup + ingest + rename advertisement (managed, baselined)" >/dev/null 2>&1 || true )
# the real Inc-2 backstop hook is what `jigc setup` installs natively — assert it is present
# and DOES block a this-commit OOB rename (not the pre-Inc-2 warn-only hook).
HOOK="$J/.git/hooks/pre-commit"
[ -x "$HOOK" ] || { echo "jigc arm has no pre-commit hook — setup did not install the backstop" >&2; exit 2; }
grep -q 'out-of-band managed-doc rename staged in this commit' "$HOOK" \
  || { echo "jigc arm hook is not the Inc-2 backstop (no this-commit OOB-rename block)" >&2; exit 2; }
echo "built jigc (managed + jigc rename advertised + real Inc-2 backstop hook)"

# ---- static arm: a real shell one-liner rename rule ----
S="$TPL/static"
seed_and_init "$S"
cp "$ARMS/static-CLAUDE.md" "$S/CLAUDE.md"
git -C "$S" add -A; git -C "$S" commit -qm "add static shell-one-liner rename rule"
echo "built static (shell one-liner rule, $(wc -l < "$S/CLAUDE.md") lines)"

# ---- plain arm: seed only, runs inert ----
seed_and_init "$TPL/plain"
echo "built plain (no rule, no verb — the omitting case, inert)"

# ==== exactly three arm templates ====
n_arms=$(ls -1d "$TPL"/jigc "$TPL"/static "$TPL"/plain 2>/dev/null | wc -l)
[ "$n_arms" -eq 3 ] || { echo "MISMATCH — expected exactly 3 arm templates, found $n_arms" >&2; exit 1; }
echo "OK — exactly 3 arm templates built (jigc, static, plain)"

# ==== docs/ subtree byte-identity across all three arms ====
echo "=== docs/ subtree byte-identity across arms ==="
hashes=$(for arm in jigc static plain; do
  find "$TPL/$arm/docs" -name '*.md' -exec sha256sum {} \; | sed "s#$TPL/$arm/##" | sort | sha256sum | awk '{print $1}'
done | sort -u | wc -l)
[ "$hashes" -eq 1 ] && echo "OK — docs/ identical across all 3 arms" \
  || { echo "MISMATCH ($hashes distinct docs/ trees)" >&2; exit 1; }

# ==== real-binary check: the jigc arm exposes `jigc rename` ====
( cd "$J" && "$JIGC" rename --help >/dev/null 2>&1 ) \
  && echo "OK — jigc arm exposes 'jigc rename' (rename --help exits 0)" \
  || { echo "jigc arm does NOT expose 'jigc rename'" >&2; exit 1; }

# ==== the static arm carries the shell one-liner rename rule ====
grep -q 'git mv' "$S/CLAUDE.md" && grep -Eq '\bsd\b' "$S/CLAUDE.md" && grep -q 'grep' "$S/CLAUDE.md" \
  && echo "OK — static arm CLAUDE.md carries the git mv + sd/grep one-liner rule" \
  || { echo "static arm CLAUDE.md is missing the git mv + sd/grep one-liner" >&2; exit 1; }

echo "build-templates-refint.sh OK — 3 arms built, docs/ identical, jigc rename available, static one-liner present"
