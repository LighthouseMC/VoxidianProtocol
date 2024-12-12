use super::*;


#[packet( prefix = 0x00, bound = S2C, stage = Login )]
pub struct DisconnectS2CPacket {
    pub reason : Text
}


#[packet( prefix = 0x01, bound = S2C, stage = Login )]
pub struct EncryptionRequestS2CPacket {
    pub server_id    : String,
    pub public_key   : LengthPrefixVec<VarInt, u8>,
    pub verify_token : LengthPrefixVec<VarInt, u8>,
    pub should_auth  : bool
}


#[packet( prefix = 0x03, bound = S2C, stage = Login )]
pub struct SetCompressionS2CPacket {
    pub threshold : VarInt
}


#[packet( prefix = 0x02, bound = S2C, stage = Login )]
pub struct LoginSuccessS2CPacket {
    pub uuid     : Uuid,
    pub username : String,
    pub props    : LengthPrefixVec<VarInt, LoginSuccessProperty>
}
#[packet_part]
pub struct LoginSuccessProperty {
    pub name      : String,
    pub value     : String,
    pub signature : Option<String>
}


#[packet( prefix = 0x04, bound = S@C, stage = Login )]
pub struct LoginPluginRequestS2CPacket {
    pub msg_id  : VarInt,
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( prefix = 0x05, bound = S2C, stage = Login )]
pub struct CookieRequestS2CPacket {
    pub key : Identifier
}


packet_full_decode!( bound = S2C, stage = Login );
