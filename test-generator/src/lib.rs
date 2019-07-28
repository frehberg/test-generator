//! # Overview
//! This crate provides `#[test_resources]` and `#[bench_resources]` procedural macro attributes
//! that generates multiple parametrized tests using one body with different resource input parameters.
//! A test is generated for each resource matching the specific resource location pattern.
//!
//! [![Crates.io](https://img.shields.io/crates/v/test-generator.svg)](https://crates.io/crates/test-generator)
//! [![MIT License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-MIT)
//! [![Apache License](http://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/frehberg/test-generator/blob/master/LICENSE-APACHE)
//! [![Example](http://img.shields.io/badge/crate-Example-red.svg)](https://github.com/frehberg/test-generator/tree/master/example)
//!
//! [Documentation](https://docs.rs/test-generator/)
//!
//! [Repository](https://github.com/frehberg/test-generator/)
//!
//! # Getting Started
//!
//! First of all you have to add this dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! test-generator = "0.3.0"
//! ```
//!
//! With Rust older than 1.30, you have to enable `proc_macro` feature and include crate. You can do this globally by adding:
//! ```ignore
//! #![feature(proc_macro)]
//! extern crate test_generator;
//! ```
//!
//! Don't forget that procedural macros are imported with `use` statement:
//!
//! ```ignore
//! use test_generator::test_resources;
//! ```
//!
//! # Example usage `test`:
//!
//! ```ignore
//! #![cfg(test)]
//! extern crate test_generator;
//!
//! use test_generator::test_resources;
//!
//! #[test_resources("res/*/input.txt")]
//! fn verify_resource(resource: &str) { assert!(std::path::Path::new(resource).exists()); }
//! ```
//!
//! Output from `cargo test` for 3 test-input-files matching the pattern, for this example:
//!
//! ```console
//! $ cargo test
//!
//! running 3 tests
//! test tests::verify_resource_res_set1_input_txt ... ok
//! test tests::verify_resource_res_set2_input_txt ... ok
//! test tests::verify_resource_res_set3_input_txt ... ok
//!
//! test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//! ```
//! # Example usage `bench`:
//!
//! ```ignore
//! #![feature(test)] // nightly feature required for API test::Bencher
//!
//! #[macro_use]
//! extern crate test_generator;
//!
//! extern crate test; /* required for test::Bencher */
//!
//! mod bench {
//!     #[bench_resources("res/*/input.txt")]
//!     fn measure_resource(b: &mut test::Bencher, resource: &str) {
//!         let path = std::path::Path::new(resource);
//!         b.iter(|| path.exists());
//!     }
//! }
//! ```
//! Output from `cargo +nightly bench` for 3 bench-input-files matching the pattern, for this example:
//!
//! ```console
//! running 3 tests
//! test bench::measure_resource_res_set1_input_txt ... bench:       2,492 ns/iter (+/- 4,027)
//! test bench::measure_resource_res_set2_input_txt ... bench:       2,345 ns/iter (+/- 2,167)
//! test bench::measure_resource_res_set3_input_txt ... bench:       2,269 ns/iter (+/- 1,527)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out
//! ```
//!
//! # Example
//! The [example](https://github.com/frehberg/test-generator/tree/master/example) demonstrates usage
//! and configuration of these macros, in combination with the crate
//! `build-deps` monitoring for any change of these resource files and conditional rebuild.
//!
//! # Internals
//! Let's assume the following code and 3 files matching the pattern "res/*/input.txt"
//! ```ignore
//! #[test_resources("res/*/input.txt")]
//! fn verify_resource(resource: &str) { assert!(std::path::Path::new(resource).exists()); }
//! ```
//! the generated code for this input resource will look like
//! ```
//! #[test]
//! #[allow(non_snake_case)]
//! fn verify_resource_res_set1_input_txt() { verify_resource("res/set1/input.txt".into()); }
//! #[test]
//! #[allow(non_snake_case)]
//! fn verify_resource_res_set2_input_txt() { verify_resource("res/set2/input.txt".into()); }
//! #[test]
//! #[allow(non_snake_case)]
//! fn verify_resource_res_set3_input_txt() { verify_resource("res/set3/input.txt".into()); }
//! ```
//! Note: The trailing `into()` method-call permits users to implement the `Into`-Trait for auto-conversations.
//!
extern crate glob;
extern crate proc_macro;

