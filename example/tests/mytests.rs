// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 or MIT License

#[cfg(test)]
#[macro_use]
extern crate test_generator;

#[cfg(test)]
mod tests {

    // For all subfolders matching "res/*/input.txt" do generate a test function
    // For example:
    // In case of the following resources in these two folders "rest1/input.txt"
    // and "res/set2/input.txt" the following test functions
    // would be created
    // ```
    // #[test]
    // fn verify_resource_res_set1_input_txt() {
    //     verify_resource("res/set1/input.txt".into());
    // }
    //
    // #[test]
    // fn verify_resource_res_set2_input_txt() {
    //     verify_resource("res/set2/input.txt".into());
    // }
    // ```
    #[test_resources("res/*/input.txt")]
    fn verify_resource(resource: &str) { assert!(std::path::Path::new(resource).exists()); }
}
