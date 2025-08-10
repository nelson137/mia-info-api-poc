use case::CaseExt;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

pub struct SubstateMacro {
    attr: SubstateMacroAttr,
    item: SubstateMacroItem,
    source_state_field: syn::Ident,
}

impl SubstateMacro {
    pub fn from_macro_attribute(attr: TokenStream2, item: TokenStream2) -> Result<Self> {
        let attr: SubstateMacroAttr = syn::parse2(attr)?;
        let item: SubstateMacroItem = syn::parse2(item)?;

        let source_state_field = match &attr.field {
            Some(field) => field.clone(),
            None => {
                let field_str = item.0.ident.to_string().to_snake();
                syn::Ident::new(&field_str, Span::call_site())
            }
        };

        Ok(Self {
            attr,
            item,
            source_state_field,
        })
    }
}

impl ToTokens for SubstateMacro {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let input_item = &self.item.0;
        let source_state = &self.attr.source_state_path;
        let ident = &input_item.ident;
        let field = &self.source_state_field;

        let from_ref_path = quote::quote! { ::axum::extract::FromRef };
        let arc_path = quote::quote! { ::std::sync::Arc };

        quote::quote! {
            #input_item

            impl #from_ref_path<#source_state> for #arc_path<dyn #ident> {
                fn from_ref(state: &#source_state) -> Self {
                    #arc_path::clone(&state.#field)
                }
            }
        }
        .to_tokens(tokens);
    }
}

pub struct SubstateMacroAttr {
    pub source_state_path: syn::Path,
    pub field: Option<syn::Ident>,
}

impl Parse for SubstateMacroAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let source_state_path: syn::Path = input.parse()?;

        let field = input
            .peek(syn::Token![,])
            .then(|| {
                let _comma: syn::Token![,] = input.parse()?;

                let field_param_marker: syn::Ident = input
                    .parse()
                    .map_err(|_| missing_field_error(input.span()))?;
                if field_param_marker != syn::Ident::new("field", field_param_marker.span()) {
                    return Err(missing_field_error(field_param_marker.span()));
                }

                let field;
                let _brace = syn::parenthesized!(field in input);

                field.parse()
            })
            .transpose()?;

        Ok(Self {
            source_state_path,
            field,
        })
    }
}

fn missing_field_error(span: Span) -> syn::Error {
    syn::Error::new(span, "expected `field(<ident>)`")
}

pub struct SubstateMacroItem(syn::ItemTrait);

impl Parse for SubstateMacroItem {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse().map(Self)
    }
}
