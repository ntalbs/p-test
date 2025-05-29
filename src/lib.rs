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
    test_cases: Vec<TestCase>,
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
        let test_cases = Punctuated::<TestCase, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();
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
        name: Option<Ident>,
        args: Expr,
        expected: Box<Option<Expr>>,
    },
    /// (case_name, args...)
    /// One of the args can be used as an expected value.
    V2 {
        name: Option<Ident>,
        args: Vec<Expr>,
    },
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        let name = if content.peek(Ident) {
            let name = content.parse()?;
            let _ = content.parse::<Token![,]>()?;
            Some(name)
        } else {
            None
        };

        if content.peek(syn::token::Paren) {
            let args = content.parse()?;
            let expected = if content.peek(Token![,]) {
                let _ = content.parse::<Token![,]>()?;
                Box::new(Some(content.parse()?))
            } else {
                Box::new(None)
            };
            Ok(TestCase::V1 {
                name,
                args,
                expected,
            })
        } else {
            let args: Vec<Expr> = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?
                .into_iter()
                .collect();
            Ok(TestCase::V2 { name, args })
        }
    }
}

fn test_case_name(name: Option<Ident>, counter: i32, n_all: usize) -> Ident {
    if let Some(name) = name {
        name
    } else {
        let name = if n_all < 10 {
            &format!("case_{counter}")
        } else if n_all < 100 {
            &format!("case_{counter:02}")
        } else if n_all < 1000 {
            &format!("case_{counter:03}")
        } else {
            &format!("case_{counter}")
        };
        Ident::new(name, proc_macro::Span::call_site().into())
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

    let mut output = quote! {
        #p_test_fn_sig {
            #p_test_fn_block
        }
    };

    let mut test_functions = quote! {};

    let mut counter = 0;
    let n_all = attr_input.test_cases.len();
    for case in attr_input.test_cases {
        counter += 1;
        match case {
            TestCase::V1 {
                name,
                args,
                expected,
            } => {
                let name = test_case_name(name, counter, n_all);
                test_functions.extend(quote! {
                    #[test]
                    fn #name() {
                        #p_test_fn_name(#args, #expected);
                    }
                })
            }
            TestCase::V2 { name, args } => {
                let name = test_case_name(name, counter, n_all);
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
