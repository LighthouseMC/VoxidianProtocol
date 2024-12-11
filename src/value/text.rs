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

#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextComponent {
    #[serde(flatten)]
    pub kind  : TextComponentKind,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none", default)]
    pub colour : Option<TextColour>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub font : Option<Identifier>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub extra : Vec<TextComponent>
}
impl TextComponent {
    pub fn with_colour(&self, colour : Option<TextColour>) -> Self {
        let mut cloned = self.clone(); cloned.colour = colour; cloned
    }
}
impl<T : Into<String>> From<T> for TextComponent { fn from(from : T) -> Self { Self {
    kind   : TextComponentKind::from(from),
    colour : None,
    font   : None,
    extra  : Vec::new()
} } }

#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TextComponentKind {
    Text { text : String }
}
impl<T : Into<String>> From<T> for TextComponentKind { fn from(from : T) -> Self {
    Self::Text { text : from.into() }
} }

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



#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::ser::to_string as to_json_string;
    use serde_json::de::from_str as from_json_str;

    #[test]
    fn json_serialise_text() {
        let text = Text::from(vec![
            TextComponent::from("Hello,").with_colour(Some(TextColour::DarkGreen)),
            TextComponent::from("World!").with_colour(Some(TextColour::RGB(255, 127, 0)))
        ]);
        let json = to_json_string(&text).unwrap();
        assert_eq!(json, "[{\"text\":\"Hello,\",\"color\":\"dark_green\"},{\"text\":\"World!\",\"color\":\"#ff7f00\"}]");
    }

    #[test]
    fn json_deserialise_text() {
        let json = "[{\"text\":\"Hello,\",\"color\":\"dark_green\"},{\"text\":\"World!\",\"color\":\"#ff7f00\"}]";
        let text = from_json_str::<Text>(json).unwrap();
        assert_eq!(text.0.as_slice(), [
            TextComponent::from("Hello,").with_colour(Some(TextColour::DarkGreen)),
            TextComponent::from("World!").with_colour(Some(TextColour::RGB(255, 127, 0)))
        ]);
    }

}

