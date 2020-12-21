use proc_macro2::TokenStream;

mod as_html_enum;
mod as_html_struct;

pub struct DeriveDataEnum<'a> {
    ident: syn::Ident,
    variants: Vec<&'a syn::Variant>,
}

pub struct DeriveDataStruct<'a> {
    ident: syn::Ident,
    fields: Vec<&'a syn::Field>,
}

pub fn expand(input: syn::DeriveInput) -> TokenStream {
    match input.data {
        syn::Data::Enum(data) => {
            let variants = data
                .variants
                .iter()
                .inspect(|variant| {
                    assert!(variant.fields.is_empty(), "only unit enums are supported");
                })
                .collect();

            let data = DeriveDataEnum {
                ident: input.ident,
                variants,
            };
            as_html_enum::to_tokens(data)
        }
        syn::Data::Struct(data) => as_html_struct::to_tokens(DeriveDataStruct {
            ident: input.ident,
            fields: data.fields.iter().collect(),
        }),
        syn::Data::Union(_) => panic!("unions are not supported"),
    }
}
