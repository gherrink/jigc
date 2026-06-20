# M29 F8 — live owner-driven hardening on a real project (project-alpha-2.0)

The F8 acceptance posture (DECISIONS 2026-06-20 M29 planning) deferred the literal
live-repo run as an owner-driven hardening step *after* the build. This artifact
records that run: M29's YAML/docker-compose `symbol-exists` driven through the
**production installed binary** against the **real `docker-compose.yml`** of the
user's `project-alpha-2.0` project.

## Binary under test (production resolution path — no env overrides)

| | sha256 |
|---|---|
| `~/.local/bin/jigc` | `c188fd0761ac6889c7734b0364dafe6f6bda0969c273abff5f9808e2fa311f81` |
| `~/.local/bin/doc-code` (sibling, eighth grammar) | `d99fc650e27ce2d46a469585513c0cabf84477af8b95241034a8a15bf9f01e2e` |

Driven via the arch-doc finalize flow (`jigc start --workflow architecture-documentation`
→ `jigc doc create/add-item/set-field` → `jigc task finalize`) in throwaway git
clones of project-alpha-2.0, `JIGC_DOC_CODE_PROBE`/`JIGC_PACK_DIR` unset, isolated
`HOME` — the actually-shipped embedded-pack + sibling-probe path. Two
`milestone-e2e-tester` agents, in parallel, on separate clones.

## Why this repo is a strong target

The real `docker-compose.yml` exercises every M29 edge in the wild: hyphenated
service keys (`php-fpm`, `php-scheduler`), a YAML **anchor** `x-php-env: &php-env`
(the key is `x-php-env`; `php-env` is only the anchor name), `<<: [*php-env]`
**merge keys** + **aliases**, and **flat-namespace collisions** — `broadcast` and
`db` are *both* `services:` keys *and* `networks:` keys.

## Results — M29 holds (no contract deviations, no crashes)

**Marquee flow:**
- **PASS** — `docker-compose.yml#nginx`, `#php-fpm` (hyphenated), `#x-php-env`
  (extension key) all accept at write and resolve at finalize → exit 0, promoted,
  no `doc-code` block.
- **BLOCK + per-item disambiguation** (both directions) — renaming `nginx:` →
  exit 3, blocks naming only `…#components/web-frontend-nginx/implemented-by`;
  symmetric on `php-fpm`. Other components stay silent.
- **Keys, not values** — after renaming the `php-fpm:` key, the value
  `hostname: 'php-fpm'` still in the file did **not** make `#php-fpm` resolve.
- **Live-resolution witness** — restoring the key lets the same arch-doc finalize
  clean (the probe reads the working tree, not a cached result).

**Over-match census in the wild:**
- **Headline (one finalize run, same file):** the anchor `docker-compose.yml#php-env`
  **BLOCKS**; the real key `docker-compose.yml#x-php-env` **PASSES**. The
  `[key]`-field constraint excludes the anchor name and admits the carrying key.
- **Flat-namespace collision confirmed (honest bound, not a defect):** `#broadcast`
  and `#db` resolve — each is both a service and a network key; the schema-blind
  extractor cannot distinguish them, exactly as documented.
- **`.yml` dispatch (F7):** `docker-compose.dev.yml#mailpit` resolves on the dev file.
- **Vanished key blocks; merge-key `<<` admitted-harmless; hostile inputs
  panic-free; findings byte-identical across re-runs (deterministic).**

**Verdict: `overall_pass: true`.** M29 validates against reality on the actual
project-alpha-2.0 compose stack. F8 hardening complete.

## Two pre-existing CLI papercuts surfaced (NOT M29 defects — filed + fixed)

The hardening, being a real end-to-end drive, surfaced two non-M29 front-door
behaviors (the YAML extractor is untouched by both):

1. **`jigc start --workflow <X>` for a `creates-task: true` workflow, with no
   `<intent>` positional, silently returns orientation** instead of rejecting with
   an "intent required" message. The design contract (write-commands.md:57,62) is
   that such a form needs the intent; the silent fallback gives no feedback.
2. **A blocking `jigc --format json task finalize` writes the JSON report to
   stderr with empty stdout** (exit 3) — so `… finalize > report.json` yields an
   empty file on a block. Machine output belongs on stdout regardless of exit code.

Both fixed post-hardening (test-first, one commit each) — see DECISIONS
2026-06-20 M29 F8 hardening.
