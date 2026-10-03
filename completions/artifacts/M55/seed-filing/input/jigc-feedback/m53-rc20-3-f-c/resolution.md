Fixed in M53's last batch (`8a40a5e2`), stamped `1.0.0-rc.21`: the mint refusal names the exit only this door has.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). `jigc task amend ""` exits 1 with `write.unslugable-title`, routed *… or omit the title — `jigc task amend` names the task after the commit it rewrites (`amend-<sha7>`)*, and `jigc task list` reports no active tasks. `jigc task amend` with no intent then exits 0 and mints `amend-<sha7>`. The control, `jigc milestone create ""`, routes at the title alone, which is right there.
