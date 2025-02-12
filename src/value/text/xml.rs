use super::*;


impl Text {

    #[doc(cfg(feature = "text_xml"))]
    pub fn from_xml<S : AsRef<str>>(xml : S) -> Self {
        let reader = XmlReader::new(xml.as_ref());
        let mut components = Vec::new();
        Self::from_xml_inner(&mut reader.into_iter(), &mut components,
            None, None, None, None, None, None, None, None, None
        );
        Text::from(components)
    }

    fn from_xml_inner(
        reader        : &mut XmlReader,
        components    : &mut Vec<TextComponent>,
        current_tag   : Option<&str>,
        colour        : Option<TextColour>,
        bold          : Option<bool>,
        italic        : Option<bool>,
        underline     : Option<bool>,
        strikethrough : Option<bool>,
        obfuscate     : Option<bool>,
        insertion     : Option<&str>,
        font          : Option<Identifier>
    ) {
        let mut current_content = String::new();

        macro push_content($method:ident, $content:expr) { {
            let content = $content;
            if (! content.is_empty()) {
                let mut text = TextComponent::$method(content);
                if let Some(colour        ) = colour        { text = text.colour        (colour                ); }
                if let Some(bold          ) = bold          { text = text.bold          (bold                  ); }
                if let Some(italic        ) = italic        { text = text.italic        (italic                ); }
                if let Some(underline     ) = underline     { text = text.underline     (underline             ); }
                if let Some(strikethrough ) = strikethrough { text = text.strikethrough (strikethrough         ); }
                if let Some(obfuscate     ) = obfuscate     { text = text.obfuscate     (obfuscate             ); }
                if let Some(insertion     ) = insertion     { text = text.insertion     (insertion.to_string() ); }
                if let Some(font          ) = font.as_ref() { text = text.font          (font.clone()          ); }
                let mut text1 = TextComponent::of_literal("");
                text1.extra.push(text);
                components.push(text1);
                current_content.clear();
            }
        } }

        loop { match (reader.next()) {
            Some(event) => { match (event) {
                XmlEvent::TagOpen(name) => {
                    push_content!(of_literal, &current_content);
                    let name = format!("{}", name);
                    let mut colour        = colour;
                    let mut bold          = bold;
                    let mut italic        = italic;
                    let mut underline     = underline;
                    let mut strikethrough = strikethrough;
                    let mut obfuscate     = obfuscate;
                    let mut insertion     = insertion;
                    let mut font          = font.clone();
                    if let Some(value) = name.strip_prefix("colour:") {
                        if let Ok(value) = TextColour::from_name(value) { colour = Some(value); }
                    } else if let Some(value) = name.strip_prefix("color:") {
                        if let Ok(value) = TextColour::from_name(value) { colour = Some(value); }
                    } else if (name == "!colour" || name == "colour") {
                        colour = None;
                    }
                    // TODO: shadow, !shadow
                    else if (name == "bold" || name == "b" || name == "strong") {
                        bold = Some(true);
                    } else if (name == "!bold" || name == "!b" || name == "!strong") {
                        bold = Some(false);
                    } else if (name == "italic" || name == "i" || name == "em" || name == "emphasis" || name == "emphasise" || name == "emphasize") {
                        italic = Some(true);
                    } else if (name == "!italic" || name == "!i" || name == "!em" || name == "!emphasis" || name == "!emphasise" || name == "!emphasize") {
                        italic = Some(false);
                    } else if (name == "underline" || name == "u" || name == "ul" || name == "underlined" || name == "under") {
                        underline = Some(true);
                    } else if (name == "!underline" || name == "!u" || name == "!ul" || name == "!underlined" || name == "!under") {
                        underline = Some(false);
                    } else if (name == "strikethrough" || name == "st" || name == "strike") {
                        strikethrough = Some(true);
                    } else if (name == "!strikethrough" || name == "!st" || name == "!strike") {
                        strikethrough = Some(false);
                    } else if (name == "obfuscate" || name == "obf" || name == "obfuscated") {
                        obfuscate = Some(true);
                    } else if (name == "!obfuscate" || name == "!obf" || name == "!obfuscated") {
                        obfuscate = Some(false);
                    } else if (name == "reset") {
                        colour        = None;
                        bold          = None;
                        italic        = None;
                        underline     = None;
                        strikethrough = None;
                        obfuscate     = None;
                        insertion     = None;
                        font          = None;
                    }
                    // TODO: click, click_event, !click, !click_event
                    // TODO: hover, hover_event, !hover, !hover_event
                    else if let Some(value) = name.strip_prefix("key:") {
                        push_content!(of_keybind, value);
                        continue;
                    } else if let Some(value) = name.strip_prefix("keybind:") {
                        push_content!(of_keybind, value);
                        continue;
                    } else if let Some(value) = name.strip_prefix("keybinding:") {
                        push_content!(of_keybind, value);
                        continue;
                    } else if let Some(value) = name.strip_prefix("lang:") {
                        push_content!(of_translate, value);
                        continue;
                    } else if let Some(value) = name.strip_prefix("translate:") {
                        push_content!(of_translate, value);
                        continue;
                    } else if let Some(value) = name.strip_prefix("translation:") {
                        push_content!(of_translate, value);
                        continue;
                    } else if let Some(value) = name.strip_prefix("translatable:") {
                        push_content!(of_translate, value);
                        continue;
                    }
                    // TODO: lang_or, translate_or, translation_or, translatable_or
                    else if let Some(value) = name.strip_prefix("insert:") {
                        insertion = Some(value);
                    } else if let Some(value) = name.strip_prefix("insertion:") {
                        insertion = Some(value);
                    } else if let Some(value) = name.strip_prefix("font:") {
                        font = Some(Identifier::from(value));
                    } else if (name == "newline" || name == "nl") {
                        push_content!(of_literal, "\n");
                        continue;
                    } else {
                        if let Ok(value) = TextColour::from_name(&name) { colour = Some(value); }
                    }
                    Self::from_xml_inner(reader, components,
                        Some(name.split(":").next().unwrap()),
                        colour,
                        bold,
                        italic,
                        underline,
                        strikethrough,
                        obfuscate,
                        insertion,
                        font
                    );
                },
                XmlEvent::TagClose(name) => {
                    let name = format!("{}", name);
                    if (name.is_empty()) {
                        break;
                    }
                    if let Some(current_tag) = current_tag && (name == current_tag) {
                        break;
                    }
                },
                XmlEvent::Data(content) => {
                    current_content += &content;
                },
            } },
            None => { break; }
        } }
        push_content!(of_literal, &current_content);
    }

}


