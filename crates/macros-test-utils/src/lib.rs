pub fn ident(i: &str) -> syn::Ident {
    syn::Ident::new(i, proc_macro2::Span::call_site())
}

#[macro_export]
macro_rules! assert_err {
    ($result:ident, $expected_message:literal) => {
        match $result {
            Err(err) => assert_eq!(err.to_string(), $expected_message),
            _ => panic!(
                "expected Ok to be an Err with message: {}",
                $expected_message
            ),
        }
    };
}

#[macro_export]
macro_rules! parse {
    () => {
        syn::parse2(proc_macro2::TokenStream::new())
    };
    ($($tt:tt)*) => {
        syn::parse2(quote::quote! { $($tt)* })
    };
}

#[macro_export]
macro_rules! path {
    ($ident:ident $(:: $rest:ident)*) => {
        ::syn::Path {
            leading_colon: ::core::option::Option::None,
            segments: ::syn::punctuated::Punctuated::from_iter([
                ::syn::PathSegment::from(
                    ::syn::Ident::new(stringify!($ident), ::proc_macro2::Span::call_site()),
                ),
                $(::syn::PathSegment::from(
                    ::syn::Ident::new(stringify!($rest), ::proc_macro2::Span::call_site()),
                ),)*
            ]),
        }
    };
    (:: $ident:ident $(:: $rest:ident)*) => {
        ::syn::Path {
            leading_colon: ::core::option::Option::Some(::core::default::Default::default()),
            segments: ::syn::punctuated::Punctuated::from_iter([
                ::syn::PathSegment::from(
                    ::syn::Ident::new(stringify!($ident), ::proc_macro2::Span::call_site()),
                ),
                $(::syn::PathSegment::from(
                    ::syn::Ident::new(stringify!($rest), ::proc_macro2::Span::call_site()),
                ),)*
            ]),
        }
    };
}
