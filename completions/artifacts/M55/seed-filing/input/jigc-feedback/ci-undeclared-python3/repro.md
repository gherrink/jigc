```sh
# from the jigc checkout: the suite's binary, run on a PATH that carries no python3
BIN=$(cargo test -p jigc --test g_compose --no-run 2>&1 | grep -o 'target/debug/deps/g_compose-[0-9a-f]*')
B=$(mktemp -d "${TMPDIR:-/tmp}/nopy.XXXXXX")
for t in git sh bash env cat; do ln -s "$(command -v $t)" "$B/$t"; done
env PATH="$B" "$BIN" dogfood_apparatus::; echo "exit $?"   # 101: 1 passed; 8 failed
#   spawn hook script: Os { code: 2, kind: NotFound, … }    <- a failure, never a skip
```
