mod as_html;
mod attrs;
mod inspectable;

#[proc_macro_derive(Inspectable, attributes(inspectable))]
pub fn inspectable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    inspectable::DeriveData::expand(input).into()
}

#[proc_macro_derive(AsHtml)]
pub fn as_html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    as_html::expand(input).into()
}
