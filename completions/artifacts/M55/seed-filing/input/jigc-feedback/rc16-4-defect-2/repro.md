```sh
rig=$(dev/jigc-rig vendored) || exit; eval "$rig"      # docs/specs/ and docs/architecture/ to move
chmod 0555 .jigc/config
$JIGC config set docs-root documentation --format json  # exit 1
#   "code": "config.repoint-failed",
#   "message": "`docs-root` was not set to `documentation`: could not write
#               <absolute path of $REPO>/.jigc/config/manifest.yaml: Permission denied … —
#               the re-point was undone",
#   "location": { "address": ".jigc/config/manifest.yaml", … }
chmod 0755 .jigc/config; git status --short             # empty: the rollback is clean
```
