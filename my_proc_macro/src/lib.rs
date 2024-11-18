extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Unpack)]
pub fn derive_unpack(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Ensure we are dealing with a struct
    let unpack_body = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            // Generate code for each field
            let unpack_statements = fields.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();

                // Check field type for special handling (String/Vec<u8>)
                if let syn::Type::Path(type_path) = &field.ty {
                    let type_name = type_path.path.segments.last().unwrap().ident.to_string();
                    if type_name == "String" {
                        // For String fields
                        quote! {
                            let #field_name = {
                                let len = read_u16_be(&mut cursor)? as usize;
                                let buf = read_bytes(&mut cursor, len)?;
                                read_utf8_string(buf)?
                            };
                        }
                    } else if type_name == "Vec" {
                        // For Vec<u8> fields
                        quote! {
                            let #field_name = {
                                let len = read_u16_be(&mut cursor)? as usize;
                                read_bytes(&mut cursor, len)?
                            };
                        }
                    } else {
                        // For other types, assuming they implement a simple read function
                        quote! {
                            let #field_name = read_u16_be(&mut cursor)?;
                        }
                    }
                } else {
                    // Fallback for unsupported types
                    quote! {
                        compile_error!("Unsupported field type in Unpack macro");
                    }
                }
            });

            // Collect all field initializations
            let field_initializers = fields.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                quote! { #field_name }
            });

            quote! {
                #(#unpack_statements)*

                Ok(#name {
                    #(#field_initializers),*
                })
            }
        } else {
            panic!("Unpack macro only supports structs with named fields.");
        }
    } else {
        panic!("Unpack macro only supports structs.");
    };

    let expanded = quote! {
        impl #name {
            pub fn unpack(data: &[u8]) -> Result<Self, String> {
                let mut cursor = Cursor::new(data);
                #unpack_body
            }
        }
    };

    TokenStream::from(expanded)
}
