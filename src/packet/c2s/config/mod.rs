use super::*;


#[packet( prefix = 0x00, bound = C2S, stage = Config )]
pub struct ClientInformationC2SPacket {
    pub locale         : String,
    pub view_distance  : u8,
    pub chat_mode      : ClientChatMode,
    pub chat_colours   : bool,
    pub skin_layers    : u8,
    pub main_hand      : ClientMainHand,
    pub text_filtering : bool,
    pub server_listing : bool
}
#[packet_part(VarInt)]
pub enum ClientChatMode {
    Enabled      = 0,
    CommandsOnly = 1,
    Hidden       = 2
}
#[packet_part(VarInt)]
pub enum ClientMainHand {
    Left  = 0,
    Right = 1
}


#[packet( prefix = 0x01, bound = C2S, stage = Config )]
pub struct CookieResponseC2SPacket {
    pub key     : Identifier,
    pub payload : Option<LengthPrefixVec<VarInt, u8>>
}


#[packet( prefix = 0x02, bound = C2S, stage = Config )]
pub struct PluginMessageC2SPacket {
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( prefix = 0x03, bound = C2S, stage = Config )]
pub struct AcknowledgeFinishConfigC2SPacket;


#[packet( prefix = 0x04, bound = C2S, stage = Config )]
pub struct KeepAliveC2SPacket(u64);


#[packet( prefix = 0x05, bound = C2S, stage = Config )]
pub struct PongC2SPacket(u32);


#[packet( prefix = 0x06, bound = C2S, stage = Config )]
pub struct ResourcePackResponseC2SPacket {
    pub uuid   : Uuid,
    pub result : ResourcePackStatus
}
#[packet_part(VarInt)]
pub enum ResourcePackStatus {
    SuccessfullyDownloaded,
    Declined,
    FailedDownload,
    Accepted,
    Downloaded,
    InvalidURL,
    FailedReload,
    Discarded
}


#[packet( prefix = 0x07, bound = C2S, stage = Config )]
pub struct KnownPacksC2SPacket {
    pub known_packs : LengthPrefixVec<VarInt, s2c::config::KnownPack>
}


packet_full_decode!( bound = C2S, stage = Config );