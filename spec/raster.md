# Raster

## *Type*: `Raster`

A buffer of pixels (image, picture, video, etc.).

### Fields

 - `size: Dimensions` Size of the image
 - `addr: ptr` List of `size.width` * `size.height` pixels by row according to
   format.
 - `format: int` Bits per component
   - `8` 8 bits (integer)
   - `16` 16 bits (integer)
   - `32` 32 bits (float)
 - `colorspace: int`
     - `0` Mask (1 component - grayscale/alphascale)
     - `1` RGBA (4 components)
     - `2` BGRA (4 components)
