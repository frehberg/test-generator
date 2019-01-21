// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

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

extern crate build_helper;
extern crate glob;

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

/// Exapanding the GLOB pattern and adding dependency to Cargo-build-process
///
/// The pattern must expand to empty list, or containing only files.
/// Cargo does not watch changes in directories. If the pattern-expansion contains a directory,
/// the function will return with error. The code in build.rs may continue or interrup the build
/// process in this case.
///
/// Example:
///
/// ```
/// // declared in Cargo.toml as "[build-dependencies]"
/// extern crate build_deps;
///
/// fn main() {
///    // Enumerate files in sub-folder "data/*", being relevant for the test-generation (as example)
///    // If function returns with error, exit with error message.
///    build_deps::rerun_if_changed_paths( "data/*" ).unwrap();
/// }
/// ```
///
pub fn rerun_if_changed_paths(pattern: &str) -> Result<(), Error> {
    let paths: Paths = glob(&pattern)
        .map_err(|err| Error::InvalidGlobPattern(err.to_string()))?;

    let mut rv = Ok(());

    for path in paths {
        let pathbuf = path
            .map_err(|err| Error::InvalidGlobPattern(err.to_string()))?;

        let std_path = std::path::Path::new(&pathbuf);
        if std_path.is_dir() || !std_path.is_file() {
            let path_as_str = pathbuf
                .into_os_string()
                .into_string()
                .map_err(|err| Error::InvalidOsString(err.to_owned()))?;

            // set error, but continue with next item, in case the build-script is ignoring errors
            rv = Err(Error::ExpandedPathExpectedFile(path_as_str.to_string()));
        } else {
            build_helper::rerun_if_changed(&pathbuf);
        }
    }

    rv
}