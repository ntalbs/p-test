use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, ItemFn, Result, Token,
};

#[derive(Clone)]
struct TestArgument {
    name: Ident,
    args: Expr,
    expected: Option<Expr>,
}

impl Parse for TestArgument {
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
        Ok(TestArgument {
            name,
            args,
            expected,
        })
    }
}

struct PTestArgs {
    test_name: Ident,
    test_arguments: Punctuated<TestArgument, Token![,]>,
}

impl Parse for PTestArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let test_name = input.parse::<Ident>()?;
        let _ = input.parse::<Token![,]>()?;
        let test_arguments = Punctuated::<TestArgument, Token![,]>::parse_terminated(input).unwrap();
        Ok(PTestArgs {
            test_name,
            test_arguments,
        })
    }
}

#[proc_macro_attribute]
pub fn p_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ptest_args = parse_macro_input!(attr as PTestArgs);

    let input = parse_macro_input!(item as ItemFn);
    let fn_sig = &input.sig;
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let mut output = quote! {};
    output.extend(quote! {
        #fn_sig {
            #fn_block
        }
    });

    let mut test_functions = quote! {};

    for pt_arg in ptest_args.test_arguments {
        let name = &pt_arg.name;
        let args = &pt_arg.args;
        let expected = &pt_arg.expected;

        test_functions.extend(quote! {
            #[test]
            fn #name() {
                #fn_name(#args, #expected);
            }
        });
    }

    let test_name = ptest_args.test_name;
    output.extend(quote! {
        #[cfg(test)]
        mod #test_name {
            use super::*;
            #test_functions
        }
    });

    output.into()
}
