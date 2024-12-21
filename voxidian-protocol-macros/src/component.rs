use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use proc_macro::{Span, TokenStream};
use inflector::Inflector;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use serde::Deserialize;
use syn::{ parse_macro_input, ItemStruct, LitStr };

/// Cache
static COMPONENTS_DATA : LazyLock<ComponentTypes> = LazyLock::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("generated").join("data_component_type.json");
    let Ok(file) = std::fs::read_to_string(&path) else { panic!("`data_component_type.json` missing ({})", path.display()) };
    let Ok(data) = serde_json::from_str::<ComponentTypes>(&file) else { panic!("`data_component_type.json` in invalid format") };
    data
});

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
    let item = parse_macro_input!( item as ItemStruct );
    let attr = parse_macro_input!( attr as LitStr );
    let data = &COMPONENTS_DATA.inner[&attr.value()];

    let idx            = data.protocol_id;
    let structure_name = &item.ident;

    (quote! {
        #item

        impl crate::value::ComponentData for #structure_name {
            const ID: u32 = #idx;
        }
    }).into()
}

pub(crate) fn component_enum_impl() -> TokenStream {
    let formatted_names: Vec<Ident> = Vec::from_iter(
        COMPONENTS_DATA.inner.keys()
            .map(|key|
                Ident::new(
                    &key.replace("minecraft:", "").to_title_case().replace(" ", ""),
                    Span::call_site().into()
                )
            )
    );

    (quote! {
        // TODO: #(#formatted_names(#formatted_names))
        #[derive(Debug, Clone)]
        pub enum DataComponents {

        }

        #[derive(Debug, Clone)]
        pub enum DataComponentTypes {
            #( #formatted_names ),*
        }

    }).into()
}