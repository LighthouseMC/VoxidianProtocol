use super::*;


#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct TextComponent {
    #[serde(flatten)]
    pub content : TextContent,
    #[serde(flatten)]
    pub style : TextStyle,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub extra : Vec<TextComponent>
}
impl TextComponent {
    pub fn of_literal<S : Into<String>>(literal : S) -> Self {
        Self::default().with_literal(literal)
    }
    pub fn of_translate<S : Into<String>>(translation : S) -> Self {
        Self::default().with_translate(translation)
    }
    pub fn of_translate_fallback<S : Into<String>, F : Into<String>>(translation : S, fallback : F) -> Self {
        Self::default().with_translate_fallback(translation, fallback)
    }
    pub fn of_translate_interpolate<S : Into<String>, I : Into<String>>(translation : S, interpolate : impl IntoIterator<Item = I>) -> Self {
        Self::default().with_translate_interpolate(translation, interpolate)
    }
    pub fn of_translate_fallback_interpolate<S : Into<String>, F : Into<String>, I : Into<String>>(translation : S, fallback : F, interpolate : impl IntoIterator<Item = I>) -> Self {
        Self::default().with_translate_fallback_interpolate(translation, fallback, interpolate)
    }
    pub fn of_keybind<S : Into<String>>(keybind : S) -> Self {
        Self::default().with_keybind(keybind)
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
        self.with_content(TextContent::Translate { translate : translation.into(), fallback : None, interpolate : Vec::new() })
    }
    pub fn with_translate_fallback<S : Into<String>, F : Into<String>>(&self, translation : S, fallback : F) -> Self {
        self.with_content(TextContent::Translate { translate : translation.into(), fallback : Some(fallback.into()), interpolate : Vec::new() })
    }
    pub fn with_translate_interpolate<S : Into<String>, I : Into<String>>(&self, translation : S, interpolate : impl IntoIterator<Item = I>) -> Self {
        self.with_content(TextContent::Translate { translate : translation.into(), fallback : None, interpolate : interpolate.into_iter().map(|e| e.into()).collect::<Vec<_>>() })
    }
    pub fn with_translate_fallback_interpolate<S : Into<String>, F : Into<String>, I : Into<String>>(&self, translation : S, fallback : F, interpolate : impl IntoIterator<Item = I>) -> Self {
        self.with_content(TextContent::Translate { translate : translation.into(), fallback : Some(fallback.into()), interpolate : interpolate.into_iter().map(|e| e.into()).collect::<Vec<_>>() })
    }
    pub fn with_keybind<S : Into<String>>(&self, keybind : S) -> Self {
        self.with_content(TextContent::Keybind { keybind : keybind.into() })
    }
    pub fn colour(&self, colour : TextColour) -> Self {
        let mut cloned = self.clone(); cloned.style.colour = Some(colour); cloned
    }
    pub fn reset_colour(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.colour = Some(TextColour::White); cloned
    }
    pub fn inherit_colour(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.colour = None; cloned
    }
    pub fn font(&self, font : Identifier) -> Self {
        let mut cloned = self.clone(); cloned.style.font = Some(font); cloned
    }
    pub fn reset_font(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.font = Some(Identifier::vanilla("default")); cloned
    }
    pub fn inherit_font(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.font = None; cloned
    }
    pub fn bold(&self, bold : bool) -> Self {
        let mut cloned = self.clone(); cloned.style.bold = Some(bold); cloned
    }
    pub fn reset_bold(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.bold = Some(false); cloned
    }
    pub fn inherit_bold(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.bold = None; cloned
    }
    pub fn italic(&self, italic : bool) -> Self {
        let mut cloned = self.clone(); cloned.style.italic = Some(italic); cloned
    }
    pub fn reset_italic(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.italic = Some(false); cloned
    }
    pub fn inherit_italic(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.italic = None; cloned
    }
    pub fn underline(&self, underline : bool) -> Self {
        let mut cloned = self.clone(); cloned.style.underline = Some(underline); cloned
    }
    pub fn reset_underline(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.underline = Some(false); cloned
    }
    pub fn inherit_underline(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.underline = None; cloned
    }
    pub fn strikethrough(&self, strikethrough : bool) -> Self {
        let mut cloned = self.clone(); cloned.style.strikethrough = Some(strikethrough); cloned
    }
    pub fn reset_strikethrough(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.strikethrough = Some(false); cloned
    }
    pub fn inherit_strikethrough(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.strikethrough = None; cloned
    }
    pub fn obfuscate(&self, obfuscate : bool) -> Self {
        let mut cloned = self.clone(); cloned.style.obfuscate = Some(obfuscate); cloned
    }
    pub fn reset_obfuscate(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.obfuscate = Some(false); cloned
    }
    pub fn inherit_obfuscate(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.obfuscate = None; cloned
    }
    pub fn insertion(&self, insertion : String) -> Self {
        let mut cloned = self.clone(); cloned.style.insertion = Some(insertion); cloned
    }
    pub fn reset_insertion(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.insertion = Some(String::new()); cloned
    }
    pub fn inherit_insertion(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.insertion = None; cloned
    }
    pub fn click_event(&self, click_event : TextClickEvent) -> Self {
        let mut cloned = self.clone(); cloned.style.click_event = Some(click_event); cloned
    }
    pub fn reset_click_event(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.click_event = Some(TextClickEvent::RunCommand { command : String::new() }); cloned
    }
    pub fn inherit_click_event(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.click_event = None; cloned
    }
    pub fn hover_event(&self, hover_event : TextHoverEvent) -> Self {
        let mut cloned = self.clone(); cloned.style.hover_event = Some(hover_event); cloned
    }
    pub fn reset_hover_event(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.hover_event = Some(TextHoverEvent::ShowText { text : Text::new() }); cloned
    }
    pub fn inherit_hover_event(&self) -> Self {
        let mut cloned = self.clone(); cloned.style.hover_event = None; cloned
    }
}
impl TextComponent {
    pub fn to_nbt(&self) -> NbtCompound {
        let mut nbt = self.content.to_nbt();
        nbt.extend(self.style.to_nbt());
        if (! self.extra.is_empty()) {
            let mut extra = Vec::new();
            for component in &self.extra { extra.push(NbtElement::Compound(component.to_nbt())); }
            nbt.insert("extra", NbtElement::List(extra));
        }
        nbt
    }
}
impl fmt::Display for TextComponent { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.content)?;
    for extra in &self.extra { write!(f, "{}", extra)?; }
    Ok(())
} }

impl PacketEncode for TextComponent {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        let nbt = Nbt {
            root: self.to_nbt(),
            ..Default::default()
        };
        buf.encode_write(nbt)?;
        Ok(())
    }
}

impl PacketDecode for TextComponent {
    fn decode(_buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        todo!()
    }
}
