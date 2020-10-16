// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 or MIT License

#[cfg(test)]
extern crate test_generator;

#[cfg(test)]
mod tests {
    use test_generator::test_resources;

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
    fn verify_resource(resource: &str) {
        assert!(std::path::Path::new(resource).exists());
    }
}

#[cfg(test)]
extern crate test_generator_utest;

// demonstrating usage of utest-harness
mod testsuite {
    use std::fs::File;
    use std::io::prelude::*;

    use test_generator_utest::utest;

    // Defining a context structure, storing the resources
    struct Context<'t> {
        file: File,
        name: &'t str,
    }

    // Setup - Initializing the resources
    fn setup<'t>(filename: &str) -> Context {
        // unwrap may panic
        Context {
            file: File::create(filename).unwrap(),
            name: filename,
        }
    }

    // Teardown - Releasing the resources
    fn teardown(context: Context) {
        let Context { file, name } = context;

        // drop file resources
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

        std::mem::drop(file);
    }

    // Test - verify feature
    fn test_write_hello_europe(ctx: &Context) {
        // may panic
        let mut file = ctx.file.try_clone().unwrap();

        // may panic
        file.write_all(b"Hello, Europe!\n").unwrap();

        std::mem::drop(file);
    }

    utest!(
        hello_world,
        || setup("/tmp/hello_world.txt"),
        |ctx_ref| test_write_hello_world(ctx_ref),
        |ctx| teardown(ctx)
    );

    utest!(
        hello_europe,
        || setup("/tmp/hello_europe.txt"),
        test_write_hello_europe,
        teardown
    );
}
