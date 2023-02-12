# 0x02 - Fetch 🧪 (`ignore-notify`)

Do an HTTP request to specified URL.

This portal can be used to implement SSE as well.

### Usage Flow
 - Send `Fetch` type as a command on the fetch portal's channel.
 - "Ready" immediately and overwrites data pointed to by `channel`, but doesn't
   yield to the guest like other `ignore` channels.  Notifies based on readiness
   of new channel with ready data from the original command.
 - Once data is received, buffer is overwritten up to `capacity` bytes, and
   `capacity` set to remaining byte count.
 - Send `FetchAction` type as command on new `channel` representing HTTP
   connection.
 - Becomes ready when data is ready to be received over HTTP and written into
   the buffer.

## *Type*: `Fetch`

### Fields
 - `url: Text` - URL to do an HTTP request to (error if invalid).
 - `headers: opt[Text]` - Newline-delimited headers to send (error if invalid).
 - `body: opt[List[byte]]` - Payload to send.
 - `method: int` - 0: GET, 1: HEAD, 2: POST, 3: PUT, 4: DELETE.
 - `channel: ptr[int]` - (Out) Pointer to channel (0 if error).
 - `capacity: ptr[int]` - (Out) Pointer to capacity of `buffer`.
 - `buffer: ptr[List[byte]]` - (Out) Pointer to buffer for receiving parts of
   the HTTP response.

## *Type*: `FetchAction`

### Variants (`int`)
 0. `Hangup` - Hang up connection to server.
 1. `Poll` - Poll for more data from server.

## *Type*: `FetchError`
An error is indicated when a notification is sent, but the buffer size has been
set to 0.  The first byte in the buffer is set to an error code:

### Variants (`byte`)
 0. `Hangup` - Server hung up.
 1. `Network` - Server unreachable (network error).
