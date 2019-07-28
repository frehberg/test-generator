// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 or MIT License

#![feature(test)] // nightly feature required for API test::Bencher

#[macro_use]
extern crate test_generator;

extern crate test; /* required for test::Bencher */

mod bench {
    // For all subfolders matching "res/*/input.txt" do generate a test function
    // For example:
    // In case of the following resources in these two folders "res/set1/input.txt"
    // and "res/set2/input.txt" the following test functions
    // would be created
    // ```
    // #[test]
    // fn measure_resource_res_set1_input_txt(b: &mut test::Bencher) {
    //     measure_resource(b, "res/set1/input.txt".into());
    // }
    //
    // #[test]
    // fn measure_resource_res_set2_input_txt(b: &mut test::Bencher) {
    //     measure_resource(b, "res/set2/input.txt".into());
    // }
    // ```
    #[bench_resources("res/*/input.txt")]
    fn measure_resource(b: &mut test::Bencher, resource: &str) {
        let path = std::path::Path::new(resource);
        b.iter(|| path.exists());
    }
}
