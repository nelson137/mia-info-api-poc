#![allow(non_snake_case)]

use mia_info_poc_macros_core::MockHelpersMacro;
use quote::quote;

#[test]
fn e2e() -> syn::Result<()> {
    let attr = quote! {};
    let item = quote! {
        trait MySubstate {}
    };

    let expected = quote! {
        #item
        impl ::std::convert::From<MockMySubstate> for ::axum::extract::State<::std::sync::Arc<dyn MySubstate>> {
            fn from(value: MockMySubstate) -> Self {
                ::axum::extract::State(::std::sync::Arc::new(value))
            }
        }
    };

    let m = MockHelpersMacro::from_macro_attribute(attr, item)?;
    let actual = quote!(#m);

    assert_eq!(actual.to_string(), expected.to_string());

    Ok(())
}
