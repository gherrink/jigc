#!/usr/bin/env bash
# 13-freeze-every-layer.sh — protocol.md §5 arm 13 (M49 Increment 3).
#
# WHICH SET THIS ARM ITERATES: a **derivation, stated as one**. There is no
# code-side table of "layers a schema can arrive from" — there are two, and they
# are the two the resolver reads (`crates/cli/src/pack.rs` →
# `assert_schema_freeze` / `assert_project_schema_shadows`):
#
#   (1) the PROJECT layer — a whole-file shadow at `.jigc/config/schemas/<ty>.yaml`,
#       which outranks every pack and is the resolved schema at every surface;
#   (2) the PACK layer — the file a doctype ships as, in whichever pack provides it.
#
# Both layers pass through ONE loader (`engine::schema::load_schema` →
# `check_schema_shape`), so the mis-keyed-leaf cell is driven at both: at the pack
# layer through a listed house pack (the only pack layer this container can author
# — it carries no dev-pack source to reshape, which is how `flow50_acceptance.rs`
# drives the same seam), and again through a project shadow as a second reading.
#
# THE FLIP, so the arm reads as a change:
#   rc.12  a project shadow that dropped four sections from `adr` validated clean
#          at exit 0, `doc schema` reported the frozen `schema-version` for an
#          unfrozen shape, and `doc create` wrote a third shape; a `patern:` typo
#          inside a `repeatable:` block made serde's untagged fall-through ERASE
#          the whole section from every surface, at exit 0.
#   rc.13  the shadow is hashed AS RESOLVED against the governing manifest, so a
#          shape change blocks every pack-loading door by name with a runnable
#          route, while a presentation-only reword (`description:` / `usage:` /
#          slot `hint:`) still shadows cleanly; the mis-keyed leaf is refused at
#          load, located at `<type>#<section>/<leaf>` and naming the key.
#
# DECLARED BOUND, quoted from completions/artifacts/M49/VERDICT.md → declared bounds:
#   "`jigc setup` completes at exit 0 over a shape-changing project schema shadow —
#    declared, not accidental. It is the bootstrap door, resolves no doctype, and
#    refusing to install over a drifted corpus is circular. The counter-case is on
#    the record: the operator is told the install succeeded while every next door
#    refuses." Cell C below MEASURES that door against the four-part standard and
#    records the score; it asserts only what the bound declares.
#
# ORDER TRAP: `jigc start` is itself a pack-loading door, so the task the in-task
# cells need (`doc create adr … --task`, `task validate`, `doc show … --task`) is
# minted BEFORE the shadow lands. Written the other way round, the arm cannot reach
# those doors and quietly asserts nothing about them.
#
# PASS CONDITION, stated before the run:
#   (A) over a shape-dropping shadow, every pack-loading door listed below exits
#       non-zero naming `adr`, the shadow path, the rule, and a `route:` — and that
#       route, run verbatim, clears the block;
#   (B) a presentation-only shadow loads clean at every door and its reword shows;
#   (C) `setup` over the shape-changing shadow exits 0 (the bound) and the next
#       door still blocks; the four-part score is reported, not asserted;
#   (D) a mis-keyed leaf inside `repeatable:` is refused at load, at both layers,
#       naming `<type>#<section>` and the key — never answered with the section
#       silently gone.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }

say "0 · adopt, and mint the task FIRST (see the order trap)"
jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version
jigc start --workflow record-decision "probe the freeze" --slug probe-the-freeze >/dev/null 2>&1
T=probe-the-freeze
bar "the task the in-task cells need exists" "test -d .jigc/tasks/$T"
SHADOW_DIR=.jigc/config/schemas
SHADOW=$SHADOW_DIR/adr.yaml
mkdir -p "$SHADOW_DIR"

