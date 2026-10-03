Fixed in M53's last batch (`8a40a5e2`), stamped `1.0.0-rc.21`. The locus takes the declared `task:<id>` spelling, naming the id the mint would have taken.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). In an unborn repository, after `jigc setup` births a root commit, `jigc task amend "root probe"` exits 1 with `amend.head-shape` at `task:root-probe`, and `jigc task amend` exits 1 with it at `task:amend-<sha7>`. `.jigc/tasks` does not exist.
