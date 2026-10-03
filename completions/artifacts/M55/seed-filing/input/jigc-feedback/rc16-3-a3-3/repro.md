```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
export PATH="$(dirname "$JIGC"):$PATH"
jigc milestone create "Area probe"
jigc milestone add-task area-probe one --workflow report-jigc-feedback
jigc milestone provision area-probe; jigc milestone execute area-probe
( cd .jigc/worktrees/one &&
  jigc doc create jigc-feedback --title "Probe row" --task one &&
  jigc doc set-field jigc-feedback:probe-row#meta/kind --value bug --task one &&
  jigc doc set-field jigc-feedback:probe-row#meta/found-in --value trial:probe --task one &&
  jigc doc set-field jigc-feedback:probe-row#meta/jigc-version --value 1.0.0-rc.22 --task one &&
  printf 'Seen.\n' | jigc doc set-slot jigc-feedback:probe-row#description --from-file - --task one )
printf 'mine\n' > .jigc/milestones/area-probe/notes.txt     # a byte jigc did not write
jigc milestone finalize area-probe            # exit 0
#   note: the working area held 1 entry jigc did not write … they were moved aside, not taken:
#     .jigc/milestones/area-probe/notes.txt → .jigc/displaced/area-probe/notes.txt
cat .jigc/displaced/area-probe/notes.txt      # mine
```
