// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>
extern crate glob;
extern crate proc_macro;

use proc_macro::TokenStream;

use self::glob::{glob, Paths};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Lit, Ident, Token};


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

/// Function-Attribute macro expanding glob-file-pattern to a list of directories
/// and generating a test-function for each one.
///
/// ```
/// extern crate test_generator;
/// #[cfg(test)]
/// mod tests {
///   use std::path::Path;
///   use test_generator::glob_expand;
///
///   glob_expand! { "data/*"; generic_test }
///
///   fn generic_test(dir_name: &str) { assert!(Path::new(dir_name).exists()); }
/// }
/// ```
/// The macro will expand the code for each subfolder in `"data/*"`, generating the following
/// code. This code is not visible in IDE. Every build-time, the code will be newly generated.
///
///```
///mod tests {
///    ///
///    ///
///    #[test]
///    fn gen_data_testset1() {
///        generic_test("data/testset1");
///    }
///
///    ///
///    ///
///    #[test]
///    fn gen_data_testset2() {
///        generic_test("data/testset2");
///    }
///}
///
///```
#[proc_macro]
pub fn glob_expand(item: TokenStream) -> TokenStream {
    println!("item: \"{}\"", item.to_string());
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
    fn concat(accu: proc_macro2::TokenStream, ts: proc_macro2::TokenStream)
              -> proc_macro2::TokenStream {
        quote! { #accu #ts }
    }

    // for each path generate a test-function and fold them to single tokenstream
    let result = paths.map(|path| {
        let path_as_str = path.expect("No such file or directory")
            .into_os_string().into_string().expect("bad encoding");

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
        let func_ident = proc_macro2::Ident::new(&func_name,
                                                 proc_macro2::Span::call_site());

        let item = quote! {
            #[test]
            fn #func_ident () {
               println!("path: {}", #path_as_str);
               let f = #lambda ;
               f( #path_as_str );
            }
        };

        item
    }).fold(empty_ts, concat);

    result.into()
}