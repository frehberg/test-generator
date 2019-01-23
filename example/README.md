[![Apache 2.0 licensed][licence-badge]][licence-url]
# Rust Test-Generator Example

A small library enumerating/filtering a user defined 
This is tan example demonstrating useage of the Rust build-script dependencies generator  `build-deps` and 
the Test Function generator `test-generator`.

## Conditional Build Process

The Test-Funciton-Generator is executed every time a new data files is added or one of the existing files is changed.
The conditional build is realized using the crate `build-deps`, combining the functionality of build-helper and glob.

The following diagram illustrates the integration of the build-script into the conditional cargo build-process.

![Build Script Intregration](docs/build-script-sequence.png)

[licence-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[licence-url]: LICENSE.md
