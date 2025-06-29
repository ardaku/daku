# Host Exports

The exported WebAssembly API contains a single function:

## *Function*: `ar()`

Retconned from "Ardaku"; Asynchronous Request function.

Returns once at least one command (with a non-zero `ready` value) completes.  An
early return can be forced with a null noöp command.

```wat
(import "daku" "ar" (func $event
    (param $cmd_size i32)   ;; List[ptr[Command]].size
    (param $cmd_addr i32)   ;; List[ptr[Command]].addr
))
```

### Parameters

 - `$cmd_size` The size of the data pointed to by `$cmd_addr`
 - `$cmd_addr` A pointer in the wasm memory to a list of `$cmd_size` commands
