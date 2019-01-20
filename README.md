[![Apache 2.0 licensed][licence-badge]][licence-url]
# Test generator

The test-generator is a test-function-generator. 

Use this macro if you need to test a single feature with various independent input files
or if you want to parallelize the test over all elements of a large array. 

This macro obsoletes copy-paste of test-functions.

### Test example: Generate for each file matching the pattern
Generate a test-function call for each file matching the pattern
```
extern crate test_generator;
#[cfg(test)]
mod tests { 
    test_generator::test_expand_paths! { test_exists; "data/*" }

    fn test_exists(dir_name: &str) { assert!(std::path::Path::new(dir_name).exists()); }
}
```
Assuming  `"data/*"` expands to "data/set1", and "data/set2" the macro will expand to
```
mod tests {
    #[test]
    fn test_exists_data_set1() {
        test_exists("data/set1");
    }

    #[test]
    fn test_exists_data_set2() {
        test_exists("data/set2");
    }
}
```
### Test example: Generate for each list-element
Generate a test-function call for each list-element
 ```
 extern crate test_generator;
 #[cfg(test)]
 mod tests {
   test_generator::test_expand_list! { test_size; [ 10, 100, 1000 ]}

   fn test_size(value: &usize) { assert!( *value > 0 ); }

   const VEC1 = [ 1, 2, 3, 4 ]; /* speaking array names */
   const VEC2 = [ 5, 6, 7, 8 ];
   test_generator::test_expand_list! { test_array_size; [ &VEC1, &VEC2 ]}
   test_generator::test_expand_list! { test_array_size; [ [1, 2, 3, 4], [ 5, 6, 7, 8 ]}

   fn test_array_size<T>(ar: &[T]) {
        assert!(ar.len() > 0);
   }
 }
 ```
 Will expand to test-functions incorporating the array-elements
 ```
mod tests {
    #[test]
    fn test_size_0000000010() { test_size(&10); }
    #[test]
    fn test_size_0000000100() { test_size(&100); }
    #[test]
    fn test_size_0000001000() { test_size(&1000); }

    #[test]
    fn test_array_size_VEC1() { test_array_size( &VEC1 ); }
    #[test]
    fn test_array_size_VEC2() { test_array_size( &VEC2 ); }

    #[test]
    fn test_array_size_01020304() { test_array_size( &[ 1, 2, 3, 4 ] ); }
    fn test_array_size_05060708() { test_array_size( &[ 5, 6, 7, 8 ] ); }
}
```
### Benchmarking example: Generate for each each file matching the pattern
 Generate a benchmark-function call for each file matching the pattern
 ```
 extern crate test_generator;
 #[cfg(test)]
 mod tests {
   test_generator::bench_expand_paths! { bench_exists; "data/*" }

   fn bench_exists(bencher: &mut test::Bencher, filename: &str) {
        let path = std::path::Path::new(filename);
        b.iter(|| { path.exists() });
    }
 }
 ```
 Assuming  `"data/*"` expands to "data/set1", and "data/set2" the macro will expand to
```
mod tests {
    #[bench]
    fn bench_exists_data_set1(bencher: & mut test::Bencher) {
        bench_exists(bencher, "data/set1");
    }

    #[bench]
    fn bench_exists_data_set2(bencher: & mut test::Bencher) {
        bench_exists(bencher, "data/set2");
    }
}
```
### Benchmark example: Generate for each list-element
 Generate a benchmark-function call for each list-element
 ```
 extern crate test_generator;
 #[cfg(test)]
 mod tests {
   test_generator::bench_expand_list! { bench_size; [ 10, 100, 1000 ]}

   fn bench_size(b: &mut test::Bencher, val: &usize) {
      let input = val;
      b.iter(|| { *input > 0 });
   }
 }
 ```
 Will expand to bench-functions incorporating the array-elements
```
mod tests {
    #[bench]
    fn bench_size_0000000010(bencher: & mut test::Bencher) {
        bench_exists(bencher, &10);
    }
    #[bench]
    fn bench_size_0000000100(bencher: & mut test::Bencher) {
        bench_exists(bencher, &100);
    }
    #[bench]
    fn bench_size_0000001000(bencher: & mut test::Bencher) {
        bench_exists(bencher, &1000);
    }
}
```

Please note, the generated function names are unique, formed by the user defined test-function 
and the input-data. So, changing the list of files or the test-input, this will be reflected in
the generated function-name, too.
 
Every time the macro is executed, for each entry in file-system or the array-listing, 
a corresponding test-function is generated.
 
The signature of the user's test-function or bench-function must declare an additional reference-parameter, 
please see above of example.

The generated tests are regular test-functions of the Rust test-framework, and will be executed in parallel.

### Adding to your project:
Add the following line to the project file _Cargo.toml_

**Cargo.toml**
```
...
edition = "2018"
...
[dev-dependencies]
test-generator = "^0.2"
```

### Limitations/Behavior 
* Lambda expressions are not supported, the generic test must be a named function.
* The generated code is not visible/accessible in IDE. 
* In the generated function-name all special chararacters are replaced by `'_'`. The special characters are `' ', '-', '*', '/', ':'`,.
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
* "data/set1" and 
* "data/set2"
* "data/set3"

```
running 24 tests
test tests::bench_exists_data_set1 ... ok
test tests::bench_exists_data_set2 ... ok
test tests::bench_exists_data_set3 ... ok
test tests::bench_size_0000000010 ... ok
test tests::bench_size_0000000100 ... ok
test tests::bench_size_0000001000 ... ok
test tests::test_array_size_01020304 ... ok
test tests::test_array_size_01020304050607_08090a0b0c ... ok
test tests::test_array_size_05060708 ... ok
test tests::test_array_size_2a2b2c2d2e2f30_31323334 ... ok
test tests::test_array_size_VEC1 ... ok
test tests::test_array_size_VEC2 ... ok
test tests::test_size_0000000001 ... ok
test tests::test_size_0000000002 ... ok
test tests::test_exists_data_set1 ... ok
test tests::test_exists_data_set2 ... ok
test tests::test_exists_data_set3 ... ok
test tests::test_string_size_welt ... ok
test tests::test_string_size_hallo ... ok
test tests::test_array_size_0000000000000156000000000000015700000000000001580000000000000159000000000000015a000000000000015b0000 ... ok
test tests::test_array_size_000000000000012d000000000000012e000000000000012f0000000000000130000000000000013100000000000001320000 ... ok

test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

[licence-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[licence-url]: LICENSE.md
