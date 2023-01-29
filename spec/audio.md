# Audio

## *Type*: `Audio`

A buffer of floating-point audio.

### Fields

 - `properties: val` - Audio properties
   - `rate: half` - Sample rate of the audio. (low).
   - `rate: byte` - Sample rate of the audio. (high).
   - `count: byte` - Number of channels.
 - `config: Opt[Positions]` - Custom channel position configuration, default is
   FLAC/SMPTE/ITU-R recommendations.
 - `size: int` - Number of samples per channel.
 - `addr: ptr` - List of `chan.count * size` 32-bit floats.

# FLAC/SMPTE/ITU-R recommendations

 - 1 Channel: Mono (Mono)
   1. `(0, 0, 0)`
 - 2 Channels: Stereo (Left, Right)
   1. `(-1, 0, 0)`
   2. `(1, 0, 0)`
 - 3 Channels: Surround 3.0 (Left, Right, Center)
   1. `(-1, 0, 0)`
   2. `(1, 0, 0)`
   3. `(0, 0, 0)`
 - 4 Channels: Surround 4.0 (FrontL, FrontR, SurroundL, SurroundR)
 - 5 Channels: Surround 5.0 (FrontL, FrontR, Front, SurroundL, SurroundR)
 - 6 Channels: Surround 5.1 (FrontL, FrontR, Front, Lfe, SurroundL, SurroundR)
 - 7 Channels: Surround 6.1 (FrontL, FrontR, Front, Lfe, Back, Left, Right)
 - 8 Channels: Surround 7.1 (FrontL, FrontR, Front, Lfe, BackL, BackR, Left, Right)
