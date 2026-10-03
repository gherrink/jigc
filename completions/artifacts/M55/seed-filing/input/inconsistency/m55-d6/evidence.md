```sh
grep -n 'enum D/I' design/methodology-docs.md        # 30, read in the checkout
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC doc schema deferral-ledger | grep 'kind:'      # kind: enum [Decision|Idea] …
```
