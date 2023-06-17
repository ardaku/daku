# 0x00 - Log

Log a message with an associated target and log level, usually to help with
debugging.

## Readiness

Becomes ready once logging has completed (stopping the process after ready
wouldn't result in a partially-formed log message).

## *Command*: `Log`

### Fields

 - `message: Text` - Message to print.
 - `target: Text` Target name; First character is log level
   - `F`: Fail (Trap the task)
   - `E`: Error
   - `W`: Warn
   - `I`: Info
   - `D`: Debug
   - `T`: Trace

### Traps

 - If log level character is invalid
 - If `message` is not valid UTF-8, or contains a NUL byte
 - If `target` is not valid UTF-8, or contains a NUL byte
 - If address at `message.addr + message.size` has no page
 - If address at `target.addr + target.size` has no page
