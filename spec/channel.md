# Channel Allocation

Channels are implicitly allocated.  Channel 0 is always a custom channel that
can send arbitrary messages to the parent task or embedder.  This feature is
useful for plugin support in applications.  The first portal listed in the "Daku
custom section" will reserve channel 1.  All portal channels stay open for the
lifetime of the application/task.

Additional channels called device channels can be both opened and closed.  They
must be opened from a portal.  When they are closed, they go to a
"garbage list".  Once a new device channel is opened, the last closed channel id
is reused for the new channel.  Channels are otherwise opened in consecutive
ascending order.
