# Changelog
All notable changes to `daku` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://github.com/AldaronLau/semver).

## [0.3.3] - 2022-10-25
### Fixed
 - Attempt to fix docs.rs build (again)

## [0.3.2] - 2022-10-25
### Fixed
 - Attempt to fix docs.rs build

## [0.3.1] - 2022-10-25
### Fixed
 - Add docs.rs metadata

## [0.3.0] - 2022-10-25
### Added
 - `api::log::init()`
 - `api::prompt::read_line()`
 - `cmd::defer()`
 - `cmd::execute()`
 - `cmd::flush()`
 - `cmd::queue()`
 - `cmd::until()`
 - `run::sleep()`
 - `run::wake()`
 - `sys` module

### Changed
 - Rename `block_on()` to `run::block_on()`
 - APIs are now found under 4 modules:
   - `api` - safe API abstractions
   - `cmd` - command queue
   - `run` - asynchronous abstractions
   - `sys` - raw (unsafe) Daku FFI bindings

### Removed
 - `cpu_info` (for now)
 - `log::info!()` - use `log` crate instead
 - `log::warn!()` - use `log` crate instead
 - `log::error!()` - use `log` crate instead
 - `log::debug!()` - use `log` crate instead
 - `log::Target` - use `log` crate instead

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
