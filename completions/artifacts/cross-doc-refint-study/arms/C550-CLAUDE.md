# UI-Doc — contributor & agent working agreement

This is **UI-Doc**, a TypeScript/pnpm monorepo that turns tag-comment blocks in
source files into component documentation. This file is the shared agreement for
how we build, test, document, and ship the project. Read it before making
changes; when in doubt, prefer the convention already established in the
surrounding code over a personal preference. Among other things, this repository keeps its architectural **decision records** under `docs/decisions/` and an architecture overview under `docs/architecture/`; the cross-reference integrity discipline below is load-bearing.

The workspace has five packages under `packages/*` plus `demos`:
`core` (the engine), `html-renderer`, and the `node` / `rollup` / `vite`
integration plugins. `@ui-doc/core` is the heart — a parser + transformer
pipeline driven by tag transformers.

## 1. Repository layout

```
packages/
  core/            the parsing + transformation engine (@ui-doc/core)
  html-renderer/   the default renderer (@ui-doc/html-renderer)
  node/            the Node file-system driver + CLI glue (@ui-doc/node)
  rollup/          the Rollup plugin (@ui-doc/rollup)
  vite/            the Vite plugin (@ui-doc/vite)
demos/             runnable examples; never published
doc-assets/        shared static assets for the rendered docs
shared/            tsconfig + lint config shared across packages
docs/architecture/ living architecture docs (kept honest against the code)
```

- Each package owns its own `package.json`, `tsconfig.json`, and `tests/`.
- The root `package.json` holds only workspace-wide scripts and dev tooling.
- Generated output (`dist/`, coverage, `node_modules/`) is never committed.

## 2. Architecture overview

The pipeline is four stages, and every contribution should be locatable in one
of them:

1. **Parsing** — `CommentBlockParser` reads source files and extracts raw
   comment blocks (a block is a doc-comment plus the code it annotates).
2. **Transformation** — registered *tag transformers* turn each tag in a block
   (`@code`, `@color`, `@example`, …) into structured context nodes.
3. **Context assembly** — the transformed nodes are merged into a `Context`,
   the renderer-agnostic data model of the whole documentation set.
4. **Rendering** — a `Renderer` (the HTML renderer by default) emits the final
   artifacts from the `Context`.

`core` owns stages 1–3 and the `Context` contract; renderers own stage 4. The
dependency arrow points inward: renderers and plugins depend on `core`, never
the reverse.

### Stage boundaries in detail

- **Parsing** is the only stage that touches raw source text. Everything
  downstream works on structured data, never on strings of source.
- **Transformation** is where domain knowledge about each tag lives. A
  transformer never reads a file and never knows about the renderer.
- **Context assembly** is a pure merge — it must be deterministic for a given
  set of transformed nodes, so two runs over the same input produce byte-equal
  context.
- **Rendering** is the only stage allowed to emit HTML, CSS, or files. If a
  string of markup appears in `core`, a boundary has been crossed.

### Why the boundaries matter

Keeping these stages separable is what lets a third party ship an alternative
renderer without forking the engine. Every change should preserve the property
that `core` could drive a renderer it has never heard of.

## 2a. Per-package responsibilities

- **`@ui-doc/core`** — parsing, the tag-transformer registry, context
  assembly, and the public type contracts. No I/O beyond reading source handed
  to it; no rendering.
- **`@ui-doc/html-renderer`** — the reference renderer; turns a `Context` into
  HTML + assets. Owns all presentational decisions.
- **`@ui-doc/node`** — the file-system driver: discovers source files, reads
  them, and writes rendered output. The only package that does real disk I/O by
  default.
- **`@ui-doc/rollup`** / **`@ui-doc/vite`** — build-tool plugins that run the
  pipeline as part of a bundler build and handle asset copying.

When unsure where code belongs, ask which stage it serves and put it in the
package that owns that stage.

## 3. Toolchain & environment

- **Node 20** (`.nvmrc` pins `v20`). Use `nvm use` before working.
- **pnpm 9.1.0** is the only supported package manager. Never use `npm` or
  `yarn` here — the lockfile is `pnpm-lock.yaml` and mixing managers corrupts it.
- Install with `pnpm install`. The `prepare` script builds `core` on install,
  so a clean clone is build-ready.
- Use a POSIX shell for the scripts below; they are not tested on PowerShell.

## 4. Local development workflow

