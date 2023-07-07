# Positions

## *Type*: `Positions`

A list of audio channel physical positions relative to the user at the origin.

The position must either be a unit vector (length of 1) or all zeros (center).

### Fields

 - `list: List[Vector]` - List of speakers / microphones in configuration.
   - `x: num` - X position (-left +right)
   - `y: num` - Y position (-up +down)
   - `z: num` - Z position (-back +front)
   - `w: num` - LFE? (0 = No, NaN = Yes)
