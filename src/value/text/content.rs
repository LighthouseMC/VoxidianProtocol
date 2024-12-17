use super::*;


#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TextContent {
    Literal {
        #[serde(rename = "text")]
        literal : String
    },
    Translate {
        #[serde(rename = "translate")]
        translate : String
    },
    Keybind {
        keybind : String
    }
}
impl Default for TextContent {
    fn default() -> Self { Self::Literal { literal : String::new() } }
}
impl TextContent {
    pub(super) fn to_nbt(&self) -> NbtCompound {
        let mut nbt = NbtCompound::new();
        match (self) {
            TextContent::Literal   { literal   } => { nbt.insert("text"      , NbtElement::String(literal   .to_string())) },
            TextContent::Translate { translate } => { nbt.insert("translate" , NbtElement::String(translate .to_string())) },
            TextContent::Keybind   { keybind   } => { nbt.insert("keybind"   , NbtElement::String(keybind   .to_string())) }
        }
        nbt
    }
}
impl fmt::Display for TextContent { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    let ( TextContent::Literal   { literal   : text }
        | TextContent::Translate { translate : text }
        | TextContent::Keybind   { keybind   : text }
    ) = self;
    write!(f, "{}", text)
} }
