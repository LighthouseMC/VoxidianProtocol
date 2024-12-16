use super::*;


#[packet( prefix = 0x00, bound = S2C, stage = Login )]
pub struct DisconnectS2CPacket {
    pub reason : Text
}


#[packet( prefix = 0x01, bound = S2C, stage = Login )]
pub struct EncryptionRequestS2CPacket {
    pub server_id    : String,
    #[redacted]
    pub public_key   : LengthPrefixVec<VarInt, u8>,
    #[redacted]
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
    pub props    : LengthPrefixHashMap<VarInt, String, LoginSuccessProperty>
}
#[packet_part]
pub struct LoginSuccessProperty {
    pub value : String,
    pub sig   : Option<String>
}


#[packet( prefix = 0x04, bound = S2C, stage = Login )]
pub struct LoginPluginRequestS2CPacket {
    pub msg_id  : VarInt,
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( prefix = 0x05, bound = S2C, stage = Login )]
pub struct CookieRequestS2CPacket {
    pub key : Identifier
}


packet_full_decode!{ LoginS2CPackets }


#[cfg(test)]
mod tests {
    use super::*;

    #[packet( prefix = 0x98, bound = S2C, stage = Login )]
    pub struct TestUnnamedS2CPacket(
        pub LengthPrefixVec<VarInt, u8>,
        #[redacted]
        pub u8
    );

    #[packet( prefix = 0x99, bound = S2C, stage = Login )]
    pub struct TestUnitS2CPacket;

    #[test]
    fn packet_field_redact_named() {
        let packet = EncryptionRequestS2CPacket {
            server_id    : "Hello!".to_string(),
            public_key   : vec![ 0, 1, 2, 3, 4, 5 ].into(),
            verify_token : vec![ 6, 7, 8, 9 ].into(),
            should_auth  : false
        };
        assert_eq!(
            format!("{:?}", packet),
            "EncryptionRequestS2CPacket { server_id: \"Hello!\", public_key: <REDACTED>, verify_token: <REDACTED>, should_auth: false }"
        )
    }

    #[test]
    fn packet_field_redact_unnamed() {
        let packet = TestUnnamedS2CPacket( vec![ 0, 1, 2, 3 ].into(), 28 );
        assert_eq!(
            format!("{:?}", packet),
            "TestUnnamedS2CPacket(LengthPrefixVec[0, 1, 2, 3], <REDACTED>)"
        )
    }

    #[test]
    fn packet_field_redact_unit() {
        let packet = TestUnitS2CPacket;
        assert_eq!(
            format!("{:?}", packet),
            "TestUnitS2CPacket"
        )
    }

}
