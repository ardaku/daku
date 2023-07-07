# 0x00 - Log

Log a message with an associated target and log level, usually to help with
debugging.

If no target is necessary, prefer empty target for traditional stdout/stderr
compatibility.  Treat `I`/`D`/`T` as stdout, and `W`/`E`/`F` as stderr,
preferring `I` and `W`.

## Readiness

Becomes ready once logging has completed (stopping the process after ready
wouldn't result in a partially-formed log message).

Will not log if log level is not provided.  This is useful for using readiness
to flush the logs, since logs are guaranteed to be ordered.

## *Command*: `Log`

### Fields

 - `message: Text` - Message to print; first character is log level
   - `F`: Fail (Trap the task)
   - `E`: Error
   - `W`: Warn
   - `I`: Info
   - `D`: Debug
   - `T`: Trace
 - `target: Text` Target name

### Traps

 0. If log level character is invalid
 1. If `message` is not valid UTF-8, or contains a NUL byte
 2. If `target` is not valid UTF-8, or contains a NUL byte
 3. If address at `message.addr + message.size` has no page
 4. If address at `target.addr + target.size` has no page
