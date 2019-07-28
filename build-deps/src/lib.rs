// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 or MIT License

//! # Rust build-script dependencies generator
//!
//! Rust build-script dependencies generator is intended for the build-script `build.rs'. All files
//! matching the user defined GLOB pattern will be added to Cargo's dependency-checker. In case
//! those files have been modified since last build-process, the build process is re-ran.
//!
//! Expanding the pattern the set _must_ not contain directories. Cargo only supports files
//! for dependency checking. If the expanded set contains a directory the function will continue
//! with next element in the list but returning with error Error::ExpandedPathExpectedFile(String)
//!
//! This way the calling build-script `build.rs` may interrupt the build-process or ignore
//! the presents of a directory along the GLOB-expansion.
//!
//! For further reading see chapter [Cargo Build-Script Output](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script)
//!
//! Note: The cargo application ist storing the build-script-output in the build directory,
//!       for example: `target/debug/build/*/output`.
extern crate glob;
use std::path::{Path};

use self::glob::{glob, Paths};

/// Error cases
#[derive(Clone, Debug)]
pub enum Error {
    /// Invalid GLOB pattern
    InvalidGlobPattern(String),

    /// The pattern contains invalid characters
    InvalidOsString(std::ffi::OsString),

    /// Expanded pattern contains a path that is no file
    ExpandedPathExpectedFile(String),
}

/// Specify a file or directory which, if changed, should trigger a rebuild.
///
/// Reference: This function stems from crate 'build-helper'
fn rerun_if_changed<P: AsRef<Path>>(path: P) {
    println!("cargo:rerun-if-changed={}", path.as_ref().display());
}

/// Exapanding the GLOB pattern and adding dependency to Cargo-build-process
///
/// For example:
/// `"data/*"`  will enumerate all files/directories in directory "data/" and watchin changes
///
/// `"data/"` - will add the directory itself to the watch-list, triggering a rerun in case new entities are added.
///
/// `"data/**/*.protobuf"` will traverse all sub-directories enumerating all protobuf files.
///
/// `"data/**"` will traverse all sub-directories enumerating all directories
///
/// **Rule of thumb**
/// Add files, if changes to files shall be detected.
///
/// Add directories, if the build-process shall be rerun in case of _new_/_removed_ files.
///
/// ```
/// // declared in Cargo.toml as "[build-dependencies]"
/// extern crate build_deps;
///
/// fn main() {
///    // Enumerate files in sub-folder "data/*", being relevant for the test-generation (as example)
///    // If function returns with error, exit with error message.
///    build_deps::rerun_if_changed_paths( "data/*" ).unwrap();
///
///    // Adding the parent directory "data" to the watch-list will capture new-files being added
///    build_deps::rerun_if_changed_paths( "data" ).unwrap();
/// }
/// ```
///
pub fn rerun_if_changed_paths(pattern: &str) -> Result<(), Error> {
    let paths: Paths = glob(&pattern).map_err(|err| Error::InvalidGlobPattern(err.to_string()))?;

    for entry in paths {
        match entry {
            Ok(path) => rerun_if_changed(&path),
            Err(e) => return Err(Error::InvalidGlobPattern(e.to_string())),
        }
    }

    Ok(())
}
