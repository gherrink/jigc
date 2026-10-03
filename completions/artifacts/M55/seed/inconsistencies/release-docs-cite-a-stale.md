---
kind: code-doc
status: resolved
date: 2026-10-03
schema-version: 1
---

# Release docs cite a stale deployment-policy id

## Sides

### implementation/release.md:62  {#side1}

The `release` environment's deployment policy admits branch `main` only, *policy 61391261*. A dated correction says that it had cited 61320798.

### GET repos/gherrink/jigc/environments/release/deployment-branch-policies  {#side2}

The GitHub API lists one policy: `61391261`, of type `branch`, named `main`.

### completions/artifacts/M54/planning-gate-record.md:104  {#side3}

M54's planning gate-record: *branch `main` only (policy id 61320798 …)*. It is a dated record.

## Description

`implementation/release.md` cited the `release` environment's `main` deployment policy as 61320798, the figure M54's planning gate-record carries. The environment's one policy is 61391261. The planning register took the row from the M54 build.

## Evidence

```sh
gh api repos/gherrink/jigc/environments/release/deployment-branch-policies \
  --jq '.total_count, (.branch_policies[] | "\(.id):\(.type):\(.name)")'   # 1, then 61391261:branch:main
grep -c 'policy 61391261' implementation/release.md                         # 1
```

## Resolution

Corrected in `release.md` by M54 Increment 9 / T4 (`e9e52ee4`, 2026-09-30), with a dated bracket: *[Corrected 2026-09-30, M54 Increment 9: this cited the `main` policy as 61320798; the environment's only policy, read by the API, is 61391261]*. The correction predates the register, which recorded the row as open on 2026-10-02 from notes the M54 build took before the fix. The gate-record keeps 61320798, because it is a dated record. UNPINNED: no test reads the policy id, because it lives in GitHub's settings and not in the repository.

Re-driven on 2026-10-03: the API lists one policy, `61391261:branch:main`, and `release.md:62` cites 61391261.
