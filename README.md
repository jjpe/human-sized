# human-readable

## Synopsis
A simple library to translate a size in bytes to a either a prefixed size of
either the decimal (e.g. `KB`) or binary (e.g. `KiB`) varieties.

## Usage
Add this dependency to your project's `Cargo.toml`:
``` toml
[dependencies]
human-sized = "0.1.0"
```

We can now get a human-readable representation of the size  e.g.
``` rust
use human_sized::binary;

let hsize = binary(4_400)?;
assert_eq!(hsize, "4 KiB");
```
