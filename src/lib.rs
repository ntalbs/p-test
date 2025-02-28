#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, ItemFn, Result, Token,
};

/// Input for `p_test` attribute, consists of test name (optional),
/// and a list of test cases. The test name will be used as a module name
/// for the test. When the name is omitted, the test function name
/// will be used instead.
struct Input {
    test_name: Option<Ident>,
    test_cases: Punctuated<TestCase, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let test_name = if input.peek(Ident) {
            let test_name = input.parse::<Ident>()?;
            let _ = input.parse::<Token![,]>()?;
            Some(test_name)
        } else {
            None
        };
        let test_cases = Punctuated::<TestCase, Token![,]>::parse_terminated(input)?;
        Ok(Input {
            test_name,
            test_cases,
        })
    }
}

/// Represent test case, consists of case name, arguments for the test function,
/// and the expected value (optional).
enum TestCase {
    /// (case_name, (args...), expected)
    V1 {
        name: Ident,
        args: Expr,
        expected: Option<Expr>,
    },
    /// (case_name, args...)
    /// One of the args can be used as expected
    V2 {
        name: Ident,
        args: Punctuated<Expr, Token![,]>,
    },
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        let name = content.parse()?;
        let _ = content.parse::<Token![,]>()?;

        if content.peek(syn::token::Paren) {
            let args = content.parse()?;
            let expected = if content.peek(Token![,]) {
                let _ = content.parse::<Token![,]>()?;
                Some(content.parse()?)
            } else {
                None
            };
            Ok(TestCase::V1 {
                name,
                args,
                expected,
            })
        } else {
            let args = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?;
            Ok(TestCase::V2 { name, args })
        }
    }
}

/// The attribute that annotates a function with arguments for parameterized test.
#[proc_macro_attribute]
pub fn p_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as Input);

    let item = parse_macro_input!(item as ItemFn);
    let p_test_fn_sig = &item.sig;
    let p_test_fn_name = &item.sig.ident;
    let p_test_fn_block = &item.block;

    let mut output = quote! {};
    output.extend(quote! {
        #p_test_fn_sig {
            #p_test_fn_block
        }
    });

    let mut test_functions = quote! {};

    for case in attr_input.test_cases {
        match case {
            TestCase::V1 {
                name,
                args,
                expected,
            } => test_functions.extend(quote! {
                #[test]
                fn #name() {
                    #p_test_fn_name(#args, #expected);
                }
            }),
            TestCase::V2 { name, args } => {
                let mut arg_list = quote! {};
                for e in args {
                    arg_list.extend(quote! { #e, });
                }
                test_functions.extend(quote! {
                    #[test]
                    fn #name() {
                        #p_test_fn_name(#arg_list);
                    }
                })
            }
        }
    }

    let test_name = attr_input.test_name.unwrap_or(p_test_fn_name.clone());
    output.extend(quote! {
        #[cfg(test)]
        mod #test_name {
            use super::*;
            #test_functions
        }
    });

    output.into()
}
