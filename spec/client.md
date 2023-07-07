# *Device*: Client 🧪

Channel representing an HTTPS connection to a client.

## *Type*: `Poll`

### Variants (`int`)
 0. `Continue` - Keep polling.
 1. `Hangup` - Hang up connection.
 - Respond with informational response status code `100-199` (FIXME: full list)
 - Respond with successful response status code `200-299` (FIXME: full list)
 - Respond with redirection message status code `300-399` (FIXME: full list)
 - Respond with client error response status code `400-499` (FIXME: full list)
 - Respond with server error response status code `500-599` (FIXME: full list)

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

 0. If `poll` invalid/unknown variant
 1. If `poll` is not `0` or `1` after response headers already sent
 2. If `poll` is non-null and not a reponse status code before headers sent
 3. If `request.size > request.capacity`
 4. If address at `content.addr + content.size` has no page
 5. If address at `error + 3` has no page
 6. If address at `request + 7` has no page
 7. If address at `request.capacity + 3`  has no page
 8. If address at `request.headers_content.addr + request.capacity` has no page
