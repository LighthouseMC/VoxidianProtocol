use super::*;


#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct TextComponent {
    #[serde(flatten)]
    pub content : TextContent,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none", default)]
    pub colour : Option<TextColour>,
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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub extra : Vec<TextComponent>
}
impl TextComponent {
    pub fn of_literal<S : Into<String>>(literal : S) -> Self {
        Self::default().with_literal(literal.into())
    }
    pub fn of_translate<S : Into<String>>(translation : S) -> Self {
        Self::default().with_translate(translation.into())
    }
    pub fn of_keybind<S : Into<String>>(keybind : S) -> Self {
        Self::default().with_keybind(keybind.into())
    }
}
impl TextComponent {
    pub fn with_content(&self, content : TextContent) -> Self {
        let mut cloned = self.clone(); cloned.content = content; cloned
    }
    pub fn with_literal<S : Into<String>>(&self, literal : S) -> Self {
        self.with_content(TextContent::Literal { literal : literal.into() })
    }
    pub fn with_translate<S : Into<String>>(&self, translation : S) -> Self {
        self.with_content(TextContent::Translate { translate : translation.into() })
    }
    pub fn with_keybind<S : Into<String>>(&self, keybind : S) -> Self {
        self.with_content(TextContent::Keybind { keybind : keybind.into() })
    }
    pub fn colour(&self, colour : TextColour) -> Self {
        let mut cloned = self.clone(); cloned.colour = Some(colour); cloned
    }
    pub fn reset_colour(&self) -> Self {
        let mut cloned = self.clone(); cloned.colour = Some(TextColour::White); cloned
    }
    pub fn inherit_colour(&self) -> Self {
        let mut cloned = self.clone(); cloned.colour = None; cloned
    }
    pub fn font(&self, font : Identifier) -> Self {
        let mut cloned = self.clone(); cloned.font = Some(font); cloned
    }
    pub fn reset_font(&self) -> Self {
        let mut cloned = self.clone(); cloned.font = Some(Identifier::vanilla("default")); cloned
    }
    pub fn inherit_font(&self) -> Self {
        let mut cloned = self.clone(); cloned.font = None; cloned
    }
    pub fn bold(&self, bold : bool) -> Self {
        let mut cloned = self.clone(); cloned.bold = Some(bold); cloned
    }
    pub fn reset_bold(&self) -> Self {
        let mut cloned = self.clone(); cloned.bold = Some(false); cloned
    }
    pub fn inherit_bold(&self) -> Self {
        let mut cloned = self.clone(); cloned.bold = None; cloned
    }
    pub fn italic(&self, italic : bool) -> Self {
        let mut cloned = self.clone(); cloned.italic = Some(italic); cloned
    }
    pub fn reset_italic(&self) -> Self {
        let mut cloned = self.clone(); cloned.italic = Some(false); cloned
    }
    pub fn inherit_italic(&self) -> Self {
        let mut cloned = self.clone(); cloned.italic = None; cloned
    }
    pub fn underline(&self, underline : bool) -> Self {
        let mut cloned = self.clone(); cloned.underline = Some(underline); cloned
    }
    pub fn reset_underline(&self) -> Self {
        let mut cloned = self.clone(); cloned.underline = Some(false); cloned
    }
    pub fn inherit_underline(&self) -> Self {
        let mut cloned = self.clone(); cloned.underline = None; cloned
    }
    pub fn strikethrough(&self, strikethrough : bool) -> Self {
        let mut cloned = self.clone(); cloned.strikethrough = Some(strikethrough); cloned
    }
    pub fn reset_strikethrough(&self) -> Self {
        let mut cloned = self.clone(); cloned.strikethrough = Some(false); cloned
    }
    pub fn inherit_strikethrough(&self) -> Self {
        let mut cloned = self.clone(); cloned.strikethrough = None; cloned
    }
    pub fn obfuscate(&self, obfuscate : bool) -> Self {
        let mut cloned = self.clone(); cloned.obfuscate = Some(obfuscate); cloned
    }
    pub fn reset_obfuscate(&self) -> Self {
        let mut cloned = self.clone(); cloned.obfuscate = Some(false); cloned
    }
    pub fn inherit_obfuscate(&self) -> Self {
        let mut cloned = self.clone(); cloned.obfuscate = None; cloned
    }
    pub fn insertion(&self, insertion : String) -> Self {
        let mut cloned = self.clone(); cloned.insertion = Some(insertion); cloned
    }
    pub fn reset_insertion(&self) -> Self {
        let mut cloned = self.clone(); cloned.insertion = Some(String::new()); cloned
    }
    pub fn inherit_insertion(&self) -> Self {
        let mut cloned = self.clone(); cloned.insertion = None; cloned
    }
    pub fn click_event(&self, click_event : TextClickEvent) -> Self {
        let mut cloned = self.clone(); cloned.click_event = Some(click_event); cloned
    }
    pub fn reset_click_event(&self) -> Self {
        let mut cloned = self.clone(); cloned.click_event = Some(TextClickEvent::RunCommand { command : String::new() }); cloned
    }
    pub fn inherit_click_event(&self) -> Self {
        let mut cloned = self.clone(); cloned.click_event = None; cloned
    }
    pub fn hover_event(&self, hover_event : TextHoverEvent) -> Self {
        let mut cloned = self.clone(); cloned.hover_event = Some(hover_event); cloned
    }
    pub fn reset_hover_event(&self) -> Self {
        let mut cloned = self.clone(); cloned.hover_event = Some(TextHoverEvent::ShowText { text : Text::new() }); cloned
    }
    pub fn inherit_hover_event(&self) -> Self {
        let mut cloned = self.clone(); cloned.hover_event = None; cloned
    }
}
impl TextComponent {
    pub(super) fn to_nbt(&self) -> NbtCompound {
        let mut nbt = self.content.to_nbt();
        if let Some(colour        ) = &self.colour        { nbt.insert("color"         , colour.to_nbt()                                            ); }
        if let Some(font          ) = &self.font          { nbt.insert("font"          , NbtElement::String ( font.to_string()                     )); }
        if let Some(bold          ) =  self.bold          { nbt.insert("bold"          , NbtElement::Byte   ( if (bold          ) { 1 } else { 0 } )); }
        if let Some(italic        ) =  self.italic        { nbt.insert("italic"        , NbtElement::Byte   ( if (italic        ) { 1 } else { 0 } )); }
        if let Some(underline     ) =  self.underline     { nbt.insert("underlined"    , NbtElement::Byte   ( if (underline     ) { 1 } else { 0 } )); }
        if let Some(strikethrough ) =  self.strikethrough { nbt.insert("strikethrough" , NbtElement::Byte   ( if (strikethrough ) { 1 } else { 0 } )); }
        if let Some(obfuscate     ) =  self.obfuscate     { nbt.insert("obfuscated"    , NbtElement::Byte   ( if (obfuscate     ) { 1 } else { 0 } )); }
        if let Some(insertion     ) = &self.insertion     { nbt.insert("insertion"     , NbtElement::String ( insertion.to_string()                )); }
        if let Some(click_event   ) = &self.click_event   { click_event.write_nbt(&mut nbt); }
        if let Some(hover_event   ) = &self.hover_event   { hover_event.write_nbt(&mut nbt); }
        // TODO: extra
        nbt
    }
}
impl fmt::Display for TextComponent { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.content)?;
    for extra in &self.extra { write!(f, "{}", extra)?; }
    Ok(())
} }
