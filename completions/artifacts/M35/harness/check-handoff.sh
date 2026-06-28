#!/usr/bin/env bash
# check-handoff.sh — the M35 rename-study "ALL GREEN AT HANDOFF, ZERO API SPENT" gate (T5).
#
# Two halves, both runnable without any LLM API (jigc is a local CLI, not an API):
#
#   (A) PRE-REGISTRATION STRUCTURAL CHECK — assert pre-registration.md carries every section
#       the done-criterion mandates, by grepping the EMITTED artifact's own heading + anchor
#       lines (not a reconstructed copy): the three arms (jigc / plain / static-one-liner),
#       the headline (turns/cost per rename) + dangling (structured managed refs ONLY) +
#       completeness-on-the-non-greppable-referrer metrics, the pre-registered WIN (jigc
#       strictly beats plain on cost AND >= static on completeness), the CONFOUNDS
#       (verb-engagement a PRIMARY measured quantity, error->recovery, small-N, the static
#       arm's real shell one-liner — not a straw baseline), and the VOID-TRIPWIRES
#       (control-violation = 0, oracle-disagreement = 0).
#
#   (B) THE NO-API HANDOFF BAR — every T1-T4 selftest/verification passes from a clean
#       checkout (the cross-doc study's "State at handoff — all green, no API spent"
#       standard): T1 check-seed.sh, T2 build-templates-refint.sh (needs the M35 `jigc rename`
#       binary), T3 analyze.py --selftest, T4 measure-refint.py + eval-sequence-refint.py
#       --selftest.
#
# Exit 0 iff every bar holds; nonzero (with FAIL lines) otherwise. This is the single command
# RUNBOOK.md step 0 references as the handoff gate.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
STUDY="$(cd "$HERE/.." && pwd)"
PREREG="$STUDY/pre-registration.md"

fails=0
fail() { echo "FAIL: $*"; fails=$((fails + 1)); }
ok()   { echo "ok: $*"; }

# Resolve a jigc that carries the M35 `rename` verb (T2's net-new arm capability): prefer an
# explicit $JIGC, then the repo's freshly-built debug binary, then the installed one.
resolve_jigc() {
  local repo cand
  repo="$(cd "$STUDY" && git rev-parse --show-toplevel 2>/dev/null || true)"
  for cand in "${JIGC:-}" "$repo/target/debug/jigc" "$repo/target/release/jigc" \
              "$HOME/.local/bin/jigc"; do
    [ -n "$cand" ] && [ -x "$cand" ] || continue
    if "$cand" rename --help >/dev/null 2>&1; then echo "$cand"; return 0; fi
  done
  return 1
}

# ============================================================ (A) pre-registration structure
echo "=== (A) pre-registration.md structural check ==="
if [ ! -f "$PREREG" ]; then
  fail "pre-registration.md is absent at $PREREG"
else
  # Each row: "<label>\t<extended-regex matched against the emitted file>". The patterns key
  # on the artifact's own heading + anchor lines, so a missing/renamed section fails RED.
  while IFS=$'\t' read -r label pat; do
    [ -z "$label" ] && continue
    if grep -Eq "$pat" "$PREREG"; then ok "section present: $label"
    else fail "pre-registration.md missing required section/anchor: $label  (/$pat/)"; fi
  done <<'ROWS'
arms heading	^##[[:space:]].*[Aa]rms\b
three arms named	jigc.*plain.*static|static.*one-liner
headline metric heading	^##[[:space:]].*[Mm]etrics\b|^##[[:space:]].*[Hh]eadline
turns/cost per rename	(turns|cost).{0,40}per[[:space:]-]rename|per[[:space:]-]rename
dangling structured-managed-only	structured managed ref
completeness non-greppable	completeness.*non-greppable|non-greppable.*referrer
win heading	^##[[:space:]].*[Ww]in\b
win condition wording	strictly.*cheaper.*plain|strictly beats plain.*cost
win completeness clause	completeness[[:space:]>=]*static|>=[[:space:]]*static.*completeness|>= static
confounds heading	^##[[:space:]].*[Cc]onfounds\b
verb-engagement primary	verb-engagement.*primary|primary.*measured quantity
error->recovery confound	error[[:space:]>-]*recovery|rename-error
small-N confound	small-N|small N|underpowered
static one-liner not straw	one-liner|not a straw
void-tripwires heading	^##[[:space:]].*[Tt]ripwires\b
control-violation = 0	control[^0-9]*=?[[:space:]]*0|control.{0,20}must be 0|control✗
oracle-disagreement = 0	oracle[^0-9]*=?[[:space:]]*0|oracle.{0,20}must be 0|oracle≠
ROWS
fi

# ============================================================ (B) the no-API handoff bar
echo "=== (B) no-API handoff bar — T1-T4 selftests/verifications ==="

run() {  # <label> <cmd...>
  local label="$1"; shift
  if "$@" >/tmp/m35-handoff.$$ 2>&1; then ok "$label"; else
    fail "$label"; sed 's/^/    /' /tmp/m35-handoff.$$; fi
  rm -f /tmp/m35-handoff.$$
}

run "T1 check-seed.sh (clean seed + non-greppable construction)" \
    bash "$HERE/check-seed.sh"
run "T3 analyze.py --selftest (verb/rename engagement)" \
    python3 "$HERE/analyze.py" --selftest
run "T4 measure-refint.py --selftest (dangling + completeness oracle)" \
    python3 "$HERE/measure-refint.py" --selftest
run "T4 eval-sequence-refint.py --selftest (cost/completeness + tripwires)" \
    python3 "$HERE/eval-sequence-refint.py" --selftest

if J="$(resolve_jigc)"; then
  TMPROOT="$(mktemp -d)"
  run "T2 build-templates-refint.sh (3 arms, docs/ identical, rename available)" \
      env JIGC="$J" ROOT="$TMPROOT" bash "$HERE/build-templates-refint.sh"
  rm -rf "$TMPROOT"
else
  fail "T2 build-templates: no jigc with the 'rename' verb found — run 'cargo build' first \
(handoff bar requires the M35 binary; set JIGC=path/to/jigc to override)"
fi

echo "---"
if [ "$fails" -eq 0 ]; then
  echo "check-handoff.sh OK — pre-registration complete + every T1-T4 verification green, zero API spent"
  exit 0
else
  echo "$fails failure(s) — handoff bar NOT met"
  exit 1
fi
