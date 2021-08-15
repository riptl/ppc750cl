mod isa;
mod writer;

#[proc_macro]
pub fn isa(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = match crate::isa::isa(input) {
        Ok(v) => v,
        Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
    };
    proc_macro::TokenStream::from(output)
}

#[proc_macro]
pub fn write_asm(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = match crate::writer::write_asm(input) {
        Ok(v) => v,
        Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
    };
    proc_macro::TokenStream::from(output)
}
