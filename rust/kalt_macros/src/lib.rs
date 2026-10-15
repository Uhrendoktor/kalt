#[proc_macro_attribute]
pub fn parser(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemFn);
    let name = &input.sig.ident;
    let vis = &input.vis;
    let args = &input.sig.inputs;
    let ret_ty = match &input.sig.output {
        syn::ReturnType::Default => quote::quote! { () },
        syn::ReturnType::Type(_, ty) => quote::quote! { #ty },
    };
    let body = &input.block;

    quote::quote! {
        #vis fn #name<'this, 'data: 'this, I: sertyp::chumsky::LocatingSequenceLike<'this, 'data>>( #args )
        -> impl chumsky::Parser<'this, I, #ret_ty, crate::parser::ParserError<'data>> {
            use sertyp::FromString;
            use chumsky::Parser;
            #body
            .labelled(sertyp::Content::from_string(stringify!(#name)))
        }
    }
    .into()
}
