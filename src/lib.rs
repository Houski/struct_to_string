/// This is the `struct_to_string` crate.
///
/// This crate provides a procedural macro to convert struct definitions
/// into a string representation. An example use case would be for
/// API documentation where you want to present the Rust structs for the
/// API response on a webpage.
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, Type};

/// The `StructToString` macro derives a `to_string` function for the struct.
///
/// # Example
/// ```
/// use struct_to_string::StructToString;
///
/// #[derive(StructToString)]
/// struct MyStruct {
///     field1: i32,
///     field2: String,
/// }
///
/// let my_struct_as_rust_string = MyStruct::to_rust_string();
///
/// // Struct to string can also be used to convert structs to other programming languages,
/// // including Python, TypeScript, Go, Java, and C#.
///
/// let my_struct_as_c_sharp_string = MyStruct::to_csharp_string();
/// ```
#[proc_macro_derive(StructToString)]
pub fn struct_to_string(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let name = ast.ident;

    let mut rust_fields = String::new();
    let mut go_fields = String::new();
    let mut python_fields = String::new();
    let mut ts_fields = String::new();
    let mut java_fields = String::new();
    let mut csharp_fields = String::new();

    if let syn::Data::Struct(data_struct) = ast.data {
        for field in data_struct.fields {
            let field_name = field.ident.expect("Field name not found");
            let field_type = field.ty;
            let field_type_tokens = quote! { #field_type }.to_string().replace(" ", "");

            let is_optional = match &field_type {
                Type::Path(type_path) => {
                    let last_segment = &type_path.path.segments.last().unwrap().ident;
                    last_segment == "Option"
                }
                _ => false,
            };

            // Rust representation
            rust_fields.push_str(&format!("    {}: {},\n", field_name, field_type_tokens));

            // Go representation
            let ts_field_name = if is_optional {
                format!("{}?", field_name)
            } else {
                format!("{}", field_name)
            };

            // TypeScript representation
            ts_fields.push_str(&format!(
                "    {}: {};\n",
                ts_field_name,
                rust_type_to_ts_type(&field_type)
            ));

            // Python representation
            python_fields.push_str(&format!(
                "    {}: {}\n",
                field_name,
                rust_type_to_python_type(&field_type)
            ));

            // Go representation
            go_fields.push_str(&format!(
                "    {} {}\n",
                field_name,
                rust_type_to_go_type(&field_type)
            ));

            java_fields.push_str(&format!(
                "    {} {} {};\n",
                "public",
                rust_type_to_java_type(&field_type),
                field_name,
            ));

            csharp_fields.push_str(&format!(
                "    {} {} {};\n",
                "public",
                rust_type_to_csharp_type(&field_type),
                field_name,
            ));
        }
    }

    let gen = quote! {
        impl #name {
            pub fn to_rust_string() -> String {
                let mut res = String::from("struct ");
                res.push_str(stringify!(#name));
                res.push_str(" {\n");
                res.push_str(#rust_fields.trim_end_matches(",\n"));
                res.push_str("\n}");
                res
            }

            pub fn to_go_string() -> String {
                let mut res = String::from("type ");
                res.push_str(stringify!(#name));
                res.push_str(" struct {\n");
                res.push_str(#go_fields);
                res.push_str("}");
                res
            }

            pub fn to_python_string() -> String {
                let mut res = String::from("@dataclass_json\n@dataclass\nclass ");
                res.push_str(stringify!(#name));
                res.push_str(":\n");
                res.push_str(#python_fields);
                res
            }

            pub fn to_typescript_string() -> String {
                let mut res = String::from("interface ");
                res.push_str(stringify!(#name));
                res.push_str(" {\n");
                res.push_str(#ts_fields);
                res.push_str("}");
                res
            }

            pub fn to_java_string() -> String {
                let mut res = String::from("public class ");
                res.push_str(stringify!(#name));
                res.push_str(" {\n");
                res.push_str(#java_fields);
                res.push_str("}");
                res
            }

            pub fn to_csharp_string() -> String {
                let mut res = String::from("public class ");
                res.push_str(stringify!(#name));
                res.push_str(" {\n");
                res.push_str(#csharp_fields);
                res.push_str("}");
                res
            }
        }
    };

    gen.into()
}

fn rust_type_to_ts_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let last_segment = type_path.path.segments.last().unwrap().ident.to_string();
            match last_segment.as_str() {
                "i32" | "u32" | "i64" | "u64" => "number",
                "f32" | "f64" => "number",
                "bool" => "boolean",
                "String" => "string",
                "char" => "string",
                "Option" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!("{} | null", rust_type_to_ts_type(inner_type));
                        }
                    }
                    "any"
                }
                "Vec" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!("{}[]", rust_type_to_ts_type(inner_type));
                        }
                    }
                    "any[]"
                }
                _ => &last_segment, // Fallback to the actual Rust type name, assuming it's a custom type or enum.
            }
            .to_string()
        }
        Type::Array(array) => {
            let inner_type = rust_type_to_ts_type(&array.elem);
            format!("{}[]", inner_type)
        }
        Type::Tuple(tuple) => {
            let types: Vec<String> = tuple
                .elems
                .iter()
                .map(|elem| rust_type_to_ts_type(elem))
                .collect();
            format!("[{}]", types.join(", "))
        }
        _ => "any".to_string(), // Fallback to 'any' for unsupported or complex types.
    }
}

fn rust_type_to_python_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let last_segment = type_path.path.segments.last().unwrap().ident.to_string();
            match last_segment.as_str() {
                "i32" | "u32" | "i64" | "u64" => "int",
                "f32" | "f64" => "float",
                "bool" => "bool",
                "String" => "str",
                "char" => "str",
                "Option" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!("Optional[{}]", rust_type_to_python_type(inner_type));
                        }
                    }
                    "any"
                }
                "Vec" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!("List[{}]", rust_type_to_python_type(inner_type));
                        }
                    }
                    "any[]"
                }
                _ => &last_segment, // Fallback to the actual Rust type name, assuming it's a custom type or enum.
            }
            .to_string()
        }
        Type::Array(array) => {
            let inner_type = rust_type_to_python_type(&array.elem);
            format!("List[{}]", inner_type)
        }
        Type::Tuple(tuple) => {
            let types: Vec<String> = tuple
                .elems
                .iter()
                .map(|elem| rust_type_to_python_type(elem))
                .collect();
            format!("Tuple[{}]", types.join(", "))
        }
        _ => "any".to_string(), // Fallback to 'any' for unsupported or complex types.
    }
}

