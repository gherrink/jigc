```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
T=$($JIGC start --workflow report-jigc-feedback "file it" | sed -n 's/^task minted: //p')
$JIGC doc create jigc-feedback --title "Probe finding" --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/kind --value bug --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/found-in --value task:probe --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/jigc-version --value 1.0.0-rc.22 --task $T > /dev/null
printf 'Observed.\n' | $JIGC doc set-slot jigc-feedback:probe-finding#description --from-file - --task $T > /dev/null
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value findings --task $T > /dev/null
printf 'file it\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
printf 'why\n' | $JIGC doc set-slot commit:$T#body --from-file - --task $T > /dev/null
$JIGC task finalize $T > /dev/null 2>&1; echo "lands: exit $?"            # 0
$JIGC doc show jigc-feedback:probe-finding --format json > "$RIG/show.json"
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["title"])' "$RIG/show.json"   # Probe finding
$JIGC doc list jigc-feedback --format json > "$RIG/list.json"
python3 -c 'import json,sys; r=json.load(open(sys.argv[1]))["docs"][0]; print(r["title"], r["fields"]["status"], r["fields"]["found-in"])' "$RIG/list.json"
#   Probe finding open task:probe        <- one listing, no show per doc
```
