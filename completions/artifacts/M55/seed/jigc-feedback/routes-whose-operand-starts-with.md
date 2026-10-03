---
kind: bug
found-in: review:M52-per-axis/(1,A1-N1)
about: reconciliation.rename
jigc-version: 1.0.0-rc.16
status: open
tier: tier-2
date: 2026-10-03
schema-version: 1
---

# Routes whose operand starts with a dash dead-end at exit 2

## Description

A route whose operand's first byte is `-` is re-read as an option by the receiving CLI, so the command the door prints dead-ends at exit 2. No bytes are lost; the recovery is. The shell is not the fault: `engine::finding::shell_safe`'s inert alphabet admits `-`, and a leading-`-` token does survive a shell as itself. The receiving parser is what reads it as a flag. The review found two producers. One is `reconciliation.rename`'s `jigc unmanage <path>` route, under a `docs-root` whose first byte is `-`, which `jigc config set` admits. The other is `migrate.source-untracked`'s `jigc migrate <path> --as <doctype>` re-run, over an ordinary file named `-dash-note.md`, with no knob set.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. The `unmanage` route, run verbatim, exits 2 with `error: unexpected argument '-/' found`. At `migrate`, a source typed as `./-dash-note.md` is now echoed back as typed, and that re-run lands. A source typed as `-- -dash-note.md` makes the door emit `jigc migrate -dash-note.md --as adr`. The debug binary's route fence catches that span and panics at exit 101, naming it as a relative operand.

## Repro

```sh
src=$PWD                                      # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC config set docs-root -                  # exit 0: the knob admits it
git add .jigc/config && git commit -qm "docs-root dash"
$JIGC start --workflow single-task "dash"
$JIGC doc create adr --title "Dash Probe"
for sl in context decision consequences; do
  printf 'Prose.\n' | $JIGC doc set-slot "adr:dash-probe#$sl" --from-file -
done
$JIGC doc set-field commit:dash#header/type --value docs
printf 'dash\n' | $JIGC doc set-slot commit:dash#summary --from-file -
$JIGC task finalize dash                      # exit 0, promoted -/decisions/dash-probe.md
git rm -q -- -/decisions/dash-probe.md
$JIGC validate                                # exit 1, reconciliation.rename
#   route: … `jigc unmanage -/decisions/dash-probe.md`
$JIGC unmanage -/decisions/dash-probe.md      # the route, verbatim
#   error: unexpected argument '-/' found     exit 2
$JIGC unmanage -- -/decisions/dash-probe.md   # the form no surface prints: exit 0

# the second producer, no knob set
rig=$("$src"/dev/jigc-rig committed-singletons) || exit; eval "$rig"
printf '# D\n' > ./-dash-note.md
$JIGC migrate --as adr -- -dash-note.md
#   debug build: panic, exit 101 — "a route's `jigc migrate` span must name a path that
#   resolves from anywhere: `jigc migrate -dash-note.md --as adr` …"
```

## Resolution
