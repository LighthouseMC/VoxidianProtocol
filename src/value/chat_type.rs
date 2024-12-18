use super::*;


pub struct ChatType {
    pub chat      : ChatDecoration,
    pub narration : ChatDecoration
}
impl RegValue for ChatType {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("chat_type");

    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("chat"      , NbtElement::Compound(self.chat      .to_nbt()));
        nbt.insert("narration" , NbtElement::Compound(self.narration .to_nbt()));
        Some(Nbt { name : String::new(), root : nbt })
    }

}


pub struct ChatDecoration {
    pub translate : String,
    pub style     : Option<TextStyle>,
    pub params    : Vec<ChatDecoParam>
}
impl ChatDecoration { pub fn to_nbt(&self) -> NbtCompound {
    let mut nbt = NbtCompound::new();
    nbt.insert("translation_key", NbtElement::String(self.translate.clone()));
    if let Some(style) = &self.style {
        nbt.insert("style", NbtElement::Compound(style.to_nbt()));
    }
    nbt.insert("parameters", NbtElement::List(self.params.iter().map(|param| {
        NbtElement::String(param.as_str().to_string())
    }).collect::<Vec<_>>()));
    nbt
} }


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ChatDecoParam {
    Sender,
    Target,
    Content
}
impl ChatDecoParam { fn as_str(&self) -> &'static str { match (self) {
    ChatDecoParam::Sender  => "sender",
    ChatDecoParam::Target  => "target",
    ChatDecoParam::Content => "content"
} } }
