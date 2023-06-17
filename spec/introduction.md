# Introduction

Daku is an asynchronous system interface API for WebAssembly with a focus on
multimedia.  Some goals may overlap with WASI, others do not.  It is developed
as a supporting specification for the Ardaku project.

It will be possible to make a fully-featured compatibility layer to implement
WASI over Daku, and Daku over WASI once asynchronous APIs are implemented in
WASI.

## Daku Specification v1.0.0-pre.0 (draft v3)

The current version of Daku targets the full WebAssembly 2.0 spec without any
non-standard or experimental features.

## Terminology

### Channel
In Daku, a channel is a type that can have data sent over it.  One can not
receive data over a channel; To "receive" data, the Daku program must first send
a memory address of a buffer to the host.  After that, the channel may become
"ready" and the buffer filled with new data.

### Portal
A portal is an interface to some type of hardware on the host.  A portal
represents the hardware directly rather than the Unix-like method of abstracting
it as a path or URI.  Portals are built in to the Daku specification as an
abstraction over a channel.  Some channels are portals, while other channels
usually represent devices found over that portal.
