use super::*;
use std::fmt;


#[derive(Ser, Deser, Clone, PartialEq, Eq, Hash, Default)]
pub struct Text(Vec<TextComponent>);
impl Text {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, component : TextComponent) {
        self.0.push(component);
    }
}
impl From<Vec<TextComponent>> for Text { fn from(from : Vec<TextComponent>) -> Self {
    Self(from)
} }
impl From<TextComponent> for Text { fn from(from : TextComponent) -> Self {
    Self(vec![ from ])
} }
impl Text {

    pub fn to_json(&self) -> String {
        if (self.0.is_empty()) {
            String::from("[{\"text\":\"\"}]")
        } else {
            to_json_string(self).unwrap()
        }
    }

    pub fn from_json<S : AsRef<str>>(json : S) -> Result<Self, serde_json::error::Error> {
        from_json_str::<Self>(json.as_ref())
    }

}
impl fmt::Debug for Text { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_json())
} }

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

#[derive(Ser, Deser, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextColour {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "dark_blue")]
    DarkBlue,
    #[serde(rename = "dark_green")]
    DarkGreen,
    #[serde(rename = "dark_aqua")]
    DarkAqua,
    #[serde(rename = "dark_red")]
    DarkRed,
    #[serde(rename = "dark_purple")]
    DarkPurple,
    #[serde(rename = "gold")]
    Gold,
    #[serde(rename = "gray")]
    Grey,
    #[serde(rename = "dark_gray")]
    DarkGrey,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "aqua")]
    Aqua,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "light_purple")]
    Pink,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "white")]
    White,
    #[serde(serialize_with = "rgb_to_hex", deserialize_with = "hex_to_rgb", untagged)]
    RGB(u8, u8, u8)
}
fn rgb_to_hex<S : Serer>(r : &u8, g : &u8, b : &u8, ser : S) -> Result<S::Ok, S::Error> {
    ser.serialize_str(&format!("#{:02x}{:02x}{:02x}", r, g, b))
}
fn hex_to_rgb<'l, D : Deserer<'l>>(deser : D) -> Result<(u8, u8, u8), D::Error> {
    let hexcode = String::deserialize(deser)?;
    if (hexcode.len() == 7) { if let Some('#') = hexcode.chars().nth(0) {
        let mut parts = hexcode.chars().skip(1).array_chunks::<2>().map(|[a, b]| { let mut c = a.to_string(); c.push(b); c });
        return Ok((
            u8::from_str_radix(&parts.next().unwrap(), 16).map_err(|_| serde::de::Error::custom("Not a hex colour code"))?,
            u8::from_str_radix(&parts.next().unwrap(), 16).map_err(|_| serde::de::Error::custom("Not a hex colour code"))?,
            u8::from_str_radix(&parts.next().unwrap(), 16).map_err(|_| serde::de::Error::custom("Not a hex colour code"))?
        ));
    } }
    Err(serde::de::Error::custom("Not a hex colour code"))
}

#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "action")]
pub enum TextClickEvent {
    #[serde(rename = "open_url")]
    OpenURL {
        #[serde(rename = "value")]
        url : String
    },
    #[serde(rename = "run_command")]
    RunCommand {
        #[serde(rename = "value")]
        command : String
    },
    #[serde(rename = "suggest_command")]
    SuggestCommand {
        #[serde(rename = "value")]
        command : String
    },
    #[serde(rename = "change_page")]
    SetBookPage {
        #[serde(rename = "value", serialize_with = "to_string_to_string")]
        command : usize
    },
    #[serde(rename = "copy_to_clipboard")]
    SetClipboard {
        #[serde(rename = "value")]
        text : String
    }
}
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
#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShowEntityTextHoverEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    name : Option<Text>,
    #[serde(rename = "type")]
    kind : Identifier,
    #[serde(rename = "id")]
    uuid : Uuid
}
fn to_string_to_string<T : ToString, S : Serer>(value : &T, ser : S) -> Result<S::Ok, S::Error> {
    ser.serialize_str(&value.to_string())
}


impl PacketEncode for Text { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.encode_write(&to_json_string(self).unwrap())
} }
impl PacketDecode for Text { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    from_json_str(&buf.read_decode::<String>()?).map_err(|_| DecodeError::InvalidData("Text data is not valid JSON".to_string()))
} }


impl fmt::Display for Text { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    for component in &self.0 { write!(f, "{}", component)?; }
    Ok(())
} }
impl fmt::Display for TextComponent { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.content)?;
    for extra in &self.extra { write!(f, "{}", extra)?; }
    Ok(())
} }
impl fmt::Display for TextContent { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    let ( TextContent::Literal   { literal     : text }
        | TextContent::Translate { translate : text }
        | TextContent::Keybind   { keybind     : text }
    ) = self;
    write!(f, "{}", text)
} }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_serialise_text() {
        let text = Text::from(vec![
            TextComponent::of_literal("Hello,").colour(TextColour::DarkGreen),
            TextComponent::of_literal("World!").colour(TextColour::RGB(255, 127, 0))
        ]);
        let json = text.to_json();
        assert_eq!(json, "[{\"text\":\"Hello,\",\"color\":\"dark_green\"},{\"text\":\"World!\",\"color\":\"#ff7f00\"}]");
    }

    #[test]
    fn json_deserialise_text() {
        let json = "[{\"text\":\"Hello,\",\"color\":\"dark_green\"},{\"text\":\"World!\",\"color\":\"#ff7f00\"}]";
        let text = Text::from_json(json).unwrap();
        assert_eq!(text.0.as_slice(), [
            TextComponent::of_literal("Hello,").colour(TextColour::DarkGreen),
            TextComponent::of_literal("World!").colour(TextColour::RGB(255, 127, 0))
        ]);
    }

}
