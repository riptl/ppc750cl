#![feature(proc_macro_quote)]

use proc_macro::TokenStream;

#[macro_use]
mod isa;

#[proc_macro]
pub fn isa(input: TokenStream) -> TokenStream {
    crate::isa::isa(input)
}
