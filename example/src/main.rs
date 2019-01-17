// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>
extern crate test_generator;

#[cfg(test)]
mod tests {
    use test_generator::glob_expand;
    use std::path::Path;
    use std::fs::File;
    use std::io::Read;
    ///
    /// For all subfolders "data/*" generate a test
    /// For example:
    /// In case of a subfolder "data/testset1" the following test function would be created
    /// ```
    /// #[test]
    /// fn gen_data_testset1() {
    ///     generic_test("data/testset1");
    /// }
    /// ```
    glob_expand! { "data/*"; generic_test }

    //
    // test reading test-data from specific dir_name
    fn generic_test(dir_name: &str) {
        let input_path = Path::new(dir_name).join("input.in");
        assert!(input_path.exists());
        let mut input_file = match File::open(input_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{}", err);
                panic!();
            }
        };
        let mut input = String::new();
        assert!(input_file.read_to_string(&mut input).is_ok());

        assert!(input.len() > 0);
    }
}

fn main() {
    println!("Hello, world!");
}
