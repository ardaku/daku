# Portals

Portals allow Daku applications to interface with the hardware and environment /
OS.  The environment is not required to grant access to all portals requested by
the application, and may either stop the application or mock out a fake
implementation to protect the user's privacy.

Implementations of portals should strive to be "as mathematical" as possible,
meaning that there's no fancy engineering abstractions - just sending and
receiving data and defining the required functionality.  This is to reduce the
risk for possibly needing to deprecating portals in the future.  That said, it's
ok to pack smaller pieces of data into one value if 32-bits can be guaranteed to
most likely never be needed.

Portals should also make use of shared high-level [types](./types.md).

## Kinds
Some portals only have one command that is sent on the channel to that portal.
But, there are some portals that can create new channels that accept different
commands and notify on different events.

There are two kinds of channels; `ignore`s and `notify`s.  An ignore channel
never returns the ready value to the user because it's implicity ready
immediately.  The two kinds are mutually exclusive; you can't wait for
notification from an ignore or tell Daku not to send a notification from a
notify.
