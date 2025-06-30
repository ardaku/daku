# 0x00 - Log

Log a message with an associated target and log level, usually to help with
debugging.

## Readiness

Becomes ready once logging has completed (stopping the process after ready
wouldn't result in a partially-formed log message).

## *Command*: `Log`

### Fields

 - `[_; _]`
   - `level: int` Log level
    0. Fatal
    1. Error
    2. Warn
    3. Info
    4. Debug
    5. Trace
    6. Stdout
    7. Stderr
   - `log: opt[_]` Log target and message
     - `target: Text` Target name
     - `message: Text` Message to print
  
### Traps

 0. If `message` is not valid UTF-8, or contains a NUL byte
 1. If `target` is not valid UTF-8, or contains a NUL byte
 2. If address at `message.addr + message.size - 1` has no page
 3. If address at `target.addr + target.size - 1` has no page
