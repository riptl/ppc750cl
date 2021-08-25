#![feature(proc_macro_span, proc_macro_def_site)]

mod isa;
//mod writer;

use std::fs::File;

use proc_macro::Span;

use crate::isa::Isa;

#[proc_macro]
pub fn opcodes(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let isa = match load_isa() {
        Ok(v) => v,
        Err(err) => return err,
    };
    match isa.gen_opcode_enum() {
        Ok(v) => v.into(),
        Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
    }
}

#[proc_macro]
pub fn fields(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let isa = match load_isa() {
        Ok(v) => v,
        Err(err) => return err,
    };
    match isa.gen_field_enum() {
        Ok(v) => v.into(),
        Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
    }
}

#[proc_macro]
pub fn ins_impl(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let isa = match load_isa() {
        Ok(v) => v,
        Err(err) => return err,
    };
    match isa.gen_ins_impl() {
        Ok(v) => v.into(),
        Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
    }
}

fn load_isa() -> Result<Isa, proc_macro::TokenStream> {
    _load_isa().map_err(|err| {
        proc_macro::TokenStream::from(
            syn::Error::new(Span::def_site().into(), err).to_compile_error(),
        )
    })
}

fn _load_isa() -> Result<Isa, Box<dyn std::error::Error>> {
    // Figure out YAML path.
    let def_site = Span::def_site();
    let rust_path = def_site.source_file().path();
    let yaml_path = rust_path.parent().unwrap().join("isa.yaml");
    // Open and deserialize YAML file.
    let yaml_file = File::open(yaml_path).map_err(|e| syn::Error::new(def_site.into(), e))?;
    let isa: Isa =
        serde_yaml::from_reader(yaml_file).map_err(|e| syn::Error::new(def_site.into(), e))?;
    Ok(isa)
}
