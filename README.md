# AB1024-EGA (AC057TC1)

An `embedded-hal` SPI driver for AB1024-EGA/AC057TC1 (as used in Inkplate 6
COLOR).

AFAIK there is no way to send partial updates to AB1024-EGA/AC057TC1. Thus,
this driver allocates a ~130kb buffer for pixel data. Included examples
assume Inkplate 6 COLOR but the driver itself should be usable with
AB1024-EGA/AC057TC1 displays connected to other hardware.  This driver is
heavily based on the [Soldered Inkplate Arduino
library](https://github.com/SolderedElectronics/Inkplate-Arduino-library),
hence the LGPL 3 license.

![Inkplate displaying a dithered version of Vincent van Gogh's The Starry
Night][image-photo]

## Running

I've tried to strike a balance between making tests and examples easy to run
even though these run in very different environments. This is why the esp
toolchain necessary for examples is not selected for you automatically by a
`rust-toolchain.toml`.  It must be selected manually (assuming it is not the
system default).

To run examples:
`cargo +esp re $EXAMPLE_NAME`

To run tests:
`cargo test --tests`

## Setting up esp32 environment

An esp32 environment is needed to run the Inkplate examples on Inkplate
hardware.  The specific architecture used on Inkplate 6 COLOR is _not_ RISC-V
but Xtensa.  There are instructions for setting up an Xtensa Rust development
environment at <https://esp-rs.github.io/>.  At time of writing the main steps
can be summarised as:

1. Install espup `cargo install espup`
2. Get espup to do environment setup `espup install`
3. Source environment init file on non-Windows OS. `. $HOME/export-esp.sh`

## TODO
- Eliminate panics in driver.  Replace unwrap with proper errors.
- Document
- Test

[image-photo]: examples/image_photo.jpg
