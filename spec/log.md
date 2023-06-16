# 0x00 - Log

Log a message with an associated target and log level, usually to help with
debugging.

## Readiness

Should only become ready once logging has completed (stopping the process
wouldn't result in a partially-formed log message).

## *Command*: `Log`

### Fields

 - `target_info: val`
   - `target_size: half` - Size of `target_addr` in bytes (maximum 255).
   - `log_level: half` - 0: fatal, 1: error, 2: warn, 3: info, 4: debug, 5:
     trace.
 - `target_addr: ptr[byte]` - UTF-8 string of `size` bytes.
 - `message: Text` - Message to print.
