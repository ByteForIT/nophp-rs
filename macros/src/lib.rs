use proc_macro::TokenStream;
use syn::DeriveInput;

fn impl_module_trait(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;
    let ident_str = ident.to_string();

    quote::quote! {
        impl Module for #ident {
            fn name(&self) -> &'static str {
                #ident_str
            }
        }
    }
    .into()
}

#[proc_macro_derive(Module)]
pub fn module_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    impl_module_trait(ast)
}
