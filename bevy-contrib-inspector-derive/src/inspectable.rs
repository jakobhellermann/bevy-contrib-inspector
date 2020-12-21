extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;

struct Field<'a> {
    ident: &'a syn::Ident,
    ty: &'a syn::Type,
    attrs: &'a Vec<syn::Attribute>,
}

pub struct DeriveData<'a> {
    ident: syn::Ident,
    fields: Vec<Field<'a>>,
    attrs: &'a Vec<syn::Attribute>,
}

impl<'a> DeriveData<'a> {
    pub fn expand(input: syn::DeriveInput) -> TokenStream {
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

        let data = DeriveData {
            ident: input.ident,
            fields,
            attrs: &input.attrs,
        };
        data.to_tokens()
    }

    pub fn to_tokens(&self) -> TokenStream {
        let DeriveData {
            ident,
            fields,
            attrs,
        } = self;

        let inspectable_fields = crate::attrs::inspectable_attributes(&attrs)
            .map(|(left, right)| quote! { #left: #right, });
        let inspectable_options = quote! {
            bevy_contrib_inspector::InspectableOptions {
                #(#inspectable_fields)*
                ..Default::default()
            }
        };

        let match_arms = fields.iter().map(|field| {
            let ident = field.ident;
            let ident_str = ident.to_string();
            let ty = &field.ty;

            quote! {
                #ident_str => if let Err(e) = <#ty as bevy_contrib_inspector::as_html::AsHtml>::update(&mut self.#ident, &value) {
                    eprintln!("failed to parse '{}': {:?}", #ident_str, e);
                }
            }
        });

        let html = html(&fields);

        quote! {
            impl bevy_contrib_inspector::Inspectable for #ident {
                fn update(&mut self, field: &str, value: &str) {
                    match field {
                        #(#match_arms,)*
                        _ => eprintln!("unexpected field '{}'", field),
                    }
                }

                fn html() -> String {
                    #html
                }

                fn options() -> bevy_contrib_inspector::InspectableOptions {
                    #inspectable_options
                }
            }
        }
    }
}

fn html<'a>(fields: &[Field<'a>]) -> TokenStream {
    let fields_as_html = fields.iter().map(|field| {
        let ty = &field.ty;
        let attrs = &field.attrs;
        let ident = &field.ident;
        let ident_str = ident.to_string();

        let as_html = quote! { <#ty as bevy_contrib_inspector::as_html::AsHtml> };
        let option_fields = crate::attrs::inspectable_attributes(&attrs)
            .map(|(left, right)| quote! { options.#left = #right; });

        quote! {
            let shared = bevy_contrib_inspector::as_html::SharedOptions {
                label: std::borrow::Cow::Borrowed(#ident_str),
                default: defaults.#ident,
            };

            let mut options = #as_html::DEFAULT_OPTIONS;
            #(#option_fields)*

            let submit_fn = concat!("(value => handleChange('", #ident_str, "', value))").to_string();

            inputs.push_str(&#as_html::as_html(shared, options, submit_fn));
        }
    });

    let css = include_str!("../static/style.css");
    let js = include_str!("../static/script.js");

    let tys = fields.iter().map(|field| &field.ty);

    quote! {
        let mut header = String::new();
        let mut footer = String::new();
        let mut field_types = std::collections::HashSet::<std::any::TypeId>::new();

        #(<#tys as bevy_contrib_inspector::as_html::AsHtml>::register_header_footer(&mut field_types, &mut header, &mut footer);)*

        let mut inputs = String::new();
        let defaults = <Self as std::default::Default>::default();
        #(#fields_as_html)*


        format!(
r#"
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8" />
{header}
<style>{css}</style>
</head>
<body>
    <script>
    const handleChangeThrottle = {inspectable_throttle};
    {js}
    </script>

    <div id="inputs">
    {inputs}
    </div>

    {footer}
</body>
</html>"#,
            header=header,
            footer=footer,
            css=#css,
            js=#js,
            inputs=inputs,
            inspectable_throttle=10, // used in ../static/script.js
        )
    }
}
