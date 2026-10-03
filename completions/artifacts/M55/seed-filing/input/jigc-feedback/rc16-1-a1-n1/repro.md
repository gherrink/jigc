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
