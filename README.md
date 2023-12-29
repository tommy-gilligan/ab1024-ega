# AB1024-EGA

An `embedded-hal` driver for AB1024-EGA (as used in Inkplate 6 COLOR). Included
examples assume Inkplate 6 COLOR but the driver itself should be usable with
AB1024-EGA connected to other boards.

## TODO
- Eliminate panics in driver.  Replace unwrap with proper errors.
- Include a dither function.
- If possible, use a dedicated partition for image in inkscape_image example.
  This should allow faster reflashing when changes are made to the code.
- Investigate alternatives to Rgb888 for embedded-graphics support.
- Refactor src/lib.rs
- Try a thresholded closest n-matches.  Randomized tie-break.
- Add partial updates (if the hardwrae supports it).  Reverse engineer?  This
  could make a buffer unnecessary.
- Document
- Test
