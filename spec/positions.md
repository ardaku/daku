# Positions

## *Type*: `Positions`

A list of audio channel physical positions relative to the user at the origin.

### Fields

 - `list: List[Vector]` - List of speakers / microphones in configuration.
   - `x: float` - X position (-left +right)
   - `y: float` - Y position (-up +down)
   - `z: float` - Z position (-back +front)
   - `w: float` - LFE or Positional? (0 = LFE, 1 = Positional)
