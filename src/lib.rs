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
        let test_cases =
            Punctuated::<TestCase, Token![,]>::parse_terminated(input)?;
        Ok(Input {
            test_name,
            test_cases,
        })
    }
}

struct TestCase {
    name: Ident,
    args: Expr,
    expected: Option<Expr>,
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        let name = content.parse()?;
        let _ = content.parse::<Token![,]>()?;
        let args = content.parse()?;
        let expected = if content.peek(Token![,]) {
            let _ = content.parse::<Token![,]>()?;
            Some(content.parse()?)
        } else {
            None
        };
        Ok(TestCase {
            name,
            args,
            expected,
        })
    }
}

/// The attribute that annotates function with arguments for parameterized test.
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
        let name = &case.name;
        let args = &case.args;
        let expected = &case.expected;

        test_functions.extend(quote! {
            #[test]
            fn #name() {
                #p_test_fn_name(#args, #expected);
            }
        });
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
