#!/usr/bin/env bash
# 15-placement-root.sh — protocol.md §5 arm 15 (M49 Increment 7 + audit finding 1, the HIGH).
#
# WHICH SET THIS ARM ITERATES: the **user-settable doors** of the `placement-root`
# knob — `jigc config set placement-root <value>` over the value axis a human can
# type: a nested directory · the empty string · a path inside git's own dir · a path
# inside jigc's own workbench — plus the two strand shapes `jigc validate` must not
# call clean. It is a hand-enumerated set over ONE door, and says so.
#
# DECLARED TEST-FENCED, NOT DRIVEN HERE: the eight mover sites the M49 audit swept
# (`config set placement-root` · `config set docs-root` · `relocate` · `rename` ·
# `migrate-corpus`'s relocation arm · finalize's promote / retire · the shared
# `move_doc` primitive) are fenced by `crates/cli/tests/placement_override.rs` (T1–T4
# arms) and `crates/cli/src/trackable.rs`'s unit tests. This arm drives the one door
# an adopter reaches with a shell, and the HIGH's exact repro on it.
#
# THE HIGH, restated so the pass condition is legible: on the unfixed build
# `jigc config set placement-root .git` ran `git mv docs/roadmap.md .git/roadmap.md`,
# git printed `error: invalid path` AND EXITED 0, the file moved on disk, the index
# dropped the source and added nothing — so every mover read that 0 as success,
# `jigc validate` graded the store clean, and the doc was gone from every clone.
#
# CAPTURE, THEN GREP. This arm runs under `pipefail`, so `jigc … | grep -q` reports
# jigc's exit, and a refusal that matched would read as FAIL. Every assertion greps a
# captured variable (learned on this arm's first container run: one green cell red).
#
# PASS CONDITION, stated before the run:
#   (a) `placement-root planning` moves the committed `docs/roadmap.md` to
#       `planning/roadmap.md` as a staged `git mv` (index shows R), file-state is
#       re-keyed, `jigc validate` carries no blocking row, `jigc doc show roadmap`
#       still reads it;
#   (b) the root-declared `VISION.md` is NOT re-rooted;
#   (c) `placement-root ""` reads back as `.` and moves the doc to the repo root;
#   (d) `placement-root .git` is REFUSED with a code + route, nothing moves, the doc
#       stays tracked (`git ls-files`) and `jigc validate` stays clean;
#   (e) a hand-`git mv` of a placement doc to a wrong home, committed, is NOT
#       "validates clean" — the strand is reported with a route (rc.12 said clean);
#       the knob-moved-doc-did-not shape is reported as `file-state.orphaned-doc`;
#   (d') LAST, because it ends with a destroying door: `placement-root .jigc` is
#       driven and RECORDED (accepted or refused — the arm measures), and if accepted
#       the arm asks whether a committed doc inside the workbench survives
#       `jigc uninstall`. A FAIL there is a product observation, not an instrument one.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
addr() { grep -oE "^$1:[a-z0-9-]+" | head -1; }             # capture, never tail -1
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }
# Run once, print the transcript, keep the bytes + exit in OUT / RC for the bars.
cap()  { printf '\n$ %s\n' "$*"; OUT="$("$@" 2>&1)"; RC=$?; printf '%s\n[exit %s]\n' "$OUT" "$RC"; }

# Author the task's commit doc and finalize — the fixture builder's own shape
# (`dev/jigc-rig committed-singletons --print-only` is where these verbs come from).
finalize() { # <task> <scope> <summary>
  jigc doc set-field "commit:$1#type" --value docs --task "$1" >/dev/null 2>&1
  jigc doc set-field "commit:$1#scope" --value "$2" --task "$1" >/dev/null 2>&1
  printf '%s\n' "$3" | jigc doc set-slot "commit:$1#summary" --from-file - --task "$1" >/dev/null 2>&1
  printf 'Built by walk arm 15.\n' | jigc doc set-slot "commit:$1#body" --from-file - --task "$1" >/dev/null 2>&1
  jigc task finalize "$1" >/dev/null 2>&1
}

jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

