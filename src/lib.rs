/// This is the `struct_to_string` crate.
///
/// This crate provides a procedural macro to convert struct definitions
/// into a string representation. An example use case would be for
/// API documentation where you want to present the Rust structs for the
/// API response on a webpage.
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// The `StructToString` macro derives a `to_string` function for the struct.
///
/// # Example
/// ```
/// #[derive(StructToString)]
/// struct MyStruct {
///     field1: i32,
///     field2: String,
/// }
///
/// let my_struct_as_string = MyStruct::to_string();
/// ```
#[proc_macro_derive(StructToString)]
pub fn struct_to_string(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);

    let name = ast.ident;
    let mut fields = String::new();

    if let syn::Data::Struct(data_struct) = ast.data {
        for field in data_struct.fields {
            let field_name = field.ident.expect("Field name not found");
            let field_type = field.ty;
            let field_type_tokens = quote! { #field_type }.to_string().replace(" ", "");
            fields.push_str(&format!("    {}: {},\n", field_name, field_type_tokens));
        }
    }

    fields = fields.trim_end_matches(",\n").to_string();

    let gen = quote! {
        impl #name {
            pub fn to_string() -> String {
                let mut res = String::from("struct ");
                res.push_str(stringify!(#name));
                res.push_str(" {\n");
                res.push_str(#fields.trim_end_matches(",\n"));
                res.push_str("\n}");
                res
            }
        }
    };

    gen.into()
}
