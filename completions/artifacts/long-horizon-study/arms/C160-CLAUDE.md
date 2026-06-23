# UI-Doc — project working agreement

This is **UI-Doc**, a TypeScript/pnpm monorepo that turns tag-comment blocks in
source files into component documentation. Read this file before making changes;
it is the shared agreement for how we build, test, document, and ship.

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

## 7. Documentation-consistency discipline

This project keeps its **documentation honest against the code**. Documentation
that names a code symbol which no longer exists is a defect — treat it with the
same seriousness as a failing test.

The public API is documented in two places that must stay in sync with the code:

- **`packages/core/README.md`** — the human-facing guide: tables, per-symbol
  sections, prose, and code examples.
- **`docs/architecture/core-public-api.md`** — the architecture doc: each
  component records the symbol that implements it as
  `implemented-by: <path>#<Symbol>`.

### The rule

Whenever you **rename, move, or remove a public symbol** (an exported class,
function, type, or interface — e.g. `UIDoc`, `CommentBlockParser`,
`MarkdownDescriptionParser`):

1. Update the **code** (the declaration and every usage, including factory
   functions and re-exports).
2. Update the **tests** that reference it.
3. Update **every documentation reference** to it, so no document names a
   symbol that is absent from the code. This includes:
   - the architecture doc's `implemented-by: …#<Symbol>` lines;
   - the README's section headings, prose mentions, **cross-references** (one
     symbol's docs referring to another), and **code examples**.

### How to be sure

After any such change, **search the whole project for the old name** and
confirm no documentation still references it, and that the architecture doc's
`implemented-by` anchors all point at symbols that exist in the code. A
documented symbol that does not resolve in the code is the failure mode this
discipline exists to prevent.

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
