# AB1024-EGA

An `embedded-hal` driver for AB1024-EGA (as used in Inkplate 6 COLOR). Included
examples assume Inkplate 6 COLOR but the driver itself should be usable with
AB1024-EGA connected to other boards.

## TODO
- Eliminate panics in driver.  Replace unwrap with proper errors.
- Document
- Test

- If possible, use a dedicated partition for image in inkscape_image example.
  This should allow faster reflashing when changes are made to the code.
- Add partial updates (if the hardwrae supports it).  Reverse engineer?  This
  could make a buffer unnecessary.
