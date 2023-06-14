# Base API

For all APIs, addresses in memory refer to the memory exported as `memory`.
All Daku types are lists of packed 32-bit little-endian values.

The exported WebAssembly API contains a single function:

## *Function*: `ar()`

Retconned from "Ardaku"; Asynchronous Request function.

Does not return until a `notify` command becomes ready; Otherwise, if at least
one `ignore` command is sent, without also sending a `notify` command, returns
immediately.

A special no-op `ignore` command can be sent as an empty command on channel 0.

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

 - The number of ready channels in the ready list (new length of ready list).

## *Type*: `Command`

Commands are the way the Daku application sends messages to the environment.

### Fields

 - `size: int` - Size of data pointed to by `addr`, in bytes.
 - `addr: ptr` - Pointer to `size` bytes to be sent as a command.
 - `channel: int` - The channel on which to send the command.
 - `ready: val` - Arbitrary 32-bit value to write to ready list (`notify` only).
