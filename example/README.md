[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-MIT)
[![Apache 2.0 Licensed](http://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-APACHE)
# Rust Test-Generator Example

This is an example demonstrating useage of the Rust build-script dependencies generator  `build-deps` and 
the test-function generator `test-generator`.

This example is using a package layout as follows:
```
├── build.rs
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── res
│   ├── set1
│   │   ├── expect.txt
│   │   └── input.txt
│   ├── set2
│   │   ├── expect.txt
│   │   └── input.txt
│   └── set3
│       ├── expect.txt
│       └── input.txt
├── src
│   └── main.rs
├── benches
│   └── mybenches.rs
└── tests
    └── mytests.rs
```

This is the package layout of the [example package](https://github.com/frehberg/test-generator/tree/master/example), 
here the tests are located in file `mytests.rs` and bench-tests are located in file `mybenches.rs`; the tests and benches depend 
on the content of the `res/` directory.

The build-script `build.rs` is used to realize conditional re-runs, in case a resource-file has changed or 
(more interesting) if new resource-files have been added to the sub-folder structure `res/`. 
 
## Conditional Build Process

The Test-Funciton-Generator is executed every time a new resource file is added or one of the existing files is changed.
The conditional build is realized using the crate [build-deps](https://crates.io/crates/build-deps), 
expanding a `glob` pattern such as "res/*/input.txt", and communicating this list to the cargo-process via an
 internal Cargo-API.

The following diagram illustrates the integration of the build-script into the conditional cargo build-process.

![ <Diagram - Build Script Intregration> ](docs/build-script-sequence.png)

## Executing the tests and benchs

The macro `test_resources` is compatible to stable Rust-compiler, whereas `benchmarking` is using an unstable compiler
 feature and therefor `bench_resources` requires the  nightly Rust-compiler.

Executing the tests, either with `stable` or `nightly` Rust
```
cargo test
```

Executing the benchmarks requires the  `nightly` release of the Rust-compiler
```
cargo +nightly bench 
```
[licence-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[licence-url]: LICENSE.md
