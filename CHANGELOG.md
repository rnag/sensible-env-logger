# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## [Unreleased]
<!--
### Features
- Added a new struct `MyStruct` with the following methods:
  - `my_method()`
  - `other_method()`
-->

## v0.3.1 (2022-08-10)

### Bug Fixes

* Correct example usage and log format as it appears in the documentation.

## v0.3.0 (2022-08-09)

### Features

- Add new helper macros to log the local date
  and local date/time ISO:
  - `init_timed_local` / `safe_init_timed_local` / `try_init_timed_local`
  - `init_timed_local_iso` / `safe_init_timed_local_iso` / `try_init_timed_local_iso`
- Add new examples `log_timed_local` and `log_timed_local_iso`
  to confirm intended functionality.

## v0.2.0 (2022-03-19)

### Features

- Add helper macros `safe_init`, `safe_init_timed`, and `safe_init_timed_short`.
  This should make it easier to run *tests* on a crate of choice.

### Bug Fixes

- Fix so module name is correctly generated.
  - In some edge cases, mainly in *tests* within binaries such `src/bin/my_bin/my_file.rs`,
    it only enabled logs on the nested module path, ex. `my_bin::my_module::tests`. Now
    it should instead enable logs on the base module, `my_bin` in this example.

## v0.1.0 (2022-03-13)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/sensible-env-logger
