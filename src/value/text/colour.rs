use super::*;


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
impl TextColour {
    pub(super) fn to_nbt(&self) -> NbtElement {
        let string = to_json_string(self).unwrap();
        NbtElement::String(string[1..(string.len() - 1)].to_string())
    }
}