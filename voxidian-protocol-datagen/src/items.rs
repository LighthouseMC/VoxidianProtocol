use proc_macro2::TokenStream;

use crate::{format_token_stream, reports::get_reports_json};

pub fn make_items() {
    let json = make_items_to_json();
    let tokens = make_items_tokens(json);
    let fmt = format_token_stream(&tokens);

    std::fs::write("../src/autogenerated/items.rs", fmt).unwrap();
}

// pub fn test() -> Item {
//     Item { id: Identifier::vanilla_const("abc") }
// }

pub(crate) fn make_items_to_json() -> Vec<(String, u32)> {
    let mut registries = get_reports_json();
    let items = registries.map.get_mut("minecraft:item").unwrap();
    let mut vc = items.entries
        .iter()
        .map(|(name, entry)| {
            (name.clone(), entry.protocol_id)
        })
        .collect::<Vec<_>>();
    vc.sort_by_key(|x| x.1);
    vc
}

pub fn make_items_tokens(item_entries: Vec<(String, u32)>) -> TokenStream {
    let mut stream = TokenStream::new();
    for entry in item_entries {
        let name = entry.0.replace("minecraft:", "");
        stream.extend(quote::quote! {
            registry.insert(Identifier::vanilla_const(#name), Item { id: Identifier::vanilla_const(#name) });
        });
    }
    quote::quote! {
        use crate::packet::*;

        impl Item {
            #[allow(dead_code)]
            #[allow(redundant_semicolons)]
            pub fn vanilla_registry() -> Registry<Item> {
                let mut registry = Registry::new();
                #stream
                registry
            }
        }
    }
}