pub struct XmlReader<'l> {
    xml : &'l str,
    i   : usize
}
impl<'l> XmlReader<'l> {
    pub fn new(xml : &'l str) -> Self {
        Self { xml, i : 0 }
    }
}
impl<'l> Iterator for XmlReader<'l> {
    type Item = XmlEvent;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.xml.chars().nth(self.i)) {
            None => None,

            Some('<') => {
                self.i += 1;
                let closing = match (self.xml.chars().nth(self.i)) {
                    Some(ch) => { ch == '/' }
                    None => { return None }
                };
                if (closing) {
                    self.i += 1;
                }
                let mut depth = 1usize;
                let mut esc = false;
                let mut name = String::new();
                loop {
                    self.i += 1;
                    if (esc) {
                        match (self.xml.chars().nth(self.i - 1)) {
                            Some(ch) => { name.push(ch); }
                            None => { return None }
                        }
                        esc = false;
                    } else {
                        match (self.xml.chars().nth(self.i - 1)) {
                            Some('\\') => { esc = true; },
                            Some('<') => {
                                depth += 1;
                                name.push('<');
                            },
                            Some('>') => {
                                depth -= 1;
                                if (depth == 0) {
                                    self.i -= 1;
                                    break;
                                }
                                name.push('>');
                            },
                            Some(ch) => { name.push(ch); }
                            None => { return None }
                        }
                    }
                }
                self.i += 1;
                let name = name.trim().to_string();
                Some(if (closing) { XmlEvent::TagClose(name) } else { XmlEvent::TagOpen(name) })
            },

            Some(_) => {
                let mut esc = false;
                let mut name = String::new();
                loop {
                    self.i += 1;
                    if (esc) {
                        match (self.xml.chars().nth(self.i - 1)) {
                            Some(ch) => { name.push(ch); }
                            None => { break }
                        }
                        esc = false;
                    } else {
                        match (self.xml.chars().nth(self.i - 1)) {
                            Some('\\') => { esc = true; },
                            Some('<') => {
                                self.i -= 1;
                                break
                            },
                            Some(ch) => { name.push(ch); }
                            None => { break }
                        }
                    }
                }
                Some(XmlEvent::Data(name))
            }

        }
    }
}


#[derive(Debug)]
pub enum XmlEvent {
    TagOpen(String),
    TagClose(String),
    Data(String)
}
