use super::*;
use s2c::config::KnownPack;


pub trait TryIntoC2SConfigPackets {
    fn try_into_c2s_config(self) -> Option<C2SConfigPackets>;
}


#[packet( "minecraft:c2s/config/client_information" )]
pub struct ClientInformationC2SConfigPacket {
    pub info : ClientInfo
}
#[packet_part]
pub struct ClientInfo {
    pub locale         : String,
    pub view_distance  : u8,
    pub chat_mode      : ClientChatMode,
    pub chat_colours   : bool,
    pub skin_layers    : SkinLayers,
    pub main_hand      : ClientMainHand,
    pub text_filtering : bool,
    pub server_listing : bool
}
impl Default for ClientInfo { fn default() -> Self {
    Self {
        locale         : "en_us".to_string(),
        view_distance  : 12,
        chat_mode      : ClientChatMode::Enabled,
        chat_colours   : true,
        skin_layers    : SkinLayers {
            cape         : true,
            jacket       : true,
            left_sleeve  : true,
            right_sleeve : true,
            left_leg     : true,
            right_leg    : true,
            hat          : true
        },
        main_hand      : ClientMainHand::Right,
        text_filtering : false,
        server_listing : true
    }
} }
#[packet_part(Var32)]
pub enum ClientChatMode {
    Enabled      = 0,
    CommandsOnly = 1,
    Hidden       = 2
}
#[packet_part(Var32)]
pub enum ClientMainHand {
    Left  = 0,
    Right = 1
}
packet_flags!{ pub struct SkinLayers<u8> {
    pub cape         : 0b00000001,
    pub jacket       : 0b00000010,
    pub left_sleeve  : 0b00000100,
    pub right_sleeve : 0b00001000,
    pub left_leg     : 0b00010000,
    pub right_leg    : 0b00100000,
    pub hat          : 0b01000000
} }


#[packet( "minecraft:c2s/config/cookie_response" )]
pub struct CookieResponseC2SConfigPacket {
    pub key     : Identifier,
    pub payload : Option<LengthPrefixVec<Var32, u8>>
}


#[packet( "minecraft:c2s/config/custom_payload" )]
pub struct CustomPayloadC2SConfigPacket {
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( "minecraft:c2s/config/finish_configuration" )]
pub struct FinishConfigurationC2SConfigPacket;


#[packet( "minecraft:c2s/config/keep_alive" )]
pub struct KeepAliveC2SConfigPacket(pub u64);


#[packet( "minecraft:c2s/config/pong" )]
pub struct PongC2SConfigPacket(pub u32);


#[packet( "minecraft:c2s/config/resource_pack" )]
pub struct ResourcePackC2SConfigPacket {
    pub uuid   : Uuid,
    pub status : ResourcePackStatus
}
#[packet_part(Var32)]
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


#[packet( "minecraft:c2s/config/select_known_packs" )]
pub struct SelectKnownPacksC2SConfigPacket {
    pub known_packs : LengthPrefixVec<Var32, KnownPack>
}


packet_full_decode!{ C2S Config }
