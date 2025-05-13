use super::*;


pub trait TryIntoC2SLoginPackets : Sized {
    fn try_into_c2s_login(self) -> Result<C2SLoginPackets, Self>;
}


#[packet( "minecraft:c2s/login/hello" )]
pub struct HelloC2SLoginPacket {
    /// According to the client.
    pub username : String,
    /// According to the client.
    pub uuid     : Uuid
}


#[packet( "minecraft:c2s/login/key" )]
pub struct KeyC2SLoginPacket {
    /// Encrypted using the server's public key.
    #[redacted]
    pub secret_key : LengthPrefixVec<Var32, u8>,
    /// Encrypted using the server's public key.
    #[redacted]
    pub verify_token : LengthPrefixVec<Var32, u8>
}


#[packet( "minecraft:c2s/login/custom_query_answer" )]
pub struct CustomQueryAnswerC2SLoginPacket {
    pub msg_id : Var32,
    pub data   : Option<ConsumeAllVec<u8>>
}


#[packet( "minecraft:c2s/login/login_acknowledged" )]
pub struct LoginAcknowledgedC2SLoginPacket;


#[packet( "minecraft:c2s/login/cookie_response" )]
pub struct CookieResponseC2SLoginPacket {
    pub key     : Identifier,
    pub payload : Option<LengthPrefixVec<Var32, u8>>
}


packet_full_decode!{ C2S Login }
