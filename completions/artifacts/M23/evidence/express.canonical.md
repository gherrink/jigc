# changelog

## Unreleased Changes


## Releases

### 4.18.0  {#4180}

<!-- fields -->
- date: 2022-04-25

#### Added  {#added}

- `res.download` support for object with all optional properties.

#### Changed  {#changed}

- `res.sendFile` to use `send@0.18.0`.
- deps: body-parser@1.20.0.

#### Removed  {#removed}

- deprecated leading colon in `name` for `app.param(name, fn)`.

### 4.18.1  {#4181}

<!-- fields -->
- date: 2022-04-29

#### Fixed  {#fixed}

- hanging on large stack of sync routes.

### 4.18.2  {#4182}

<!-- fields -->
- date: 2022-10-08

#### Fixed  {#fixed}

- regression routing a large stack in a single route.
- `req.resume()` called after `res.pipe()`.

#### Changed  {#changed}

- deps: body-parser@1.20.1 (deps: qs@6.11.0).
- deps: qs@6.11.0.