# The frozen `adr` shape as the embedded dev pack ships it on 1.0.0-rc.13
# (`crates/cli/pack/schemas/adr.yaml`, comments stripped) — the presentation-only
# cell needs the whole shape byte-faithful except for the keys it rewords. If the
# pack's `adr` moves, cell B's "loads clean" bar reddens: that is the freeze doing
# its job on this arm, not a bug in the arm.
write_frozen_adr() { # <description> <context-hint>
cat > "$SHADOW" <<EOF
type: adr
location: decisions/
id-from: title
description: $1
usage: a choice is worth preserving with its rationale, so a later reader can recover why the call was made or supersede it on the record.

sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date, type: date, set: on-create }
      - { id: supersedes, type: ref, to: adr, card: "0..*", inverse: superseded-by }
      - { id: cites-code, type: code-anchor }
  - id: context
    slot: { hint: "$2" }
  - id: options
    slot: { optional: true, hint: "Alternatives weighed and why they lost — leave it empty when the call was obvious; the heading renders either way." }
  - id: decision
    slot: { hint: "What we decided, in a sentence or two." }
  - id: consequences
    slot: { hint: "Tradeoffs and follow-on effects." }
EOF
}
# The same shape with the `options` section DROPPED — a shape change, nothing else.
write_shape_dropping_adr() {
cat > "$SHADOW" <<'EOF'
type: adr
location: decisions/
id-from: title
description: A dated architectural decision record, capturing the context a choice was made in, the choice itself, and its consequences, with an optional link to the decision it supersedes.
usage: a choice is worth preserving with its rationale, so a later reader can recover why the call was made or supersede it on the record.

sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date, type: date, set: on-create }
      - { id: supersedes, type: ref, to: adr, card: "0..*", inverse: superseded-by }
      - { id: cites-code, type: code-anchor }
  - id: context
    slot: { hint: "Why a decision was needed — the forces at play." }
  - id: decision
    slot: { hint: "What we decided, in a sentence or two." }
  - id: consequences
    slot: { hint: "Tradeoffs and follow-on effects." }
EOF
}

# ---------------------------------------------------------------------------
say "A · THE PROJECT LAYER — a shadow that drops \`options\` blocks every pack-loading door"
write_shape_dropping_adr
# The doors, and why each is on the list: the menu, the store sweep, the schema
# read, the staged content read, the create-gate write, the task gate preview, and
# the index read — one of every read/write kind that resolves a doctype.
DOORS=(
  "describe"
  "validate"
  "doc schema adr"
  "doc show commit:$T --task $T"
  "doc create adr --title Probe --task $T"
  "task validate $T"
  "doc list"
)
BLOCKED_ALL=0
for d in "${DOORS[@]}"; do
  # shellcheck disable=SC2086
  OUT="$(jigc $d 2>&1)"; RC=$?
  printf '\n$ jigc %s\n%s\n[exit %s]\n' "$d" "$(printf '%s' "$OUT" | head -c 400)…" "$RC"
  ok=1
  [ "$RC" -ne 0 ] || ok=0
  printf '%s' "$OUT" | grep -q 'doctype `adr`'                     || ok=0
  printf '%s' "$OUT" | grep -q '.jigc/config/schemas/adr.yaml'      || ok=0
  printf '%s' "$OUT" | grep -q 'freeze forbids at every layer'      || ok=0
  printf '%s' "$OUT" | grep -q 'route: `rm '                        || ok=0
  if [ "$ok" -eq 1 ]; then echo "  OK    jigc $d — blocks · names adr · names the shadow · states the rule · carries a route";
  else echo "  FAIL  jigc $d — did not block with all four (exit $RC)"; BLOCKED_ALL=1; fi
done
bar "every listed door blocks with the four-part message" "test $BLOCKED_ALL -eq 0"
bar "no adr was created through the blocked create door" "! ls .jigc/tasks/$T/docs 2>/dev/null | grep -q 'adr'"

# Recorded, not asserted: `jigc task list` answers at exit 0 over the same
# shadow. It reads the workbench and resolves no doctype, so it is not on the door
# list above — the measurement is here so a reader does not have to wonder.
TL="$(jigc task list 2>&1)"; TLRC=$?
echo "  MEASURED · jigc task list over the shadow: exit $TLRC — $(printf '%s' "$TL" | head -1)"

