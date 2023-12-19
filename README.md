# ipl3checksum

![PyPI - Downloads]
![GitHub License]
![GitHub release (latest SemVer)]
![PyPI]
![GitHub contributors]

[PyPI - Downloads]: <https://img.shields.io/pypi/dm/ipl3checksum>
[GitHub License]: <https://img.shields.io/github/license/Decompollaborate/ipl3checksum>
[GitHub release (latest SemVer)]: <https://img.shields.io/github/v/release/Decompollaborate/ipl3checksum>
[PyPI]: <https://img.shields.io/pypi/v/ipl3checksum>
[GitHub contributors]: <https://img.shields.io/github/contributors/Decompollaborate/ipl3checksum?logo=purple>

A library to calculate the IPL3 checksum for N64 ROMs.

Written in Rust. Python and C bindings available.

## How to use it?

To calculate the checksum of a ROM:

```py
import ipl3checksum

romBytes = # A big endian bytes-like object
cickind = ipl3checksum.CICKind.CIC_6102_7101

# or calculateChecksumAutodetect to let the library guess the correct CIC kind
checksum = ipl3checksum.calculateChecksum(romBytes, cickind)

# If this assert fails it is because the library was not able to compute the
# checksum, probably because the passed rom was too small
assert checksum is not None

print(f"{checksum[0]:08X}")
print(f"{checksum[1]:08X}")
```

This library also contains a CIC detector:

```py
cickind = ipl3checksum.detectCIC(romBytes)
# Either a `ipl3checksum.CICKind` object or `None`` if was not able to detect
# the CIC kind
print(cickind)
```

## Features

- Supports all 6 retail CIC variants.
- Supports the CIC 5101 variant (used on Aleck 64 games).
- Can calculate the checksum of a ROM using the algorithm of any of the
supported CIC variants.
- Can detect any of the supported CIC variants.
- Fast calculation written in Rust.

### Restrictions/requirements

- The library assumes the passed ROM contains a ROM header at offset range
`[0x0, 0x40]` and a correct IPL3 is at `[0x40, 0x1000]`
- Since the checksum algorithm is calculated on the first MiB after IPL3 (from
`0x1000` to `0x101000`), then the library expects the passed ROM to be at least
`0x101000` bytes long, otherwise the library will reject the ROM.
  - If your ROM is not big enough then it is suggested then pad your ROM with
    zeroes until it reaches that size.

## Installing

### Python version

First you need to install the library, one way of doing it is via `pip`.

```bash
python3 -m pip install -U ipl3checksum
```

If you use a `requirements.txt` file in your repository, then you can add
this library with the following line:

```txt
ipl3checksum>=1.1.0,<2.0.0
``````

Now you can invoke the library from your script.

#### Development version

The unstable development version is located at the
[develop](https://github.com/Decompollaborate/ipl3checksum/tree/develop)
branch. PRs should be made into that branch instead of the main one.

Since this library uses Rust code then you'll need a Rust compiler installed
on your system. To build the Python bindings you'll also need `maturin`
installed via `pip`.

The recommended way to install a locally cloned repo the following.

```bash
python3 -m pip install .
```

In case you want to mess with the latest development version without wanting to
clone the repository, then you could use the following commands:

```bash
python3 -m pip uninstall ipl3checksum
python3 -m pip install git+https://github.com/Decompollaborate/ipl3checksum.git@develop
```

NOTE: Installing the development version is not recommended unless you know what
you are doing. Proceed at your own risk.

### Rust version

See this crate at <https://crates.io/crates/ipl3checksum>.

To add this library to your project using Cargo:

```bash
cargo add ipl3checksum
```

Or add the following line manually to your `Cargo.toml` file:

```toml
ipl3checksum = "1.1.0"
```

### C bindings

This library provides bindings to call this library from C code. They are
available on the [releases](https://github.com/decompals/ipl3checksum/releases)
tab.

To build said bindings from source, enable the `c_bindings` Rust feature:

```bash
cargo build --lib --features c_bindings
```

Headers are located at [bindings/c/include](bindings/c/include).

#### Windows executables

Due to Rust requirements, linking the C bindings of this library when building
a C program adds extra library dependencies. Those libraries are the following:

```plain_text
-lws2_32 -lntdll -lbcrypt -ladvapi32 -luserenv
```

## Examples

Various examples for the Python bindings are provided in the
[frontends folder](src/ipl3checksum/frontends).

Those examples are distributed with the Python library as cli tools. Each one
of them can be executed with either `ipl3checksum utilityname` or
 `python3 -m ipl3checksum utilityname`, for example `ipl3checksum detect_cic`.

The list can be checked in runtime with `ipl3checksum --help`. Suboptions for
each tool can be checked with `ipl3checksum utilityname --help`.

- `detect_cic`: Tries to detect the cic used from the given big endian rom.
- `sum`: Calculates the ipl3 checksum o a given big endian rom.

## Versioning and changelog

This library follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
We try to always keep backwards compatibility, so no breaking changes should
happen until a major release (i.e. jumping from 1.X.X to 2.0.0).

To see what changed on each release check either the [CHANGELOG.md](CHANGELOG.md)
file or check the [releases page on Github](https://github.com/Decompollaborate/ipl3checksum/releases).
You can also use [this link](https://github.com/Decompollaborate/ipl3checksum/releases/latest)
to check the latest release.

## Where does this come from?

This algorithm comes directly from the IPL3, which each variant is part of the
first 0x1000 bytes of the rom of every retail N64 ROM.

There are various implementations floating around on the internet, but for this
specific one was reverse-engineered by myself. I made this because I couldn't
find a library to calculate this checksum, so I decided to reverse-engineer it
myself instead of taking somebody else's work. It also was an interesting
learning experience.

## Note about licensing

Most of the repository is licensed under the [MIT license](LICENSE), but I also
made a [reference implementation](docs/reference_implementation.md) that is part
of the public domain (licensed under CC0-1.0), feel free to use it however you
prefer (acknowledgment is always appreciated, but not required).

## I want to learn more! What is an IPL3? What is CIC?

I'm not really the guy that can answer all your hunger for knowledge, but here
are a few links that may be helpful:

- CIC-NUS: <https://n64brew.dev/wiki/CIC-NUS>
- Initial Program Load 3 (IPL3) <https://n64brew.dev/wiki/Initial_Program_Load#IPL3>
- List of retail games, containing which CIC they use: <https://docs.google.com/spreadsheets/d/1WgZ7DZSzWwYIxwg03yoN9NK_0okuSx9dVL2u5MWPQ60/edit#gid=1247952340>
- Research about the CIC 6105: <https://github.com/Dragorn421/n64checksum>
- Disassembly of all the retail IPL3 binaries: <https://github.com/decompals/N64-IPL/blob/main/src/ipl3.s>

## References

- "IPL3 checksum algorithm" section of the "PIF-NUS" article on n64brew.dev: <https://n64brew.dev/wiki/PIF-NUS#IPL3_checksum_algorithm>
  - Used for getting the "8-bit IPL3" seed value.
- List of retail games, containing which CIC they use: <https://docs.google.com/spreadsheets/d/1WgZ7DZSzWwYIxwg03yoN9NK_0okuSx9dVL2u5MWPQ60/edit#gid=1247952340>
