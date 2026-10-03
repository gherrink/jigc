---
kind: bug
found-in: review:M52-per-axis/(1,A1-N3)
about: write.slug-name-ceiling
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Flatten-exemption list omits write.slug-name-ceiling

## Description

This is a record defect. A carried deferral's list of the flatten exemptions is falsified by a fourth family. `implementation/decisions-pending.md` carries M52's completion-audit row **(b)**. It says `command-output-contract.md`'s rule, *a reject that carries a finding takes the findings arm*, is broader than the binary. It names `store.malformed-slug`, `pack.resource-missing` and the `workflow-refs.*` flatten path as the deliberate exemptions. There is a fourth, in the one family the contract's own target-form table lists under a declared URI target, `write.*`. At one door, `jigc doc set-field`, with one argument, `write.not-present` takes the `{findings, schema_version}` arm with a key. `write.slug-name-ceiling` takes `{error}`. It carries a locus in the declared normal form and a route, so the exemption's two M50 grounds do not transfer to it. The binary's behaviour is not new. The enumeration in the record is falsely complete.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. With `--format json`, `doc set-field` over a missing item returns `{findings, schema_version}` with `write.not-present` keyed. Over a 300-byte slug head it returns `{"error": "blocking · write.slug-name-ceiling — …\n  at: roadmap:<head>#…\n  route: …"}`. decisions-pending's row (b) still names the three exemptions. *READ, not driven:* the record half, by reading decisions-pending.md at this commit.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC start --workflow single-task "axis one"
$JIGC doc set-field "roadmap:roadmap#milestones/m-beta/title" --value X --task axis-one --format json
#   exit 1  {"schema_version": 3, "findings": [{"code": "write.not-present", "key": {…}, …}]}
A=$(printf 'a%.0s' $(seq 1 300))
$JIGC doc set-field "roadmap:$A#milestones/m-alpha/title" --value X --task axis-one --format json
#   exit 1  {"error": "blocking · write.slug-name-ceiling — the `<slug>` head of address … is
#            300 bytes — over the 165-byte ceiling …\n  at: …\n  route: …"}
```

## Resolution