say "FIXTURE · two committed placement docs — a nested home (roadmap) and a root home (vision)"
jigc start --workflow planning "plan the first wave" >/dev/null 2>&1
TP="$(newtask)"
jigc doc create roadmap --title Roadmap --task "$TP" >/dev/null 2>&1
MS="$(jigc doc add-item 'roadmap:roadmap#milestones' --title M-Alpha --task "$TP" 2>&1 | grep -oE '^roadmap:roadmap#milestones/[a-z0-9-]+' | head -1)"
printf 'That the composed loop lands one task end to end.\n' | jigc doc set-slot "$MS/proves" --from-file - --task "$TP" >/dev/null 2>&1
printf 'Increment 1 — the enumeration seam.\n' | jigc doc set-slot "$MS/decomposition" --from-file - --task "$TP" >/dev/null 2>&1
jigc doc create decisions-log --title 'Decisions Log' --task "$TP" >/dev/null 2>&1
finalize "$TP" planning 'mint the running docs'
jigc start --workflow form-vision "form the project vision" >/dev/null 2>&1
TV="$(newtask)"
jigc doc create vision --title Vision --task "$TV" >/dev/null 2>&1
for s in thesis invariants open-questions; do
  printf 'Written for walk arm 15.\n' | jigc doc set-slot "vision:vision#$s" --from-file - --task "$TV" >/dev/null 2>&1
done
finalize "$TV" vision 'form the project vision'
step git ls-files docs VISION.md
bar "roadmap is committed at its declared nested home"  "git ls-files --error-unmatch docs/roadmap.md"
bar "vision is committed at its declared ROOT home"     "git ls-files --error-unmatch VISION.md"
TL="$(jigc task list 2>&1)"
bar "no task is live — the knob doors run over a settled store" "! printf '%s' \"\$TL\" | grep -q '^  [a-z0-9]'"
step jigc config get placement-root

say "(a) · placement-root planning — the re-point MOVES the doc it would strand"
cap jigc config set placement-root planning; SA="$OUT"
RA="$(jigc config get placement-root 2>&1)"
step git status --porcelain
step git diff --cached --name-status
DA="$(git diff --cached --name-status 2>&1)"
bar "the knob reads back as planning, from the project layer" "printf '%s' \"\$RA\" | grep -q 'placement-root = planning  (project)'"
bar "the re-point names the move it made, per file"       "printf '%s' \"\$SA\" | grep -q 'docs/roadmap.md → planning/roadmap.md'"
bar "the index carries the move as a rename (R), not a delete + add" \
    "printf '%s' \"\$DA\" | grep -qE '^R[0-9]*\s+docs/roadmap.md\s+planning/roadmap.md'"
bar "the file is at the new home and gone from the old"  "test -f planning/roadmap.md && ! test -e docs/roadmap.md"
step cat .jigc/state/file-state.json
bar "file-state is re-keyed to planning/roadmap.md"      "grep -q '\"planning/roadmap.md\"' .jigc/state/file-state.json"
bar "…and no longer keys docs/roadmap.md"                "! grep -q '\"docs/roadmap.md\"' .jigc/state/file-state.json"
cap jigc validate; VA="$OUT"; VARC=$RC
bar "jigc validate exits 0 with no blocking row over the moved store" "test $VARC -eq 0 && ! printf '%s' \"\$VA\" | grep -q '^blocking'"
cap jigc doc show roadmap --format json; JA="$OUT"
bar "jigc doc show roadmap still reads the doc at its identity" \
    "printf '%s' \"\$JA\" | node -e 'const d=JSON.parse(require(\"fs\").readFileSync(0,\"utf8\")); process.exit(d.slug===\"roadmap\"&&d.sections.milestones.length===1?0:1)'"
cap jigc doc list roadmap; LA="$OUT"
bar "jigc doc list names the resolved path" "printf '%s' \"\$LA\" | grep -q 'planning/roadmap.md'"

say "(b) · the root-declared VISION.md is NOT re-rooted"
bar "VISION.md is still at the repo root"               "test -f VISION.md && ! test -e planning/VISION.md"
bar "…and the index did not touch it"                    "! printf '%s' \"\$DA\" | grep -q VISION.md"
bar "…and the re-point's own output never named it"      "! printf '%s' \"\$SA\" | grep -q VISION"
git add -A >/dev/null 2>&1; git commit -qm "walk 15: placement-root planning" >/dev/null 2>&1

