mod component;
pub use component::*;
mod content;
pub use content::*;
mod colour;
pub use colour::*;
mod click_event;
pub use click_event::*;
mod hover_event;
pub use hover_event::*;
mod nbt;
pub use nbt::*;


use super::*;


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

    pub fn to_nbt(&self) -> NbtText {
        NbtText(if (self.0.is_empty()) {
            NbtElement::String(String::new())
        } else {
            NbtElement::List(self.0.iter().map(|component| NbtElement::Compound(component.to_nbt())).collect::<Vec<_>>())
        })
    }

    pub fn from_json<S : AsRef<str>>(json : S) -> Result<Self, serde_json::error::Error> {
        from_json_str::<Self>(json.as_ref())
    }

}
impl fmt::Debug for Text { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_json())
} }
impl fmt::Display for Text { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    for component in &self.0 { write!(f, "{}", component)?; }
    Ok(())
} }
impl PacketEncode for Text { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.encode_write(to_json_string(self).unwrap())
} }
impl PacketDecode for Text { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    from_json_str(&buf.read_decode::<String>()?).map_err(|_| DecodeError::InvalidData("Text data is not valid JSON".to_string()))
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

    #[test]
    fn nbt_serialise_colour() {
        assert_eq!(
            TextColour::Red.to_nbt(),
            NbtElement::String("red".to_string())
        );
        assert_eq!(
            TextColour::RGB(255, 127, 0).to_nbt(),
            NbtElement::String("#ff7f00".to_string())
        );
    }

}