use proc_macro::TokenStream;

use self::glob::{glob, Paths};
use quote::quote;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, ExprLit, Ident, Lit, Token, ItemFn};

// Form canonical name without any punctuation/delimiter or special character
fn canonical_fn_name(s: &str) -> String {
    // remove delimiters and special characters
    s.replace(
        &['"', ' ', '.', ':', '-', '*', '/', '\\', '\n', '\t', '\r'][..],
        "_",
    )
}

/// Return the concatenation of two token-streams
fn concat_ts_cnt(
    accu: (u64, proc_macro2::TokenStream),
    other: proc_macro2::TokenStream,
) -> (u64, proc_macro2::TokenStream) {
    let (accu_cnt, accu_ts) = accu;
    (accu_cnt + 1, quote! { #accu_ts #other })
}

/// MacroAttributes elements
struct MacroAttributes {
    glob_pattern: Lit,
}

/// MacroAttributes parser
impl Parse for MacroAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let glob_pattern: Lit = input.parse()?;
        if ! input.is_empty() {
            panic!("found multiple parameters, expected one");
        }

        Ok(MacroAttributes {
            glob_pattern,
        })
    }
}

/// Macro generating test-functions, invoking the fn for each item matching the resource-pattern.
///
/// The resource-pattern must not expand to empty list, otherwise an error is raised.
/// The generated test-functions is aregular tests, being compiled by the rust-compiler; and being
/// executed in parallel by the test-framework.
/// ```
/// #[cfg(test)]
/// mod tests {
///   extern crate test_generator;
///
///   #[test_resources("res/*/input.txt")]
///   fn verify_resource(resource: &str) { assert!(std::path::Path::new(resource).exists()); }
/// }
/// ```
///
///
#[proc_macro_attribute]
pub fn test_resources(attrs: TokenStream, func: TokenStream) -> TokenStream {
    let MacroAttributes { glob_pattern } = parse_macro_input!(attrs as MacroAttributes);

    let pattern = match glob_pattern {
        Lit::Str(l) => l.value(),
        Lit::Bool(l) => panic!(format!("expected string parameter, got '{}'", &l.value)),
        Lit::Byte(l) => panic!(format!("expected string parameter, got '{}'", &l.value())),
        Lit::ByteStr(_) => panic!("expected string parameter, got byte-string"),
        Lit::Char(l) => panic!(format!("expected string parameter, got '{}'", &l.value())),
        Lit::Int(l) => panic!(format!("expected string parameter, got '{}'", &l.value())),
        Lit::Float(l) => panic!(format!("expected string parameter, got '{}'", &l.value())),
        _ => panic!("expected string parameter"),
    };

    let func_copy: proc_macro2::TokenStream = func.clone().into();

    let func_ast: ItemFn = syn::parse(func)
        .expect("failed to parse tokens as a function");

    let func_ident = func_ast.ident;

    let paths: Paths = glob(&pattern).expect(&format!("No such file or directory {}", &pattern));

    // for each path generate a test-function and fold them to single tokenstream
    let result = paths
        .map(|path| {
            let path_as_str = path
                .expect("No such file or directory")
                .into_os_string()
                .into_string()
                .expect("bad encoding");
            let test_name = format!("{}_{}", func_ident.to_string(), &path_as_str);

            // create function name without any delimiter or special character
            let test_name = canonical_fn_name(&test_name);

            // quote! requires proc_macro2 elements
            let test_ident = proc_macro2::Ident::new(&test_name, proc_macro2::Span::call_site());

            let item = quote! {
                #[test]
                #[allow(non_snake_case)]
                fn # test_ident () {
                    # func_ident ( #path_as_str .into() );
                }
            };

            item
        })
        .fold((0, func_copy), concat_ts_cnt);

    // panic, the pattern did not match any file or folder
    if result.0 == 0 {
        let msg: String = format!("no resource matching the pattern {}", &pattern);
        panic!(msg);
    }
    // transforming proc_macro2::TokenStream into proc_macro::TokenStream
    result.1.into()
}

