# changelog

## Unreleased Changes


## Releases

### 10.0.1  {#1001}

<!-- fields -->
- date: 2023-04-15

#### Fixed  {#fixed}

- export the `Help.visibleGlobalOptions()` helper missing from the type definitions ([#1896]).

### 11.0.0  {#1100}

<!-- fields -->
- date: 2023-06-20

#### Changed  {#changed}

- **Breaking:** Commander now requires Node.js v16 or higher ([#1929]).

#### Fixed  {#fixed}

- subcommand help now respects a configured output width ([#1918]).

### 11.1.0  {#1110}

<!-- fields -->
- date: 2023-10-13

#### Added  {#added}

- allow using `InvalidArgumentError` from custom argument processing ([#1973]).

#### Fixed  {#fixed}

- TypeScript: widen the `Option.argChoices` typing to accept readonly arrays ([#1948]).

### 12.0.0  {#1200}

<!-- fields -->
- date: 2024-02-03

#### Added  {#added}

- add `.saveStateBeforeParse()` and `.restoreStateBeforeParse()` for reuse of a configured command ([#2057]).

#### Changed  {#changed}

- **Breaking:** Commander now requires Node.js v18 or higher ([#2027]).
- **Breaking:** `.parse()` and `.parseAsync()` now accept options as the second parameter ([#2098]).

#### Deprecated  {#deprecated}

- the trailing-comma form of `.option()` flag lists is deprecated in favour of explicit arrays ([#2104]).

#### Removed  {#removed}

- **Breaking:** removed the long-deprecated `.command('*')` default-command form ([#2061]).

### 12.1.0  {#1210}

<!-- fields -->
- date: 2024-05-18

#### Added  {#added}

- TypeScript: add `startup` to the `AddHelpTextPosition` type used by `.addHelpText()` ([#2197]).

#### Changed  {#changed}

- update package-lock to address `braces` advisory ([#2208]).
