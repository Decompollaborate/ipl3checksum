# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Prebuilt binaries for Python 3.13.

### Fixed

- Fix some typos

## [1.2.0] - 2024-01-11

### Added

- `CICKind.get_entrypoint`: Returns the entrypoint address that would be used
  on runtime.
- `CICKind.calculate_checksum`: Convinience method that wraps
  `checksum::calculate_checksum`.
- Python bindings:
  - Expose `Ipl3ChecksumError` to Python as a new exception for each error of
    the enum. Refer to `ipl3checksum.exceptions`.

### Changed

- Rewrite the checksum algorithm for readability and simplicity.

## [1.1.1] - 2023-12-23

### Fixed

- Python bindings:
  - Fix `detectCIC` and `detect_cic_raw` functions not accepting `bytearray`
    objects.
- Fix some typos.

## [1.1.0] - 2023-12-22

### Added

- Add Rust support.
- New static methods for `CICKind`.
  - `CICKind.fromHashMd5`: Returns a CIC kind based on the passed md5 hash.
  - `CICKind.fromName`: Returns a CIC kind based a string representing its name.
- Add C bindings.
- Add support for the IPL3 5101 variant (Used by Aleck 64 games).
- New frontends:
  - `check`: Checks if the checksum in the ROM matches the calculated one.
  - `detect_cic`: Allows to detect the cic type used by a rom.
  - `sum`: Calculates the ipl3 checksum of a rom.

### Changed

- Library was reimplemented in Rust, allowing faster runtime calculation.
  - The Python API is still the same.

### Fixed

- Fix links in `CHANGELOG.md`

## [1.0.1] - 2023-09-21

### Added

- Allow invoking `ipl3checksum` as a CLI program.
  - Currently it only allows the `-V`/`--version` argument, which prints the
version of the library.
- A `CHANGELOG.md`
- Cleanup the `README.md`
  - Reorder sections.
  - Add more notes about installing and the develop version.
  - Reference the changelog.
  - List features.
- Add a `py.typed` file.

## [1.0.0] - 2023-09-20

### Added

- Initial relase

[unreleased]: https://github.com/Decompollaborate/ipl3checksum/compare/main...develop
[1.2.0]: https://github.com/Decompollaborate/ipl3checksum/compare/1.1.1...1.2.0
[1.1.1]: https://github.com/Decompollaborate/ipl3checksum/compare/1.1.0...1.1.1
[1.1.0]: https://github.com/Decompollaborate/ipl3checksum/compare/1.0.1...1.1.0
[1.0.1]: https://github.com/Decompollaborate/ipl3checksum/compare/1.0.0...1.0.1
[1.0.0]: https://github.com/Decompollaborate/ipl3checksum/releases/tag/1.0.0
