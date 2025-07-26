#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, ItemFn, LitBool, LitStr, Result, Token,
};

#[derive(PartialEq)]
enum Name {
    Some(Ident),
    None,
}

impl Parse for Name {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) {
            let name = Name::Some(input.parse()?);
            let _ = input.parse::<Token![,]>()?;
            Ok(name)
        } else if input.peek(LitStr) {
            let name = input.parse::<LitStr>()?;
            let _ = input.parse::<Token![,]>()?;
            if name.value().is_empty() {
                Ok(Name::None)
            } else {
                Ok(Name::Some(Ident::new(&slugify(&name.value()), name.span())))
            }
        } else {
            Ok(Name::None)
        }
    }
}

/// Input for `p_test` attribute, consists of test name (optional),
/// and a list of test cases. The test name will be used as a module name
/// for the test. When the name is omitted, the test function name
/// will be used instead.
struct Input {
    use_args_for_case_name: bool,
    test_cases: Vec<TestCase>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let use_args_for_case_name =
            if input.peek(Ident) && input.peek2(Token![=]) && input.peek3(LitBool) {
                let option = input.parse::<Ident>()?;
                if option != "use_args_for_case_name" {
                    return Err(syn::Error::new(
                        option.span(),
                        "Expected 'use_args_for_case_name' option",
                    ));
                }
                let _ = input.parse::<Token![=]>()?;
                let use_args_for_case_name = input.parse::<LitBool>()?.value;
                let _ = input.parse::<Token![,]>()?;
                use_args_for_case_name
            } else {
                false
            };
        let test_cases = Punctuated::<TestCase, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();
        Ok(Input {
            use_args_for_case_name,
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
    name: Name,
    args: Vec<Expr>,
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Name>()?;

        let content;
        let _ = parenthesized!(content in input);

        let args: Vec<Expr> = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(TestCase { name, args })
    }
}

fn case_name_with_counter(name: Name, counter: i32, n_all: usize) -> Ident {
    match name {
        Name::Some(name) => name,
        Name::None => {
            let name = if n_all < 10 {
                format!("case_{counter}")
            } else if n_all < 100 {
                format!("case_{counter:02}")
            } else if n_all < 1000 {
                format!("case_{counter:03}")
            } else {
                format!("case_{counter}")
            };
            Ident::new(&name, proc_macro::Span::call_site().into())
        }
    }
}

fn case_name_with_args(args: &[Expr]) -> Ident {
    let name = args
        .iter()
        .map(|e| slugify(&e.to_token_stream().to_string()))
        .collect::<Vec<_>>()
        .join("_");

    if name.is_empty() {
        Ident::new("case", proc_macro::Span::call_site().into())
    } else {
        Ident::new(&name, proc_macro::Span::call_site().into())
    }
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
    for TestCase { name, args } in attr_input.test_cases {
        counter += 1;
        let name = if name == Name::None && attr_input.use_args_for_case_name && !args.is_empty() {
            case_name_with_args(&args)
        } else {
            case_name_with_counter(name, counter, n_all)
        };

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

    output.extend(quote! {
        #[cfg(test)]
        mod #p_test_fn_name {
            use super::*;
            #test_functions
        }
    });

    output.into()
}
