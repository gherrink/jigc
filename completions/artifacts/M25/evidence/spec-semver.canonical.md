# semantic-versioning

## Goal

Given a version number MAJOR.MINOR.PATCH, increment MAJOR for incompatible API changes, MINOR for backward-compatible added functionality, and PATCH for backward-compatible bug fixes; pre-release and build metadata are available as extensions to the MAJOR.MINOR.PATCH format.

## Context

In systems with many dependencies, releasing new versions can become dependency hell: specifications too tight cause version lock, too loose cause version promiscuity. Semantic Versioning is a simple set of rules for assigning and incrementing version numbers so that the number and the way it changes convey meaning about the underlying code; for it to work a project must first declare a clear, precise public API.

## Criteria

### Declare a public API  {#declare-a-public-api}

Software using Semantic Versioning MUST declare a public API, in the code itself or in documentation; however it is done it SHOULD be precise and comprehensive.

### Version number form X.Y.Z  {#version-number-form-xyz}

A normal version number MUST take the form X.Y.Z where X, Y, Z are non-negative integers without leading zeroes (X major, Y minor, Z patch); each element MUST increase numerically, e.g. 1.9.0 -> 1.10.0 -> 1.11.0.

### Released versions are immutable  {#released-versions-are-immutable}

Once a versioned package has been released, the contents of that version MUST NOT be modified; any modification MUST be released as a new version.

### Major version zero is unstable  {#major-version-zero-is-unstable}

Major version zero (0.y.z) is for initial development: anything MAY change at any time and the public API SHOULD NOT be considered stable.

### Version 1.0.0 defines the public API  {#version-100-defines-the-public-api}

Version 1.0.0 defines the public API; how the version is incremented after this release depends on that public API and how it changes.

### Increment PATCH for bug fixes  {#increment-patch-for-bug-fixes}

Patch version Z (x.y.Z, x > 0) MUST be incremented if only backward-compatible bug fixes are introduced, a bug fix being an internal change that fixes incorrect behavior.

### Increment MINOR for added functionality  {#increment-minor-for-added-functionality}

Minor version Y (x.Y.z, x > 0) MUST be incremented for new backward-compatible public-API functionality or when public functionality is marked deprecated, MAY be incremented for substantial private improvements, and resets patch to 0.

### Increment MAJOR for breaking changes  {#increment-major-for-breaking-changes}

Major version X (X.y.z, X > 0) MUST be incremented for any backward-incompatible public-API change; it MAY include minor and patch changes, and resets patch and minor to 0.

### Pre-release versions  {#pre-release-versions}

A pre-release version MAY be denoted by a hyphen and dot-separated ASCII-alphanumeric/hyphen identifiers after the patch version (non-empty, no leading zeroes on numerics); it has lower precedence than the associated normal version and signals instability. Examples: 1.0.0-alpha, 1.0.0-alpha.1, 1.0.0-x.7.z.92.

### Build metadata  {#build-metadata}

Build metadata MAY be denoted by a plus sign and dot-separated ASCII-alphanumeric/hyphen identifiers after the patch or pre-release version; it MUST be ignored when determining precedence, so two versions differing only in build metadata have equal precedence. Examples: 1.0.0-alpha+001, 1.0.0+20130313144700.

### Precedence ordering  {#precedence-ordering}

Precedence is calculated by separating the version into major, minor, patch, then pre-release identifiers (build metadata excluded), comparing left to right: major/minor/patch numerically; a pre-release has lower precedence than the equal normal version; pre-release identifiers compare per-field (numeric numerically, alphanumeric in ASCII order, numeric below non-numeric, a larger set above a smaller). Example: 1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-beta < 1.0.0-rc.1 < 1.0.0.
