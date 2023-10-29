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

### Data

Data starts with a WebAssembly integer (ULEB128-encoded 32-bit unsigned)
representing the channel number.  Channel 0 is for custom host APIs (which can
in turn allocate new channels).  If `size` is set to 0 and `addr` is null (0),
then interpret as pass (no-op command, forcing `ar()` to return immediately).
If `size` is set to 0 and `addr` is non-null (not 0), then update the ready list
with `Ready`.

## *Type*: `Ready`

Update ready list (list of channels which are ready).

### Fields

 - `ready_size: int` Ready list size (non-zero)
 - `ready_addr: ptr[int]` Ready list address (non-null)
