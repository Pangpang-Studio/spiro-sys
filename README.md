# Rust binding to libspiro

> This project is a fork of https://github.com/MFEK/spiro-sys.rlib.

This is the low-level binding built with `bindgen`.

`build.rs` builds `libspiro` from the Git submodule ([GitHub `fontforge/libspiro`](https://github.com/fontforge/libspiro/issues)).

TODO: Safe Rust wrapper.

Note: LICENSE file refers only to my `build.rs`. Refer to `libspiro` for C code licensing and authorship.

## Example

Please see `tests/spiro_to_beziers.rs`, which outputs an SVG path for the below shape.

![](https://raw.githubusercontent.com/mfeq/spiro-sys/36cd77e43a07c07c7f2c41ba241f765e69cd0998/doc/path5.svg)
