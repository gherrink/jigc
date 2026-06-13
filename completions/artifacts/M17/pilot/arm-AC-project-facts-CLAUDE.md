## Project

**Galey**

Galey is a schema-first, framework-agnostic block editor for the web, built on top of ProseMirror. It serves two audiences equally — end users editing CMS content and developers integrating it into their own applications — by keeping the core small and shipping almost everything else as declarative extensions.

The name comes from *galley*, the printing tray a compositor used to hold set type before it became a page.

**Core Value:** **The integrator owns the look; the editor owns the structure — and neither side loses data silently.**

When tradeoffs arise, this is the compass: small core, declarative extensions, no magic, never lose data silently. If a feature requires the core to grow or to behave magically, it goes into an extension or it does not ship.

### Constraints

- **Tech stack — locked**: TypeScript, pnpm workspaces, ProseMirror as the editor foundation — Mentioned in `MILESTONES.md` ship gates and `PROJECT_IDEA.md` architecture; non-negotiable.
- **Tech stack — research-resolved**: Test runner (likely Vitest, but confirm), monorepo orchestrator (Turborepo / Nx / none), bundler (tsup / Rollup / Vite library mode), linter (Biome / ESLint+Prettier), TS config conventions for libraries — Edges left open for the project-researcher to confirm against current 2025-2026 best practices.
- **Architecture — small core**: `@galey/core` is vanilla JS + ProseMirror only. Zero framework dependency in the core. React/Vue/vanilla adapters are equal citizens layered on top — Compass principle from `PROJECT_IDEA.md` §1.
- **Architecture — declarative extensions only**: Contributions describe what they add, never imperatively poke at the editor at startup — Enables introspection, debugging, and override. From `PROJECT_IDEA.md` §5.
- **Data integrity — never lose data silently**: Migration failures quarantine the block and preserve raw JSON; constraint violations strip with a structured warning; unknown block types preserve JSON behind a fallback — From `PROJECT_IDEA.md` §8 unifying rule.
- **Free-time cadence**: No external deadline, no estimates, no dates. Ship gates are honest yes/no — From `MILESTONES.md` opening section.
- **Document shape close to ProseMirror native JSON**: Adds only `version` per node and a top-level `meta` — Keeps interop cheap.
- **Class-name configuration is not optional**: Structural CSS ships locked and not-overridable; presentational class names always come from a config map — Avoids Gutenberg's "broken editors that are nearly impossible to debug" failure mode.

## Technology Stack

