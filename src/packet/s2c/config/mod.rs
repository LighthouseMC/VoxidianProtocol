use super::*;


#[packet( "minecraft:s2c/config/cookie_request" )]
pub struct CookieRequestS2CConfigPacket {
    pub key : Identifier
}


#[packet( "minecraft:s2c/config/custom_payload" )]
pub struct CustomPayloadS2CConfigPacket {
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( "minecraft:s2c/config/disconnect" )]
pub struct DisconnectS2CConfigPacket {
    pub reason : NbtText
}


#[packet( "minecraft:s2c/config/finish_configuration" )]
pub struct FinishConfigurationS2CConfigPacket;


#[packet( "minecraft:s2c/config/keep_alive" )]
pub struct KeepAliveS2CConfigPacket(pub u64);


#[packet( "minecraft:s2c/config/ping" )]
pub struct PingS2CConfigPacket(pub u32);


#[packet( "minecraft:s2c/config/reset_chat" )]
pub struct ResetChatS2CConfigPacket;


#[packet( "minecraft:s2c/config/registry_data" )]
pub struct RegistryDataS2CConfigPacket {
    pub registry : Identifier,
    pub entries  : LengthPrefixVec<VarInt, RegistryDataEntry>
}
#[packet_part]
pub struct RegistryDataEntry {
    pub id   : Identifier,
    pub data : Option<Nbt>
}


#[packet( "minecraft:s2c/config/resource_pack_pop" )]
pub struct ResourcePackPopS2CConfigPacket {
    pub uuid : Option<Uuid>
}


#[packet( "minecraft:s2c/config/resource_pack_push" )]
pub struct ResourcePackPushS2CConfigPacket {
    pub uuid   : Uuid,
    pub url    : String,
    pub hash   : String,
    pub forced : bool,
    pub prompt : Option<Text>
}


#[packet( "minecraft:s2c/config/store_cookie" )]
pub struct StoreCookieS2CConfigPacket {
    pub key     : Identifier,
    pub payload : LengthPrefixVec<VarInt, u8>
}


#[packet( "minecraft:s2c/config/transfer" )]
pub struct TransferS2CConfigPacket {
    pub host : String,
    pub port : VarInt
}


#[packet( "minecraft:s2c/config/update_enabled_features" )]
pub struct UpdateEnabledFeaturesS2CConfigPacket {
    pub features : LengthPrefixVec<VarInt, Identifier>
}


#[packet( "minecraft:s2c/config/update_tags" )]
pub struct UpdateTagsS2CConfigPacket(TODO);


#[packet( "minecraft:s2c/config/select_known_packs" )]
pub struct SelectKnownPacksS2CConfigPacket {
    pub known_packs : LengthPrefixVec<VarInt, KnownPack>
}
#[packet_part]
pub struct KnownPack {
    pub namespace : String,
    pub id        : String,
    pub version   : String
}


#[packet( "minecraft:s2c/config/custom_report_details" )]
pub struct CustomReportDetailsS2CConfigPacket {
    pub details : LengthPrefixVec<VarInt, ReportDetail>
}
#[packet_part]
pub struct ReportDetail {
    pub title : String,
    pub desc  : String
}


#[packet( "minecraft:s2c/config/server_links" )]
pub struct ServerLinksS2CConfigPacket {
    pub links : LengthPrefixVec<VarInt, ServerLink>
}
#[packet_part]
pub struct ServerLink {
    pub kind : Either<BuiltInServerLink, Text>
}
#[packet_part(VarInt)]
pub enum BuiltInServerLink {
    BugReport           = 0,
    CommunityGuidelines = 1,
    Support             = 2,
    Status              = 3,
    Feedback            = 4,
    Community           = 5,
    Website             = 6,
    Forums              = 7,
    News                = 8,
    Announcements       = 9
}


packet_full_decode!{ S2C Config }
