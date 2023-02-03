# Base API

For all APIs, addresses in memory refer to the memory exported as `memory`.
All Daku types are lists of packed 32-bit little-endian values.

The exported WebAssembly API contains a single function:

## *Function*: `ar()`

Retconned from "Ardaku"; Asynchronous Request function.

```wat
(import "daku" "ar" (func $event
    (param $cmd_size i32)   ;; List[Command].size
    (param $cmd_addr i32)   ;; List[Command].addr
    (result i32)            ;; List[Uint32].size 
))
```

### Parameters

 - `$cmd_size`: The size of the data pointed to by `$cmd_addr`
 - `$cmd_addr`: A pointer in the wasm memory to a list of `$cmd_size` commands

### Returns

 - The new size of the WebAssembly module's global `$ready_list`

## *Type*: `Command`

Commands are the way the Daku application sends messages to the environment.

### Fields

 - `size: int` - Size of data pointed to by `addr`, in bytes.
 - `addr: ptr` - Pointer to `size` bytes to be sent as a command.
 - `channel: int` - The channel on which to send the command.
 - `ready: val` - Arbitrary user value written to the ready list when the
   requested notifier is ready.

## Setting Up The Ready List

At the beginning of a Daku application, it is required to send the "Connect"
command on channel 0.  Channel 0 is always open and after connecting can be used
to send arbitrary payloads to the environment.  The Daku API does not specify
how this data is formed, so it's up to the embedder to decide.  This feature is
specifically for non-standard extensions to the API, such as being able to write
a high-level operating system driver depending on Daku.

## *Type*: `Connect`

The connect command must only be sent once at the beginning of the app.

### Fields

 - `portals_size: int` - Number of portals to connect to.
 - `portals_addr: ptr` - Input array of portal IDs, Output array of channels. 
 - `ready_size: int` - Maximum capacity of the ready list at `ready_addr`.
 - `ready_addr: ptr` - Pointer to the ready list with capacity for `ready_size`
   values.