say "(c) · placement-root \"\" canonicalizes to . and moves the doc to the repo root"
cap jigc config set placement-root ""
cap jigc config get placement-root; RC_="$OUT"
bar "the empty string reads back as ."                   "printf '%s' \"\$RC_\" | grep -q 'placement-root = \.  (project)'"
step git diff --cached --name-status
DC="$(git diff --cached --name-status 2>&1)"
bar "the doc moved to the repo root as a rename"          "printf '%s' \"\$DC\" | grep -qE '^R[0-9]*\s+planning/roadmap.md\s+roadmap.md' && test -f roadmap.md"
bar "VISION.md is still exactly where it was"             "test -f VISION.md && ! printf '%s' \"\$DC\" | grep -q VISION.md"
PC="$(jigc doc show 'roadmap#milestones/m-alpha/proves' 2>&1)"
bar "jigc doc show roadmap reads the root-homed doc"     "printf '%s' \"\$PC\" | grep -q 'composed loop'"
git add -A >/dev/null 2>&1; git commit -qm "walk 15: placement-root ." >/dev/null 2>&1

say "(d) · placement-root .git — THE HIGH: refused, nothing moves, the doc stays tracked"
cap jigc config set placement-root .git; RD="$OUT"; RDRC=$RC
bar "it refuses (non-zero)"                               "test $RDRC -ne 0"
bar "…with a code"                                        "printf '%s' \"\$RD\" | grep -q 'config.untrackable-root'"
bar "…that names git's own behaviour, so the reader knows WHY" "printf '%s' \"\$RD\" | grep -q 'error: invalid path'"
bar "…and a route"                                        "printf '%s' \"\$RD\" | grep -q 'route:'"
step git status --porcelain
bar "the working tree is untouched — no move was attempted" "test -z \"\$(git status --porcelain)\""
step git ls-files roadmap.md
bar "the doc is still tracked at its home"                "git ls-files --error-unmatch roadmap.md && test -f roadmap.md"
bar "…and nothing landed under .git/"                     "! test -e .git/roadmap.md"
GD="$(jigc config get placement-root 2>&1)"
bar "the knob is unchanged"                               "printf '%s' \"\$GD\" | grep -q 'placement-root = \.  (project)'"
cap jigc validate; VD="$OUT"; VDRC=$RC
bar "jigc validate is still clean (exit 0, no blocking row)" "test $VDRC -eq 0 && ! printf '%s' \"\$VD\" | grep -q '^blocking'"
cap jigc config set placement-root .git --format json; JD="$OUT"
bar "the JSON envelope carries the same code"             "printf '%s' \"\$JD\" | grep -q 'config.untrackable-root'"

cap jigc config set placement-root planning
git add -A >/dev/null 2>&1; git commit -qm "walk 15: back to planning" >/dev/null 2>&1
bar "the store is back at planning/ for the strand cells" "test -f planning/roadmap.md && jigc doc show roadmap >/dev/null 2>&1"

say "(e) · a hand-git-mv of a placement doc to a wrong home, committed — NOT 'validates clean'"
mkdir -p notes
step git mv planning/roadmap.md notes/roadmap.md
git commit -qm "walk 15: hand-move roadmap" >/dev/null 2>&1
cap jigc validate; VE="$OUT"; VERC=$RC
bar "the strand is reported — the sweep does NOT say the store validates clean" \
    "! printf '%s' \"\$VE\" | grep -qi 'validates clean' && printf '%s' \"\$VE\" | grep -q 'roadmap.md'"
bar "…and the report carries a route"                    "printf '%s' \"\$VE\" | grep -q 'route:'"
bar "…and the wrong-home copy is named"                  "printf '%s' \"\$VE\" | grep -q 'notes/roadmap.md'"
echo "  MEASURED · codes the hand-move draws (the charter expected file-state.orphaned-doc; an"
echo "             out-of-band git mv is ALSO a reconciliation rename, which fires first):"
printf '%s' "$VE" | grep -E '^(blocking|advisory)' | grep -oE '· [a-z-]+\.[a-z-]+' | sed 's/^· /             /'
echo "             exit: $VERC"
bar "the sweep exits non-zero over an out-of-band rename (M35's structural-identity rule)" "test $VERC -ne 0"
git mv notes/roadmap.md planning/roadmap.md >/dev/null 2>&1; git commit -qm "walk 15: restore" >/dev/null 2>&1