/// Macro generating bench-functions, invoking the fn for each item matching the resource-pattern.
///
/// The resource-pattern must not expand to empty list, otherwise an error is raised.
/// The generated test-functions is a regular bench, being compiled by the rust-compiler; and being
/// executed in sequentially by the bench-framework.
/// ```ignore
/// #![feature(test)] // nightly feature required for API test::Bencher
/// #[cfg(test)]
/// mod tests {
///   extern crate test_generator;
///
///   #[bench_resources("res/*/input.txt")]
///   fn measure_resource(b: &mut test::Bencher, resource: &str) {
///      let path = std::path::Path::new(resource);
///      b.iter(|| path.exists());
///   }
/// }
/// ```
#[proc_macro_attribute]
pub fn bench_resources(attrs: TokenStream, func: TokenStream) -> TokenStream {
    let MacroAttributes { glob_pattern } = parse_macro_input!(attrs as MacroAttributes);

    let pattern = match glob_pattern {
        Lit::Str(l) => l.value(),
        Lit::Bool(l) => panic!(format!("expected string parameter, got '{}'", &l.value)),
        Lit::Byte(l) => panic!(format!("expected string parameter, got '{}'", &l.value())),
        Lit::ByteStr(_) => panic!("expected string parameter, got byte-string"),
        Lit::Char(l) => panic!(format!("expected string parameter, got '{}'", &l.value())),
        Lit::Int(l) => panic!(format!("expected string parameter, got '{}'", &l.value())),
        Lit::Float(l) => panic!(format!("expected string parameter, got '{}'", &l.value())),
        _ => panic!("expected string parameter"),
    };

    let func_copy: proc_macro2::TokenStream = func.clone().into();

    let func_ast: ItemFn = syn::parse(func)
        .expect("failed to parse tokens as a function");

    let func_ident = func_ast.ident;

    let paths: Paths = glob(&pattern).expect(&format!("No such file or directory {}", &pattern));

    // for each path generate a test-function and fold them to single tokenstream
    let result = paths
        .map(|path| {
            let path_as_str = path
                .expect("No such file or directory")
                .into_os_string()
                .into_string()
                .expect("bad encoding");
            let test_name = format!("{}_{}", func_ident.to_string(), &path_as_str);

            // create function name without any delimiter or special character
            let test_name = canonical_fn_name(&test_name);

            // quote! requires proc_macro2 elements
            let test_ident = proc_macro2::Ident::new(&test_name, proc_macro2::Span::call_site());

            let item = quote! {
                #[bench]
                #[allow(non_snake_case)]
                fn # test_ident (b: &mut test::Bencher) {
                    # func_ident ( b, #path_as_str .into() );
                }
            };

            item
        })
        .fold((0, func_copy), concat_ts_cnt);

    // panic, the pattern did not match any file or folder
    if result.0 == 0 {
        let msg: String = format!("no resource matching the pattern {}", &pattern);
        panic!(msg);
    }

    // transforming proc_macro2::TokenStream into proc_macro::TokenStream
    result.1.into()
}

//
// ------------------ deprecated features ------------------
//

const CONTENT_MAX_LEN: usize = 100;

