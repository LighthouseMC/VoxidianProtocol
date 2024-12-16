use super::*;


#[packet( prefix = 0x00, bound = C2S, stage = Login )]
pub struct LoginStartC2SPacket {
    /// According to the client.
    pub username : String,
    /// According to the client.
    pub uuid     : Uuid
}


#[packet( prefix = 0x01, bound = C2S, stage = Login )]
pub struct EncryptionResponseC2SPacket {
    /// Encrypted using the server's public key.
    pub secret_key : LengthPrefixVec<VarInt, u8>,
    /// Encrypted using the server's public key.
    pub verify_token : LengthPrefixVec<VarInt, u8>
}


#[packet( prefix = 0x02, bound = C2S, stage = Login )]
pub struct LoginPluginResponseC2SPacket {
    pub msg_id : VarInt,
    pub data   : Option<ConsumeAllVec<u8>>
}


#[packet( prefix = 0x03, bound = C2S, stage = Login )]
pub struct LoginAcknowledgeC2SPacket;


#[packet( prefix = 0x04, bound = C2S, stage = Login )]
pub struct CookieResponseC2SPacket {
    pub key     : Identifier,
    pub payload : Option<LengthPrefixVec<VarInt, u8>>
}


packet_full_decode!{ LoginC2SPackets }
