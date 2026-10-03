```sh
src=$PWD                                         # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh 2>&1); echo "capture: exit $?"   # 0
eval "$rig" > /dev/null 2>&1; echo "eval: exit $?"               # 1: the construction log is not shell
echo "REPO=[$REPO]"                              # REPO=[]
test "$PWD" = "$src" && echo "still in the jigc checkout"        # git -C "$REPO" … would act here
```