/// Return the concatenation of two token-streams
fn concat_ts(
    accu: proc_macro2::TokenStream,
    other: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! { #accu #other }
}


/// Parser elements
struct GlobExpand {
    glob_pattern: Lit,
    lambda: Ident,
}

/// Parser reading the Literal and function-identifier from token-stream
impl Parse for GlobExpand {
    fn parse(input: ParseStream) -> Result<Self> {
        let glob_pattern: Lit = input.parse()?;
        input.parse::<Token![;]>()?;
        let lambda: Ident = input.parse()?;

        Ok(GlobExpand {
            glob_pattern,
            lambda,
        })
    }
}

/// Prefix for each generated test-function
const PREFIX: &str = "gen_";

// Compose a new function-identifier from input
fn fn_ident_from_path(fn_ident: &Ident, path: &PathBuf) -> Ident {
    let path_as_str = path
        .clone()
        .into_os_string()
        .into_string()
        .expect("bad encoding");

    // prefixed name & remove delimiters and special characters
    let stringified = format!("{}_{}", fn_ident.to_string(), &path_as_str);

    // quote! requires proc_macro2 elements
    let gen_fn_ident = proc_macro2::Ident::new(
        &canonical_fn_name(&stringified),
        proc_macro2::Span::call_site(),
    );

    gen_fn_ident
}

// Compose a new function-identifier from input
fn fn_ident_from_string(fn_ident: &Ident, name: &str) -> Ident {
    // use at most CONTENT_MAX_LEN
    let safe_len = std::cmp::min(name.len(), CONTENT_MAX_LEN);
    let safe_name = &name[0..safe_len];

    // prefixed name & remove delimiters and special characters
    let stringified = format!("{}_{}", fn_ident.to_string(), safe_name);
    // quote! requires proc_macro2 elements
    let gen_fn_ident = proc_macro2::Ident::new(
        &canonical_fn_name(&stringified),
        proc_macro2::Span::call_site(),
    );

    gen_fn_ident
}

// Stringify the expression: arrays are enumerated, identifier-names are embedded
fn expr_stringified(expr: &Expr, int_as_hex: bool) -> String {
    let stringified = match expr {
        Expr::Lit(lit) => match lit {
            ExprLit {
                lit: litval,
                attrs: _,
            } => match litval {
                Lit::Int(lit) => {
                    let val = lit.value();
                    if int_as_hex {
                        // if u8-range, use two digits, otherwise 16
                        if val > 255 {
                            // not a u8
                            format!("{:016x}", val)
                        } else {
                            format!("{:02x}", val as u8)
                        }
                    } else {
                        format!("{:010}", val)
                    }
                }
                Lit::Char(lit) => {
                    let val = lit.value();
                    format!("{}", val)
                }
                Lit::Str(lit) => {
                    let val = lit.value();
                    val
                }
                Lit::Float(lit) => {
                    let val = lit.value();
                    format!("{}", val)
                }
                _ => panic!(),
            },
        },
        Expr::Array(ref array_expr) => {
            let elems = &array_expr.elems;
            let mut composed = String::new();
            // do not
            let mut cnt: usize = 0;
            // concat as hex-numbers, group by 8
            for expr in elems.iter() {
                // after 8 elements, always insert '_', do not begin with '_'
                if cnt > 0 && cnt % 8 == 0 {
                    composed.push_str("_");
                }
                cnt = cnt + 1;

                let expr_str = expr_stringified(&expr, true);
                composed.push_str(&expr_str);
            }
            composed
        }
        Expr::Path(ref expr_path) => {
            let path = &expr_path.path;
            let leading_colon = path.leading_colon.is_some();
            let mut composed = String::new();

            for segment in &path.segments {
                if !composed.is_empty() || leading_colon {
                    composed.push_str("_")
                }
                let ident = &segment.ident;
                composed.push_str(&ident.to_string());
            }
            composed
        }
        Expr::Reference(ref reference) => {
            let ref_expr = &reference.expr;

            expr_stringified(&ref_expr, int_as_hex)
        }
        _ => panic!(),
    };
    stringified
}

// Compose a new function-identifier from input
fn fn_ident_from_expr(fn_ident: &Ident, expr: &Expr) -> Ident {
    let stringified = expr_stringified(expr, false);

    fn_ident_from_string(fn_ident, &format!("{}", &stringified))
}

/// **deprecated** Function-Attribute macro expanding glob-file-pattern to a list of directories
/// and generating a test-function for each one.
///
/// ```
/// #[cfg(test)]
/// mod tests {
///   extern crate test_generator;
///   test_generator::glob_expand! { "res/*"; test_exists }
///
///   fn test_exists(filename: &str) { assert!(std::path::Path::new(filename).exists()); }
/// }
/// ```
/// The macro will expand the code for each subfolder in `"res/*"`, generating the following
/// code. This code is not visible in IDE. Every build-time, the code will be newly generated.
///
///```
/// #[cfg(test)]
/// mod tests {
///    #[test]
///    fn gen_res_set1() {
///        test_exists("res/testset1");
///    }
///
///    #[test]
///    fn gen_res_set2() {
///        test_exists("res/testset2");
///    }
/// }
///
///```

#[proc_macro]
pub fn glob_expand(item: TokenStream) -> TokenStream {
    let GlobExpand {
        glob_pattern,
        lambda,
    } = parse_macro_input!(item as GlobExpand);

    let pattern = if let Lit::Str(s) = glob_pattern {
        s.value()
    } else {
        panic!();
    };

    let empty_ts: proc_macro2::TokenStream = "".parse().unwrap();

    let paths: Paths = glob(&pattern).expect("Failed to read testdata dir.");

    /// helper, concatting two token-streams
    fn concat(
        accu: proc_macro2::TokenStream,
        ts: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        quote! { # accu # ts }
    }

    // for each path generate a test-function and fold them to single tokenstream
    let result = paths
        .map(|path| {
            let path_as_str = path
                .expect("No such file or directory")
                .into_os_string()
                .into_string()
                .expect("bad encoding");

            // remove delimiters and special characters
            let canonical_name = path_as_str
                .replace("\"", " ")
                .replace(" ", "_")
                .replace("-", "_")
                .replace("*", "_")
                .replace("/", "_");

            // form an identifier with prefix
            let mut func_name = PREFIX.to_string();
            func_name.push_str(&canonical_name);

            // quote! requires proc_macro2 elements
            let func_ident = proc_macro2::Ident::new(&func_name, proc_macro2::Span::call_site());

            let item = quote! {
                # [test]
                fn # func_ident () {
                    let f = #lambda;
                    f( #path_as_str );
                }
            };

            item
        })
        .fold(empty_ts, concat);

    // transforming proc_macro2::TokenStream into proc_macro::TokenStream
    result.into()
}

/// Parser elements
struct ExpandPaths {
    fn_ident: Ident,
    glob_pattern: Lit,
}

/// Parser
impl Parse for ExpandPaths {
    fn parse(input: ParseStream) -> Result<Self> {
        let fn_ident: Ident = input.parse()?;
        input.parse::<Token![; ]>()?;
        let glob_pattern: Lit = input.parse()?;

        Ok(ExpandPaths {
            glob_pattern,
            fn_ident,
        })
    }
}

/// **deprecated** Generate a test-function call for each file matching the pattern
/// ```
/// extern crate test_generator;
/// #[cfg(test)]
/// mod tests {
///   test_generator::test_expand_paths! { test_exists; "res/*" }
///
///   fn test_exists(dir_name: &str) { assert!(std::path::Path::new(dir_name).exists()); }
/// }
/// ```
/// Assuming  `"res/*"` expands to "res/set1", and "res/set2" the macro will expand to
///```
/// mod tests {
///    #[test]
///    fn test_exists_res_set1() {
///        test_exists("res/set1");
///    }
///
///    #[test]
///    fn test_exists_res_set2() {
///        test_exists("res/set2");
///    }
/// }
///```
#[proc_macro]
pub fn test_expand_paths(item: TokenStream) -> TokenStream {
    let ExpandPaths {
        fn_ident,
        glob_pattern,
    } = parse_macro_input!(item as ExpandPaths);

    let pattern = if let Lit::Str(s) = glob_pattern {
        s.value()
    } else {
        panic!();
    };

    let empty_ts: proc_macro2::TokenStream = "".parse().unwrap();

    let paths: Paths = glob(&pattern).expect("Invalid 'paths' pattern.");

    // for each path generate a test-function and fold them to single tokenstream
    let result = paths
        .map(|path| {
            // check for error, shadow the name
            let path = path.expect("No such file or directory");

            // form a function identifier, each path is unique => no index required
            let gen_fn_ident = fn_ident_from_path(&fn_ident, &path);

            let path_as_str = path.into_os_string().into_string().expect("bad encoding");

            let item = quote! {
                # [test]
                fn #gen_fn_ident () {
                    #fn_ident ( #path_as_str );
                }
            };

            item
        })
        .fold(empty_ts, concat_ts);

    // transforming proc_macro2::TokenStream into proc_macro::TokenStream
    result.into()
}

/// **deprecated** Generate a benchmark-function call for each file matching the pattern
/// ```
/// extern crate test_generator;
/// #[cfg(test)]
/// mod tests {
///   test_generator::bench_expand_paths! { bench_exists; "res/*" }
///
///   fn bench_exists(bencher: &mut test::Bencher, filename: &str) {
///        let path = std::path::Path::new(filename);
///        b.iter(|| { path.exists() });
///    }
/// }
/// ```
/// Assuming  `"res/*"` expands to "res/set1", and "res/set2" the macro will expand to
///```ignore
/// #[cfg(test)]
/// mod tests {
///    #[bench]
///    fn bench_exists_res_set1(bencher: & mut test::Bencher) {
///        bench_exists(bencher, "res/set1");
///    }
///
///    #[bench]
///    fn bench_exists_res_set2(bencher: & mut test::Bencher) {
///        bench_exists(bencher, "res/set2");
///    }
/// }
///```
#[proc_macro]
pub fn bench_expand_paths(item: TokenStream) -> TokenStream {
    let ExpandPaths {
        fn_ident,
        glob_pattern,
    } = parse_macro_input!(item as ExpandPaths);

    let pattern = if let Lit::Str(s) = glob_pattern {
        s.value()
    } else {
        panic!();
    };

    let empty_ts: proc_macro2::TokenStream = "".parse().unwrap();

    let paths: Paths = glob(&pattern).expect("Invalid 'paths' pattern.");

    // for each path generate a test-function and fold them to single tokenstream
    let result = paths
        .map(|path| {
            // check for error, shadow the name
            let path = path.expect("No such file or directory");

            // form a function identifier, each path is unique => no index required
            let gen_fn_ident = fn_ident_from_path(&fn_ident, &path);

            let path_as_str = path.into_os_string().into_string().expect("bad encoding");

            let item = quote! {
                # [bench]
                fn #gen_fn_ident (bencher: & mut test::Bencher) {
                    #fn_ident (bencher, #path_as_str );
                }
            };

            item
        })
        .fold(empty_ts, concat_ts);

    // transforming proc_macro2::TokenStream into proc_macro::TokenStream
    result.into()
}

