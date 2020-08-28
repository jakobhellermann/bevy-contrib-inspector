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

    let options = Options::from_attrs(&input.attrs);

    let fields = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().expect("field should be named");
            Field {
                ident,
                ty: &field.ty,
            }
        })
        .collect();

    let derive_data = DeriveData {
        ident: input.ident,
        fields,
        options,
    };

    expand(derive_data).into()
}

#[derive(Default)]
struct Options {
    port: Option<u16>,
}
impl Options {
    fn from_attrs(attrs: &[syn::Attribute]) -> Self {
        let mut options = Options::default();

        for name_value in attrs
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
        {
            match name_value.path.get_ident() {
                None => panic!("unexpected path: {:?}", name_value.path),
                Some(ident) if ident == "port" => match name_value.lit {
                    syn::Lit::Int(port) => options.port = Some(port.base10_parse().unwrap()),
                    _ => panic!("expected port integer"),
                },
                Some(ident) => panic!("unexpected parameter: {}", ident),
            }
        }

        options
    }
}

struct Field<'a> {
    ident: &'a syn::Ident,
    ty: &'a syn::Type,
}

struct DeriveData<'a> {
    ident: syn::Ident,
    fields: Vec<Field<'a>>,
    options: Options,
}

fn expand(data: DeriveData) -> TokenStream {
    let DeriveData {
        ident,
        fields,
        options,
    } = data;

    let port = options
        .port
        .map(|port| quote! { port: #port, })
        .unwrap_or_default();
    let inspectable_options = quote! {
        bevy_inspector::InspectableOptions {
            #port
            ..Default::default()
        }
    };

    let match_arms = fields.iter().map(|field| {
        let ident = field.ident;
        let ident_str = ident.to_string();

        quote! {
            #ident_str => match value.parse() {
                Ok(val) => self.#ident = val,
                Err(e) => eprintln!("oops"),
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
        quote! {
            let mut options = <#ty as bevy_inspector::AsHtml>::DEFAULT_OPTIONS;
            // options.min = 5;
            inputs.push_str(&<#ty as bevy_inspector::AsHtml>::as_html(
                options,
                "(value => handleChange('slider', value))")
            );
        }
    });

    quote! {
        let mut inputs = String::new();
        #(#fields_as_html)*

        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head></head>
            <body>
                <script>
                    function handleChange(field, data) {{
                        let body = field + ':' + data;
                        return fetch("", {{ method: "PUT", body }});
                    }}
                </script>
                <div>
                {inputs}
                </div>
            </body>
            </html>"#,
            inputs=inputs
        )
    }
}
