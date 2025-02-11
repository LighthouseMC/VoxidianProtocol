use super::*;


#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "action")]
pub enum TextHoverEvent {
    #[serde(rename = "show_text")]
    ShowText {
        #[serde(rename = "contents")]
        text : Text
    },
    // TODO: ShowItem
    #[serde(rename = "show_entity")]
    ShowEntity {
        #[serde(rename = "contents")]
        show_entity : ShowEntityTextHoverEvent
    }
}
impl TextHoverEvent {
    pub(super) fn write_nbt(&self, buf : &mut NbtCompound) {
        let (action, contents) = match (self) {
            Self::ShowText   { text        } => ("show_text"   , text.to_nbt().into_inner()                 ),
            Self::ShowEntity { show_entity } => ("show_entity" , NbtElement::Compound(show_entity.to_nbt()) ),
        };
        buf.insert("action", NbtElement::String(action.to_string()));
        buf.insert("contents", contents)
    }
}

#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShowEntityTextHoverEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    name : Option<Text>,
    #[serde(rename = "type")]
    kind : Identifier,
    #[serde(rename = "id")]
    uuid : Uuid
}
impl ShowEntityTextHoverEvent {
    fn to_nbt(&self) -> NbtCompound {
        let mut nbt = NbtCompound::new();
        if let Some(name) = &self.name { nbt.insert("name", NbtElement::String(name.to_json().into_inner())); }
        nbt.insert("kind", NbtElement::String(self.kind.to_string()));
        nbt.insert("uuid", NbtElement::String(self.uuid.to_string()));
        nbt
    }
}
