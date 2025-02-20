use super::*;


#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct TextStyle {
    #[serde(rename = "color", skip_serializing_if = "Option::is_none", default)]
    pub colour : Option<TextColour>,
    // TODO: shadow_colour
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub font : Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bold : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub italic : Option<bool>,
    #[serde(rename = "underlined", skip_serializing_if = "Option::is_none", default)]
    pub underline : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub strikethrough : Option<bool>,
    #[serde(rename = "obfuscated", skip_serializing_if = "Option::is_none", default)]
    pub obfuscate : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub insertion : Option<String>,
    #[serde(rename = "clickEvent", skip_serializing_if = "Option::is_none", default)]
    pub click_event : Option<TextClickEvent>,
    #[serde(rename = "hoverEvent", skip_serializing_if = "Option::is_none", default)]
    pub hover_event : Option<TextHoverEvent>,
}
impl TextStyle {
    pub fn to_nbt(&self) -> NbtCompound {
        let mut nbt = NbtCompound::new();
        if let Some(colour        ) = &self.colour        { nbt.insert("color"         , colour.as_nbt()                                            ); }
        if let Some(font          ) = &self.font          { nbt.insert("font"          , NbtElement::String ( font.to_string()                     )); }
        if let Some(bold          ) =  self.bold          { nbt.insert("bold"          , NbtElement::Byte   ( if (bold          ) { 1 } else { 0 } )); }
        if let Some(italic        ) =  self.italic        { nbt.insert("italic"        , NbtElement::Byte   ( if (italic        ) { 1 } else { 0 } )); }
        if let Some(underline     ) =  self.underline     { nbt.insert("underlined"    , NbtElement::Byte   ( if (underline     ) { 1 } else { 0 } )); }
        if let Some(strikethrough ) =  self.strikethrough { nbt.insert("strikethrough" , NbtElement::Byte   ( if (strikethrough ) { 1 } else { 0 } )); }
        if let Some(obfuscate     ) =  self.obfuscate     { nbt.insert("obfuscated"    , NbtElement::Byte   ( if (obfuscate     ) { 1 } else { 0 } )); }
        if let Some(insertion     ) = &self.insertion     { nbt.insert("insertion"     , NbtElement::String ( insertion.to_string()                )); }
        if let Some(click_event) = &self.click_event {
            let mut subnbt = NbtCompound::new();
            click_event.write_nbt(&mut subnbt);
            nbt.insert("clickEvent", NbtElement::Compound(subnbt));
        }
        if let Some(hover_event) = &self.hover_event {
            let mut subnbt = NbtCompound::new();
            hover_event.write_nbt(&mut subnbt);
            nbt.insert("hoverEvent", NbtElement::Compound(subnbt));
        }
        nbt
    }
}
