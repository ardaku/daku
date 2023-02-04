# Positions

## *Type*: `Positions`

A list of audio channel physical positions relative to the user at the origin.

The position must either be a unit vector (length of 1) or all zeros (center).

### Fields

 - `list: List[Vector]` - List of speakers / microphones in configuration.
   - `x: float` - X position (-left +right)
   - `y: float` - Y position (-up +down)
   - `z: float` - Z position (-back +front)
   - `w: float` - LFE? (0 = No, 1 = Yes)
