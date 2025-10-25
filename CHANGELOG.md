# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

### Categories each change fall into

* **Added**: for new features.
* **Changed**: for changes in existing functionality.
* **Deprecated**: for soon-to-be removed features.
* **Removed**: for now removed features.
* **Fixed**: for any bug fixes.
* **Security**: in case of vulnerabilities.


## [Unreleased]
### Added
- Specify `links` manifest key `mnl-sys`. This allows dependants to pass custom build flags.


## [0.2.2] - 2022-02-11
### Fixed
- Just releasing `mnl` with correct minimal dependency specification
  on `mnl-sys` (0.2.1).


## [0.2.1] - 2022-02-11
### Fixed
- Specify dependency versions more exactly to allow building with minimal versions
  of the entire dependency tree


## [0.2.0] - 2019-09-23
### Added
- Add `cb_run2` with support for callback closures.

### Changed
- Upgraded crates to Rust 2018 edition.


## [0.1.0] - 2018-08-29
### Added
- Bindings to `libmnl 1.0.4`
- Initial safe abstraction. Just basic socket support.