## Locked decisions (not researched, listed for context only)
| Decision | Status |
|---|---|
| TypeScript (strict) | LOCKED |
| pnpm workspaces | LOCKED |
| ProseMirror as the editor foundation (`prosemirror-state`, `prosemirror-view`, `prosemirror-model`, `prosemirror-transform`, `prosemirror-commands`, `prosemirror-keymap`, `prosemirror-history`) | LOCKED |
| Monorepo packages: `@galey/core`, `@galey/react`, `@galey/vue`, `@galey/vanilla` (later: `@galey/serializer-html`, `@galey/serializer-markdown`) | LOCKED |
| Output format: structured JSON close to ProseMirror's native shape | LOCKED |
| License: open-source (MIT recommended, no compelling reason to deviate) | LOCKED |
## Recommended Stack
### Core Technologies (verified versions, May 2026)
| Technology | Version | Purpose | Why Recommended |
|---|---|---|---|
| TypeScript | 6.0.3 | Type system, compiler, type-only consumer | Locked. v5.x is also fine; v6 is the current latest. Use `strict: true` plus the additional flags below. Type-checking via `tsc --noEmit`; types emitted via `tsc -b` when project references are enabled, or via the bundler's `--dts` mode otherwise. |
| pnpm | 10.33.2 | Workspace manager, install, hoisting strategy, package linking | Locked. `pnpm workspaces` (via `pnpm-workspace.yaml`) is the modern minimum. Lerna is **not** needed in 2026 — pnpm absorbed its responsibilities. |
| Node.js | 22 LTS (Jod) — minimum runtime; CI matrix on 22 + 24 | Runtime for tooling, tests, scripts | Node 22 (Jod) is in active LTS until April 2027 and is the safe baseline for `engines.node` in published packages. Node 20 (Iron) reaches end of support on April 30, 2026. Node 24 (Krypton) is the newest active LTS (since November 2025) and should be on the CI matrix. Set `"engines": { "node": ">=22.12.0" }` in published packages. |
| ProseMirror modules | `prosemirror-model@1.25.4`, `prosemirror-state@1.4.4`, `prosemirror-view@1.41.8`, `prosemirror-transform`, `prosemirror-commands`, `prosemirror-keymap`, `prosemirror-history` | Editor foundation | Locked. Pin to `^1.x` — the core ProseMirror modules are exceptionally stable; semver is honored. |
### Supporting Libraries (research-resolved)
#### Test runner: **Vitest 4.1.5**
| Library | Version | Purpose | Confidence |
|---|---|---|---|
| `vitest` | ^4.1.5 | Unit + integration test runner | **HIGH** |
| `@vitest/browser` | ^4.1.5 | Real-browser test mode (Playwright-driven) for ProseMirror contenteditable cases that DOM emulators can't fake | **HIGH** |
| `jsdom` | ^29.1.1 | Default DOM emulator for the bulk of unit tests | **HIGH** |
| `happy-dom` | ^20.9.0 | *Not recommended for Galey* — see "What NOT to Use" | — |
| `prosemirror-test-builder` | ^1.x | Doc/position helpers for ProseMirror tests | **HIGH** |
- Native TS + ESM, zero ts-jest-style transformer pain.
- Vite-powered, so rebuilds are instant when wired into `tsconfig` paths or workspace package linking.
- First-class workspace support — you can run all `@galey/*` test suites with one command.
- The same runner can run both Node-environment tests (constraint runtime, migration runtime, schema validation, JSON round-trip) and DOM-environment tests (NodeView mounting, plugin behavior, ProseMirror transactions in jsdom) using `// @vitest-environment jsdom` per-file pragmas.
- `@vitest/browser` (Playwright provider) is the escape hatch for the small set of tests that genuinely need contenteditable behavior — see Q8 below.
- *Bun test* — Fast and increasingly capable, but the ProseMirror DOM testing story is rougher and the workspace-test ergonomics in pnpm-managed monorepos are less proven. Re-evaluate at M11+ if perf becomes the bottleneck.
- *Node's built-in `node:test`* — Improving rapidly, but no UI, no built-in DOM environment, and weaker watch-mode DX. Fine for the *core's* pure-function tests in isolation; not worth the split runner config.
- *Jest* — Avoid for new TS+ESM libraries in 2026; the configuration overhead (ts-jest, babel-jest, transformIgnorePatterns) is no longer justified when Vitest exists.
#### Monorepo orchestrator: **None for now — pnpm workspaces only**
| Tool | Verdict | Reason |
|---|---|---|
| pnpm workspaces alone | **RECOMMENDED for M0–M3** | 4 packages with simple build dependencies. No remote cache need. No multi-thousand-task graph. The conventional advice for ≤5 packages is: don't add an orchestrator. |
| Turborepo (`turbo@2.9.7`) | **Add later if needed** (likely M9–M11 once the standard block set + serializers exist) | Excellent local caching, simple `turbo.json` config, well-suited to this kind of repo. Trigger to adopt: build/test pipeline > 30s on a warm machine, or you find yourself writing custom topological scripts. |
| Nx (`nx@22.7.1`) | **Skip** | Strong feature set, but heavier and more opinionated. Galey is a small library set, not an enterprise polyrepo replacement. The plugin/generator surface is overkill. |
| Lerna | **Do NOT use** | Effectively superseded — pnpm workspaces handle linking, Changesets handles publishing, Turborepo handles orchestration. Lerna's role is gone. |
#### Bundler: **tsup 8.5.1** (with a watchful eye on tsdown)
| Tool | Version | Verdict |
|---|---|---|
| `tsup` | ^8.5.1 | **RECOMMENDED for M0** |
| `tsdown` | 0.21.10 | **Watch — reconsider at M11/M12** |
| Rollup | 4.60.2 | **Skip unless tsup hits a wall** |
| Vite library mode | 8.0.10 | **Skip** for headless libraries |
| Bun build | — | **Skip** |
- esbuild-based, near-zero config: `tsup src/index.ts --format esm,cjs --dts --sourcemap --clean` produces ESM + CJS + `.d.ts` with sourcemaps and proper tree-shake hints.
- Generates the right `package.json` `exports` map shape when paired with the conventions below.
- Industry default for "small TS library" — used by thousands of npm packages in this exact niche.
- Each `@galey/*` package gets its own `tsup.config.ts` — they can diverge per package without a shared bundler config becoming a chokepoint.
#### Linter / formatter: **Biome 2.4.13**
| Tool | Verdict | Confidence |
|---|---|---|
| Biome (`@biomejs/biome@2.4.13`) | **RECOMMENDED** | HIGH |
| ESLint 10.3.0 + Prettier 3.8.3 + typescript-eslint 8.59.1 | Acceptable fallback | HIGH |
- Single tool replaces ESLint + Prettier + import-sort + organize-imports. One config, one cache, one CI step.
- Biome v2 ("Biotype", late 2025) introduced type-aware lint rules without spinning up the TypeScript compiler — so `noFloatingPromises`-class rules work, fast.
- v2.3 (January 2026) covers ~491 rules — enough for a vanilla TS library. The ESLint-only differential (Vue / Svelte specific rules, Storybook plugins, custom org plugins) does not affect Galey's surface today.
- Speed matters because the lint step gates every PR (M0 ship gate explicitly requires `pnpm lint`). Biome is 10–20× faster than ESLint+Prettier on equivalent codebases — relevant on first-clone CI runs.
- Galey is *vanilla TypeScript* in the core. The places where ESLint's plugin ecosystem dominates (React-specific rules, Vue parser quirks) only become relevant in `@galey/react` and `@galey/vue` — and Biome's React rule set is sufficient for the adapter packages, which are thin by design.
- `noFloatingPromises` / equivalent — ProseMirror plugins and async migration runners are full of returning-the-transaction-vs-running-it pitfalls.
- `useImportType` / `useExportType` — keeps the `import { Schema } from 'prosemirror-model'` style hygienic; matters for tree-shaking and avoids accidental runtime imports of types-only modules.
- `noExplicitAny` (error, not warn) — ProseMirror's types are precise; reaching for `any` in NodeView/plugin code is a smell.
#### TypeScript config: single base + per-package `extends`, project references **YES** at 4 packages
- The ship-gate triple `pnpm install / build / test / lint` runs on every push. With references, `tsc -b` does incremental builds, and the typecheck step stays under one second on warm runs.
- References enforce package boundaries — they prevent `@galey/core` from accidentally importing from `@galey/react`, which is the **architectural rule** the project must protect at all costs (vanilla core).
- The reference-graph is also documentation: looking at the `references` array in the root `tsconfig.json` tells a new contributor exactly how the packages compose.
- The cost is real but small: each package adds ~6 lines to its `tsconfig.json`. For 4 packages, that's an evening's setup. The compound payoff (faster type-checks, enforced boundaries, clean editor "Go to Definition" across packages) is worth it.
#### Changelog + release tooling: **Changesets**
| Tool | Version | Verdict |
|---|---|---|
| `@changesets/cli` | ^2.31.0 | **RECOMMENDED** |
| semantic-release | — | **Skip** |
| release-please | — | Acceptable alternative, but Changesets fits a free-time project's cadence better |
| Manual versioning | — | Skip — defers a problem you'll regret at M19 |
- Built monorepo-first. Knows that `@galey/core` and `@galey/react` are different release units and handles independent versioning + linked versioning bumps when needed.
- Free-time friendly: you write a one-line markdown changeset when you make a meaningful change; later, when you're ready, `pnpm changeset version` bumps versions and rolls changelogs; `pnpm changeset publish` publishes to npm. No commit-message discipline required (which matters for solo work where you might commit `wip` and `oops`).
- Generates a per-package `CHANGELOG.md` automatically — this becomes the M19 "changelog from M0 to 1.0.0 exists" ship gate practically for free.
- The GitHub Action (`changesets/action`) opens a "Version Packages" PR automatically when changesets accumulate on `main` — push-button releases.
- Built on the assumption of one repo = one package. Monorepo support exists via plugins but is fragile and tag-management-heavy (real risk of git tag collisions).
- Forces conventional commit messages on every commit. Free-time projects don't reliably satisfy that.
- The "PROJECT.md says quality over speed" framing is exactly the case where automated discipline beats memory. Future-you, six months in, will not remember whether you bumped `@galey/core` or not.
#### CI: GitHub Actions
| Tool | Purpose | Notes |
|---|---|---|
| `actions/checkout@v4` | Clone | Standard. |
| `pnpm/action-setup@v4` | Install pnpm | Pin to a major. |
| `actions/setup-node@v4` with `cache: 'pnpm'` | Install Node, cache pnpm store | Built-in caching is sufficient — no need for `actions/cache` separately. |
| `changesets/action@v1` | Release PR + npm publish on push to `main` | Use the `NPM_TOKEN` secret only when M19 is approached. |
- **22 (LTS Jod)** — the minimum supported runtime, must pass.
- **24 (LTS Krypton)** — the current active LTS, also must pass. Catches forward-compat issues early.
- Drop **20** entirely — EOL on April 30, 2026 (already passed by this milestone cycle's start).
- A *browser* matrix is unnecessary at M0. Add Playwright-driven browser tests at M1 only when a single Linux runner has them passing — the cross-OS / cross-browser matrix can come at M11–M12.
- **Cache the pnpm store** (built-in via `setup-node`'s `cache: 'pnpm'`) — ProseMirror has many small modules; cold installs are surprisingly slow.
- **Cache the Vitest run** via `vitest --coverage --coverage.reporter=text` (no special action; just don't use `--no-cache`).
- **Cache Playwright browsers** when `@vitest/browser` is added — `~/.cache/ms-playwright` is the path. Playwright's cache key should include `playwright-version` to invalidate on browser updates.
- **Concurrency:** `concurrency: { group: ${{ github.ref }}, cancel-in-progress: true }` — free-time project, you'll push small fix-up commits and don't want stale CI runs piling up.
#### Browser-test strategy for ProseMirror's `contenteditable`
### Development Tools
| Tool | Purpose | Notes |
|---|---|---|
| `publint` (^0.3.18) | Validate `package.json` shape on each publish | Run in CI on every PR that touches a package. Catches `exports` map mistakes early. |
| `@arethetypeswrong/cli` (^0.18.2) | Validate that `.d.ts` resolves correctly under both ESM and CJS | Run on every PR that changes build output. Pairs with publint. |
| `tsx` (or Node's `--experimental-strip-types` on Node 22+) | Run TS scripts directly without a build | For `migrate(document, { dryRun: true })` CLI from M6, and for ad-hoc scripts. |
| `npm-check-updates` | Periodic dependency upgrade audits | Free-time cadence — run quarterly, not in CI. |
## Installation (M0 bootstrap)
# At the repo root
# Workspace + tooling
# Per-package, in each packages/<name>
# When @vitest/browser comes online (M3+)
## Alternatives Considered
| Recommended | Alternative | When to Use Alternative |
|---|---|---|
| Vitest | Bun test | If/when the suite grows past ~10s on warm runs *and* a Bun runtime is acceptable for tests. Re-evaluate at M11. |
| Vitest | Node `node:test` | If the project ever decides to drop the DOM-test layer entirely (it won't — ProseMirror requires DOM). |
| pnpm workspaces only | Turborepo | When `pnpm -r build` warm time exceeds ~30s. Likely M9–M11. |
| pnpm workspaces only | Nx | If Galey ever spawns ~20+ packages with shared generators. Not in v1.0.0's plan. |
| tsup | tsdown | Once tsdown reaches 1.0 stable (likely 2026 H2) and Galey has a build-time pain point. Migration is cheap. |
| tsup | Rollup direct | Only if a specific output transformation tsup can't express becomes blocking. Hasn't happened in practice. |
| tsup | tsc-only (zshy / no bundler) | If Galey decides to ship ESM-only and accept that integrators using older toolchains get a worse experience. Not the current plan. |
| Biome | ESLint + Prettier | If a critical lint rule from `@typescript-eslint` (specifically a type-aware one not in Biome) becomes load-bearing. Not foreseeable at M0–M3. |
| Changesets | release-please | If you'd rather Google's tool than the changesets community's. Functionally similar at small scale. |
| Project references (composite) | Single root tsconfig + path aliases | At 1–2 packages, you could skip references. With 4 packages and the architectural "core must not import adapters" rule, the boundary enforcement is worth it. |
| jsdom (test DOM) | happy-dom | Speed-sensitive monorepos with simpler DOM needs. ProseMirror's DOM needs are not simple. |
## What NOT to Use
| Avoid | Why | Use Instead |
|---|---|---|
| **Lerna** | Superseded. pnpm workspaces handles linking; Changesets handles publishing; Turborepo handles orchestration. Lerna's role is gone in 2026. | pnpm workspaces (+ Changesets, + Turborepo if needed later) |
| **Jest** (for new TS+ESM libraries) | ts-jest / babel-jest config overhead, slower than Vitest, ESM support remains second-class. The `jest-prosemirror` ecosystem package exists but is a Remirror artifact, not a fit for vanilla ProseMirror. | Vitest |
| **happy-dom** *for ProseMirror specifically* | Trades correctness for speed precisely on Selection/Range/MutationObserver — the surfaces ProseMirror uses most. Vitest's discussion thread #1607 is explicit. | jsdom for DOM unit tests; `@vitest/browser` for contenteditable |
| **ESM-only publishing** at M0 | Library consumers in 2026 still include CJS-only environments (older Jest, older Storybook, some Next.js configs, some Electron apps). The cost of dual-publishing via tsup is near zero. | Dual ESM+CJS via tsup with proper `exports` map |
| **Path aliases without project references** | Works locally, but a `tsc --noEmit` against the whole repo becomes O(everything) instead of O(changed). Boundary enforcement also disappears. | Project references with `composite: true` |
| **Bun test as primary runner** (in 2026) | Maturing but the ProseMirror DOM-test ergonomics are unproven at the scope Galey will hit. | Vitest, with Bun as a *consumption* target left unblocked |
| **tsc as the only build tool** for this dual-format use case | Generates ESM or CJS but not both cleanly without a wrapper script. Doesn't bundle — every internal file gets its own emit. | tsup |
| **Conventional Commits enforcement** (commitlint, husky-blocked commits) | Friction-heavy for free-time work. Changesets is the right discipline layer instead — explicit changeset files when changes matter. | Changesets workflow |
| **Yarn (Classic or Berry/v4)** | Not locked-out, but pnpm is locked in PROJECT.md. Yarn Berry's pnp mode interacts poorly with TypeScript project references in some configs. | Stay with pnpm. |
| **Storybook at M0–M3** | Galey has no UI to demo yet (M3 produces the first toolbar). Storybook adds noise with no payoff yet. | Defer to M11 (standard block set), if at all. |
## Stack Patterns by Variant
- Skip tsup for that package and rely on tsc-only emit through project references
- Keep tsup for the framework adapters and serializers
- Likely never needed; mentioned for completeness
- That single package can drop CJS output (`format: ['esm']` in tsup.config.ts)
- Document it in that package's README
- Don't let one package's choice pull the rest of the monorepo to ESM-only by default
- Add `format: ['esm', 'cjs', 'iife']` to that package's tsup config and a `globalName`
- Only relevant for `@galey/vanilla` if at all
- Defer until a real consumer asks
## Version Compatibility
| Package A | Compatible With | Notes |
|---|---|---|
| `prosemirror-view@1.41.x` | `prosemirror-state@1.4.x`, `prosemirror-model@1.25.x` | All ProseMirror 1.x modules are designed to interop. Pin with `^1.x`. |
| `vitest@4.x` | `jsdom@29.x`, `happy-dom@20.x`, Vite 6/7/8 internally | Vitest 4 ships its own Vite peer; no separate Vite install needed for testing. |
| `tsup@8.5.x` | `typescript@5.x` and `typescript@6.x` | tsup defers `.d.ts` generation to your installed `tsc`. |
| `@biomejs/biome@2.4.x` | `typescript@5.x`, `typescript@6.x` | Biome has its own parser; doesn't peer on TS, but type-aware rules work better with TS 5.5+. |
| `@changesets/cli@2.31.x` | pnpm 9.x and 10.x | Native pnpm workspace support. |
| `pnpm@10.x` | `node@>=20.5` | pnpm 10 requires Node 20.5+; we're targeting 22+ anyway. |
## Confidence Summary
| Recommendation | Confidence | Confirm by |
|---|---|---|
| Vitest 4 over Bun/Jest/node:test | HIGH | M0 — works the moment it's installed |
| jsdom over happy-dom for ProseMirror | HIGH | M1 — first DOM-touching test |
| `@vitest/browser` for contenteditable | MEDIUM-HIGH | M3 — when first toolbar test is written |
| pnpm workspaces only (no Turborepo at M0) | HIGH | Re-evaluate at M9 |
| tsup over tsdown over Rollup over Vite-lib | HIGH for tsup; MEDIUM for the "wait on tsdown" call | M11 — re-evaluate when 5+ packages exist |
| Biome over ESLint+Prettier | HIGH | M9/M10 — re-evaluate if React/Vue need a specific typescript-eslint rule |
| TypeScript project references at 4 packages | HIGH | M0 — boundary-enforcement payoff is immediate |
| Changesets for releases | HIGH | M19 |
| Node 22 baseline, Node 22 + 24 CI matrix | HIGH | M0 |
| MIT license | MEDIUM (assumed unless something compelling argues otherwise — confirm with project owner before M0) | M0 |
| `@galey` npm scope reservation | MEDIUM (PROJECT_IDEA §14 says reserved — verify before M19) | M19 |
## Sources
- npm registry (verified versions, May 2026): `tsup@8.5.1`, `vitest@4.1.5`, `@biomejs/biome@2.4.13`, `turbo@2.9.7`, `@changesets/cli@2.31.0`, `typescript@6.0.3`, `pnpm@10.33.2`, `@vitest/browser@4.1.5`, `jsdom@29.1.1`, `happy-dom@20.9.0`, `prosemirror-state@1.4.4`, `prosemirror-view@1.41.8`, `prosemirror-model@1.25.4`, `eslint@10.3.0`, `prettier@3.8.3`, `typescript-eslint@8.59.1`, `tsdown@0.21.10`, `publint@0.3.18`, `@arethetypeswrong/cli@0.18.2` — HIGH
- [Node.js Releases](https://nodejs.org/en/about/previous-releases) — Node 20 EOL April 2026; Node 22 LTS until April 2027; Node 24 active LTS — HIGH
- [tsup vs tsdown vs unbuild 2026 — PkgPulse](https://www.pkgpulse.com/guides/tsup-vs-tsdown-vs-unbuild-typescript-library-bundling-2026) — bundler comparison — MEDIUM
- [Switching from tsup to tsdown — Alan Norbauer](https://alan.norbauer.com/articles/tsdown-bundler/) — migration realism check — MEDIUM
- [Dual Publishing ESM and CJS Modules with tsup — johnnyreilly](https://johnnyreilly.com/dual-publishing-esm-cjs-modules-with-tsup-and-are-the-types-wrong) — exports-map shape, attw integration — HIGH
- [Biome vs ESLint vs Oxlint: JS Linter 2026 — PkgPulse](https://www.pkgpulse.com/guides/biome-vs-eslint-vs-oxlint-2026) — ecosystem maturity, v2.3 rule count — MEDIUM
- [Biome Replaces ESLint in 2026 — byteiota](https://byteiota.com/biome-replaces-eslint-in-2026-10-20x-faster-linting/) — performance and adoption signals — MEDIUM
- [Vitest discussion #1607: jsdom vs happy-dom](https://github.com/vitest-dev/vitest/discussions/1607) — direct community evidence on ProseMirror-relevant DOM correctness — HIGH
- [happy-dom vs jsdom 2026 — PkgPulse](https://www.pkgpulse.com/blog/happy-dom-vs-jsdom-2026) — speed/correctness tradeoff numbers — MEDIUM
- [Effective Slate Testing — Marcus Wood](https://marcuswood.io/blog/effective-slate-testing-using-react-testing-library/) — explicit "JSDOM does not support contenteditable" — HIGH (corroborated)
- [ProseMirror discuss: Unit testing ProseMirror](https://discuss.prosemirror.net/t/unit-testing-prosemirror/532) — official community guidance — HIGH
- [prosemirror-test-builder](https://github.com/ProseMirror/prosemirror-test-builder) — official test helper — HIGH
- [Patterns for testing components using Lexical](https://github.com/facebook/lexical/discussions/2659) — adjacent-editor testing patterns; Playwright component testing endorsement — MEDIUM
- [TypeScript: Documentation — Project References](https://www.typescriptlang.org/docs/handbook/project-references.html) — official — HIGH
- [Everything You Need to Know About TypeScript Project References — Nx Blog](https://nx.dev/blog/typescript-project-references) — composite/declaration/incremental rationale — HIGH
- [TypeScript Monorepo best practices 2026 — hsb.horse](https://hsb.horse/en/blog/typescript-monorepo-best-practice-2026/) — strict-flag conventions — MEDIUM
- [Monorepo Tools 2026: Turborepo vs Nx vs Lerna vs pnpm Workspaces — viadreams](https://viadreams.cc/en/blog/monorepo-tools-2026/) — sizing recommendations — MEDIUM
- [Turborepo vs Nx vs Moon 2026 — PkgPulse](https://www.pkgpulse.com/guides/turborepo-vs-nx-vs-moon-build-tools-2026) — orchestrator comparison — MEDIUM
- [Changesets — GitHub](https://github.com/changesets/changesets) — official — HIGH
- [Changesets vs Semantic Release — Brian Schiller](https://brianschiller.com/blog/2023/09/18/changesets-vs-semantic-release/) — monorepo tag-collision evidence — HIGH (still current)
- [The Ultimate Guide to NPM Release Automation — Oleksii Popov](https://oleksiipopov.com/blog/npm-release-automation/) — comparison of Changesets / release-please / semantic-release for personal projects — MEDIUM
- [Rules | publint](https://publint.dev/rules) — official — HIGH
- [@arethetypeswrong/cli — GitHub](https://github.com/arethetypeswrong/arethetypeswrong.github.io) — official — HIGH

## Conventions

Conventions not yet established. Will populate as patterns emerge during development.

## Architecture

Architecture not yet mapped. Follow existing patterns found in the codebase.
