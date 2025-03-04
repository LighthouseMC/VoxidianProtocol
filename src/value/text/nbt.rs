use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct NbtText(pub(super) NbtElement);
impl NbtText {
    pub fn into_inner(self) -> NbtElement {
        self.0
    }
}
impl PacketEncode for NbtText {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        self.0.encode(buf)
    }
}
impl PacketDecode for NbtText {
    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(Self(NbtElement::decode(buf)?))
    }
}

impl From<NbtText> for Text {
    fn from(value: NbtText) -> Self {
        match value.into_inner() {
            NbtElement::String(_) => Text::new(),
            NbtElement::List(list) => {
                let mut txt = Text::new();
                for element in list {
                    if let NbtElement::Compound(compound) = element {
                        let c = compound_to_text_component(compound);
                        txt.push(c);
                    }
                }
                txt
            }
            NbtElement::Compound(compound) => {
                let mut txt = Text::new();
                let c = compound_to_text_component(compound);
                txt.push(c);
                txt
            }
            e => unreachable!("{:?}", e),
        }
    }
}

fn compound_to_text_component(compound: NbtCompound) -> TextComponent {
    let content = compound_to_content(compound.clone());
    let style = compound_to_style(compound.clone());

    let extra = if let Some(NbtElement::List(extra_list)) = compound.get("extra") {
        extra_list
            .iter()
            .filter_map(|elem| {
                if let NbtElement::Compound(comp) = elem {
                    Some(compound_to_text_component(comp.clone()))
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    TextComponent {
        content,
        style,
        extra,
    }
}

fn compound_to_content(compound: NbtCompound) -> TextContent {
    if let Some(NbtElement::String(text)) = compound.get("text") {
        TextContent::Literal {
            literal: text.clone(),
        }
    } else {
        todo!()
    }
}

fn compound_to_style(compound: NbtCompound) -> TextStyle {
    TextStyle {
        colour: compound.get("color").and_then(|elem| {
            if let NbtElement::String(s) = elem {
                TextColour::from_name(s).ok()
            } else {
                None
            }
        }),
        font: compound.get("font").and_then(|elem| {
            if let NbtElement::String(s) = elem {
                Some(Identifier::from(s))
            } else {
                None
            }
        }),
        bold: compound.get("bold").and_then(|elem| {
            if let NbtElement::Byte(b) = elem {
                Some(*b != 0)
            } else {
                None
            }
        }),
        italic: compound.get("italic").and_then(|elem| {
            if let NbtElement::Byte(b) = elem {
                Some(*b != 0)
            } else {
                None
            }
        }),
        underline: compound.get("underlined").and_then(|elem| {
            if let NbtElement::Byte(b) = elem {
                Some(*b != 0)
            } else {
                None
            }
        }),
        strikethrough: compound.get("strikethrough").and_then(|elem| {
            if let NbtElement::Byte(b) = elem {
                Some(*b != 0)
            } else {
                None
            }
        }),
        obfuscate: compound.get("obfuscated").and_then(|elem| {
            if let NbtElement::Byte(b) = elem {
                Some(*b != 0)
            } else {
                None
            }
        }),
        insertion: None,   // TODO
        click_event: None, // TODO
        hover_event: None, // TODO
    }
}
