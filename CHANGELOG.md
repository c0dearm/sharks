# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2] - 2020-03-9
### Changed
- Share structs now derives the `Clone` trait

## [0.3.1] - 2020-01-23
### Changed
- Sharks recover method now accepts any iterable collection

## [0.3.0] - 2020-01-22
### Added
- Share struct which allows to convert from/to byte vectors

### Changed
- Methods use the new Share struct, instead of (GF245, Vec<GF256>) tuples

## [0.2.0] - 2020-01-21
### Added
- Computations performed over GF256 (much faster)
- Secret can now be arbitrarily long

### Changed
- Some method names and docs
- Maximum number of shares enforced by Rust static types instead of conditional branching

### Removed
- Modular arithmetic around Mersenne primes

## [0.1.1] - 2020-01-13
### Fixed
- Typo in cargo description

### Removed
- Maintenance badges in cargo file

## [0.1.0] - 2020-01-13
### Added
- Initial version
