use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, ItemFn, Result, Token,
};

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

#[proc_macro_attribute]
pub fn p_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut output = quote! {};
    let attr = Punctuated::<TestArgument, Token![,]>::parse_terminated
        .parse(attr)
        .unwrap();

    let input = parse_macro_input!(item as ItemFn);
    let fn_sig = &input.sig;
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    output.extend(quote! {
        #fn_sig {
            #fn_block
        }
    });

    let mut test_functions = quote! {};

    for pt_arg in attr.iter() {
        let name = &pt_arg.name;
        let args = &pt_arg.args;
        let expected = &pt_arg.expected;

        test_functions.extend(quote! {
            #[test]
            fn #name() {
                // if let Some(exp) = #expected {
                    assert_eq!(#fn_name #args, #expected);
                // } else {
                //     #fn_name #args;
                // }
            }
        });
    }

    output.extend(quote! {
        #[cfg(test)]
        mod tests {
            use super::*;
            #test_functions
        }
    });

    output.into()
}
