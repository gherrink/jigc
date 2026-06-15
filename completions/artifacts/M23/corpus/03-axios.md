# Changelog

## [1.6.2](https://github.com/axios/axios/compare/v1.6.1...v1.6.2) (2023-11-14)

### Bug Fixes

* **formdata:** fixed content-type header normalization for non-standard browser environments ([#6056](https://github.com/axios/axios/pull/6056))
* **dns:** fixed lookup function decoration to support all signatures ([#6011](https://github.com/axios/axios/pull/6011))

### Features

* **withXSRFToken:** added withXSRFToken option as a workaround to support the old withCredentials-coupled behavior ([#6046](https://github.com/axios/axios/pull/6046))

## [1.6.1](https://github.com/axios/axios/compare/v1.6.0...v1.6.1) (2023-11-08)

### Bug Fixes

* **formdata:** fixed informative error handling for browser-only multipart payloads ([#6053](https://github.com/axios/axios/pull/6053))

### Performance Improvements

* **trim:** lifted regexp-based trimming for a small startup gain ([#6034](https://github.com/axios/axios/pull/6034))
