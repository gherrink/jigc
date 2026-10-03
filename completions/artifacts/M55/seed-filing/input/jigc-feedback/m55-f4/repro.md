```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
git clone -q --bare "$REPO" "$RIG/origin.git"; git remote add origin "$RIG/origin.git"
git fetch -q origin; git branch -q --set-upstream-to=origin/main main
git clone -q "$RIG/origin.git" "$RIG/mate"                              # a teammate's clone
sed -i.bak 's/^A deterministic CLI assembles exactly the context a task needs\.$/&\
A teammate'"'"'s pulled line./' "$RIG/mate/VISION.md"; rm "$RIG/mate/VISION.md.bak"   # a line in the thesis
git -C "$RIG/mate" -c user.name=Mate -c user.email=mate@example.com commit -qam "docs: teammate edit"
git -C "$RIG/mate" push -q origin main; git pull -q --ff-only            # the pull
T=$($JIGC start --workflow single-task "sharpen the open questions" | sed -n 's/^task minted: //p')
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value vision --task $T > /dev/null
printf 'sharpen the open questions\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
printf 'Seed probe prose, written by the task.\n' | $JIGC doc set-slot vision:vision#open-questions --from-file - --task $T > /dev/null
$JIGC task validate $T; echo "validate: exit $?"     # 0, advisory reconciliation.absorb at VISION.md
$JIGC task finalize $T > /dev/null 2>&1; echo "finalize: exit $?"           # 0
git show HEAD:VISION.md | grep -c -e "A teammate's pulled line." -e "Seed probe prose"   # 2: both sides land
```
