use super::*;


#[packet( prefix = 0x00, bound = C2S, stage = Login )]
pub struct LoginStartC2SPacket {
    pub username : String,
    pub uuid     : Uuid
}


#[packet( prefix = 0x01, bound = C2S, stage = Login )]
pub struct EncryptionResponseC2SPacket {
    /// Encrypted using the server's public key.
    pub secret_key : LengthPrefixVec<VarInt, u8>,
    /// Encrypted using the server's public key.
    pub verify_token : LengthPrefixVec<VarInt, u8>
}


#[packet( prefix = 0x03, bound = C2S, stage = Login )]
pub struct LoginAcknowledgeC2SPacket;


packet_full_decode!( bound = C2S, stage = Login );
