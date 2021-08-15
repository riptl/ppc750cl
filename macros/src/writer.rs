use std::iter::FromIterator;
use std::string::ToString;

use proc_macro2::{Delimiter, Group, TokenStream};
use quote::quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, ExprPath, Ident};

struct Arguments {
    formatter: Expr,
    ins: Expr,
    args: Punctuated<Argument, syn::token::Semi>,
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

/// A single part of an instruction.
///
/// Examples:
/// ```ignore
/// (op.mnemonic, rc, oe) -> mnemonic;
/// d -> fpr;
/// ```
struct Argument {
    sources: Vec<Expr>,
    target: Ident,
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse source part.
        let lookahead = input.lookahead1();
        let sources;
        if lookahead.peek(syn::token::Paren) {
            // Parse multiple if we found a parenthesis.
            let content;
            syn::parenthesized!(content in input);
            sources = content
                .parse_terminated::<Expr, syn::token::Comma>(Expr::parse)?
                .into_iter()
                .collect();
        } else if lookahead.peek(syn::LitStr) {
            let expr = input.parse::<ExprLit>()?.into();
            sources = vec![expr];
        } else if lookahead.peek(syn::LitInt) {
            let expr = input.parse::<ExprLit>()?.into();
            sources = vec![expr];
        } else {
            let expr = input.parse::<ExprPath>()?.into();
            sources = vec![expr];
        }
        input.parse::<syn::token::Colon>()?;
        let target = input.parse()?;
        Ok(Self { sources, target })
    }
}

impl Arguments {
    fn format_mnemonic(&self) -> Vec<TokenStream> {
        let arg = &self.args[0];
        assert!(!arg.sources.is_empty());
        // Print the mnemonic.
        let mut calls = vec![self.format_call(&arg.target, self.ins_call(&arg.sources[0]))];
        // Print any mnemonic suffixes.
        for src in arg.sources.iter().skip(1) {
            calls.push(self.format_call(
                &Ident::new(&src.into_token_stream().to_string(), src.span()),
                self.ins_call(src),
            ));
        }
        calls
    }

    fn format_call(&self, method_arg: &Ident, args: TokenStream) -> TokenStream {
        let arg_str = method_arg.to_string();
        let method_name = format!("write_{}", arg_str);
        let method_name = Ident::new(&method_name, method_arg.span());
        let formatter = &self.formatter;
        if arg_str == "branch_target" {
            quote!(#formatter.write_branch_target(#args, self.addr)?)
        } else {
            quote!(#formatter.#method_name(#args)?)
        }
    }

    fn ins_call(&self, call: &Expr) -> TokenStream {
        match call {
            Expr::Lit(_) => call.to_token_stream(),
            _ => {
                let ins = &self.ins;
                quote!(#ins.#call())
            }
        }
    }
}

pub(crate) fn write_asm(input: TokenStream) -> syn::Result<TokenStream> {
    let arguments: Arguments = syn::parse2(input)?;
    assert!(!arguments.args.is_empty());

    // Create a list of calls to execute.
    let mut calls = Vec::<TokenStream>::new();
    calls.extend(arguments.format_mnemonic());
    let mut offset_open = false;
    for (i, arg) in arguments.args.iter().enumerate().skip(1) {
        // Separate operands from one another unless the last one was an offset.
        if !offset_open {
            if i == 1 {
                calls.push(
                    arguments
                        .format_call(&Ident::new("opcode_separator", arg.target.span()), quote!()),
                );
            } else {
                calls.push(arguments.format_call(
                    &Ident::new("operand_separator", arg.target.span()),
                    quote!(),
                ));
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
            calls.push(arguments.format_call(
                &Ident::new(&(arg.target.to_string() + "_open"), arg.target.span()),
                format_args_punct.to_token_stream(),
            ));
            offset_open = true;
        } else {
            calls.push(arguments.format_call(&arg.target, format_args_punct.to_token_stream()));
            if offset_open {
                calls.push(
                    arguments.format_call(&Ident::new("offset_close", arg.target.span()), quote!()),
                );
                offset_open = false;
            }
        }
    }

    // Wrap calls in a block returning Ok(()).
    calls.push(quote!(std::io::Result::Ok(())));
    let statements = Punctuated::<TokenStream, syn::token::Semi>::from_iter(calls);
    let tokens = Group::new(Delimiter::Brace, statements.to_token_stream());

    Ok(tokens.to_token_stream())
}
