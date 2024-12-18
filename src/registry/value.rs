use crate::value::{ Identifier, Nbt };


pub trait RegValue {

    const REGISTRY_ID : Identifier;

    fn to_registry_data_packet(&self) -> Option<Nbt>;

}
