use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::as_html::DeriveDataStruct;

pub fn to_tokens(data: DeriveDataStruct<'_>) -> TokenStream {
    let DeriveDataStruct { ident, fields } = data;

    let html = html(&ident, fields.as_slice());

    let parse_arms = fields.iter().enumerate().map(|(i, field)| {
        let ty = &field.ty;
        let field_name = field_name(field, i);

        let accessor = field.ident.as_ref().map_or_else(
            || syn::Index::from(i).to_token_stream(),
            |name| quote!{#name}
        );

        quote! { #field_name => self.#accessor = <#ty as bevy_contrib_inspector::AsHtml>::parse(value).map_err(|e| format!("{:?}", e))? }
    });

    quote! {
    #[allow(warnings)]
    impl bevy_contrib_inspector::as_html::AsHtml for #ident {
        type Err = String;
        type Options = ();
        const DEFAULT_OPTIONS: Self::Options = ();

        fn as_html(
            shared: bevy_contrib_inspector::as_html::SharedOptions<Self>,
            (): Self::Options,
            submit_fn: String,
        ) -> String {
            #html
        }

        // TODO footer and header

        fn parse(_: &str) -> Result<Self, Self::Err> {
            unreachable!("AsHtml::update will be used instead")
        }

        fn update(&mut self, value: &str) -> Result<(), Self::Err> {
            let mut iter = value.splitn(2, ':');
            let (field, value) = iter.next().zip(iter.next())
                .ok_or_else(|| format!("expected '<name>:<value>', got '{}'", value))?;

            match field {
                #(#parse_arms,)*
                other => return Err(format!("unexpected field '{}'", other))
            }

            Ok(())
        }
    }
    }
}

fn field_name(field: &syn::Field, i: usize) -> String {
    field
        .ident
        .as_ref()
        .map_or_else(|| format!("#{}", i), |name| name.to_string())
}

fn html(struct_name: &syn::Ident, fields: &[&syn::Field]) -> TokenStream {
    let fields = fields.into_iter().enumerate().map(|(i, field)| {
        let ty = &field.ty;
        let field_name = field_name(field, i);

        let as_html = quote! { <#ty as bevy_contrib_inspector::as_html::AsHtml> };

        let submit_fn = quote! {
            format!("((value) => {submit_fn}('{field}:'+value))", submit_fn = submit_fn, field = #field_name)
        };

        quote! {
            let shared_options = bevy_contrib_inspector::as_html::SharedOptions {
                label: std::borrow::Cow::Borrowed(#field_name),
                default: Default::default(),
            };
            let octave_html = #as_html::as_html(
                shared_options,
                #as_html::DEFAULT_OPTIONS,
                #submit_fn,
            );
            html.push_str(&octave_html);
        }
    });

    let struct_name_str = struct_name.to_string();

    quote! {
        let mut html = format!("<b>{}</b>", #struct_name_str);
        #(#fields)*
        html.push_str("<br />");
        html
    }
}
