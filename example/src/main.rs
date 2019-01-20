// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>
#![feature(test)]
extern crate test_generator;

#[cfg(test)]
mod tests {
    extern crate test; /* required for test::Bencher */

    ///
    /// For all subfolders "data/*" generate a test function
    /// For example:
    /// In case of a subfolder "data/testset1" the following test function would be created
    /// ```
    /// #[test]
    /// fn gen_data_testset1() {
    ///     generic_test("data/testset1");
    /// }
    /// ```
    test_generator::glob_expand! { "data/*"; test_exists }

    ///
    /// For all subfolders `"data/*1"` (with letter `1`) generate a test function; each of them starting with same prefix `"group1_"`
    /// For example:
    /// In case of a subfolder "data/testset1" the following test function would be created
    /// ```
    /// #[test]
    /// fn gen_data_testset1() {
    ///     generic_test("data/testset1");
    /// }
    /// ```
    test_generator::test_expand_paths! { test_exists; "data/*" }

    //
    // test reading test-data from specific dir_name
    fn test_exists(filename: &str) {
        assert!(std::path::Path::new(filename).exists());
    }

    ///
    ///
    test_generator::test_expand_list! { test_size; [ 1, 2 ] }

    // User's test function
    fn test_size(value: &usize) {
        assert!(*value > 0);
    }

    //
    //
    test_generator::test_expand_list! { test_array_size; [ [1, 2, 3, 4], [ 5, 6, 7, 8 ] ] }
    test_generator::test_expand_list! { test_array_size; [ [1,2,3,4,5,6,7,8,9,10,11,12], [42,43,44,45,46,47, 48,49,50,51,52] ] }
    test_generator::test_expand_list! { test_array_size; [ [342,343,344,345,346,347, 348,349,350,351,352], [301,302,303,304,305,306,307,308,309,3010,3011,3012] ] }

    // use speaking names for array-elements, showing up
    const VEC1: [u8; 100] = [42; 100];
    const VEC2: [u8; 200] = [42; 200];
    test_generator::test_expand_list! { test_array_size; [ &VEC1, &VEC2 ] }

    //
    // generic test function expecting arrays of various types
    fn test_array_size<T>(ar: &[T]) {
        assert!(ar.len() > 0);
    }

    ///
    ///
    test_generator::test_expand_list! { test_string_size; [ "hallo", "welt" ] }

    //
    // testing the functionality for `test_input`
    fn test_string_size(string: &str) {
        assert!(string.len() > 0);
    }

    ///
    ///
    ///
    test_generator::bench_expand_paths! { bench_exists; "data/*" }

    //
    // test reading test-data from specific dir_name
    fn bench_exists(b: &mut test::Bencher, filename: &str) {
        let path = std::path::Path::new(filename);
        b.iter(|| path.exists());
    }

    ///
    ///
    ///
    test_generator::bench_expand_list! { bench_size; [10,100,1000] }

    //
    // test reading test-data from specific dir_name
    fn bench_size(b: &mut test::Bencher, val: &usize) {
        let input = val;
        b.iter(|| *input > 0);
    }
}

fn main() {
    println!("Hello, world!");
}