say "A · the route, run VERBATIM, clears the block"
ROUTE="$(jigc describe 2>&1 | sed -n 's/.*route: `\(rm [^`]*\)`.*/\1/p' | head -1)"
echo "  route: $ROUTE"
bar "the route is an rm of the shadow file itself, inside this repo's project config" \
    "test \"\$ROUTE\" = \"rm /work/$SHADOW\""
if [ "$ROUTE" = "rm /work/$SHADOW" ]; then
  step bash -c "$ROUTE"
fi
bar "the shadow is gone"                          "test ! -e $SHADOW"
step jigc describe
bar "…and the door that printed the route now opens (describe exit 0)" "jigc describe >/dev/null 2>&1"
bar "…as does the schema read, at the frozen shape (options present)"   "jigc doc schema adr 2>&1 | grep -q 'options: slot'"

# ---------------------------------------------------------------------------
say "B · THE DOCUMENTED CAPABILITY — a presentation-only shadow loads clean"
write_frozen_adr "A house-worded ADR." "House hint: the forces at play."
step jigc describe --doctypes
bar "describe loads (exit 0)"                       "jigc describe >/dev/null 2>&1"
bar "…and shows the reworded description"           "jigc describe --doctypes 2>&1 | grep -q 'adr is A house-worded ADR.'"
bar "doc schema adr loads at the frozen version, options intact" \
    "jigc doc schema adr 2>&1 | grep -q 'doctype: adr (schema-version 2)' && jigc doc schema adr 2>&1 | grep -q 'options: slot'"
bar "validate is clean"                             "jigc validate >/dev/null 2>&1"
bar "the create-gate write goes through"            "jigc doc create adr --title 'Probe the freeze' --task $T >/dev/null 2>&1"
bar "…and the staged doc reads back through --task" "jigc doc show adr:probe-the-freeze --task $T >/dev/null 2>&1"
bar "task validate runs (exit reflects content findings, not a pack-load fault)" \
    "! jigc task validate $T 2>&1 | grep -q 'pack-load freeze check failed'"
rm "$SHADOW"

# ---------------------------------------------------------------------------
say "C · THE DECLARED BOUND — setup over the shape-changing shadow, scored on the four-part standard"
write_shape_dropping_adr
S="$(jigc setup 2>&1)"; SRC=$?
printf '%s\n[exit %s]\n' "$S" "$SRC"
q1=no; q2=no; q3=no; q4="n/a"
printf '%s' "$S" | grep -q 'adr'                       && q1=yes
printf '%s' "$S" | grep -qi 'shadow\|freeze\|block\|refus' && q2=yes
printf '%s' "$S" | grep -q 'route:'                    && q3=yes
score=0; [ $q1 = yes ] && score=$((score+1)); [ $q2 = yes ] && score=$((score+1)); [ $q3 = yes ] && score=$((score+1))
echo
echo "  MEASURED · setup exit: $SRC"
echo "  MEASURED · four-part standard at the setup door:"
echo "             names what it objects to (adr)      : $q1"
echo "             says the consequence (next doors block): $q2"
echo "             names a route                        : $q3"
echo "             the route runs                       : $q4 (no route to run)"
echo "             score: $score/4 — the declared bound, measured; VERDICT.md declares exactly this counter-case"
bar "the bound holds AS DECLARED: setup exits 0 over the shape-changing shadow" "test $SRC -eq 0"
bar "…while the very next door blocks"                                          "! jigc describe >/dev/null 2>&1"
ROUTE="$(jigc describe 2>&1 | sed -n 's/.*route: `\(rm [^`]*\)`.*/\1/p' | head -1)"
[ "$ROUTE" = "rm /work/$SHADOW" ] && bash -c "$ROUTE"
bar "the shadow is cleared again via the printed route" "test ! -e $SHADOW && jigc describe >/dev/null 2>&1"