fn rust_type_to_go_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let last_segment = type_path.path.segments.last().unwrap().ident.to_string();
            match last_segment.as_str() {
                "i8" => "int8",
                "u8" => "uint8",
                "i16" => "int16",
                "u16" => "uint16",
                "i32" => "int32",
                "u32" => "uint32",
                "i64" => "int64",
                "u64" => "uint64",
                "i128" => "big.Int",
                "u128" => "big.Int",
                "f32" => "float32",
                "f64" => "float64",
                "bool" => "bool",
                "String" => "string",
                "char" => "rune",
                "&str" => "string",
                "Option" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!("*{}", rust_type_to_go_type(inner_type));
                        }
                    }
                    "any"
                }
                "Vec" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!("[]{}", rust_type_to_go_type(inner_type));
                        }
                    }
                    "any[]"
                }
                _ => &last_segment, // Fallback to the actual Rust type name, assuming it's a custom type or enum.
            }
            .to_string()
        }
        Type::Array(array) => {
            let inner_type = rust_type_to_go_type(&array.elem);
            let array_length = match &array.len {
                syn::Expr::Lit(expr_lit) => {
                    if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                        lit_int.base10_parse::<usize>().unwrap()
                    } else {
                        let lit_token_stream = expr_lit.lit.to_token_stream();
                        panic!(
                            "Invalid array length expression: {}",
                            quote!(#lit_token_stream)
                        );
                    }
                }
                _ => panic!("Invalid array length expression:"),
            };
            format!("[{}]{}", array_length, inner_type)
        }
        Type::Tuple(tuple) => {
            let types: Vec<String> = tuple
                .elems
                .iter()
                .map(|elem| rust_type_to_go_type(elem))
                .collect();
            format!(
                "struct{{}} // CANNOT CONVERT THIS TO THE GO PROGRAMMING LANGUAGE. TUPLES ARE UNSUPPORTED BY GO: ({})",
                types.join(", ")
            )
        }
        _ => "any".to_string(), // Fallback to 'any' for unsupported or complex types.
    }
}

