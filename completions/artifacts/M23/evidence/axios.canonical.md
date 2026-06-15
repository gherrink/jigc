# changelog

## Unreleased Changes


## Releases

### 1.6.1  {#161}

<!-- fields -->
- date: 2023-11-08
- link: https://github.com/axios/axios/compare/v1.6.0...v1.6.1

#### Fixed  {#fixed}

- **formdata:** informative error handling for browser-only multipart payloads ([#6053]).

#### Changed  {#changed}

- **trim:** lifted regexp-based trimming for a small startup gain ([#6034]). (foreign category: Performance Improvements)

### 1.6.2  {#162}

<!-- fields -->
- date: 2023-11-14
- link: https://github.com/axios/axios/compare/v1.6.1...v1.6.2

#### Fixed  {#fixed}

- **formdata:** content-type header normalization for non-standard browser environments ([#6056]).
- **dns:** lookup function decoration to support all signatures ([#6011]).

#### Added  {#added}

- **withXSRFToken:** option as a workaround to support the old withCredentials-coupled behavior ([#6046]).