/// Parser elements
struct ExpandList {
    fn_ident: Ident,
    listing: Expr,
}

/// Parser
impl Parse for ExpandList {
    fn parse(input: ParseStream) -> Result<Self> {
        let fn_ident: Ident = input.parse()?;
        input.parse::<Token![; ]>()?;
        let listing: syn::Expr = input.parse()?;

        Ok(ExpandList { fn_ident, listing })
    }
}

/// **deprecated** Generate a test-function call for each list-element
/// ```
/// extern crate test_generator;
/// #[cfg(test)]
/// mod tests {
///   test_generator::test_expand_list! { test_size; [ 10, 100, 1000 ]}
///
///   fn test_size(value: &usize) { assert!( *value > 0 ); }
///
///   const VEC1: &[u8] = &[ 1, 2, 3, 4 ]; /* speaking array names */
///   const VEC2: &[u8] = &[ 5, 6, 7, 8 ];
///   test_generator::test_expand_list! { test_array_size; [ &VEC1, &VEC2 ]}
///   test_generator::test_expand_list! { test_array_size; [ [1, 2, 3, 4], [ 5, 6, 7, 8 ] ] }
///
///   fn test_array_size<T>(ar: &[T]) {
///        assert!(ar.len() > 0);
///   }
/// }
/// ```
/// Will expand to test-functions incorporating the array-elements
///```
/// #[cfg(test)]
/// mod tests {
///    #[test]
///    fn test_size_0000000010() { test_size(&10); }
///    #[test]
///    fn test_size_0000000100() { test_size(&100); }
///    #[test]
///    fn test_size_0000001000() { test_size(&1000); }
///
///    #[test]
///    fn test_array_size_VEC1() { test_array_size( &VEC1 ); }
///    #[test]
///    fn test_array_size_VEC2() { test_array_size( &VEC2 ); }
///
///    #[test]
///    fn test_array_size_01020304() { test_array_size( &[ 1, 2, 3, 4 ] ); }
///    fn test_array_size_05060708() { test_array_size( &[ 5, 6, 7, 8 ] ); }
///
///    fn test_array_size<T>(ar: &[T]) {
///        assert!(ar.len() > 0);
///    }
/// }
///```
#[proc_macro]
pub fn test_expand_list(item: TokenStream) -> TokenStream {
    let ExpandList { fn_ident, listing } = parse_macro_input!(item as ExpandList);

    let expr_array = if let Expr::Array(expr_array) = listing {
        expr_array
    } else {
        panic!();
    };

    let empty_ts: proc_macro2::TokenStream = "".parse().unwrap();

    let elems: syn::punctuated::Punctuated<Expr, _> = expr_array.elems;

    let item = elems
        .iter()
        .map(|expr| {
            let gen_fn_ident = fn_ident_from_expr(&fn_ident, expr);
            let ref_symbol_ts: proc_macro2::TokenStream = match expr {
                Expr::Reference(_) => "".parse().unwrap(),
                _ => "&".parse().unwrap(),
            };

            quote! {
                #[test]
                fn #gen_fn_ident() {
                    let local = #ref_symbol_ts #expr;
                    #fn_ident ( local );
                }
            }
        })
        .fold(empty_ts, concat_ts);

    // transforming proc_macro2::TokenStream into proc_macro::TokenStream
    item.into()
}