say "(e') · the knob moved, the doc did not — file-state.orphaned-doc with its route"
# Written straight into the manifest so the move floor never runs: the exact state
# placement_override.rs T3 arm 4 found `jigc validate` calling clean before Increment 7.
step sed -i 's/placement-root: planning/placement-root: notes/' .jigc/config/manifest.yaml
git add -A >/dev/null 2>&1; git commit -qm "walk 15: hand-edit the knob" >/dev/null 2>&1
step jigc config get placement-root
cap jigc validate; VF="$OUT"
bar "the stranded doc is reported as file-state.orphaned-doc" "printf '%s' \"\$VF\" | grep -q 'file-state.orphaned-doc'"
bar "…naming the doc where it sits"                       "printf '%s' \"\$VF\" | grep -q 'planning/roadmap.md'"
bar "…and the home it should be at"                       "printf '%s' \"\$VF\" | grep -q 'notes/roadmap.md'"
bar "…with a route naming the three exits (move+ingest · re-point · unmanage)" \
    "printf '%s' \"\$VF\" | grep -q 'jigc ingest' && printf '%s' \"\$VF\" | grep -q 'placement-root' && printf '%s' \"\$VF\" | grep -q 'jigc unmanage planning/roadmap.md'"
bar "VISION.md draws no strand row — a root home is never re-rooted" "! printf '%s' \"\$VF\" | grep 'file-state.orphaned-doc' | grep -q 'VISION.md'"
# Re-point through the door so the knob and the docs agree again (the docs are at
# planning/, nothing sits at notes/, so the move floor has nothing to move).
cap jigc config set placement-root planning
git add -A >/dev/null 2>&1; git commit -qm "walk 15: knob back to planning" >/dev/null 2>&1
cap jigc validate; VG="$OUT"; VGRC=$RC
bar "the store is clean again once knob and docs agree" "test $VGRC -eq 0 && ! printf '%s' \"\$VG\" | grep -q 'orphaned-doc'"

say "(d') · placement-root .jigc — the workbench: driven and RECORDED, then asked to survive uninstall"
# The workbench's own `.gitignore` ignores tasks/ index/ state/ … but not `.jigc/`
# itself (`.jigc/config/` is committed), so this is a legal, trackable root. Whether
# it SHOULD be one is the question the last two bars put to the binary. Last in the
# arm because `jigc uninstall` is a destroying door and nothing runs after it.
cap jigc config set placement-root .jigc; SJ="$OUT"
RJ="$(jigc config get placement-root 2>&1)"; echo "$RJ"
step git status --porcelain
step git check-ignore -v .jigc/roadmap.md
echo "  OBSERVE  placement-root .jigc: $(printf '%s' "$RJ" | head -1); moved: $(git status --porcelain | grep -c '^R')"
if [ -f .jigc/roadmap.md ]; then
  git add -A >/dev/null 2>&1; git commit -qm "walk 15: placement-root .jigc" >/dev/null 2>&1
  bar "(recorded, not required) the doc is tracked inside the workbench" "git ls-files --error-unmatch .jigc/roadmap.md"
  PJ="$(jigc doc show 'roadmap#milestones/m-alpha/proves' 2>&1)"
  bar "…and readable at its identity"                     "printf '%s' \"\$PJ\" | grep -q 'composed loop'"
  cap jigc uninstall; UJ="$OUT"; UJRC=$RC
  step git status --porcelain
  bar "a COMMITTED doc homed under the workbench survives \`jigc uninstall\` on disk" "test -f .jigc/roadmap.md"
  bar "…and in the index (no staged/unstaged deletion of it)" "! git status --porcelain | grep -qE '^( D|D )\s+\.jigc/roadmap.md'"
  bar "…or, failing that, uninstall at least REFUSED or NAMED the committed docs it took" \
      "test $UJRC -ne 0 || printf '%s' \"\$UJ\" | grep -q 'roadmap.md'"
  echo "  OBSERVE  if the bars above FAIL: \`jigc uninstall\` removed committed managed docs the knob"
  echo "           had just accepted, at exit 0, naming neither — the .git refusal's sibling on"
  echo "           the .jigc axis (recoverable from git here; the door's narration is the finding)."
else
  echo "  OBSERVE  placement-root .jigc was refused or moved nothing — recorded above; no uninstall probe."
fi

say "SUMMARY"
echo "  the placement-root door over {planning, \"\", .git, .jigc} + the two strand shapes;"
echo "  the eight mover sites are test-fenced (crates/cli/tests/placement_override.rs), not driven."
if [ "$FAIL" -eq 0 ]; then echo "ARM 15 PASS"; else echo "ARM 15 FAIL"; fi
exit "$FAIL"
