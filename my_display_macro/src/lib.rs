extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SimpleDisplay)]
pub fn simple_display_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident; // The name of the enum

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let debug_str = format!("{:?}", self);
                let end_pos1 = debug_str.find('(').unwrap_or(debug_str.len());
                let end_pos2 = debug_str.find('{').unwrap_or(debug_str.len());
                let ep = if end_pos1 > end_pos2 {end_pos2} else {end_pos1};
                let trimmed_str = &debug_str[..ep];
                let lowercase_str = trimmed_str.to_lowercase();
                write!(f, "{}", lowercase_str)
            }
        }
    };

    TokenStream::from(expanded)
}
