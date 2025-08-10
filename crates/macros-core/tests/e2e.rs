#![allow(non_snake_case)]

use mia_info_poc_macros_core::SubstateMacro;
use quote::quote;

#[test]
fn explicit_subfield() -> syn::Result<()> {
    let attr = quote! { MyState, field(my_sub) };
    let item = quote! {
        trait MySubstate {}
    };

    let expected = quote! {
        #item
        impl ::axum::extract::FromRef<MyState> for ::std::sync::Arc<dyn MySubstate> {
            fn from_ref(state: &MyState) -> Self {
                ::std::sync::Arc::clone(&state.my_sub)
            }
        }
    };

    let m = SubstateMacro::from_macro_attribute(attr, item)?;
    let actual = quote!(#m);

    assert_eq!(actual.to_string(), expected.to_string());

    Ok(())
}

#[test]
fn auto_subfield() -> syn::Result<()> {
    let attr = quote! { MyState };
    let item = quote! {
        trait MySubstate {}
    };

    let expected = quote! {
        #item
        impl ::axum::extract::FromRef<MyState> for ::std::sync::Arc<dyn MySubstate> {
            fn from_ref(state: &MyState) -> Self {
                ::std::sync::Arc::clone(&state.my_substate)
            }
        }
    };

    let m = SubstateMacro::from_macro_attribute(attr, item)?;
    let actual = quote!(#m);

    assert_eq!(actual.to_string(), expected.to_string());

    Ok(())
}
