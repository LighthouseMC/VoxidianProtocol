use super::*;


#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatusResponse {
    pub version : StatusResponseVersion,
    pub players : StatusResponsePlayers,
    #[serde(rename = "description")]
    pub desc : Text,
    #[serde(rename = "favicon", serialize_with = "add_png_b64_header", deserialize_with = "remove_png_b64_header")]
    pub favicon_png_b64 : String,
    #[serde(rename = "enforcesSecureChat")]
    pub enforce_chat_reports : bool,
    /// Used by No Chat Reports
    #[serde(rename = "preventsChatReports", skip_serializing_if = "is_false", default)]
    pub prevent_chat_reports : bool
}
fn is_false(value : &bool) -> bool { !value }
fn add_png_b64_header<S : Serer>(png_b64 : &str, ser : S) -> Result<S::Ok, S::Error> {
    ser.serialize_str(&format!("data:image/png;base64,{}", png_b64))
}
fn remove_png_b64_header<'l, D : Deserer<'l>>(deser : D) -> Result<String, D::Error> {
    let with_header = String::deserialize(deser)?;
    let without_header = with_header.strip_prefix("data:image/png;base64,").ok_or(serde::de::Error::custom("Not a hex colour code"))?;
    Ok(without_header.to_string())
}
impl StatusResponse {
    pub fn to_packet(&self) -> StatusResponseS2CPacket {
        StatusResponseS2CPacket(to_json_string(self).unwrap())
    }
}

#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatusResponseVersion {
    pub name     : String,
    pub protocol : usize
}

#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatusResponsePlayers {
    pub online : usize,
    pub max    : usize,
    pub sample : Vec<StatusResponsePlayerSample>
}

#[derive(Ser, Deser, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatusResponsePlayerSample {
    pub name : String,
    #[serde(rename = "id")]
    pub uuid : Uuid
}

#[packet( prefix = 0x00, bound = S2C, stage = Status, allow_priv = true )]
pub struct StatusResponseS2CPacket(String);
impl StatusResponseS2CPacket {
    pub fn to_response(&self) -> Result<StatusResponse, DecodeError> {
        from_json_str(&self.0).map_err(|_| DecodeError::InvalidData("Status response is not valid JSON".to_string()) )
    }
}


#[packet( prefix = 0x01, bound = S2C, stage = Status )]
pub struct PongResponseS2CPacket {
    pub timestamp : u64
}



packet_full_decode!{ StatusS2CPackets }



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn encode_decode_status_response() {
        let response_to_encode = StatusResponse {
            version : StatusResponseVersion {
                name     : "1.21.4".to_string(),
                protocol : 769
            },
            players : StatusResponsePlayers {
                online : 3,
                max    : 20,
                sample : vec![ StatusResponsePlayerSample {
                    name : "TotobirdCreation".to_string(),
                    uuid : uuid::uuid!("bd9e79ad-1065-4045-8b08-87346cff42a7")
                } ]
            },
            desc : Text::from(vec![ TextComponent::of_literal("Hello!") ]),
            favicon_png_b64 : "favicon_base64_string".to_string(),
            enforce_chat_reports : false,
            prevent_chat_reports : false
        };
        let packet = response_to_encode.to_packet();
        assert_eq!(packet.0, "{\"version\":{\"name\":\"1.21.4\",\"protocol\":769},\"players\":{\"online\":3,\"max\":20,\"sample\":[{\"name\":\"TotobirdCreation\",\"id\":\"bd9e79ad-1065-4045-8b08-87346cff42a7\"}]},\"description\":[{\"text\":\"Hello!\"}],\"favicon\":\"data:image/png;base64,favicon_base64_string\",\"enforcesSecureChat\":false}");
        let decoded_response = packet.to_response().unwrap();
        assert_eq!(response_to_encode, decoded_response);
    }

}
