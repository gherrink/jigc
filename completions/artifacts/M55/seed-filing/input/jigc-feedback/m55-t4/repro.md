```sh
# from the jigc checkout
dev/gate --quick | grep -E '^gate: (git|       SDKROOT)'
#   gate: git    <the real git, not /usr/bin/git>
#   gate:        SDKROOT … -- the real git, not the /usr/bin/git xcrun trampoline
cargo test -p jigc --test g_migrate dev_gate_report::the_gate_names_the_git_the_suite_runs_under   # 1 passed
```
