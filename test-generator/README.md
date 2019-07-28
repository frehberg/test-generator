[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-MIT)
[![Apache License](http://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-APACHE)
# Test generator

This crate provides `#[test_resources]` and `#[bench_resources]` procedural macro attributes
that generates multiple parametrized tests using one body with different resource input parameters.
A test is generated for each resource matching the specific resource location pattern.

The following examples assume the package layout is as follows:

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

## Example usage `test`:

 ```
 #![cfg(test)]
 extern crate test_generator;

 use test_generator::test_resources;

 #[test_resources("res/*/input.txt")]
 fn verify_resource(resource: &str) { assert!(std::path::Path::new(resource).exists()); }
 ```

 Output from `cargo test` for 3 test-input-files matching the pattern, for this example:

 ```
 $ cargo test

 running 3 tests
 test tests::verify_resource_res_set1_input_txt ... ok
 test tests::verify_resource_res_set2_input_txt ... ok
 test tests::verify_resource_res_set3_input_txt ... ok

 test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 ```
 ## Example usage `bench`:

 ```
 #![feature(test)] // nightly feature required for API test::Bencher

 extern crate test; /* required for test::Bencher */

 extern crate test_generator;
 use test_generator::bench_resources;

 mod bench {
     #[bench_resources("res/*/input.txt")]
     fn measure_resource(b: &mut test::Bencher, resource: &str) {
         let path = std::path::Path::new(resource);
         b.iter(|| path.exists());
     }
 }
 ```
 Output from `cargo +nightly bench` for 3 bench-input-files matching the pattern, for this example:

 ```
 running 3 tests
 test bench::measure_resource_res_set1_input_txt ... bench:       2,492 ns/iter (+/- 4,027)
 test bench::measure_resource_res_set2_input_txt ... bench:       2,345 ns/iter (+/- 2,167)
 test bench::measure_resource_res_set3_input_txt ... bench:       2,269 ns/iter (+/- 1,527)

 test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out
 ```

## Example
 The [example](https://github.com/frehberg/test-generator/tree/master/example) demonstrates usage
 and configuration of these macros, in combination with the crate
 `build-deps` monitoring for any change of these resource files and conditional rebuild.

## Internals
 Let's assume the following code and 3 files matching the pattern "res/*/input.txt"
 ```
 #[test_resources("res/*/input.txt")]
 fn verify_resource(resource: &str) { assert!(std::path::Path::new(resource).exists()); }
 ```
 the generated code for this input resource will look like
 ```
 #[test]
 #[allow(non_snake_case)]
 fn verify_resource_res_set1_input_txt() { verify_resource("res/set1/input.txt".into()); }
 #[test]
 #[allow(non_snake_case)]
 fn verify_resource_res_set2_input_txt() { verify_resource("res/set2/input.txt".into()); }
 #[test]
 #[allow(non_snake_case)]
 fn verify_resource_res_set3_input_txt() { verify_resource("res/set3/input.txt".into()); }
 ```
 Note: The trailing `into()` method-call permits users to implement the `Into`-Trait for auto-conversations.


## Conditional Build Process

The Test-Funciton-Generator shall be rerun every time a new resource-file is added or one of the existing ones have been changed.

The conditional build can be realized using the crate `build-deps`, that is combining the functionality of the
crates `build-helper` and `glob`. The user specifies a directory or a set of files, or a  a filter-pattern to be watched by 
cargo process shall for changes. In case of changes, the build-process of the Rust-sources is re-run.

The following diagram illustrates the integration of the build-script into the conditional cargo build-process.

![ <Diagram - Build Script Intregration> ](docs/build-script-sequence.png)


## GLOB Filter Pattern Examples

The filter may be a glob-pattern containing wildcards, for example:

`"res/*"`  will enumerate all files/directories in directory "res/" and watching changes

`"res/"` - will add the directory itself to the watch-list, triggering a rerun in case new entities are added.

`"res/**/*.protobuf"` will traverse all sub-directories enumerating all protobuf files.

`"res/**"` will traverse all sub-directories enumerating all directories
