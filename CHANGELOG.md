# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2020-02-20
### Fixed
- Empty lines will be ignored (PR #1)

## [0.1.1] - 2020-02-07
### Changed
- Argument to `parse_file` is changed to `AsRef<Path>`

## [0.1.0] - 2020-02-02
### Added
- Achieved Feature parity with parser implementation in [dalance/svlint](https://github.com/dalance/svlint/)
- Parse files, include directories and defines
- Support environment variables