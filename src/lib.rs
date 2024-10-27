use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Token};

/// Parse the ceasar cipher of a string literal with the given offset.
///
/// `args[0]` - str literal
/// `args[1]` - integer literal
#[proc_macro]
pub fn caesar_cipher(input: TokenStream) -> TokenStream {
    let input_args =
        parse_macro_input!(input with Punctuated<syn::Expr, Token![,]>::parse_terminated);
    let args = input_args.iter().collect::<Vec<_>>();

    if args.len() != 2 {
        return syn::Error::new_spanned(
            input_args,
            "Expected 2 arguments: string and shift output",
        )
        .to_compile_error()
        .into();
    }

    let invalid_string_error =
        syn::Error::new_spanned(&input_args, "The first argument should be a Str literal!");
    let input_string = match &args[0] {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Str(val) => val,
            _ => {
                return invalid_string_error.to_compile_error().into();
            }
        },
        _ => {
            return invalid_string_error.to_compile_error().into();
        }
    };
    let invalid_shift_error =
        syn::Error::new_spanned(&input_args, "The second argument should be an Int literal!");
    let shift = match &args[1] {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Int(val) => val,
            _ => {
                return invalid_string_error.to_compile_error().into();
            }
        },
        _ => {
            return invalid_shift_error.to_compile_error().into();
        }
    };
    let shift_int = match shift.base10_parse::<u8>() {
        Ok(val) => val,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut output = String::new();

    for c in input_string.value().chars() {
        output.push(match c {
            'A'..='Z' => ((((c as u8) - b'A' + shift_int) % 26) + b'A') as char,
            'a'..='z' => ((((c as u8) - b'a' + shift_int) % 26) + b'a') as char,
            _ => c,
        });
    }

    quote!(#output).into()
}
