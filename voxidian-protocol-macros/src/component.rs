use proc_macro::Span;
use std::collections::HashMap;
use std::path::PathBuf;
use proc_macro2::{TokenStream};
use quote::__private::ext::RepToTokensExt;
use quote::{quote, ToTokens};
use serde::Deserialize;
use syn::{parse_macro_input, ItemStruct, LitStr};

#[derive(Deserialize)]
#[serde(transparent)]
struct ComponentTypes {
    inner: HashMap<String, ComponentData>
}

#[derive(Deserialize)]
struct ComponentData {
    pub protocol_id: u32
}
pub(crate) fn component_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    if(attr.is_empty()) {
        proc_macro::Span::call_site().warning("components need a constant raw string attribute").emit();
    }
    let attr_stream: proc_macro::TokenStream = attr.clone().into();
    let item_stream: proc_macro::TokenStream = item.clone().into();

    let structure = match syn::parse::<ItemStruct>(item_stream) {
        Ok(s) => s,
        Err(e) => { panic!("{}", e.to_string()); }
    };

    let attr_path = match syn::parse::<LitStr>(attr_stream) {
        Ok(v) => v,
        Err(e) => { panic!("{}", e.to_string()); }
    }.value();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("generated").join("data_component_type.json");
    let json_string = std::fs::read_to_string(path).expect("/generated/data_component_type.json file must be present");
    let types: ComponentTypes = serde_json::from_str(&json_string).expect("valid json expected");

    let data = types.inner.get(&attr_path).expect("expected valid entry into components table");
    let idx = data.protocol_id;
    let structure_name = &structure.ident;
    quote! {
        #structure

        impl crate::value::ComponentData for #structure_name {
            const ID: u32 = #idx;
        }
    }
}