/// **deprecated** Generate a benchmark-function call for each list-element
/// ```
/// extern crate test_generator;
/// #[cfg(test)]
/// mod tests {
///   test_generator::bench_expand_list! { bench_size; [ 10, 100, 1000 ]}
///
///   fn bench_size(b: &mut test::Bencher, val: &usize) {
///      let input = val;
///      b.iter(|| { *input > 0 });
///   }
/// }
/// ```
/// Will expand to bench-functions incorporating the array-elements
///```
///#[cfg(test)]
///mod tests {
///    #[bench]
///    fn bench_size_0000000010(bencher: & mut test::Bencher) {
///        bench_exists(bencher, &10);
///    }
///    #[bench]
///    fn bench_size_0000000100(bencher: & mut test::Bencher) {
///        bench_exists(bencher, &100);
///    }
///    #[bench]
///    fn bench_size_0000001000(bencher: & mut test::Bencher) {
///        bench_exists(bencher, &1000);
///    }
///}
///```
#[proc_macro]
pub fn bench_expand_list(item: TokenStream) -> TokenStream {
    let ExpandList { fn_ident, listing } = parse_macro_input!(item as ExpandList);

    let expr_array = if let Expr::Array(expr_array) = listing {
        expr_array
    } else {
        panic!();
    };

    let empty_ts: proc_macro2::TokenStream = "".parse().unwrap();

    let elems: syn::punctuated::Punctuated<Expr, _> = expr_array.elems;

    let item = elems
        .iter()
        .map(|expr| {
            let gen_fn_ident = fn_ident_from_expr(&fn_ident, expr);
            let ref_symbol_ts: proc_macro2::TokenStream = match expr {
                Expr::Reference(_) => "".parse().unwrap(),
                _ => "&".parse().unwrap(),
            };

            quote! {
                # [bench]
                fn #gen_fn_ident (bencher: & mut test::Bencher) {
                    let local = #ref_symbol_ts #expr;
                    #fn_ident (bencher, local );
                }
            }
        })
        .fold(empty_ts, concat_ts);

    // transforming proc_macro2::TokenStream into proc_macro::TokenStream
    item.into()
}
