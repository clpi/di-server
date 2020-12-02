use proc_macro::TokenStream;

#[proc_macro_derive(TypeName)]
pub fn derive_args(input: TokenStream) -> TokenStream {
    TokenStream::new()
}

pub trait Arg  {}

fn impl_args(ast: &syn::DeriveInput) {
    let name = &ast.ident;
    quote! {
        impl Arg for #name {
        }
    };
}
