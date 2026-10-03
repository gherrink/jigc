```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC --format json doc show 'nosuchtype:x' 2> show.json; echo "exit $?"   # 1, the envelope on stderr
grep -A2 '"key"' show.json        # "target": "nosuchtype:x"   <- the doc address
$JIGC --format json doc schema nosuchtype 2> schema.json; echo "exit $?"  # 1
grep -A2 '"key"' schema.json      # "target": "nosuchtype"     <- bare, as the contract fixes
```