fn rust_type_to_java_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let last_segment = type_path.path.segments.last().unwrap().ident.to_string();
            match last_segment.as_str() {
                "i8" => "byte",
                "u8" => "short",
                "i16" => "short",
                "u16" => "int",
                "i32" => "int",
                "u32" => "long",
                "i64" => "long",
                "u64" => "BigInteger",
                "i128" => "BigInteger",
                "u128" => "BigInteger",
                "f32" => "float",
                "f64" => "double",
                "bool" => "boolean",
                "String" => "String",
                "char" => "char",
                "Option" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!(
                                "{}",
                                convert_java_primitive_type_to_wrapper_class(
                                    rust_type_to_java_type(inner_type).as_str()
                                )
                            );
                        }
                    }
                    "Object"
                }
                "Vec" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!(
                                "List<{}>",
                                convert_java_primitive_type_to_wrapper_class(
                                    rust_type_to_java_type(inner_type).as_str()
                                )
                            );
                        }
                    }
                    "List<Object>"
                }
                _ => &last_segment, // Fallback to the actual Rust type name, assuming it's a custom type or enum.
            }
            .to_string()
        }
        Type::Array(array) => {
            let inner_type = rust_type_to_java_type(&array.elem);
            format!("{}[]", &inner_type)
        }
        Type::Tuple(tuple) => {
            let types: Vec<String> = tuple
                .elems
                .iter()
                .map(|elem| {
                    convert_java_primitive_type_to_wrapper_class(
                        rust_type_to_java_type(elem).as_str(),
                    )
                })
                .collect();
            format!("Tuple<{}>", types.join(", "))
        }
        _ => "Object".to_string(), // Fallback to 'Object' for unsupported or complex types.
    }
}

fn convert_java_primitive_type_to_wrapper_class(inner_type: &str) -> String {
    match inner_type {
        "byte" => "Byte",
        "short" => "Short",
        "int" => "Integer",
        "long" => "Long",
        "float" => "Float",
        "double" => "Double",
        "char" => "Character",
        "boolean" => "Boolean",
        _ => inner_type,
    }
    .to_string()
}

fn rust_type_to_csharp_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let last_segment = type_path.path.segments.last().unwrap().ident.to_string();
            match last_segment.as_str() {
                "i8" => "sbyte",
                "u8" => "byte",
                "i16" => "short",
                "u16" => "ushort",
                "i32" => "int",
                "u32" => "uint",
                "i64" => "long",
                "u64" => "ulong",
                "i128" => "BigInteger",
                "u128" => "BigInteger",
                "f32" => "float",
                "f64" => "double",
                "bool" => "bool",
                "String" => "string",
                "char" => "char",
                "Option" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!("{}?", rust_type_to_csharp_type(inner_type));
                        }
                    }
                    "Object"
                }
                "Vec" => {
                    let arguments = &type_path.path.segments.last().unwrap().arguments;
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_bracketed_args.args.first()
                        {
                            return format!("List<{}>", rust_type_to_csharp_type(inner_type));
                        }
                    }
                    "List<Object>"
                }
                _ => &last_segment, // Fallback to the actual Rust type name, assuming it's a custom type or enum.
            }
            .to_string()
        }
        Type::Array(array) => {
            let inner_type = rust_type_to_csharp_type(&array.elem);
            format!("{}[]", &inner_type)
        }
        Type::Tuple(tuple) => {
            let types: Vec<String> = tuple
                .elems
                .iter()
                .map(|elem| rust_type_to_csharp_type(elem))
                .collect();
            format!("({})", types.join(", "))
        }
        _ => "Object".to_string(), // Fallback to 'Object' for unsupported or complex types.
    }
}
