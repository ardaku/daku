# 0x00 - Log (`ignore`)

Log a message with an associated target and log level, usually to help with
debugging.

### Usage Flow
 - Send `Log` on the log portal's channel.
 - "Ready" immediately and concurrently writes the log message

## *Type*: `Log`

### Fields

 - `target_info: val`
   - `target_size: half` - Size of `target_addr` in bytes (maximum 255).
   - `log_level: half` - 0: fatal, 1: error, 2: warn, 3: info, 4: debug, 5:
     trace.
 - `target_addr: ptr[byte]` - UTF-8 string of `size` bytes.
 - `message: Text` - Message to print.
