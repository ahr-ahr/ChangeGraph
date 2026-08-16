# Changelog

All notable changes to ChangeGraph are documented in this file.

The project is currently in early development and has not reached its first stable release.

The format of this changelog is inspired by [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows Semantic Versioning where applicable.

## [Unreleased]

### Added

- Initial project documentation.
- System and component architecture documentation.
- Graph model documentation.
- OpenTelemetry integration documentation.
- Protocol and API contract documentation.
- StatePack architecture and format documentation.
- Repository and development workflow documentation.
- Initial repository contribution guidelines.
- Initial Apache License 2.0 licensing.

### Planned

- Protobuf graph specification.
- Rust-based ChangeGraph core.
- Graph node and edge implementation.
- Telemetry ingestion model.
- Impact analysis engine.
- OpenTelemetry ingestion and semantic mapping.
- ChangeGraph CLI.
- Initial Node.js, PHP, and Python integration examples.
- Initial test fixtures and graph/impact test suites.

## [0.1.0]

> Not released yet.

Version `0.1.0` will represent the first development milestone with a usable end-to-end foundation.

### Planned

- Graph protocol implementation.
- Rust graph core.
- Trace and span ingestion.
- Dependency relationship construction.
- Initial impact analysis.
- OpenTelemetry integration.
- CLI commands for graph inspection and impact analysis.
- Initial automated tests.
- Basic cross-language integration examples.

### Not Included

The following capabilities are outside the initial `0.1.0` scope unless explicitly added later:

- Full StatePack capture.
- State restoration.
- Execution replay.
- Production-scale distributed deployment.
- Advanced visualization UI.

## Versioning

ChangeGraph will use semantic versioning for released versions:

```text
MAJOR.MINOR.PATCH
```

In general:

- `MAJOR` indicates incompatible public changes.
- `MINOR` indicates backward-compatible functionality.
- `PATCH` indicates backward-compatible fixes.

Pre-release versions may use identifiers such as:

```text
0.1.0-alpha.1
0.1.0-beta.1
```

## Changelog Guidelines

Changes should be added under `Unreleased` before a release is created.

When a version is released:

1. Move the relevant changes from `Unreleased` into a versioned section.
2. Add the release date.
3. Create the corresponding Git tag.
4. Start a new `Unreleased` section.

Example:

```text
## [0.1.0] - YYYY-MM-DD

### Added

- ...

### Changed

- ...

### Fixed

- ...
```

The changelog should describe user-visible or project-relevant changes rather than every internal commit.
