// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

// declared in Cargo.toml as "[build-dependencies]"
extern crate build_deps;

fn main() {
    // Enumerate files in sub-folder "res/*", being relevant for the test-generation (as example)
    // If function returns with error, exit with error message.
    build_deps::rerun_if_changed_paths("res/*/*").unwrap();

    // Adding the parent directory "res" to the watch-list will capture new-files being added
    build_deps::rerun_if_changed_paths("res/*").unwrap();
}
