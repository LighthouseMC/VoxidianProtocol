use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;
use proc_macro::{Span, TokenStream};
use inflector::Inflector;
use proc_macro2::Ident;
use quote::quote;
use serde::Deserialize;
use syn::{parse_macro_input, ItemStruct, LitInt, LitStr};

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
        #[packet_part]
        #[derive(Clone)]
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

    let formatted_values: Vec<LitInt> = Vec::from_iter(
        COMPONENTS_DATA.inner.values()
            .map(|value|
                LitInt::new(
                    &value.protocol_id.to_string(),
                    Span::call_site().into()
                )
            )
    );

    (quote! {
        #[derive(Debug, Clone)]
        pub enum DataComponents {
            #( #formatted_names(#formatted_names) ),*
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum DataComponentTypes {
            #( #formatted_names ),*
        }

        impl DataComponents {
            pub fn as_type(&self) -> DataComponentTypes {
                match self {
                    #( DataComponents::#formatted_names { .. } => DataComponentTypes::#formatted_names ),*
                }
            }
        }

        impl PacketEncode for DataComponents {
            fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
                match self {
                    #( DataComponents::#formatted_names(component) => {
                        buf.encode_write(VarInt::from(DataComponentTypes::#formatted_names.protocol_id() as i32))?;
                        buf.encode_write(component)
                    } ),*
                }
            }
        }

        impl PacketDecode for DataComponents {
            fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
                let id = buf.read_decode::<VarInt>()?;
                match id.as_i32() {
                    #( #formatted_values => {
                        let component = buf.read_decode::<#formatted_names>()?;
                        Ok(DataComponents::#formatted_names(component))
                    } ),*
                    _ => Err(DecodeError::InvalidData("Invalid component type".to_string()))
                }
            }
        }
        
        impl DataComponentTypes {
            pub const fn protocol_id(&self) -> u16 {
                match self {
                    #( DataComponentTypes::#formatted_names => #formatted_values ),*
                }
            }

            pub fn all_types() -> Vec<DataComponentTypes> {
                vec![
                    #( DataComponentTypes::#formatted_names ,)*
                ]
            }
        }

        impl PacketEncode for DataComponentTypes {
            fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
                match self {
                    #( DataComponentTypes::#formatted_names => {
                        buf.encode_write(VarInt::from(#formatted_values))?;
                        Ok(())
                    } ),*
                }
            }
        }

        impl PacketDecode for DataComponentTypes {
            fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
                let id = buf.read_decode::<VarInt>()?;
                match id.as_i32() {
                    #( #formatted_values => Ok(DataComponentTypes::#formatted_names), )*
                    _ => Err(DecodeError::InvalidData("Invalid component type".to_string()))
                }
            }
        }

    }).into()
}