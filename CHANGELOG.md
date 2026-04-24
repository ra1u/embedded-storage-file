# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-04-24

### Added
- `Debug` implementation for all public types (`NorMemory`, `NorStorage`,
  `MmapFile`, `NorMemoryAsync`). Thanks to @Wojciech-Graj (#2).
- GitHub Actions CI workflow running `cargo fmt --check`, `cargo clippy -D warnings`,
  build, test and `cargo doc` on every push and pull request.
- Manually-triggered GitHub Actions workflow for publishing the crate to
  [crates.io](https://crates.io/crates/embedded-storage-file).
- Crates.io version and docs.rs API-documentation badges in the README.
- Explicit `rust-version = "1.87"` MSRV declaration in `Cargo.toml`.
- CI matrix covering `rust ∈ {1.87, stable, beta}` × `deps ∈ {latest, minimal}`,
  with the `minimal` cells resolving direct dependencies to their declared
  floors via `cargo +nightly update -Z direct-minimal-versions`.

### Changed
- README links updated to render correctly on both GitHub and docs.rs
  (inline links to `embedded_storage`, `embedded_storage_async` and `memmap2`;
  direct link to the integration tests).
- Internal code cleanup to satisfy `clippy -D warnings`: no behavioural change.
- Widened dependency version ranges to the oldest releases whose public API
  the crate actually uses, verified by building and testing against the floors:
  - `embedded-storage` — `"0.3.1"` → `">=0.3.1, <0.4"`.
  - `embedded-storage-async` — `"0.4.1"` → `">=0.4.1, <0.5"` (`0.4.0` is
    nightly-only and is excluded).
  - `memmap2` — `"0.9.5"` → `">=0.5, <0.10"`.
  - Dev-dependencies: `rand` → `">=0.9, <0.10"`, `tokio` → `">=1, <2"`.

## [0.1.0]

- Initial release: file-backed and in-RAM `NorFlash` / `Storage` implementations
  for [`embedded-storage`](https://docs.rs/embedded-storage) and
  [`embedded-storage-async`](https://docs.rs/embedded-storage-async).

[0.2.0]: https://github.com/ra1u/embedded-storage-file/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ra1u/embedded-storage-file/releases/tag/v0.1.0
