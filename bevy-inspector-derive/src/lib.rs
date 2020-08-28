extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_derive(Inspectable, attributes(inspectable))]
pub fn derive_answer_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let data = match input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("only structs are supported"),
    };

    let fields = match data.fields {
        syn::Fields::Named(fields) => fields.named,
        _ => panic!("only named fields are supported"),
    };

    let fields = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().expect("field should be named");
            Field {
                ident,
                ty: &field.ty,
                attrs: &field.attrs,
            }
        })
        .collect();

    let derive_data = DeriveData {
        ident: input.ident,
        fields,
        attrs: &input.attrs,
    };

    expand(derive_data).into()
}

enum AttrFieldMode {
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
fn struct_fields_from_attrs(attrs: &[syn::Attribute], mode: AttrFieldMode) -> TokenStream {
    let values = attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok())
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

struct Field<'a> {
    ident: &'a syn::Ident,
    ty: &'a syn::Type,
    attrs: &'a Vec<syn::Attribute>,
}

struct DeriveData<'a> {
    ident: syn::Ident,
    fields: Vec<Field<'a>>,
    attrs: &'a Vec<syn::Attribute>,
}

fn expand(data: DeriveData) -> TokenStream {
    let DeriveData {
        ident,
        fields,
        attrs,
    } = data;

    let inspectable_fields = struct_fields_from_attrs(&attrs, AttrFieldMode::StructInitializer);
    let inspectable_options = quote! {
        bevy_inspector::InspectableOptions {
            #inspectable_fields
            ..Default::default()
        }
    };

    let match_arms = fields.iter().map(|field| {
        let ident = field.ident;
        let ident_str = ident.to_string();

        quote! {
            #ident_str => match value.parse() {
                Ok(val) => self.#ident = val,
                Err(e) => eprintln!("failed to parse '{}': {}", #ident_str, e),
            }
        }
    });

    let html = html(&fields);

    quote! {
        impl bevy_inspector::Inspectable for #ident {
            fn update(&mut self, field: &str, value: String) {
                match field {
                    #(#match_arms)*,
                    _ => eprintln!("unexpected field '{}'", field),
                }
            }

            fn html() -> String {
                #html
            }

            fn options() -> bevy_inspector::InspectableOptions {
                #inspectable_options
            }
        }
    }
}

fn html<'a>(fields: &[Field<'a>]) -> TokenStream {
    let fields_as_html = fields.iter().map(|field| {
        let ty = &field.ty;
        let attrs = &field.attrs;
        let ident_str = field.ident.to_string();

        let as_html = quote! { <#ty as bevy_inspector::AsHtml> };
        let option_fields =
            struct_fields_from_attrs(&attrs, AttrFieldMode::SetOnStruct(quote! { options }));
        quote! {
            let mut options = #as_html::DEFAULT_OPTIONS;
            #option_fields
            let submit_fn = concat!("(value => handleChange('", #ident_str, "', value))");
            inputs.push_str(&#as_html::as_html(#ident_str, options, &submit_fn));
        }
    });

    let css = include_str!("../static/style.css");
    let js = include_str!("../static/script.js");

    quote! {
        let mut inputs = String::new();
        #(#fields_as_html)*

        format!(
r#"
<!DOCTYPE html>
<html>
<head>
<style>{css}</style>
</head>
<body>
    <script>
    {js}
    </script>
    <div id="inputs">
    {inputs}
    </div>
</body>
</html>"#,
            css=#css,
            js=#js,
            inputs=inputs
        )
    }
}
