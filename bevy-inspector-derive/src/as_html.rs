use proc_macro2::TokenStream;
use quote::quote;

pub struct DeriveData<'a> {
    ident: syn::Ident,
    variants: Vec<&'a syn::Variant>,
}

impl DeriveData<'_> {
    pub fn expand(input: syn::DeriveInput) -> TokenStream {
        let data = match input.data {
            syn::Data::Enum(data) => data,
            _ => panic!("only unit enum are supported"),
        };

        let variants = data
            .variants
            .iter()
            .inspect(|variant| {
                assert!(variant.fields.is_empty(), "only unit enums are supported");
            })
            .collect();

        let data = DeriveData {
            ident: input.ident,
            variants,
        };
        data.to_tokens()
    }

    fn to_tokens(&self) -> TokenStream {
        let DeriveData { ident, variants } = self;

        let html = html(variants.as_slice());

        let parse_arms = variants.iter().map(|variant| {
            let var_ident = &variant.ident;
            let var_ident_str = var_ident.to_string();

            quote! { #var_ident_str => Ok(#ident::#var_ident) }
        });

        quote! {
        impl bevy_inspector::as_html::AsHtml for #ident {
            type Err = String;
            type Options = ();
            const DEFAULT_OPTIONS: Self::Options = ();

            fn as_html(
                shared: bevy_inspector::as_html::SharedOptions<Self>,
                (): Self::Options,
                submit_fn: &'static str,
            ) -> String {
                #html
            }

            fn parse(value: &str) -> Result<Self, Self::Err> {
                match value {
                    #(#parse_arms,)*
                    _ => Err(value.to_string()),
                }
            }
        }
        }
    }
}

fn html(variants: &[&syn::Variant]) -> TokenStream {
    let var_ident_strs = variants.iter().map(|v| v.ident.to_string());

    quote! {
    let mut html = String::new();
    html.push_str(&format!(
        r#"
        <div class="row">
            <label class="cell text-right">{}:</label>
            <div class="cell">"#,
        shared.label,
    ));

    for field in &[#(#var_ident_strs),*] {
        html.push_str(&format!(
            r#"
            <label>
                <input type="radio" value="{value}" name="{name}" {checked} oninput="{}(this.value)"/>
                {value}
            </label>
            "#,
            submit_fn,
            value = field,
            name = shared.label,
            checked=if format!("{:?}", shared.default) == *field { "checked" } else {""}
        ));
    }

    html.push_str(r#"</div></div>"#);
    html
    }
}
