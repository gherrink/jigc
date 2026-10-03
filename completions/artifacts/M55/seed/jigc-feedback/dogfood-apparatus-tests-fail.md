---
kind: inconvenience
found-in: milestone:road-to-1.0.0/CI-undeclared-python3
about: test:dogfood_apparatus
jigc-version: 1.0.0-rc.21
status: open
date: 2026-10-03
schema-version: 1
---

# Dogfood apparatus tests fail at spawn without python3 and never skip

## Description

Eight `dogfood_apparatus::*` tests drive a Python apparatus (`log-event.py`, `tally.py`, `jrun`) and needed `python3` without declaring it. In a minimal container they failed at `spawn hook script: Os { code: 2, NotFound }`, while GitHub's `ubuntu-latest` ships `python3`, so CI was green on a dependency it happened to satisfy. M54 Increment 7 landed half of the row: `crates/cli/tests/dogfood_apparatus.rs` names the dependency in its module doc, every spawn takes the interpreter from one constant, `PYTHON`, and `dev/runner-faithful` installs `python3`. What is still owed, keyed to M57, is the other half: a skip with a reason, or a rewrite in something the toolchain already guarantees, decided rather than assumed, since a test that silently skips is its own failure mode.

Re-driven on this build (the checkout at `982f910f`): **still open** on its M57 half. With `python3` off `PATH`, the suite's binary runs `dogfood_apparatus::` to 1 passed and 8 failed, each failure a `NotFound` at spawn (`spawn tally script`, `spawn hook script`, `run jrun`), and none of them is skipped.

## Repro

```sh
# from the jigc checkout: the suite's binary, run on a PATH that carries no python3
BIN=$(cargo test -p jigc --test g_compose --no-run 2>&1 | grep -o 'target/debug/deps/g_compose-[0-9a-f]*')
B=$(mktemp -d "${TMPDIR:-/tmp}/nopy.XXXXXX")
for t in git sh bash env cat; do ln -s "$(command -v $t)" "$B/$t"; done
env PATH="$B" "$BIN" dogfood_apparatus::; echo "exit $?"   # 101: 1 passed; 8 failed
#   spawn hook script: Os { code: 2, kind: NotFound, … }    <- a failure, never a skip
```

## Resolution