# ---------------------------------------------------------------------------
say "D · A MIS-KEYED LEAF INSIDE repeatable: — refused at load, at BOTH layers"
say "D1 · the pack layer — a listed house pack whose \`memo\` block carries \`patern:\`"
ORIG_PACKS="$(cat .jigc/config/packs.yaml)"
H="$(mktemp -d /tmp/housepack13-XXXXXX)"
mkdir -p "$H/config" "$H/schemas"
printf 'pack-id: house13\n' > "$H/config/defaults.yaml"
cat > "$H/schemas/memo.yaml" <<'EOF'
type: memo
location: memos/
id-from: title
description: A memo.
usage: a memo is worth keeping.
sections:
  - id: items
    repeatable:
      id-from: title
      block:
        - { id: title, type: string, patern: "^[A-Z]" }
EOF
printf 'compose-embedded-methodology: true\npacks:\n  - %s\n' "$H" > .jigc/config/packs.yaml
step jigc doc schema memo
D1="$(jigc doc schema memo 2>&1)"; D1RC=$?
bar "doc schema memo refuses the load (exit non-zero)"           "test $D1RC -ne 0"
bar "…located at <type>#<section>/<leaf>"                        "printf '%s' \"\$D1\" | grep -q 'memo#items/title'"
bar "…naming the offending key"                                  "printf '%s' \"\$D1\" | grep -q 'unknown field `patern`'"
bar "…and the section is NOT silently erased (no exit-0 answer without it)" \
    "! { test $D1RC -eq 0 && ! printf '%s' \"\$D1\" | grep -q 'items'; }"
step jigc describe
DM="$(jigc describe 2>&1)"; DMRC=$?
bar "the menu door refuses the same pack the same way"           "test $DMRC -ne 0 && printf '%s' \"\$DM\" | grep -q 'memo#items/title'"
printf '%s\n' "$ORIG_PACKS" > .jigc/config/packs.yaml
bar "the pack list is restored and the menu opens again"         "jigc describe >/dev/null 2>&1"

say "D2 · the project layer — the same typo in a \`commit\` shadow (the rc.12 repro, on the shipped block)"
cat > "$SHADOW_DIR/commit.yaml" <<'EOF'
type: commit
description: A Conventional-Commits message.
usage: you need to record what a change does and why at the moment it lands.

sections:
  - id: header
    header: true
    fields:
      - { id: type, type: enum, of: [feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert] }
      - { id: scope, type: string, optional: true }
      - { id: implements, type: ref, to: spec, card: "0..1", inverse: implemented-by }
  - id: summary
    slot: { hint: "The subject line — what changed, imperative mood, <=50 chars." }
  - id: body
    slot: { hint: "Why this change — the motivation and any notable context.", optional: true }
  - id: trailers
    repeatable:
      id-from: key
      block:
        - { id: key, type: string, patern: "^[A-Z]" }
        - { id: value, type: string }
EOF
step jigc doc schema commit
D2="$(jigc doc schema commit 2>&1)"; D2RC=$?
bar "doc schema commit refuses the load (exit non-zero)"     "test $D2RC -ne 0"
bar "…names the shadow file"                                 "printf '%s' \"\$D2\" | grep -q '.jigc/config/schemas/commit.yaml'"
bar "…located at commit#trailers/key"                        "printf '%s' \"\$D2\" | grep -q 'commit#trailers/key'"
bar "…naming the offending key"                              "printf '%s' \"\$D2\" | grep -q 'unknown field `patern`'"
bar "…and trailers is NOT silently erased"                   "! { test $D2RC -eq 0 && ! printf '%s' \"\$D2\" | grep -q 'trailers'; }"
rm "$SHADOW_DIR/commit.yaml"
bar "everything restored: describe and validate open again" "jigc describe >/dev/null 2>&1 && jigc validate >/dev/null 2>&1"

say "SUMMARY"
echo "  layers driven: project shadow (A/B/C, D2) · pack file via a listed house pack (D1) — the two the resolver reads"
echo "  the setup score in C is a measurement of a declared bound, not a verdict"
if [ "$FAIL" -eq 0 ]; then echo "ARM 13 PASS"; else echo "ARM 13 FAIL"; fi
exit "$FAIL"
