# Text

## *Type*: `Text` (subtype of `List`)

A buffer of UTF-8 text.

### Fields

 - `size: int` Number of bytes pointed to at `addr`.
 - `addr: ptr[byte]` UTF-8 string of `size` bytes.
