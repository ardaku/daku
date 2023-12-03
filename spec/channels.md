# Channels

## Allocation

Channels are implicitly allocated.  Channel 0 is always the custom portal
channel that can send arbitrary messages to the host.  The first portal listed
in the `daku` WebAssembly custom section will reserve channel 1.  All portal
channels stay open for the lifetime of the guest.

Additional channels called device channels can be both opened and closed.  They
must be opened from a portal.  When they are closed, they go to a
"garbage list".  Once a new device channel is opened, the last closed channel id
is reüsed for the new channel.  Channels are otherwise opened in consecutive
ascending order.

## Channel 0

A channel 0 command is sent as a [`Buffer`](./buffer.html).


