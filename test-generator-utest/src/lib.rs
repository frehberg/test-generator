//! # Overview
//! This crate provides the macro `utest!(..)` Implementing the 3 phases setup/test/teardown.
//! [![Crates.io](https://img.shields.io/crates/v/test-generator.svg)](https://crates.io/crates/test-generator-utest)
//! [![MIT License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-MIT)
//! [![Apache License](http://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-APACHE)
//! [![Example](http://img.shields.io/badge/crate-Example-red.svg)](https://github.com/frehberg/test-generator/tree/master/example)
//!
//! [Documentation](https://docs.rs/test-generator-utest/)
//!
//! [Repository](https://github.com/frehberg/test-generator/)
//!
//! # Getting Started
//!
//! First of all you have to add this dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! test-generator-utest = "^0.2"
//! ```
//! The test-functionality is supports stable Rust.

//! ```ignore
//! #![cfg(test)]
//! extern crate test_generator-utest;
//!
//! // Don't forget that procedural macros are imported with `use` statement,
//! // for example importing the macro 'test_resources'
//! #![cfg(test)]
//! use test_generator-utest::utest;
//! ```
//!

/// Macro implementing the 3 phases setup/test/teardown
///
/// # Usage
///
/// The `utest` functionality supports the stable release of Rust-compiler since version 1.30.
///
/// ```ignore
/// #[cfg(test)]
/// extern crate test_generator_utest;
///
/// // demonstrating usage of utest-harness
/// mod testsuite {
///     use std::fs::File;
///     use std::io::prelude::*;
///
///     use test_generator_utest::utest;
///
///     utest!(hello_world,
///         || setup("/tmp/hello_world.txt"),
///         |ctx_ref| test_write_hello_world(ctx_ref),
///         |ctx|teardown(ctx));
///
///     utest!(hello_europe,
///         || setup("/tmp/hello_europe.txt"),
///         test_write_hello_europe,
///         teardown);
///
///     // Defining a context structure, storing the resources
///     struct Context<'t> { file: File, name: &'t str }
///
///     // Setup - Initializing the resources
///     fn setup<'t>(filename: &str) -> Context {
///         // unwrap may panic
///         Context { file: File::create(filename).unwrap(), name: filename }
///     }
///
///     // Teardown - Releasing the resources
///     fn teardown(context: Context) {
///         let Context { file, name } = context;
///         // drop file resources explicitly
///         std::mem::drop(file);
///         // unwrap may panic
///         std::fs::remove_file(name).unwrap();
///     }
///
///     // Test - verify feature
///     fn test_write_hello_world(ctx: &Context) {
///         // may panic
///         let mut file = ctx.file.try_clone().unwrap();
///         // may panic
///         file.write_all(b"Hello, world!\n").unwrap();
///         // !!!! although this assertion will fail, the teardown function will be invoked
///         assert_eq!(1, 0);
///     }
///
///     // Test - verify feature
///     fn test_write_hello_europe(ctx: &Context) {
///         // may panic
///         let mut file = ctx.file.try_clone().unwrap();
///         // may panic
///         file.write_all(b"Hello, Europe!\n").unwrap();
///     }
/// }
/// ```
#[macro_export]
macro_rules! utest {
    ( $id: ident, $setup:expr, $test:expr, $teardown:expr ) => {
        #[test]
        fn $id() {
            let context = std::panic::catch_unwind(|| $setup());

            assert!(context.is_ok());

            // unwrap the internal context item
            let ctx = match context {
                Ok(ctx) => ctx,
                Err(_) => unreachable!(),
            };

            let result = std::panic::catch_unwind(|| $test(&ctx));

            let finalizer = std::panic::catch_unwind(|| $teardown(ctx));

            assert!(result.is_ok());

            assert!(finalizer.is_ok());
        }
    };
}