- `pnpm workspace:build` — build every package in dependency order.
- `pnpm workspace:test` — run the full Jest suite across packages.
- `pnpm --filter @ui-doc/core test` — run one package's tests.
- `pnpm --filter @ui-doc/core build --watch` — rebuild `core` on change.
- `pnpm lint` / `pnpm fix:js` — lint and autofix.
- `pnpm prettier` — format; never hand-format.
- Use the `demos/` package to exercise an end-to-end change against real input.

## 5. Build system

- Each package builds with `tsc` against its own `tsconfig.json`, extending the
  shared base in `shared/`.
- Build order is enforced by pnpm's topological sort; `core` builds first.
- The `prepare` lifecycle script builds `core` on `pnpm install` — a broken
  `core` build blocks the whole workspace install, so keep `core` compiling.
- Do not import from another package's `dist/`; import the package name and let
  the workspace resolution map it to source during development.

## 6. TypeScript conventions

- `strict` is on. Do not introduce `any` — prefer `unknown` + narrowing, or a
  precise type. A new `any` must be justified in a comment.
- Public API types live in dedicated `*.types.ts` files; runtime code imports
  types from there. Keep types and implementation separable.
- Prefer `type` aliases for unions/utility shapes and `interface` for object
  contracts that may be implemented by classes.
- No default exports for utilities; the tag-transformer modules are the one
  established exception (they export `default tag`).
- Avoid enums; use string-literal unions unless an enum is already established.
- Exported functions and classes carry a doc comment; internal helpers may omit
  one when the name is self-describing.

## 7. Code style

- Formatting is enforced by Prettier (`pnpm prettier`) — never hand-format; run
  the tool. Lint with ESLint (`pnpm lint:js`); fix with `pnpm fix:js`.
- Naming: `PascalCase` for classes/types/interfaces, `camelCase` for
  functions/variables, `SCREAMING_SNAKE_CASE` for module-level constants.
- Keep functions small and single-purpose. Extract a named helper rather than
  nesting more than two levels of control flow.
- No unused imports or variables — the lint gate rejects them.
- Prefer early returns over deep `else` ladders.
- Keep line length within the Prettier print width; let the formatter wrap.

## 8. Imports & module boundaries

- Within a package, import by relative path; across packages, import by the
  package name (`@ui-doc/core`), never by reaching into another package's
  `src/`.
- The `core` package must not depend on any renderer or plugin package — the
  dependency arrow points inward. Renderers and plugins depend on `core`, not
  the reverse.
- Barrel files (`index.ts`) re-export the public surface; internal-only modules
  are not re-exported.
- Avoid circular imports; if two modules need each other, extract the shared
  contract into a `*.types.ts` both can import.

## 9. The block & parser model

- A *block* pairs a doc-comment with the source construct it documents; see
  `Block.types.ts` for the shape.
- `CommentBlockParser` is event-driven: it emits parse events that downstream
  consumers subscribe to, rather than returning a monolithic tree.
- Parser errors are raised as typed errors (see §12) and carry the source
  location so the caller can report it.
- The parser is allocation-sensitive — it runs over every source file. Keep
  per-block work light and avoid retaining references to whole source strings.

## 10. The tag-transformer subsystem

- A tag transformer is a module under `packages/core/src/tag-transformers/`
  exporting `{ name, transform }` as `tag` and registered in `index.ts`.
- Tag names are kebab-case in author-facing docs; the registry array order is
  not significant.
- The shipped transformers include `code`, `color`, `example`, `hide-code`,
  `icon`, `location`, `order`, `page`, `section`, `showcase`, `space`,
  `variation`, and `variations`. Study an existing one before adding a new tag.
- A transformer receives a parsed tag `Spec` and returns context nodes; it must
  be pure with respect to its input and must not perform I/O.
- A new tag ships with: the transformer, its registration, a unit test, and a
  `### @tag` section in the core README.
- Transformer failures throw `TagTransformerError` via the
  `createTagTransformerError` helper, never a bare `Error`.

### Shipped tag catalogue

Each tag has a single responsibility; study the closest existing one before
adding a new tag:

- `@code` — captures the annotated source as a renderable code sample.
- `@color` — parses a colour value (hex, rgb, named) into a swatch node; bad
  input raises `CSSParseError` / `ColorParseError`.
- `@example` — marks a runnable example block to be rendered alongside the docs.
- `@hide-code` — suppresses the default code rendering for a block.
- `@icon` — associates an icon with the documented entry.
- `@location` — records where in the source the block originated.
- `@order` — overrides the default ordering of entries within a section.
- `@page` — assigns the entry to a documentation page.
- `@section` — groups entries under a named section heading.
- `@showcase` — flags an entry for prominent display.
- `@space` — controls spacing in the rendered layout.
- `@variation` / `@variations` — declares visual variations of a component.

