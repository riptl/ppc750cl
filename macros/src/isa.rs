use std::iter::FromIterator;
use std::ops::Range;

use itertools::Itertools;
use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use syn::LitInt;

#[derive(Default)]
pub(crate) struct BitRange(Range<u8>);

impl<'de> Deserialize<'de> for BitRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let range_str: String = Deserialize::deserialize(deserializer)?;
        if let Some((start_str, stop_str)) = range_str.split_once("..") {
            let start = start_str.parse::<u8>().map_err(serde::de::Error::custom)?;
            let stop = stop_str.parse::<u8>().map_err(serde::de::Error::custom)?;
            Ok(Self(start..stop))
        } else {
            let bit_idx = range_str.parse::<u8>().map_err(serde::de::Error::custom)?;
            Ok(Self(bit_idx..bit_idx))
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct Field {
    name: String,
    desc: String,
    bits: BitRange,
    signed: bool,
    split: bool,
    arg: Option<String>,
}

impl Field {
    fn variant_identifier(&self) -> Option<TokenTree> {
        if self.name.strip_suffix(".nz").is_none() {
            Some(to_rust_ident(&self.name))
        } else {
            None
        }
    }

    fn express_value(&self, code: TokenStream) -> TokenStream {
        let mask_stop = self.bits.0.end;
        let mask_size = self.bits.0.len();
        quote! {
            (((#code) >> (32 - #mask_stop)) & ((1 << #mask_size) - 1))
        }
    }

    fn express_value_self(&self) -> TokenStream {
        self.express_value(quote!(self.code))
    }

    fn enum_variant_definition(&self) -> Option<TokenStream> {
        let ident = self.variant_identifier()?;
        Some(if let Some(arg) = &self.arg {
            let arg = TokenTree::Ident(Ident::new(arg, Span::call_site()));
            quote! {
                #ident(#arg),
            }
        } else {
            quote! {
                #ident,
            }
        })
    }

    fn construct_variant(&self, code: TokenStream) -> TokenStream {
        let field_variant = self.variant_identifier();
        if let Some(arg) = &self.arg {
            let field_arg = TokenTree::Ident(Ident::new(arg, Span::call_site()));
            let value = self.express_value(code);
            quote! {
                Field::#field_variant(#field_arg(#value as _))
            }
        } else {
            quote! {
                Field::#field_variant
            }
        }
    }

    fn construct_variant_self(&self) -> TokenStream {
        self.construct_variant(quote!(self.code))
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct Opcode {
    name: String,
    desc: String,
    bitmask: u32,
    pattern: u32,
    modifiers: Vec<String>,
    side_effects: Vec<String>,
    args: Vec<String>,
    defs: Vec<String>,
    uses: Vec<String>,
}

impl Opcode {
    fn variant_identifier(&self) -> syn::Result<TokenTree> {
        to_rust_variant(&self.name)
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct Mnemonic {
    name: String,
    opcode: String,
    args: Vec<String>,
    condition: String,
    #[serde(rename = "match")]
    matcher: Vec<MatchField>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct MatchField {
    arg: String,
    value: u32,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct Modifier {
    name: String,
    suffix: char,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct Isa {
    fields: Vec<Field>,
    opcodes: Vec<Opcode>,
    mnemonics: Vec<Mnemonic>,
}

impl Isa {
    pub(crate) fn gen_opcode_enum(&self) -> syn::Result<TokenStream> {
        // Create enum variants.
        let enum_variants = self
            .opcodes
            .iter()
            .map(|opcode| -> syn::Result<TokenStream> {
                let ident = opcode.variant_identifier()?;
                Ok(quote! {
                    #ident,
                })
            })
            .try_collect::<TokenStream, Vec<TokenStream>, syn::Error>()?
            .into_iter();
        let enum_variants = TokenStream::from_iter(enum_variants);

        // Create functions.
        let mnemonic_fn = self.gen_mnemonic_fn()?;
        let detect_fn = self.gen_opcode_detect()?;

        // Create final enum.
        let opcode_enum = quote! {
            #[derive(Debug, Copy, Clone, Eq, PartialEq)]
            pub enum Opcode {
                Illegal = -1,
                #enum_variants
            }
            impl Opcode {
                #mnemonic_fn
                #detect_fn
            }
        };
        Ok(opcode_enum)
    }

    fn gen_mnemonic_fn(&self) -> syn::Result<TokenStream> {
        // Create match arms.
        let match_arms = self
            .opcodes
            .iter()
            .map(|opcode| {
                let variant = opcode.variant_identifier()?;
                let literal = Literal::string(&opcode.name);
                Ok(quote! {
                    Opcode::#variant => #literal,
                })
            })
            .try_collect::<TokenStream, Vec<TokenStream>, syn::Error>()?
            .into_iter();
        let match_arms = TokenStream::from_iter(match_arms);
        // Create final function.
        let mnemonic_fn = quote! {
            pub fn mnemonic(self) -> &'static str {
                match self {
                    Opcode::Illegal => "<illegal>",
                    #match_arms
                }
            }
        };
        Ok(mnemonic_fn)
    }

    pub(crate) fn gen_opcode_detect(&self) -> syn::Result<TokenStream> {
        // Generate if chain.
        let if_chain = self
            .opcodes
            .iter()
            .map(|opcode| {
                let bitmask_str = format!("{:>#8x}", opcode.bitmask);
                let bitmask = LitInt::new(&bitmask_str, Span::call_site());
                let pattern_str = format!("{:>#8x}", opcode.pattern);
                let pattern = LitInt::new(&pattern_str, Span::call_site());
                let identifier = opcode.variant_identifier()?;
                Ok(quote! {
                    if code & #bitmask == #pattern {
                        return Opcode::#identifier;
                    }
                })
            })
            .try_collect::<TokenStream, Vec<TokenStream>, syn::Error>()?
            .into_iter();
        let if_chain = TokenStream::from_iter(if_chain);
        // Generate function.
        let func = quote! {
            pub fn detect(code: u32) -> Self {
                #if_chain
                Opcode::Illegal
            }
        };
        Ok(func)
    }

    pub(crate) fn gen_field_enum(&self) -> syn::Result<TokenStream> {
        // Create enum variants.
        let mut enum_variants = Vec::new();
        for field in &self.fields {
            if let Some(field) = field.enum_variant_definition() {
                enum_variants.push(field);
            }
        }
        let enum_variants = TokenStream::from_iter(enum_variants.into_iter());

        // Create final enum.
        let field_enum = quote! {
            #[derive(Debug, Copy, Clone, Eq, PartialEq)]
            pub enum Field {
                #enum_variants
            }
        };
        Ok(field_enum)
    }

    pub(crate) fn gen_ins_impl(&self) -> syn::Result<TokenStream> {
        // Map fields by name.
        let mut field_by_name = HashMap::<String, &Field>::new();
        for field in &self.fields {
            field_by_name.insert(field.name.clone(), field);
        }
        // Generate match arms for each opcode.
        let mut field_match_arms = Vec::new();
        let mut def_match_arms = Vec::new();
        let mut use_match_arms = Vec::new();
        for opcode in &self.opcodes {
            // Generate fields of opcode.
            // TODO Support mnemonics.
            let mut fields = Vec::new();
            for arg in &opcode.args {
                let field: &Field = field_by_name.get(arg).ok_or_else(|| {
                    syn::Error::new(Span::call_site(), format!("undefined field {}", arg))
                })?;
                let variant = field.construct_variant_self();
                fields.extend(quote! { #variant, })
            }
            let fields = TokenStream::from_iter(fields.into_iter());
            // Emit match arm.
            let ident = opcode.variant_identifier()?;
            field_match_arms.push(quote! {
                Opcode::#ident => vec![#fields],
            });

            let mut defs = Vec::new();
            for arg in &opcode.defs {
                let field: &Field = field_by_name.get(arg).ok_or_else(|| {
                    syn::Error::new(Span::call_site(), format!("undefined field {}", arg))
                })?;
                let variant = field.construct_variant_self();
                defs.extend(quote! { #variant, })
            }
            let defs = TokenStream::from_iter(defs.into_iter());
            let ident = opcode.variant_identifier()?;
            def_match_arms.push(quote! {
                Opcode::#ident => vec![#defs],
            });

            let mut uses = Vec::new();
            let mut special_uses = Vec::new();
            for arg in &opcode.uses {
                // Detect non-zero modifier.
                let mut arg = arg.as_str();
                let mut non_zero = false;
                if let Some(substr) = arg.strip_suffix(".nz") {
                    non_zero = true;
                    arg = substr;
                }
                // Get underlying field.
                let field: &Field = field_by_name.get(arg).ok_or_else(|| {
                    syn::Error::new(Span::call_site(), format!("undefined field {}", arg))
                })?;
                let variant = field.construct_variant_self();
                if non_zero {
                    let value = field.express_value_self();
                    special_uses.extend(quote! {
                        if (#value) != 0 {
                            uses.push(#variant);
                        }
                    })
                } else {
                    uses.extend(quote! {
                        #variant,
                    })
                }
            }
            let uses = TokenStream::from_iter(uses.into_iter());
            let ident = opcode.variant_identifier()?;
            let special_uses = TokenStream::from_iter(special_uses.into_iter());
            use_match_arms.push(quote! {
                Opcode::#ident => {
                    let mut uses = vec![#uses];
                    #special_uses
                    uses
                },
            });
        }
        let field_match_arms = TokenStream::from_iter(field_match_arms.into_iter());
        let def_match_arms = TokenStream::from_iter(def_match_arms.into_iter());
        let use_match_arms = TokenStream::from_iter(use_match_arms.into_iter());
        // Generate final fields function.
        let ins_impl = quote! {
            impl Ins {
                fn _fields(&self) -> Vec<Field> {
                    match self.op {
                        Opcode::Illegal => vec![],
                        #field_match_arms
                        _ => todo!()
                    }
                }

                fn _defs(&self) -> Vec<Field> {
                    match self.op {
                        Opcode::Illegal => vec![],
                        #def_match_arms
                        _ => todo!()
                    }
                }

                fn _uses(&self) -> Vec<Field> {
                    match self.op {
                        Opcode::Illegal => vec![],
                        #use_match_arms
                        _ => todo!()
                    }
                }
            }
        };
        Ok(ins_impl)
    }
}

/// Converts the given key into an identifier.
fn to_rust_ident(key: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(&key.replace(".", "_"), Span::call_site()))
}

/// Converts the given key into a struct variant key.
fn to_rust_variant(key: &str) -> syn::Result<TokenTree> {
    Ok(TokenTree::Ident(Ident::new(
        &to_rust_variant_str(key).map_err(|e| syn::Error::new(Span::call_site(), e))?,
        Span::call_site(),
    )))
}

fn to_rust_variant_str(key: &str) -> Result<String, String> {
    let mut s = String::new();
    let mut chars = key.chars();
    loop {
        // Make first char uppercase.
        let c = match chars.next() {
            None => return Ok(s),
            Some(c) => c,
        };
        s.push(match c {
            'a'..='z' => c.to_ascii_uppercase(),
            'A'..='Z' => c,
            _ => return Err(format!("invalid identifier: {}", key)),
        });
        loop {
            let c = match chars.next() {
                None => return Ok(s),
                Some(c) => c,
            };
            match c.to_ascii_lowercase() {
                '0'..='9' | 'a'..='z' => s.push(c),
                '_' => break,
                '.' => {
                    s.push('_');
                    break;
                }
                _ => return Err(format!("invalid character in opcode name: {}", key)),
            }
        }
    }
}
