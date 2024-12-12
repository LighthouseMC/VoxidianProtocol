use super::*;


#[packet( prefix = 0x00, bound = S2C, stage = Config )]
pub struct CookieRequestS2CPacket {
    pub key : Identifier
}


#[packet( prefix = 0x01, bound = S2C, stage = Config )]
pub struct PluginMessageS2CPacket {
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( prefix = 0x02, bound = S2C, stage = Config )]
pub struct DisconnectS2CPacket {
    pub reason : Text
}


#[packet( prefix = 0x03, bound = S2C, stage = Config )]
pub struct FinishConfigS2CPacket;


#[packet( prefix = 0x04, bound = S2C, stage = Config )]
pub struct KeepAliveS2CPacket(u64);


#[packet( prefix = 0x05, bound = S2C, stage = Config )]
pub struct PingS2CPacket(u32);


#[packet( prefix = 0x06, bound = S2C, stage = Config )]
pub struct ResetChatS2CPacket;


#[packet( prefix = 0x07, bound = S2C, stage = Config )]
pub struct RegistryDataS2CPacket {
    pub registry : Identifier,
    pub entries  : LengthPrefixVec<VarInt, RegistryDataEntry>
}
#[packet_part]
pub struct RegistryDataEntry {
    pub id   : Identifier,
    pub data : Option<NBT>
}


#[packet( prefix = 0x08, bound = S2C, stage = Config )]
pub struct RemoveResourcePackS2CPacket {
    pub uuid : Option<Uuid>
}


#[packet( prefix = 0x09, bound = S2C, stage = Config )]
pub struct AddResourcePackS2CPacket {
    pub uuid   : Uuid,
    pub url    : String,
    pub hash   : String,
    pub forced : bool,
    pub prompt : Option<Text>
}


#[packet( prefix = 0x0A, bound = S2C, stage = Config )]
pub struct StoreCookieS2CPacket {
    pub key     : Identifier,
    pub payload : LengthPrefixVec<VarInt, u8>
}


#[packet( prefix = 0x0B, bound = S2C, stage = Config )]
pub struct TransferS2CPacket {
    pub host : String,
    pub port : VarInt
}


#[packet( prefix = 0x0C, bound = S2C, stage = Config )]
pub struct FeatureFlagsS2CPacket {
    pub features : LengthPrefixVec<VarInt, Identifier>
}


#[packet( prefix = 0x0D, bound = S2C, stage = Config )]
pub struct UpdateTagsS2CPacket {
    pub tags : LengthPrefixVec<VarInt, UpdateTags>
}
#[packet_part]
pub struct UpdateTags {
    pub registry : Identifier,
    pub tag      : LengthPrefixVec<VarInt, UpdateTagsTag>
}
#[packet_part]
pub struct UpdateTagsTag {
    pub name    : Identifier,
    pub entries : LengthPrefixVec<VarInt, VarInt>
}


#[packet( prefix = 0x0E, bound = S2C, stage = Config )]
pub struct KnownPacksS2CPacket {
    pub known_packs : LengthPrefixVec<VarInt, KnownPack>
}
#[packet_part]
pub struct KnownPack {
    pub namespace : String,
    pub id        : String,
    pub version   : String
}


#[packet( prefix = 0x0F, bound = S2C, stage = Config )]
pub struct CustomReportDetailsS2CPacket {
    pub details : LengthPrefixVec<VarInt, CustomReportDetail>
}
#[packet_part]
pub struct CustomReportDetail {
    pub title : String,
    pub desc  : String
}


#[packet( prefix = 0x10, bound = S2C, stage = Config )]
pub struct ServerLinksS2CPacket {
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


packet_full_decode!( bound = S2C, stage = Config );
