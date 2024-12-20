use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
use syn::{ parse_macro_input, ItemStruct, LitStr };

/// Cache
static COMPONENTS_DATA : Mutex<Option<ComponentTypes>> = Mutex::new(None);
#[derive(Deserialize)]
#[serde(transparent)]
struct ComponentTypes {
    inner: HashMap<String, ComponentData>
}
#[derive(Deserialize)]
struct ComponentData {
    pub protocol_id: u32
}
macro get_components_data(let $pat:pat) {
    COMPONENTS_DATA.clear_poison();
    let mut components_data = match (COMPONENTS_DATA.lock()) {
        Ok  (guard ) => guard,
        Err (_     ) => { COMPONENTS_DATA.clear_poison(); COMPONENTS_DATA.lock().unwrap() }
    };
    let $pat = components_data.get_or_insert_with(
        || {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("generated").join("data_component_type.json");
            let Ok(file) = std::fs::read_to_string(&path) else { panic!("`data_component_type.json` missing ({})", path.display()) };
            let Ok(data) = serde_json::from_str::<ComponentTypes>(&file) else { panic!("`data_component_type.json` in invalid format") };
            data
        }
    );
}


pub(crate) fn component_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!( item as ItemStruct );
    let attr = parse_macro_input!( attr as LitStr );

    get_components_data!(let types);
    let data = &types.inner[&attr.value()];

    let idx            = data.protocol_id;
    let structure_name = &item.ident;
    quote! {
        #item

        impl crate::value::ComponentData for #structure_name {
            const ID: u32 = #idx;
        }
    }.into()
}