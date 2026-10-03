```sh
rig=$(dev/jigc-rig committed-singletons --start single-task "probe author") || exit; eval "$rig"
printf 'title: "!!!"\n' > payload.yaml
$JIGC doc author adr --from-file payload.yaml; echo "exit $?"
#   1, create.empty-title — `jigc doc create adr` needs a title …   <- a verb the operator did not run
#   route: re-run with a non-empty `--title` …                      <- a flag `doc author` does not take
$JIGC doc author --help | grep -c -- '--title'                     # 0: the door has no such flag
```
