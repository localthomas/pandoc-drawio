<!--
SPDX-FileCopyrightText: 2022 localthomas
SPDX-License-Identifier: MIT OR Apache-2.0
 -->

# pandoc-drawio

A pandoc filter that converts draw.io files into common image/vector formats automatically.
Inspired by [pandoc-drawio-filter](https://github.com/tfc/pandoc-drawio-filter), but written in Rust and supporting more output formats.

Currently two output formats are supported:

| Image Output | Pandoc Format |
| ------------ | ------------- |
| SVG | html, html4, html5 |
| PDF | pdf |

## Requirements

Although this tool is a static binary, it requires the executables `xvfb-run` and `drawio` in the `PATH` and currently only works on Linux.
The paths to these executables can be set via configuration flags, use `--help` to see a reference of all available flags.

## Development

To build the third-party license information, the [cargo-about](https://github.com/EmbarkStudios/cargo-about) cargo plugin is required.

This project provides a `flake.nix` and a `shell.nix` file, which can be used with a [flake-enabled nix](https://nixos.wiki/wiki/Flakes) tool to build binaries, images, enter a development shell, and run checks (REUSE compliance and formatting).
Use the flake command `nix flake show` to see what is available and run `nix flake check` before committing.

A test with a single drawio file and pandoc can be executed via the `test.sh` script in the folder `/tests`.

#### License

A list of third-party licenses can be obtained by executing the binary with the `--credits` flag.

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSES/Apache-2.0.txt) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSES/MIT.txt) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
