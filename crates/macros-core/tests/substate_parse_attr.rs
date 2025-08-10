#![allow(non_snake_case)]

use mia_info_poc_macros_core::SubstateMacroAttr;
use mia_info_poc_macros_core_test_utils::*;
use syn::Result;

#[test]
fn macro_contents__empty() -> syn::Result<()> {
    let result: Result<SubstateMacroAttr> = parse!();
    assert_err!(result, "unexpected end of input, expected identifier");
    Ok(())
}

#[test]
fn macro_contents__source__no_comma_no_field() -> syn::Result<()> {
    let attr: SubstateMacroAttr = parse! {
        MyState
    }?;
    assert_eq!(attr.source_state_path, path!(MyState));
    assert_eq!(attr.field, None);
    Ok(())
}

#[test]
fn macro_contents__source__no_field() {
    let result: Result<SubstateMacroAttr> = parse! {
        MyState,
    };
    assert_err!(result, "expected `field(<ident>)`");
}

#[test]
fn macro_contents__source__invalid_field() -> syn::Result<()> {
    let result: Result<SubstateMacroAttr> = parse! {
        MyState, xyz
    };
    assert_err!(result, "expected `field(<ident>)`");
    Ok(())
}

#[test]
fn macro_contents__source__field_no_parens() -> syn::Result<()> {
    let result: Result<SubstateMacroAttr> = parse! {
        MyState, field
    };
    assert_err!(result, "unexpected end of input, expected parentheses");
    Ok(())
}

#[test]
fn macro_contents__source__empty_field() -> syn::Result<()> {
    let result: Result<SubstateMacroAttr> = parse! {
        MyState, field()
    };
    assert_err!(result, "unexpected end of input, expected identifier");
    Ok(())
}

#[test]
fn macro_contents__source__field() -> syn::Result<()> {
    let attr: SubstateMacroAttr = parse! {
        MyState, field(my_sub)
    }?;
    assert_eq!(attr.source_state_path, path!(MyState));
    assert_eq!(attr.field, Some(ident("my_sub")));
    Ok(())
}