### Authoring a transformer

A transformer is `{ name, transform }` exported as `default tag`. `transform`
receives a parsed `Spec` and returns context nodes. It must be pure: no disk,
no network, no global mutation. Validate the `Spec` shape up front and throw a
`TagTransformerError` with an actionable message when it is malformed.

## 11. The context & rendering contract

- The `Context` (`Context.types.ts`) is the renderer-agnostic data model; treat
  it as a published contract — a change to it ripples to every renderer.
- A `Renderer` (`Renderer.types.ts`) consumes a `Context` and emits artifacts;
  the default is `@ui-doc/html-renderer`.
- Description prose is parsed from Markdown to HTML by
  `MarkdownDescriptionParser` before it reaches the renderer.
- Keep renderer-specific concerns out of `core`; if you find yourself encoding
  HTML in `core`, the boundary has been crossed.

## 12. Error handling

- Throw typed errors from `src/errors/` (e.g. `TagTransformerError`,
  `BlockParseError`, `CSSParseError`), not bare `Error`, when the failure is
  domain-specific and a caller might catch it.
- Never swallow errors silently. If you catch, either handle meaningfully or
  rethrow with context.
- Validate external/source input at the boundary (the parser); trust internal
  data past that point.
- Error subclasses live one-per-file under `src/errors/` and are re-exported
  from `src/errors/index.ts`.

## 13. Testing discipline

- Tests use **Jest** and live in each package's `tests/` directory, mirroring
  the `src/` layout. A `src/Foo.ts` is tested by `tests/foo.test.ts`.
- Every bug fix lands with a regression test that fails before the fix.
- Test names describe behavior: `should <do X> when <condition>`.
- Keep tests deterministic — no real clock, no network, no filesystem unless
  the unit under test is about the filesystem.
- Run `pnpm workspace:test` before opening a PR; a red suite blocks merge.

## 14. Test fixtures & snapshots

- Shared input fixtures live beside the tests that use them; do not reach across
  packages for a fixture.
- Snapshot tests are allowed for rendered HTML output, but review snapshot
  diffs deliberately — an unreviewed snapshot update hides regressions.
- Prefer a focused assertion over a broad snapshot when the contract is small.

## 15. Debugging guide

- Reproduce against a `demos/` entry first; it is the fastest end-to-end loop.
- The parser's event stream is the best observation point — log events rather
  than stepping through the whole pipeline.
- For a rendering bug, dump the `Context` and confirm whether the defect is in
  assembly (stages 1–3, `core`) or rendering (stage 4, the renderer).
- For a tag bug, unit-test the transformer in isolation with a hand-built
  `Spec` before suspecting the parser.

## 16. Performance

- The parser runs over every source file — keep per-block work allocation-light.
- Avoid quadratic scans over the block list; index by key where lookups repeat.
- Measure before optimizing; don't trade readability for unproven speed.
- Large doc sets are the stress case; test changes against a realistic input
  size, not just a single block.

## 17. Internationalization & accessibility

- Author-facing strings emitted by the renderer should be overridable; do not
  hard-code English copy where a theme might localize it.
- Rendered output should be navigable and labelled; prefer semantic HTML in the
  renderer over presentational markup.

## 17a. Logging & observability

- `core` does not log directly; it emits events. Consumers (the node driver,
  plugins) decide how to surface them. This keeps the engine quiet by default
  and observable when a host opts in.
- When you need to trace the pipeline, subscribe to the parser's event stream
  rather than scattering `console.log`. The event stream is the supported
  observation point and is stable across releases.
- Never leave a `console.log` in committed code; the lint gate flags stray
  logging and the PR checklist calls it out.
- Error objects carry their source location; preserve it when rethrowing so the
  host can point a user at the offending block.

## 17b. Configuration

- Pipeline configuration is data, not code: a host passes a config object into
  the engine rather than mutating globals.
- Defaults live next to the code they configure and are documented in the owning
  package's README; do not duplicate a default value in two places.
- A new configuration option is a public-surface change — document it, give it a
  sensible default, and add a test that exercises the non-default path.
- Resolve configuration once at the boundary and pass the resolved values
  inward; inner code should not re-read raw config.

## 17c. Coding patterns we prefer

- **Composition over inheritance** — the one established base class is the typed
  event emitter; prefer composing small helpers over deep class hierarchies.
- **Make illegal states unrepresentable** — model with precise unions so an
  invalid combination does not type-check, rather than guarding it at runtime.
- **Parse, don't validate** — turn untrusted input into a precise type once, at
  the boundary, then trust the type downstream.
