use proc_macro2::TokenStream;
use quote::quote;

pub enum AttrFieldMode {
    StructInitializer,
    SetOnStruct(TokenStream),
}
/// from `#[inspectable(min = 8, max = 32, default = 16)]`,
/// generate either
/// ```rust,ignore
/// min: 8,
/// max: 32,
/// default: 16,
/// ```
/// or
/// ```rust,ignore
/// #struct.min = 8;
/// #struct.max = 32;
/// #struct.default = 16;
/// ```
pub fn struct_fields_from_attrs(attrs: &[syn::Attribute], mode: AttrFieldMode) -> TokenStream {
    let values = attrs
        .iter()
        .map(|attr| attr.parse_meta().expect("cannot parse attribute meta"))
        .filter(|meta| {
            meta.path()
                .get_ident()
                .map_or(false, |ident| ident == "inspectable")
        })
        .flat_map(|meta| match meta {
            syn::Meta::Path(_) => panic!("unexpected empty #[inspectable] attribute"),
            syn::Meta::NameValue(_) => panic!("unexpected name-value attribute"),
            syn::Meta::List(list) => list.nested,
        })
        .map(|nested_meta| match nested_meta {
            syn::NestedMeta::Lit(_) => panic!("unexpected literal in #[inspectable(..:)]"),
            syn::NestedMeta::Meta(meta) => meta,
        })
        .map(|meta| match meta {
            syn::Meta::Path(_) => panic!("unexpected empty #[inspectable] attribute"),
            syn::Meta::List(_) => panic!("unexpected attribute list"),
            syn::Meta::NameValue(name_value) => name_value,
        })
        .map(|name_value| match name_value.path.get_ident() {
            None => panic!("unexpected path: {:?}", name_value.path),
            Some(ident) => {
                let lit = &name_value.lit;
                match &mode {
                    AttrFieldMode::StructInitializer => quote! { #ident: #lit, },
                    AttrFieldMode::SetOnStruct(name) => quote! { #name.#ident = #lit; },
                }
            }
        });

    quote! {
        #(#values)*
    }
}
