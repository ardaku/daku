# Introduction

Daku is an asynchronous host interface abstraction API for WebAssembly plugins,
drivers, applications, and more!  It is developed as a supporting specification
for the Ardaku project (Ardaku is an engine for running Daku modules for the
listed use-cases).

## Daku Specification v1.0.0-pre.0 (draft v10)

The current version of Daku targets the full WebAssembly 2.0 spec without any
non-standard or experimental features.

## Goals
 - Modular
 - Minimal (in API surface, and memory footprint)
 - Asynchronous
 - Stable base API
 - As simple and efficient as possible
 - Reduced context switching
 - Security-first
 - First-class multimedia portals
 - Portals compatible with WASI versions via 2-way abstractions
