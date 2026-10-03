```sh
rig=$(dev/jigc-rig committed-singletons --start quick-fix "seam probe") || exit; eval "$rig"
$JIGC doc set-field commit:seam-probe#header/type --value fix
printf 'seam\n' | $JIGC doc set-slot commit:seam-probe#summary --from-file -
echo w > work.txt; git add work.txt
$JIGC task validate seam-probe                # exit 0
REAL_GIT=$(command -v git); mkdir -p "$RIG/shim"
cat > "$RIG/shim/git" <<EOS
#!/bin/sh
if [ ! -e "$RIG/shim/fired" ]; then
  case " \$* " in *" add "*) touch "$RIG/shim/fired"
    "$REAL_GIT" -C "$REPO" bisect start; "$REAL_GIT" -C "$REPO" bisect bad;; esac
fi
exec "$REAL_GIT" "\$@"
EOS
chmod +x "$RIG/shim/git"
PATH="$RIG/shim:$PATH" $JIGC task finalize seam-probe     # exit 1
#   blocking · repo.operation-in-progress — a bisect is in progress — the repository is
#     not in a committable state
#     route: conclude it, or abandon it with `git bisect reset`, then re-run this command
git rev-parse HEAD                            # unmoved
git diff --cached --name-only                 # work.txt, still staged
ls .jigc/tasks/seam-probe/docs/               # commit:seam-probe.md provenance.json
```
