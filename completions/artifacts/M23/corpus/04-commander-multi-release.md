# Changelog

All notable changes to this project will be documented in this file.

## [12.1.0] (2024-05-18)

### Added

- TypeScript: add `startup` to the `AddHelpTextPosition` type used by `.addHelpText()` ([#2197])

### Changed

- update package-lock to address `braces` advisory ([#2208])

## [12.0.0] (2024-02-03)

### Added

- add `.saveStateBeforeParse()` and `.restoreStateBeforeParse()` for reuse of a configured command ([#2057])

### Changed

- **Breaking:** Commander now requires Node.js v18 or higher ([#2027])
- **Breaking:** `.parse()` and `.parseAsync()` now accept options as the second parameter ([#2098])

### Deprecated

- the trailing-comma form of `.option()` flag lists is deprecated in favour of explicit arrays ([#2104])

### Removed

- **Breaking:** removed the long-deprecated `.command('*')` default-command form ([#2061])

## [11.1.0] (2023-10-13)

### Added

- allow using `InvalidArgumentError` from custom argument processing ([#1973])

### Fixed

- TypeScript: widen the `Option.argChoices` typing to accept readonly arrays ([#1948])

## [11.0.0] (2023-06-20)

### Changed

- **Breaking:** Commander now requires Node.js v16 or higher ([#1929])

### Fixed

- subcommand help now respects a configured output width ([#1918])

## [10.0.1] (2023-04-15)

### Fixed

- export the `Help.visibleGlobalOptions()` helper missing from the type definitions ([#1896])
