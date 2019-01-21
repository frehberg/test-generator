[![Apache 2.0 licensed][licence-badge]][licence-url]
# Rust build-script dependencies generator 

This is the Rust build-script dependencies generator for data/IDL files

It is integrated into the build.script `build.rs` and prints the path-names of 
data files or test-input-files to console . The output will be evaluated by cargo-build-tool
and the compilatoin is re-run if specified files have changed since the last build.

## Setup

This  illustrates a setup. Intention is to rerun the build-process if the files in 
directory "data/*" have been modified. During the rerun build, the modified files might be read 
to generate code  with proc_macros.

A complete example can be found at github [test-generator/example](https://github.com/frehberg/test-generator/tree/master/example)
#### Cargo.toml

```
[package]
name = "datatester"
build = "build.rs"

...
[build-dependencies]
build-deps = "^0.1"
...
```
#### build.rs
```

// declared in Cargo.toml as "[build-dependencies]"
extern crate build_deps;

fn main() {
   // Enumerate files in sub-folder "data/*", being relevant for the test-generation (as example)
    // If function returns with error, exit with error message.
    build_deps::rerun_if_changed_paths( "data/*" ).unwrap();
}
```

[licence-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[licence-url]: LICENSE.md
