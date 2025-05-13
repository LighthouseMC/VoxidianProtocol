use super::*;


pub trait TryIntoS2CLoginPackets : Sized {
    fn try_into_s2c_login(self) -> Result<S2CLoginPackets, Self>;
}


#[packet( "minecraft:s2c/login/login_disconnect" )]
pub struct LoginDisconnectS2CLoginPacket {
    pub reason : JsonText
}


#[packet( "minecraft:s2c/login/hello" )]
pub struct HelloS2CLoginPacket {
    pub server_id    : String,
    #[redacted]
    pub public_key   : LengthPrefixVec<Var32, u8>,
    #[redacted]
    pub verify_token : LengthPrefixVec<Var32, u8>,
    pub should_auth  : bool
}


#[packet( "minecraft:s2c/login/login_finished" )]
pub struct LoginFinishedS2CLoginPacket {
    pub uuid     : Uuid,
    pub username : String,
    pub props    : LengthPrefixHashMap<Var32, String, LoginSuccessProperty>
}
#[packet_part]
pub struct LoginSuccessProperty {
    pub value : String,
    pub sig   : Option<String>
}


#[packet( "minecraft:s2c/login/login_compression" )]
pub struct LoginCompressionS2CLoginPacket {
    pub threshold : Var32
}


#[packet( "minecraft:s2c/login/custom_query" )]
pub struct CustomQueryS2CLoginPacket {
    pub msg_id  : Var32,
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( "minecraft:s2c/login/cookie_request" )]
pub struct CookieRequestS2CLoginPacket {
    pub key : Identifier
}


packet_full_decode!{ S2C Login }


#[cfg(test)]
mod tests {
    use super::*;

    #[packet( "minecraft:s2c/status/status_response" )]
    pub struct StatusResponseS2CStatusPacket(
        pub LengthPrefixVec<Var32, u8>,
        #[redacted]
        pub u8
    );

    #[packet( "minecraft:c2s/status/status_request" )]
    pub struct StatusRequestC2SStatusPacket;

    #[test]
    fn packet_field_redact_named() {
        let packet = HelloS2CLoginPacket {
            server_id    : "Hello!".to_string(),
            public_key   : vec![ 0, 1, 2, 3, 4, 5 ].into(),
            verify_token : vec![ 6, 7, 8, 9 ].into(),
            should_auth  : false
        };
        assert_eq!(
            format!("{:?}", packet),
            "HelloS2CLoginPacket { server_id: \"Hello!\", public_key: <REDACTED>, verify_token: <REDACTED>, should_auth: false }"
        )
    }

    #[test]
    fn packet_field_redact_unnamed() {
        let packet = StatusResponseS2CStatusPacket( vec![ 0, 1, 2, 3 ].into(), 28 );
        assert_eq!(
            format!("{:?}", packet),
            "StatusResponseS2CStatusPacket(LengthPrefixVec[0, 1, 2, 3], <REDACTED>)"
        )
    }

    #[test]
    fn packet_field_redact_unit() {
        let packet = StatusRequestC2SStatusPacket;
        assert_eq!(
            format!("{:?}", packet),
            "StatusRequestC2SStatusPacket"
        )
    }

}
