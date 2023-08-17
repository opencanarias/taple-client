# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2023-08-17

### Fixed

- Rest payload size increased
- Dockerfile rust version updated to 1.67 to avoid contracts compilation error
- taple-sign wrong output format fixed

## [0.2.1] - 2023-08-03

### Changed

- Changing taple-sign to use new Signature function and… (#77)

### Fixed

- Fixing usage of Signature::new (#78) (#79)

### Removed

- Removing unnecessary things from taple-sign (#80) (#81)

## [0.2.0] - 2023-07-26

### Added

- Support for TAPLE smart contracts
- DB implementation based on LevelDB
- API REST endpoint for transfers and subjects preauthorization

### Changed

- TAPLE Core updated to 0.2. 
- API REST improved
- Configuration improved
- TAPLE Tools are now part of TAPLE Client

### Fixed

- Several bugs fixed and improvements.

### Removed

- API Key removed
- Quickstart scripts removed

## [0.1.4] - 2023-02-23

### Changed

- README.md improved
- Workspace dependencies updated
- Database path env name changed to TAPLE_DATABASE_PATH

### Fixed

- cli --version message

## [0.1.3] - 2023-02-22

### Added

- Support for hidden parameters
- Pagination in GET Governances
- Dockerfile added (#8)

### Changed

- Extensible configuration
- Public Rest Handlers
- Public REST API routes

### Fixed

- Indication of errors more in line with the real reasons

## [0.1.0] - 2022-11-30

### Added

- First release

[0.2.2]: https://github.com/opencanarias/taple-client/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/opencanarias/taple-client/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/opencanarias/taple-client/compare/v0.1.0...v0.2.0
[0.1.4]: https://github.com/opencanarias/taple-client/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/opencanarias/taple-client/compare/v0.1.0...v0.1.3
[0.1.0]: https://github.com/opencanarias/taple-client/releases/tag/v0.1.0
