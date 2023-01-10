# Audio

## *Type*: `Audio`

A buffer of floating-point audio.

### Fields

 - `rate: int` - Sample rate of the audio.
 - `chan: val` - FIXME
   - `config: half` - Positions of channels.
   - `count: half` - Number of channels.
 - `size: int` - Number of samples per channel.
 - `addr: ptr` - List of `chan.count * size` 32-bit floats.
