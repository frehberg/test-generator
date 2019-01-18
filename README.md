[![Apache 2.0 licensed][licence-badge]][licence-url]
# Test generator

The test-generator is a test-function-generator. 

The user will  specify two elements: 1) a _glob_ file-pattern 
(such as `"data/*"`) and 2) a generic test-function. The macro will list all entries in file-system and will generate a test-function for each entry in file-system.

This will improve and speed-up testing: Instead of iterating all 
test-sets within a single test-function, this macro will create a test-function for each test-set located in file-system.
These test-functions are executed concurrently and independently from each other. In case of test-failures, it will 
be easier to identify the causing test-set.
 
Moreover the developer no longer has got to keep in sync two locations: 1) the growing/shrinking number of test-sets
and 2) the corresponding, explicit test-functions.
Instead, every time the macro is executed, for each entry in file-system a corresponding test-function
is generated.

### Usage:
Add the following line to the project file _Cargo.toml_

**Cargo.toml**
```
...
edition = "2018"
...
[dev-dependencies]
test-generator = "^0.1"
```

Define tests depending on the file expansion "data/*"

**main.rs**
```rust 
extern crate test_generator;

#[cfg(test)]
mod tests {
    use test_generator::glob_expand;
    use std::path::Path;
    use std::fs::File;
    use std::io::Read;

    // 
    // macro expanding tests
    glob_expand! { "data/*"; generic_test }
    
    //
    // test reading test-data from specific dir_name
    fn generic_test(dir_name: &str) {
        // Every testset-directory contains a file "input.in" being read
        let input_path = Path::new(dir_name).join("input.in");
        let mut input_file = match File::open(input_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{}", err);
                panic!();
            }
        };
        let mut input = String::new();
       
        //verify File-IO was successfull
        assert!(input_file.read_to_string(&mut input).is_ok());
        
        // call the unit-test with contents of file (here should be your code)
        assert!(input.len() > 0);
    }
}
```

### Generator output

The example in this crate contains two testsets "data/testset1/" and data/testset2/".

Invoking the macro `glob_expand! { "data/*"; generic_test }`, two tests will be generated.
Both are invoking the function `generic_test` and the string argument 
directing a specific testset-directory. 

```
mod tests {
    ///
    ///
    #[test]
    fn gen_data_testset1() {
        generic_test("data/testset1");
    }
    
    ///
    ///
    #[test]
    fn gen_data_testset2() {
        generic_test("data/testset2");
    }
}
```
As you will notice from code above, the user's test function **must** have the signature `fn(dir_name: &str)`. 

#### Limitations/Behavior 
* Lambda expressions are not supported, the generic test must be a named function.
* The generated code is not visible/accessible in IDE. 
* The function-name of each generated function has a prefix "gen_", and all special chararacters are replaced by `'_'`. The special characters are `' ', '-', '*', '/'`.
* The library "glob" is used to expand the file patterns, supporting all features of "glob".
* Everytime the hosting file is re-build, the macro will be evaluated and test-functions are created
* Note: If using incremental builds, the test functions may be generated on first build only; adding new testsets to the directory, does not trigger a rebuild/generation. You may enforce a rebuild the following way `cargo clean -p <local-pkg> && cargo build`  

### Test output
This crate is shipped with an example. Invoking the following cargo command 
in the folder ./example, the following results are print to console.
```console
$ cargo test
```
will produce a test-output for both sub-folders according to the pattern "data/*":
* "data/testset1" and 
* "data/testset2"

```
   Compiling proc-macro2 v0.4.25
   Compiling unicode-xid v0.1.0
   Compiling glob v0.2.11
   Compiling quote v0.6.10
   Compiling syn v0.15.26
   Compiling test-generator v0.1.0 (./test-generator)
   Compiling test-generator-example v0.1.0 (./test-generator/example)
item: ""data/*" ; generic_test"
    Finished dev [unoptimized + debuginfo] target(s) in 7.74s
     Running target/debug/deps/test_generator_example-36aa9ae6846d8e41

running 2 tests
test tests::gen_data_testset1 ... ok
test tests::gen_data_testset2 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

[licence-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[licence-url]: LICENSE.md