- **Small, named helpers** — extract a named function rather than nesting; the
  name documents intent better than a comment.
- **Pure where possible** — keep transformation and assembly free of side
  effects so they are trivially testable.

## 17d. Review expectations

- A reviewer checks: is the change in the right stage/package? Does it preserve
  the inward dependency arrow? Is the public surface documented? Are there
  tests, and do they describe behaviour?
- Reviews are about the code, not the author. Leave specific, actionable
  comments; prefer a suggestion over a complaint.
- The author drives the change to green: address every comment or explain why
  not, then re-request review.
- Large diffs get split; a reviewer may ask for a PR to be decomposed before
  review when it spans multiple concerns.

## 17e. Working with the demos

- `demos/` is the fastest end-to-end loop and the place to validate a real
  change against real input before writing a unit test.
- A demo may import from `packages/*` freely; it is not published, so it does
  not constrain the public surface.
- When a bug is reported, reproduce it in a demo first; the reproduction often
  becomes the regression test's fixture.
- Keep demos runnable — a broken demo is a broken onboarding path for the next
  contributor.

## 17f. Accessibility & output quality

- Rendered output should be navigable, labelled, and semantic; prefer semantic
  HTML in the renderer over presentational markup.
- Author-facing copy emitted by the renderer should be overridable so a theme
  can localize it; do not hard-code English where a host might translate.
- Treat the rendered documentation as a product surface: a confusing or
  inaccessible default is a defect, not a cosmetic nit.

## 18. Cross-reference integrity discipline

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

## 19. Commit conventions

- Conventional Commits: `type(scope): subject`, imperative, ≤ 50 chars subject.
- Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`,
  `build`, `ci`, `chore`, `revert`.
- One logical change per commit. Do not mix a refactor with a feature.
- Reference the affected package in the scope (`feat(core): …`).
- The body explains *why*, not *what* — the diff already shows what changed.

## 20. Branching & pull-request workflow

- Branch from `main`; name branches `type/short-description`.
- Keep PRs focused — one concern per PR makes review tractable.
- Rebase onto `main` before requesting review; resolve conflicts locally.
- A PR description states the motivation, the approach, and the test evidence.

## 21. Pull-request checklist

Before requesting review, confirm: build passes, lint clean, tests green,
no stray `console.log`, the change is scoped to one concern, and any public-API
change is reflected in docs and a changeset.

## 22. Release process & changesets

- Releases are driven by changesets. A change to a published package's public
  surface requires a changeset describing the bump.
- `@ui-doc/*` packages are published together; keep their versions coherent.
- A breaking change to a public export requires a major bump — flag it in the
  PR and call it out in the changeset.

## 23. Dependencies & security

- Add a dependency only when it earns its weight; prefer the platform/std lib.
- Runtime deps go in the owning package's `package.json`, not the workspace root.
- Pin ranges conservatively; run `pnpm security` before adding anything new.
- Review transitive additions in the lockfile diff — an unexpected subtree is a
  reason to reconsider the dependency.

## 24. Backwards compatibility

- The `@ui-doc/*` packages are published. A breaking change to a public export
  requires a changeset and a major bump — flag it in the PR.
- Deprecate before removing where feasible; keep a re-export shim for one minor.
- Document a migration note for any breaking rename in the package README.

## 25. CI pipeline

- CI runs install, build, lint, and the full test suite on every PR.
- A red pipeline blocks merge; do not merge around a failing required check.
- Keep CI fast — parallelize across packages and avoid redundant rebuilds.

## 26. Common recipes

- **Add a tag transformer:** create the module, register it in
  `tag-transformers/index.ts`, add a unit test, document the `### @tag` in the
  README.
- **Add a renderer feature:** extend the renderer package, not `core`; thread
  any new data through the `Context` contract deliberately.
- **Add a public export:** export it from the package barrel and document it.

## 27. Gotchas

- The `prepare` script builds on install — a broken `core` build blocks the
  whole workspace install. Keep `core` compiling.
- `demos` is not published; it may import from `packages/*` freely for examples.
- CSS/asset copying is handled by the rollup/vite plugins, not `core`.
- Mixing package managers corrupts the lockfile — pnpm only.

## 28. Glossary

- **Block** — a doc-comment paired with the source it annotates.
- **Tag transformer** — a module that turns one tag into context nodes.
- **Context** — the renderer-agnostic data model of the documentation set.
- **Renderer** — a consumer of the `Context` that emits final artifacts.
- **Spec** — the parsed representation of a single tag handed to a transformer.
