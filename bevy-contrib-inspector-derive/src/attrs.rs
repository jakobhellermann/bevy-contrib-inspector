fn parse_inspectable_attributes(
    input: syn::parse::ParseStream,
) -> syn::Result<impl Iterator<Item = (Box<syn::Expr>, Box<syn::Expr>)>> {
    use syn::parse::Parse;
    let x: syn::punctuated::Punctuated<_, syn::Token![,]> =
        input.parse_terminated(syn::ExprAssign::parse)?;

    Ok(x.into_iter()
        .map(|expr_assign| (expr_assign.left, expr_assign.right)))
}

/// extracts [(min, 8), (field, vec2(1.0, 1.0))] from `#[inspectable(min = 8, field = vec2(1.0, 1.0))]`,
pub fn inspectable_attributes(
    attrs: &[syn::Attribute],
) -> impl Iterator<Item = (Box<syn::Expr>, Box<syn::Expr>)> + '_ {
    attrs
        .iter()
        .filter(|attr| attr.path.get_ident().map_or(false, |p| p == "inspectable"))
        .flat_map(|attr| attr.parse_args_with(parse_inspectable_attributes).unwrap())
}
