use super::*;


#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct TextComponent {
    #[serde(flatten)]
    pub content : TextContent,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none", default)]
    pub colour : Option<TextColour>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub font : Option<Identifier>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub bold : bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub italic : bool,
    #[serde(rename = "underlined", skip_serializing_if = "is_false", default)]
    pub underline : bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub strikethrough : bool,
    #[serde(rename = "obfuscated", skip_serializing_if = "is_false", default)]
    pub obfuscate : bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub insertion : Option<String>,
    #[serde(rename = "clickEvent", skip_serializing_if = "Option::is_none", default)]
    pub click_event : Option<TextClickEvent>,
    #[serde(rename = "hoverEvent", skip_serializing_if = "Option::is_none", default)]
    pub hover_event : Option<TextHoverEvent>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub extra : Vec<TextComponent>
}
fn is_false(value : &bool) -> bool { !value }
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
        self.with_content(TextContent::Translate { translation : translation.into() })
    }
    pub fn with_keybind<S : Into<String>>(&self, keybind : S) -> Self {
        self.with_content(TextContent::Keybind { keybind : keybind.into() })
    }
    pub fn with_colour(&self, colour : Option<TextColour>) -> Self {
        let mut cloned = self.clone(); cloned.colour = colour; cloned
    }
    pub fn with_font(&self, font : Option<Identifier>) -> Self {
        let mut cloned = self.clone(); cloned.font = font; cloned
    }
    pub fn with_bold(&self, bold : bool) -> Self {
        let mut cloned = self.clone(); cloned.bold = bold; cloned
    }
    pub fn with_italic(&self, italic : bool) -> Self {
        let mut cloned = self.clone(); cloned.italic = italic; cloned
    }
    pub fn with_underline(&self, underline : bool) -> Self {
        let mut cloned = self.clone(); cloned.underline = underline; cloned
    }
    pub fn with_strikethrough(&self, strikethrough : bool) -> Self {
        let mut cloned = self.clone(); cloned.strikethrough = strikethrough; cloned
    }
    pub fn with_obfuscate(&self, obfuscate : bool) -> Self {
        let mut cloned = self.clone(); cloned.obfuscate = obfuscate; cloned
    }
    pub fn with_insertion(&self, insertion : Option<String>) -> Self {
        let mut cloned = self.clone(); cloned.insertion = insertion; cloned
    }
    pub fn with_click_event(&self, click_event : Option<TextClickEvent>) -> Self {
        let mut cloned = self.clone(); cloned.click_event = click_event; cloned
    }
    pub fn with_hover_event(&self, hover_event : Option<TextHoverEvent>) -> Self {
        let mut cloned = self.clone(); cloned.hover_event = hover_event; cloned
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
        #[serde(rename = "translatable")]
        translation : String
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



#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::ser::to_string as to_json_string;
    use serde_json::de::from_str as from_json_str;

    #[test]
    fn json_serialise_text() {
        let text = Text::from(vec![
            TextComponent::of_literal("Hello,").with_colour(Some(TextColour::DarkGreen)),
            TextComponent::of_literal("World!").with_colour(Some(TextColour::RGB(255, 127, 0)))
        ]);
        let json = to_json_string(&text).unwrap();
        assert_eq!(json, "[{\"text\":\"Hello,\",\"color\":\"dark_green\"},{\"text\":\"World!\",\"color\":\"#ff7f00\"}]");
    }

    #[test]
    fn json_deserialise_text() {
        let json = "[{\"text\":\"Hello,\",\"color\":\"dark_green\"},{\"text\":\"World!\",\"color\":\"#ff7f00\"}]";
        let text = from_json_str::<Text>(json).unwrap();
        assert_eq!(text.0.as_slice(), [
            TextComponent::of_literal("Hello,").with_colour(Some(TextColour::DarkGreen)),
            TextComponent::of_literal("World!").with_colour(Some(TextColour::RGB(255, 127, 0)))
        ]);
    }

}

