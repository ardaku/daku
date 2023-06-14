# Ready List

The "Ready List" is a list of the notifications from notify channels.

It should be an immutable global packed `i64` exported with the name `daku`.

 - `size: u32`: Maximum size of the ready list
 - `addr: ptr`: Pointer to a fixed-size array of `val`s
