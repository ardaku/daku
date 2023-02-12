# Introduction

Daku is an asynchronous system interface API for WebAssembly with a focus on
multimedia.  Some goals may overlap with WASI, others do not.  It is developed
as a supporting specification for the Ardaku project.

## Daku Specification v1.0.0-beta.0 (draft)

The current version of Daku targets the full WebAssembly 2.0 spec without any
non-standard or experimental features.

## Terminology

### Channel
In Daku, a channel is a type that can have data sent over it.  One can not
receive data over a channel; To "receive" data, the Daku program must first send
a memory address of a buffer to the host.  After that, the channel may become
"ready" and the buffer filled with new data.  This is only possible for `notify`
channels.

There are three kinds of channels in total; `ignore`s, `notify`s, and
`ignore-notify`s.  An ignore channel never becomes ready because it's implicity
ready immediately.  The two kinds are mutually exclusive; you can't wait for
notification from an ignore or tell Daku not to send a notification from a
notify.  The `ignore-notify` channel type is used for the creation of new
channels.  An `ignore-notify` channel becomes ready immediately like an `ignore`
channel, but doesn't notify until the new channel is ready.

### Portal
A portal is an interface to some type of hardware on the host.  A portal
represents the hardware directly rather than the Unix-like method of abstracting
it as a path or URI.  Portals are built in to the Daku specification as an
abstraction over a channel.  Some channels are portals, while other channels
usually represent devices found over that portal.
