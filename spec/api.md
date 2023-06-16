# Base API

For all APIs, addresses in memory refer to the memory exported as `memory`.
All Daku types are lists of packed 32-bit little-endian values.

The exported WebAssembly API contains a single function:

## *Function*: `ar()`

Retconned from "Ardaku"; Asynchronous Request function.

Returns once at least one command (with a non-zero `ready` value) completes.  An
early return can be forced with a `Pass` no-op command.

```wat
(import "daku" "ar" (func $event
    (param $cmd_size i32)   ;; List[Command].size
    (param $cmd_addr i32)   ;; List[Command].addr
    (result i32)            ;; List[Uint32].size 
))
```

### Parameters

 - `$cmd_size` The size of the data pointed to by `$cmd_addr`
 - `$cmd_addr` A pointer in the wasm memory to a list of `$cmd_size` commands

### Returns

 - The number of ready channels in the ready list (new length of ready list).

## *Type*: `Command`

Commands are the way the Daku application sends messages to the environment.

### Fields

 - `size: int` Size of data pointed to by `addr`, in bytes.
 - `addr: ptr` Pointer to `size` bytes to be sent as a command.
 - `channel: int` The channel on which to send the command.
 - `ready: val` Arbitrary non-zero 32-bit value to write to ready list, or zero
   to forget.

## *Type*: `Ready` (subtype of `Command`)

Update ready list.

### Fields

 - `size: int(0)` Empty buffer
 - `addr: ptr(0)` Pointer to null
 - `ready_size: int` Ready list size (non-zero)
 - `ready_addr: ptr[int]` Ready list address (non-null)

## *Type*: `Pass` (subtype of `Ready`)

No-op command.  Forces `ar()` to return immediately.

### Fields

 - `size: int(0)` Empty buffer
 - `addr: ptr(0)` Pointer to null
 - `ready_size: int(0)` Empty ready list
 - `ready_addr: ptr(0)` Null ready list
