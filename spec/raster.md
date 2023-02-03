# Raster

## *Type*: `Raster`

A buffer of pixels (image, picture, video, etc.).

### Fields

 - `size: val`
   - `width: half` - Width of the image
   - `height: half` - Height of the image
 - `addr: ptr` - RGBA list of `size` pixels according to format.  Components
   must be packed into a power of two for the total bytes of all components in a
   pixel.
 - `format: int`: Bits per component
   - `8`: 8 bits (integer)
   - `16`: 16 bits (integer)
   - `32`: 32 bits (float)
 - `colorspace: int`
     - `0`: Mask (1 component - grayscale/alphascale)
     - `1`: RGBA (4 components)
     - `2`: BGRA (4 components)
