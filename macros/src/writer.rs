use std::iter::FromIterator;
use std::string::ToString;

use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Semi;
use syn::{Expr, Ident};

struct Arguments {
    formatter: Expr,
    ins: Expr,
    args: Punctuated<Argument, Semi>,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let formatter = input.parse()?;
        input.parse::<syn::token::Comma>()?;
        let ins = input.parse()?;
        input.parse::<syn::token::FatArrow>()?;
        let content;
        syn::braced!(content in input);
        let args = Punctuated::parse_terminated(&content)?;
        Ok(Self {
            formatter,
            ins,
            args,
        })
    }
}

struct Argument {
    sources: Punctuated<Expr, syn::token::Comma>,
    target: Ident,
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let sources = content.parse_terminated(Expr::parse)?;
        input.parse::<syn::token::RArrow>()?;
        let target = input.parse()?;
        Ok(Self { sources, target })
    }
}

impl Arguments {
    fn format_mnemonic(&self, tokens: &mut Vec<TokenTree>) {
        let arg = &self.args[0];
        assert!(!arg.sources.is_empty());
        self.format_call(tokens, &arg.target, self.ins_call(&arg.sources[0]))
    }

    fn format_call(&self, tokens: &mut Vec<TokenTree>, method_arg: &Ident, args: TokenStream) {
        let method_name = format!("write_{}", method_arg.to_string());
        let method_name = Ident::new(&method_name, method_arg.span());
        let formatter = &self.formatter;
        tokens.extend(quote!(#formatter.#method_name(#args)?;))
    }

    fn ins_call(&self, call: &Expr) -> TokenStream {
        let ins = &self.ins;
        quote!(#ins.#call())
    }
}

pub(crate) fn write_asm(input: TokenStream) -> syn::Result<TokenStream> {
    let arguments: Arguments = syn::parse2(input)?;
    assert!(!arguments.args.is_empty());

    let mut tokens = Vec::<TokenTree>::new();
    arguments.format_mnemonic(&mut tokens);
    let mut offset_open = false;
    for (i, arg) in arguments.args.iter().enumerate().skip(1) {
        // Separate operands from one another unless the last one was an offset.
        if !offset_open {
            if i == 1 {
                arguments.format_call(
                    &mut tokens,
                    &Ident::new("opcode_separator", arg.target.span()),
                    quote!(),
                );
            } else {
                arguments.format_call(
                    &mut tokens,
                    &Ident::new("operand_separator", arg.target.span()),
                    quote!(),
                );
            }
        }
        // Arguments to out.write_x(...);
        let format_args = arg.sources.iter().map(|src| arguments.ins_call(src));
        let format_args_punct: Punctuated<TokenStream, syn::token::Comma> =
            Punctuated::from_iter(format_args);
        // Create call.
        if arg.target.to_string().starts_with("offset") {
            // Offsets are a special case since we need to call close afterwards.
            if offset_open {
                return Err(syn::Error::new(
                    arg.target.span(),
                    "two consecutive offset arguments",
                ));
            }
            arguments.format_call(
                &mut tokens,
                &Ident::new(&(arg.target.to_string() + "_open"), arg.target.span()),
                format_args_punct.to_token_stream(),
            );
            offset_open = true;
        } else {
            arguments.format_call(
                &mut tokens,
                &arg.target,
                format_args_punct.to_token_stream(),
            );
            if offset_open {
                arguments.format_call(
                    &mut tokens,
                    &Ident::new("offset_close", arg.target.span()),
                    quote!(),
                );
                offset_open = false;
            }
        }
    }

    Ok(TokenStream::from_iter(tokens.into_iter()))
}