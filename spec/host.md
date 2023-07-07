# *Device*: Host 🧪

Channel representing an HTTPS connection to a server.

## *Type*: `FetchError`
An error may be indicated once either `Fetch` or `Host` command becomes ready.

To indicate an error, `capacity` will be set to 0.  `buffer.size` will be set to
an error code:

### Variants (`int`)
 0. `Network` - Server unreachable (network error).
 1. `Hangup` - Server hung up.

## Readiness

Becomes ready once either

 - The resource host can't be reached (network error), `capacity` set to 0.
 - The resource host hung up, `capacity` set to 0.
 - The resource host has sent back a chunk of the resource (a new device
    channel has been opened)
   1. Buffer is not big enough, `buffer` overwritten up to `capacity` bytes, and
      `capacity` modified how many more bytes are required
   2. Buffer is big enough, `buffer` overwritten, `capacity` unchanged.

## *Command*: `Host`

### Variants (`int`)
 0. `Poll` - Poll for more data from server.
 1. `Hangup` - Hang up connection to server.

### Traps
 0. If variant is unknown (not `0` or `1`)
