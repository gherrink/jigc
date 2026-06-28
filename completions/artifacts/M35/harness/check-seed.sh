#!/usr/bin/env bash
# check-seed.sh — the T1 no-API verifier for the M35 rename-study FIXED INPUTS.
#
# Three bars, all runnable without any LLM API (the jigc binary is a local CLI, not an API):
#
#   (1) CLEAN SEED — build-seed.sh writes a corpus on which the arm-agnostic oracle
#       `measure-refint.py --no-jigc` reports 0 dangling edges, AND (when `jigc` is on PATH)
#       a `jigc setup`+`jigc ingest`'d copy reports `jigc validate` 0 findings. This is the
#       cross-doc clean-seed bar, tightened for M34's mandatory schema-version stamp.
#
#   (2) NON-GREPPABILITY (the construction, PROVEN not asserted) — every edit flagged
#       `"nongreppable": true` in sequence.json must (a) have a prompt that does NOT contain
#       its target slug as literal text (the check FAILS if the slug leaks — that failure IS
#       the proof the construction holds), and (b) record in `breaks` a STRUCTURED managed
#       referrer to that target — an `arch-doc ... #cites -> adr:<target>` edge at >=2 hops.
#
#   (3) >=1 non-greppable edit exists at all (the net-new for M35).
#
# Exit 0 iff every bar holds; nonzero (with a FAIL line) otherwise.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
STUDY="$(cd "$HERE/.." && pwd)"
JIGC="${JIGC:-$HOME/.local/bin/jigc}"
SEQ="$HERE/sequence.json"
PROMPTS="$STUDY/prompts"

fails=0
fail() { echo "FAIL: $*"; fails=$((fails + 1)); }
ok()   { echo "ok: $*"; }

# ----------------------------------------------------------------- (1) clean seed
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
sh "$HERE/build-seed.sh" "$TMP" >/dev/null

dangling="$(python3 "$HERE/measure-refint.py" --repo "$TMP" --no-jigc \
            | python3 -c 'import sys,json; print(json.load(sys.stdin)["n_dangling"])')"
if [ "$dangling" = "0" ]; then ok "measure-refint.py: 0 dangling on the seed"
else fail "measure-refint.py reports $dangling dangling edges on the seed (expected 0)"; fi

if [ -x "$JIGC" ]; then
  ( cd "$TMP"
    git init -q; git config user.email s@s; git config user.name s
    git add -A; git commit -qm seed >/dev/null
    "$JIGC" setup  >/dev/null 2>&1
    "$JIGC" ingest >/dev/null 2>&1 )
  vfind="$("$JIGC" -C "$TMP" validate --format json 2>/dev/null \
           | python3 -c 'import sys,json; print(len(json.load(sys.stdin).get("findings",[])))' 2>/dev/null \
           || echo ERR)"
  # `jigc` has no -C flag in every build; fall back to a subshell cd.
  if [ "$vfind" = "ERR" ]; then
    vfind="$(cd "$TMP" && "$JIGC" validate --format json 2>/dev/null \
             | python3 -c 'import sys,json; print(len(json.load(sys.stdin).get("findings",[])))')"
  fi
  if [ "$vfind" = "0" ]; then ok "jigc validate: 0 findings on the ingested seed"
  else fail "jigc validate reports $vfind findings on the ingested seed (expected 0)"; fi
else
  echo "skip: jigc not at \$JIGC ($JIGC) — measure --no-jigc bar still enforced"
fi

# ----------------------------------------------------------------- (2)+(3) non-greppable
# Emit, per non-greppable edit: "<target>\t<prompt-file>\t<breaks-joined>\t<hops>"
mapfile -t NG < <(python3 - "$SEQ" <<'PY'
import json, sys
seq = json.load(open(sys.argv[1]))
for e in seq["edits"]:
    if e.get("nongreppable") is True:
        print("\t".join([e["target"], f"edit-{e['n']}.txt",
                         " || ".join(e.get("breaks", [])), str(e.get("hops", ""))]))
PY
)

if [ "${#NG[@]}" -ge 1 ]; then ok "${#NG[@]} non-greppable edit(s) declared in sequence.json"
else fail "no edit flagged \"nongreppable\": true in sequence.json (the M35 net-new is missing)"; fi

for row in "${NG[@]}"; do
  IFS=$'\t' read -r target promptfile breaks hops <<<"$row"
  pf="$PROMPTS/$promptfile"
  if [ ! -f "$pf" ]; then fail "non-greppable edit references missing prompt $promptfile"; continue; fi

  # (2a) the slug must NOT appear in the prompt text — literal substring. This is the
  #      RED hinge: if the slug leaks, the construction is broken and the check fails.
  if grep -qF -- "$target" "$pf"; then
    fail "$promptfile leaks the non-greppable target slug '$target' (prompt must withhold it)"
  else
    ok "$promptfile withholds the target slug '$target'"
  fi

  # (2b) the breaks must record a STRUCTURED managed referrer (arch-doc cites) to the
  #      target at >=2 hops — a transitive/greppable chain is not the construction.
  if printf '%s' "$breaks" | grep -Eq "arch-doc:[a-z0-9-]+#cites -> adr:$target\b"; then
    ok "sequence.json records the structured arch-doc cites referrer to '$target'"
  else
    fail "edit for '$target' records no 'arch-doc:...#cites -> adr:$target' structured referrer in breaks"
  fi
  if [ "${hops:-0}" -ge 2 ] 2>/dev/null; then ok "'$target' referrer is at $hops hops (>=2)"
  else fail "'$target' referrer hops=$hops (<2; the far structured case is required)"; fi
done

echo "---"
if [ "$fails" -eq 0 ]; then echo "check-seed.sh OK — clean seed + non-greppable construction holds"; exit 0
else echo "$fails failure(s)"; exit 1; fi
