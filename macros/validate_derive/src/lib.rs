extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Validate)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_derive_validate(&ast)
}

fn impl_derive_validate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let validation_tokens = match &ast.data {
        syn::Data::Struct(_) => panic!("#[derive(Validate)] does not support structs"),
        syn::Data::Enum(_) => quote! {
            match self {
                #name::Unknown(string) => Ok(ValidationResult::Failed {
                    reasons: vec![FailureReason {
                        message: format!("#name unknown: {}", string),
                        context,
                    }],
                }),
                _ => Ok(ValidationResult::Passed),
            }
        },
        syn::Data::Union(_) => panic!("#[derive(Validate)] does not support Unions"),
    };

    let gen = quote! {
        impl Validate for #name {
            fn validate_with_context(&self, context: ValidationContext, ) -> Result<ValidationResult, ValidationError> {
                #validation_tokens
            }
        }
    };
    gen.into()
}
/*
fn validate_struct(data: &syn::DataStruct) -> TokenStream {
    if data.fields.iter().any(|field| field.ident.is_none()) {
        validate_struct_tuple(data)
    } else {
        validate_struct_named(data)
    }
}

fn validate_struct_named(data: &syn::DataStruct) -> TokenStream {
    todo!()
}

fn validate_struct_tuple(data: &syn::DataStruct) -> TokenStream {
    todo!()
}

fn validate_enum(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        match self {
            #name::Unknown(string) => Ok(ValidationResult::Failed {
                reasons: vec![FailureReason {
                    message: format!("#name unknown: {}", string),
                    context,
                }],
            }),
            _ => Ok(ValidationResult::Passed),
        }
    };
    gen.into()
}
*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
