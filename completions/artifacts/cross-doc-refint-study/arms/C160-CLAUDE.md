# UI-Doc — project working agreement

This is **UI-Doc**, a TypeScript/pnpm monorepo that turns tag-comment blocks in
source files into component documentation. Read this file before making changes;
it is the shared agreement for how we build, test, document, and ship. Among other things, this repository keeps its architectural **decision records** under `docs/decisions/` and an architecture overview under `docs/architecture/`; the cross-reference integrity discipline below is load-bearing.

The workspace has five packages under `packages/*` plus `demos`:
`core` (the engine), `html-renderer`, and the `node` / `rollup` / `vite`
integration plugins. `@ui-doc/core` is the heart — a parser + transformer
pipeline driven by tag transformers.

## 1. Toolchain & environment

- **Node 20** (`.nvmrc` pins `v20`). Use `nvm use` before working.
- **pnpm 9.1.0** is the only supported package manager. Never use `npm` or
  `yarn` here — the lockfile is `pnpm-lock.yaml` and mixing managers corrupts it.
- Install with `pnpm install`. Build all packages with `pnpm workspace:build`.
- Run the full test suite with `pnpm workspace:test`; lint with `pnpm lint`.
- Do not commit `dist/` or `node_modules/`. Generated output stays out of git.

## 2. TypeScript conventions

- `strict` is on. Do not introduce `any` — prefer `unknown` + narrowing, or a
  precise type. A new `any` must be justified in a comment.
- Public API types live in dedicated `*.types.ts` files; runtime code imports
  types from there. Keep types and implementation separable.
- Prefer `type` aliases for unions/utility shapes and `interface` for object
  contracts that may be implemented by classes.
- No default exports for utilities; the tag-transformer modules are the one
  established exception (they export `default tag`).
- Avoid enums; use string-literal unions unless an enum is already established.

## 3. Code style

- Formatting is enforced by Prettier (`pnpm prettier`) — never hand-format; run
  the tool. Lint with ESLint (`pnpm lint:js`); fix with `pnpm fix:js`.
- Naming: `PascalCase` for classes/types/interfaces, `camelCase` for
  functions/variables, `SCREAMING_SNAKE_CASE` for module-level constants.
- Keep functions small and single-purpose. Extract a named helper rather than
  nesting more than two levels of control flow.
- No unused imports or variables — the lint gate rejects them.
- Prefer early returns over deep `else` ladders.

## 4. Imports & module boundaries

- Within a package, import by relative path; across packages, import by the
  package name (`@ui-doc/core`), never by reaching into another package's
  `src/`.
- The `core` package must not depend on any renderer or plugin package — the
  dependency arrow points inward. Renderers and plugins depend on `core`, not
  the reverse.
- Barrel files (`index.ts`) re-export the public surface; internal-only modules
  are not re-exported.

## 5. Error handling

- Throw typed errors from `src/errors/` (e.g. `TagTransformerError`), not bare
  `Error`, when the failure is domain-specific and a caller might catch it.
- Never swallow errors silently. If you catch, either handle meaningfully or
  rethrow with context.
- Validate external/source input at the boundary (the parser); trust internal
  data past that point.

## 6. Testing discipline

- Tests use **Jest** and live in each package's `tests/` directory, mirroring
  the `src/` layout. A `src/Foo.ts` is tested by `tests/foo.test.ts`.
- Every bug fix lands with a regression test that fails before the fix.
- Test names describe behavior: `should <do X> when <condition>`.
- Keep tests deterministic — no real clock, no network, no filesystem unless
  the unit under test is about the filesystem.
- Run `pnpm workspace:test` before opening a PR; a red suite blocks merge.

## 7. Cross-reference integrity discipline

This project keeps a set of **decision records** under `docs/decisions/` (one Architecture
Decision Record per file) and a **`docs/architecture/`** overview that ties them together.
These documents **reference each other**, and the references must always resolve:

- An ADR may record that it **supersedes** an earlier decision, via a
  `supersedes: adr:<slug>` field in its front-matter.
- The architecture overview **cites** the decisions that govern the system, via a
  `cites: [adr:<slug>, …]` field in its front-matter.

Each `adr:<slug>` names another decision file (`docs/decisions/<slug>.md`). A reference that
names a decision which no longer exists is a defect — treat a dangling cross-reference with
the same seriousness as a failing test.

### The rule

Whenever you **delete, rename, split, or otherwise restructure any decision record**, keep
**every cross-reference across all documents valid** — no `supersedes:` and no `cites:`
entry, in any file, may point at a decision slug that no longer exists. **This includes
references in documents you were not asked to touch.**

### How to be sure

After any such change, **search the whole `docs/` tree for the old slug** and confirm no
`supersedes:` or `cites:` field anywhere still names a decision that is now absent. Update or
remove every reference you find. A cross-reference that does not resolve to an existing
decision is the failure mode this discipline exists to prevent.

## 8. Tag transformers

- A tag transformer is a module under `packages/core/src/tag-transformers/`
  exporting `{ name, transform }` as `tag` and registered in `index.ts`.
- Tag names are kebab-case in author-facing docs; the registry array order is
  not significant.
- A new tag ships with: the transformer, its registration, a unit test, and a
  `### @tag` section in the core README.

## 9. Commit conventions

- Conventional Commits: `type(scope): subject`, imperative, ≤ 50 chars subject.
- Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`,
  `build`, `ci`, `chore`, `revert`.
- One logical change per commit. Do not mix a refactor with a feature.
- Reference the affected package in the scope (`feat(core): …`).

## 10. Dependencies

- Add a dependency only when it earns its weight; prefer the platform/std lib.
- Runtime deps go in the owning package's `package.json`, not the workspace root.
- Pin ranges conservatively; run `pnpm security` before adding anything new.

## 11. Performance

- The parser runs over every source file — keep per-block work allocation-light.
- Avoid quadratic scans over the block list; index by key where lookups repeat.
- Measure before optimizing; don't trade readability for unproven speed.

## 12. Backwards compatibility

- The `@ui-doc/*` packages are published. A breaking change to a public export
  requires a changeset and a major bump — flag it in the PR.
- Deprecate before removing where feasible; keep a re-export shim for one minor.

## 13. Pull-request checklist

Before requesting review, confirm: build passes, lint clean, tests green,
no stray `console.log`, the change is scoped to one concern, and any public-API
change is reflected in docs and a changeset.

## 14. Gotchas

- The `prepare` script builds on install — a broken `core` build blocks the
  whole workspace install. Keep `core` compiling.
- `demos` is not published; it may import from `packages/*` freely for examples.
- CSS/asset copying is handled by the rollup/vite plugins, not `core`.
