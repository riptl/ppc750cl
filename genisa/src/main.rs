use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::Range;
use std::process::{Command, Stdio};

use itertools::Itertools;
use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use serde::{Deserialize, Deserializer};
use syn::LitInt;

macro_rules! token_stream {
    ($stream:ident) => {
        TokenStream::from_iter($stream.into_iter())
    };
}

fn main() {
    if let Err(err) = _main() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn _main() -> Result<()> {
    let isa = load_isa()?;

    let mut unformatted_code = Vec::<u8>::new();
    writeln!(&mut unformatted_code, "{}", quote! {
        use crate::prelude::*;
    })?;
    writeln!(&mut unformatted_code, "{}", isa.gen_opcode_enum()?)?;
    writeln!(&mut unformatted_code, "{}", isa.gen_field_enum()?)?;
    writeln!(&mut unformatted_code, "{}", isa.gen_ins_impl()?)?;

    let formatted_code = rustfmt(unformatted_code);
    File::create("./disasm/src/generated.rs")?
        .write_all(&formatted_code)?;

    Ok(())
}

fn rustfmt(code: Vec<u8>) -> Vec<u8> {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn rustfmt");

    let mut stdin = rustfmt.stdin.take().unwrap();
    std::thread::spawn(move || {
        let _ = stdin.write_all(&code);
    });

    let rustfmt_res = rustfmt.wait_with_output().expect("failed to run rustfmt");
    if !rustfmt_res.status.success() {
        panic!("rustfmt failed");
    }

    rustfmt_res.stdout
}

#[derive(Default)]
pub(crate) struct BitRange(Range<u8>);

impl<'de> Deserialize<'de> for BitRange {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
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
    fn variant_identifier(&self) -> Result<TokenTree> {
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

fn load_isa() -> Result<Isa> {
    let yaml_file = File::open("isa.yaml")?;
    let isa: Isa = serde_yaml::from_reader(yaml_file)?;
    Ok(isa)
}

impl Isa {
    fn gen_opcode_enum(&self) -> Result<TokenStream> {
        // Create enum variants.
        let enum_variants = self
            .opcodes
            .iter()
            .map(|opcode| -> Result<TokenStream> {
                let ident = opcode.variant_identifier()?;
                Ok(quote! {
                    #ident,
                })
            })
            .try_collect::<TokenStream, Vec<TokenStream>, Error>()?;
        let enum_variants = token_stream!(enum_variants);

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

    fn gen_mnemonic_fn(&self) -> Result<TokenStream> {
        // Create match arms.
        let match_arms = self
            .opcodes
            .iter()
            .map(|opcode| {
                let variant = opcode.variant_identifier()?;
                let literal =
                    Literal::string(opcode.name.strip_suffix('.').unwrap_or(&opcode.name));
                Ok(quote! {
                    Opcode::#variant => #literal,
                })
            })
            .try_collect::<TokenStream, Vec<TokenStream>, Error>()?;
        let match_arms = token_stream!(match_arms);
        // Create final function.
        let mnemonic_fn = quote! {
            pub(crate) fn _mnemonic(self) -> &'static str {
                match self {
                    Opcode::Illegal => "<illegal>",
                    #match_arms
                }
            }
        };
        Ok(mnemonic_fn)
    }

    pub(crate) fn gen_opcode_detect(&self) -> Result<TokenStream> {
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
            .try_collect::<TokenStream, Vec<TokenStream>, Error>()?;
        let if_chain = token_stream!(if_chain);
        // Generate function.
        let func = quote! {
            pub(crate) fn _detect(code: u32) -> Self {
                #if_chain
                Opcode::Illegal
            }
        };
        Ok(func)
    }

    pub(crate) fn gen_field_enum(&self) -> Result<TokenStream> {
        // Create enum variants.
        let mut enum_variants = Vec::new();
        for field in &self.fields {
            if let Some(field) = field.enum_variant_definition() {
                enum_variants.push(field);
            }
        }
        let enum_variants = token_stream!(enum_variants);

        // Create final enum.
        let field_enum = quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Copy, Clone, Eq, PartialEq)]
            pub enum Field {
                #enum_variants
            }
        };
        Ok(field_enum)
    }


    pub(crate) fn gen_ins_impl(&self) -> Result<TokenStream> {
        // Map fields by name.
        let mut field_by_name = HashMap::<String, &Field>::new();
        for field in &self.fields {
            field_by_name.insert(field.name.clone(), field);
        }
        // Generate match arms for each opcode.
        let mut field_match_arms = Vec::new();
        let mut def_match_arms = Vec::new();
        let mut use_match_arms = Vec::new();
        let mut modifier_match_arms = Vec::new();
        for opcode in &self.opcodes {
            // Generate fields of opcode.
            // TODO Support mnemonics.
            let mut fields = Vec::new();
            for arg in &opcode.args {
                let field: &Field = field_by_name.get(arg).ok_or_else(|| {
                    Error::from(format!("undefined field {}", arg))
                })?;
                let variant = field.construct_variant_self();
                fields.extend(quote! { #variant, })
            }
            let fields = token_stream!(fields);
            // Emit match arm.
            let ident = opcode.variant_identifier()?;
            field_match_arms.push(quote! {
                Opcode::#ident => vec![#fields],
            });

            // Generate modifiers.
            let mut set_modifiers: Vec<TokenTree> = Vec::new();
            for modifier in &opcode.modifiers {
                set_modifiers.extend(match modifier.as_str() {
                    "OE" => quote! { m.oe = self.bit(21); },
                    "Rc" => quote! { m.rc = self.bit(31); },
                    "AA" => quote! { m.aa = self.bit(30); },
                    "LK" => quote! { m.lk = self.bit(31); },
                    _ => {
                        return Err(format!("unsupported modifier {}", modifier).into());
                    }
                })
            }
            for modifier in &opcode.side_effects {
                set_modifiers.extend(match modifier.as_str() {
                    "OE" => quote! { m.oe = true; },
                    "Rc" => quote! { m.rc = true; },
                    "AA" => quote! { m.aa = true; },
                    "LK" => quote! { m.lk = true; },
                    _ => {
                        return Err(format!("unsupported modifier {}", modifier).into());
                    }
                })
            }
            let set_modifiers = token_stream!(set_modifiers);
            modifier_match_arms.push(quote! {
                Opcode::#ident => {
                    let mut m = Modifiers::default();
                    #set_modifiers
                    m
                }
            });

            // Generate defs.
            let mut defs = Vec::new();
            for arg in &opcode.defs {
                let field: &Field = field_by_name.get(arg).ok_or_else(|| {
                    syn::Error::new(Span::call_site(), format!("undefined field {}", arg))
                })?;
                let variant = field.construct_variant_self();
                defs.extend(quote! { #variant, })
            }
            let defs = token_stream!(defs);
            let ident = opcode.variant_identifier()?;
            def_match_arms.push(quote! {
                Opcode::#ident => vec![#defs],
            });

            // Generate uses.
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
            let uses = token_stream!(uses);
            let ident = opcode.variant_identifier()?;
            let special_uses = token_stream!(special_uses);
            use_match_arms.push(quote! {
                Opcode::#ident => {
                    let mut uses = vec![#uses];
                    #special_uses
                    uses
                },
            });
        }
        let field_match_arms = token_stream!(field_match_arms);
        let def_match_arms = token_stream!(def_match_arms);
        let use_match_arms = token_stream!(use_match_arms);
        let modifier_match_arms = token_stream!(modifier_match_arms);
        // Generate final fields function.
        let ins_impl = quote! {
            impl Ins {
                pub(crate) fn _fields(&self) -> Vec<Field> {
                    match self.op {
                        Opcode::Illegal => vec![],
                        #field_match_arms
                    }
                }

                #[allow(unused_mut)]
                pub(crate) fn _defs(&self) -> Vec<Field> {
                    match self.op {
                        Opcode::Illegal => vec![],
                        #def_match_arms
                    }
                }

                #[allow(unused_mut)]
                pub(crate) fn _uses(&self) -> Vec<Field> {
                    match self.op {
                        Opcode::Illegal => vec![],
                        #use_match_arms
                    }
                }

                #[allow(unused_mut)]
                pub(crate) fn _modifiers(&self) -> Modifiers {
                    match self.op {
                        Opcode::Illegal => Modifiers::default(),
                        #modifier_match_arms
                    }
                }

                pub(crate) fn _simplified(self) -> SimplifiedIns {
                    SimplifiedIns {
                        mnemonic: self.op.mnemonic(),
                        modifiers: self._modifiers(),
                        args: vec![],
                        ins: self,
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

/// Converts the given key into an enum variant key.
fn to_rust_variant(key: &str) -> Result<TokenTree> {
    Ok(TokenTree::Ident(Ident::new(
        &to_rust_variant_str(key)?,
        Span::call_site(),
    )))
}

fn to_rust_variant_str(key: &str) -> Result<String> {
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
            _ => return Err(format!("invalid identifier: {}", key).into()),
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
                _ => return Err(format!("invalid character in opcode name: {}", key).into()),
            }
        }
    }
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
