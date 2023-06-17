# *Device*: Client 🧪

Channel representing an HTTPS connection to a client.

## *Type*: `Poll`

### Variants (`int`)
 0. `Continue` - Keep polling.
 1. `Hangup` - Hang up connection.

## Readiness

Becomes ready once either

 - The client has headers and possible content body ready to be retrieved (no
   error is set to 0).
 - The client has headers, but they are too large for `request`
   (`request.capacity` set to required capacity)
 - Errors have overflown (at least one error set to `255`)
 - The client hung up (`poll` set to `Hangup`)
 - The client is ready for more data (`poll` set to `Continue`)

## *Command*: `Client`

### Fields

 - `content: List[byte]` Content body to send.
 - `poll: opt[Poll]` (In/Out) Pointer to poll state (In:Server, Out:Client)
 - `request: opt[_]` Request buffer
   - `capacity: ptr[int]` (In/Out) Pointer to capacity of `headers_content`.
   - `headers_content: ptr[Text]` (In/Out) Pointer to headers and content.

### Traps

 - If `request.size > request.capacity`
 - If address at `content.addr + content.size` has no page
 - If address at `error + 3` has no page
 - If address at `request + 7` has no page
 - If address at `request.capacity + 3`  has no page
 - If address at `request.headers_content.addr + request.capacity` has no page
