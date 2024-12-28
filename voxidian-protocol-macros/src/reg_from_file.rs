use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Type};

#[derive(serde::Deserialize)]
struct ItemRegistryContainer {
    #[serde(flatten)]
   map: std::collections::HashMap<String, ItemRegistryEntry> 
}

#[derive(serde::Deserialize)]
struct ItemRegistryEntry {
    protocol_id: i32
}

pub(crate) fn item_reg_from_file_impl(input: TokenStream) -> TokenStream {
    let file = parse_macro_input!(input as syn::LitStr);
    let file_contents = std::fs::read_to_string(&file.value()).unwrap();

    let registry_ids = serde_json::from_str::<ItemRegistryContainer>(&file_contents).unwrap();

    let mut registry_keys_in_order = registry_ids.map
        .keys()
        .collect::<Vec<_>>();
    registry_keys_in_order.sort_by_key(|k| registry_ids.map.get(*k).unwrap().protocol_id);

    let mut reg_tok = proc_macro2::TokenStream::new();
    for key in registry_keys_in_order {
        let split_str1 = key.split(":");
        let split_str2 = key.split(":");
        reg_tok = (quote! {
            #reg_tok
            registry.insert(Identifier::new(#(#split_str1),*), Item { id: Identifier::new(#(#split_str2),*) });
        });
    }


    (quote! {
        {
            let mut registry = Registry::new();
            #reg_tok
            registry
        }
    }).into()
}