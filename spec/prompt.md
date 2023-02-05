# 0x01 - Prompt (`notify`) 

Receive a line of textual user input from a debugging console.

The text appended to the buffer won't contain the newline character.

### Usage Flow
 - Send a `Prompt` on the prompt portal's channel.
 - Once user enters a line of text in the prompt, become ready.
 - Test if capacity is unchanged
   - Capacity is unchanged
     - Line has been appended to the text buffer
   - Capacity is changed
     - Guest reallocates buffer to requested size
     - Re-sends now modified `Prompt` with new capacity, and possibly new
       `command.addr`
     - Capacity will be unchanged, and line of text appended to text buffer
 - Won't be ready until next line of text has been entered

---

 - Send a `Prompt` on the prompt portal's channel.
 - Send a `Prompt` again with different buffer on the prompt portal's channel
   before ready to overwrite which buffer is being used.

## *Type*: `Prompt`

Read textual user input from some source.

### Fields

 - `capacity: ptr[int]` - Pointer to capacity of `command`.
 - `command: ptr[Text]` - Pointer to user-sent line of input.
