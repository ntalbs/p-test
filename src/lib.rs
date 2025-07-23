#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, ItemFn, Result, Token, LitStr,
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

/// Represent test case, consists of case name (optional), 
/// and a list of arguments for the test function, (case_name, args...)
/// One of the args can be used as an expected value.
/// If the case name is omitted, the case name will be generated 
/// in `case_{n}` format, where `n` is the case number.
struct TestCase {
    name: Option<LitStr>,
    args: Vec<Expr>,
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        let name = if content.peek(LitStr) {
            let name = content.parse()?;
            let _ = content.parse::<Token![,]>()?;
            Some(name)
        } else {
            None
        };
            let args: Vec<Expr> = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?
                .into_iter()
                .collect();
            Ok(TestCase { name, args })
    }
}

fn test_case_name(name: Option<LitStr>, counter: i32, n_all: usize) -> Ident {
    let n = if let Some(name) = name {
        slugify(&name.value())
    } else {
        if n_all < 10 {
            format!("case_{counter}")
        } else if n_all < 100 {
            format!("case_{counter:02}")
        } else if n_all < 1000 {
            format!("case_{counter:03}")
        } else {
            format!("case_{counter}")
        }
    };
    Ident::new(&n, proc_macro::Span::call_site().into())
}

fn slugify(name: &str) -> String {
    let mut s: String = name
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();

    if s.starts_with(|c: char| c.is_numeric()) {
        s.insert(0, '_');
    }

    s
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
    for TestCase { name, args} in attr_input.test_cases {
        counter += 1;
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
