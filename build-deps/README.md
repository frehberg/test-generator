[![Apache 2.0 licensed][licence-badge]][licence-url]
# Rust build-script dependencies generator 

This is the Rust build-script dependencies generator for data/IDL files

The functionality shall be integrated into the build.script `build.rs`. The function `rerun_if_changed_paths(glob_pattern: &str)`
will expand the GLOB pattern and will print the files-paths and directory paths to console. The Cargo-tool will evaluate
the output. The compilation of the crate is re-run if specified files have changed since the last build.

#### GLOB Pattern Examples

`"data/*"`  will enumerate all files/directories in directory "data/" and watchin changes

`"data/"` - will add the directory itself to the watch-list, triggering a rerun in case new entities are added.

`"data/**/*.protobuf"` will traverse all sub-directories enumerating all protobuf files.

`"data/**"` will traverse all sub-directories enumerating all directories

##### Rule of thumb

Add files, if changes to files shall be detected.

Add directories, if the build-process shall be rerun in case of _new_ files.

## Setup

This  illustrates a setup. Intention in this example is to rerun the build-process if the files in 
directory "data/*" have been modified or new files have been added to that directory.

The build-process will execute proc_macros reading those files and generating Rust-code.

A complete example/setup can be found at github [test-generator/example](https://github.com/frehberg/test-generator/tree/master/example)

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

    // Adding the parent directory "data" to the watch-list will capture new-files being added
    build_deps::rerun_if_changed_paths( "data" ).unwrap();
}
```

[licence-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[licence-url]: LICENSE.md
