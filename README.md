# AB1024-EGA

An `embedded-hal` driver for AB1024-EGA (as used in Inkplate 6 COLOR). Included
examples assume Inkplate 6 COLOR but the driver itself should be usable with
AB1024-EGA connected to other boards.

I've tried to strike a balance between making tests and examples easy to run even though these run in very different environments. This is why the esp toolchain necessary for examples is not selected for you automatically by a `rust-toolchain.toml`.  It must be selected manually (assuming it is not the system default).

To run examples:
`cargo +esp re inkplate_image`

To run tests:
`cargo test --tests`

## TODO
- Eliminate panics in driver.  Replace unwrap with proper errors.
- Document
- Test
- State pattern for init

- If possible, use a dedicated partition for image in inkscape_image example.
  This should allow faster reflashing when changes are made to the code.
- Add partial updates (if the hardwrae supports it).  Reverse engineer?  This
  could make a buffer unnecessary.
