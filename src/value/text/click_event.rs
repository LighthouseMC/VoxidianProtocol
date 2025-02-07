use super::*;


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
        page : usize
    },
    #[serde(rename = "copy_to_clipboard")]
    SetClipboard {
        #[serde(rename = "value")]
        text : String
    }
}
fn to_string_to_string<T : ToString, S : Serer>(value : &T, ser : S) -> Result<S::Ok, S::Error> {
    ser.serialize_str(&value.to_string())
}
impl TextClickEvent {
    pub(super) fn write_nbt(&self, buf : &mut NbtCompound) {
        let (action, value) = match (self) {
            Self::OpenURL        { url     } => ("open_url"          , url     .to_string()),
            Self::RunCommand     { command } => ("run_command"       , command .to_string()),
            Self::SuggestCommand { command } => ("suggest_command"   , command .to_string()),
            Self::SetBookPage    { page    } => ("change_page"       , page    .to_string()),
            Self::SetClipboard   { text    } => ("copy_to_clipboard" , text    .to_string()),
        };
        buf.insert("action", NbtElement::String(action.to_string()));
        buf.insert("value", NbtElement::String(value));
    }
}
