[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-MIT)
[![Apache 2.0 Licensed](http://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-APACHE)
# Test generator UTest

This crates implements a 3-phase utest-harness for Rust. The 3 phases are:

* Setup: the setup function must initialize the context; in case the function panics/aborts, the utest is aborted
```fn() -> Context```
* Test: if the setup did return with valud context item, this context is used to invoke the test.
```fn( & Context )```
* Teardown: no matter if the test-function above did panic/abort, the teardown function is invoked to release the context
```fn( Context )```

No matter of any panic or failing assertion in the second phase (feature testing), the teardown function is invoked. The test will either succeed, or otherwise unwinding a failure in the setup-phase, the test-phase or the teardown-phase. 

This crate has been inspired by the [post of Eric Opines](https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab).

## Usage

Please see the test file 'mytests.rs at the executable [example](https://github.com/frehberg/test-generator/tree/master/example).

```rust
#[cfg(test)]
extern crate test_generator_utest;

// demonstrating usage of utest-harness
mod testsuite {
    use std::fs::File;
    use std::io::prelude::*;

    use test_generator_utest::utest;

    // Defining Utest formed by setup, test and teardown
    utest!(hello_world,
        || setup("/tmp/hello_world.txt"),
        |ctx_ref| test_write_hello_world(ctx_ref),
        |ctx|teardown(ctx));


    // Defining Utest formed by setup, test and teardown
    utest!(hello_europe,
        || setup("/tmp/hello_europe.txt"),
        test_write_hello_europe,
        teardown);

    // Defining a context structure, storing the resources
    struct Context<'t> { file: File, name: &'t str }

    // Setup - Initializing the resources
    fn setup<'t>(filename: &str) -> Context {
        // unwrap may panic
        Context { file: File::create(filename).unwrap(), name: filename }
    }

    // Teardown - Releasing the resources
    fn teardown(context: Context) {
        let Context { file, name } = context;
        // explicit dropping of file resource (would be done implcitly otherwise)
        std::mem::drop(file);
        // unwrap may panic
        std::fs::remove_file(name).unwrap();
    }

    // Test - verify feature
    fn test_write_hello_world(ctx: &Context) {
        // may panic
        let mut file = ctx.file.try_clone().unwrap();
        // may panic
        file.write_all(b"Hello, world!\n").unwrap();
        // although this assertion will cause a panic, the teardown function will be invoked 
        assert_eq!(1, 0);
    }

    // Test - verify feature
    fn test_write_hello_europe(ctx: &Context) {
        // may panic
        let mut file = ctx.file.try_clone().unwrap();
        // may panic
        file.write_all(b"Hello, Europe!\n").unwrap();
       
        assert_eq!(1,1);
    }
}
```

Executing the example code ```cargo test -p test-generator-example testsuite```
the testsuote above will print the following output.
 
```
running 2 tests
test testsuite::hello_europe ... ok
test testsuite::hello_world ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out
```
