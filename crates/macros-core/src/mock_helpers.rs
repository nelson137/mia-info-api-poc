use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::Result;

pub struct MockHelpersMacro {
    item: syn::ItemTrait,
    mock_ident: syn::Ident,
}

impl MockHelpersMacro {
    pub fn from_macro_attribute(_attr: TokenStream2, item: TokenStream2) -> Result<Self> {
        let item: syn::ItemTrait = syn::parse2(item)?;

        let mock_ident_str = format!("Mock{}", item.ident);
        let mock_ident = syn::Ident::new(&mock_ident_str, item.ident.span());

        Ok(Self { item, mock_ident })
    }
}

impl ToTokens for MockHelpersMacro {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let input_item = &self.item;
        let ident = &input_item.ident;
        let mock_ident = &self.mock_ident;

        let from_path = quote::quote!(::std::convert::From);
        let arc_path = quote::quote!(::std::sync::Arc);
        let state_path = quote::quote!(::axum::extract::State);

        quote::quote! {
            #input_item

            impl #from_path<#mock_ident> for #state_path<#arc_path<dyn #ident>> {
                fn from(value: #mock_ident) -> Self {
                    #state_path(#arc_path::new(value))
                }
            }
        }
        .to_tokens(tokens);
    }
}
