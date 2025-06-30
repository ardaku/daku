# Runtime

The Daku runtime can be used alone or with other parts of the Daku
specification.  When using Daku, your target is a Daku plugin app.  The Daku
runtime specification defines how to asynchronously communicate between the
plugin and the host app (which could either be another WASM sandboxed Daku
plugin or a native Daku engine like Ardaku).

## Guest Exports (Host Imports)

 - Export 32-bit memory as `m`
 - Export function `main() -> ()` as `a`
 - Export ready list global `List[ptr[Command]]` pointer as `r`

## Guest Imports (Host Exports)

 - Import function `ar(cmd_size, cmd_addr)` Asynchronous Request (Retconned from
   "Ardaku")

## Plugin Communication Flow

 - Host calls guest's `a()` function
 - Guest creates a list of commands
 - Guest calls host's `ar()` function with list of commands
 - Host executes commands, when at least one completes, returns from `ar()`
   having set the data at pointer `r` in memory `m` to a list (using default
   length as capacity) of completed `ptr[Command]` list
 - Guest loops through the `ptr[Command]` ready-list and processes them, may
   either free or reüse the command structures
 - Guest generates new list of commands, calling `ar()` again (repeat)
 - Guest's `a()` function completes and plugin has stopped

## *Function*: `ar()`

```wat
(import "daku" "ar" (func $event
   (param $cmd_size i32) ;; List[ptr[Command]].size
   (param $cmd_addr i32) ;; List[ptr[Command]].addr
))
```

### Parameters

 - `$cmd_size` The size of the data pointed to by `$cmd_addr`
 - `$cmd_addr` A pointer in the wasm memory to a `Command` list of size
   `$cmd_size`
