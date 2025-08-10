use mia_info_poc_macros_core::SubstateMacro;
use proc_macro::TokenStream;

macro_rules! parse_macro_attribute_input {
    ($parser:path : ($attr:ident, $item:ident)) => {
        match $parser($attr.into(), $item.into()) {
            Ok(m) => m,
            Err(e) => return e.to_compile_error().into(),
        }
    };
}

#[proc_macro_attribute]
pub fn substate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let substate = parse_macro_attribute_input!(SubstateMacro::from_macro_attribute:(attr, item));
    quote::quote!(#substate).into()
}
