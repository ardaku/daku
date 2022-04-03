# Changelog
All notable changes to `daku` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://github.com/AldaronLau/semver).

## [0.2.0] - 2022-04-03
### Added
 - `block_on()`
 - `cpu_info::extensions()`
 - `cpu_info::width()`
 - `cpu_info::Width`
 - `log::info!()`
 - `log::warn!()`
 - `log::error!()`
 - `log::debug!()`
 - `log::Target`

### Changed
 - Portals are now in their own modules (`arch` and `Arch` moved to `cpu_info`)

### Removed
 - `run()`

## [0.1.0] - 2022-03-05
### Added
 - `arch()`: Get the underlying CPU architecture
 - `run()`: High-level async API inspired by `kevent()`
