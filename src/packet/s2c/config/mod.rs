use super::*;


pub trait TryIntoS2CConfigPackets : Sized {
    fn try_into_s2c_config(self) -> Result<S2CConfigPackets, Self>;
}


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
    pub entries  : LengthPrefixVec<Var32, (Identifier, Option<Nbt>)>
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
    pub prompt : Option<JsonText>
}


#[packet( "minecraft:s2c/config/store_cookie" )]
pub struct StoreCookieS2CConfigPacket {
    pub key     : Identifier,
    pub payload : LengthPrefixVec<Var32, u8>
}


#[packet( "minecraft:s2c/config/transfer" )]
pub struct TransferS2CConfigPacket {
    pub host : String,
    pub port : Var32
}


#[packet( "minecraft:s2c/config/update_enabled_features" )]
pub struct UpdateEnabledFeaturesS2CConfigPacket {
    pub features : LengthPrefixVec<Var32, Identifier>
}


#[packet( "minecraft:s2c/config/update_tags" )]
pub struct UpdateTagsS2CConfigPacket {
    registry: Identifier,
    tag_info: TagInformation
}


#[packet( "minecraft:s2c/config/select_known_packs" )]
pub struct SelectKnownPacksS2CConfigPacket {
    pub known_packs : LengthPrefixVec<Var32, KnownPack>
}
impl Default for SelectKnownPacksS2CConfigPacket { fn default() -> Self { Self {
    known_packs : vec![
        KnownPack {
            namespace : Cow::Borrowed("minecraft"),
            id        : Cow::Borrowed("core"),
            version   : Cow::Borrowed(MINECRAFT_VERSION)
        },
        KnownPack {
            namespace : Cow::Borrowed("minecraft"),
            id        : Cow::Borrowed("vanilla"),
            version   : Cow::Borrowed(MINECRAFT_VERSION)
        }
    ].into()
} } }
#[packet_part]
#[derive(PartialEq)]
pub struct KnownPack {
    pub namespace : Cow<'static, str>,
    pub id        : Cow<'static, str>,
    pub version   : Cow<'static, str>
}


#[packet( "minecraft:s2c/config/custom_report_details" )]
pub struct CustomReportDetailsS2CConfigPacket {
    pub details : LengthPrefixVec<Var32, ReportDetail>
}
#[packet_part]
pub struct ReportDetail {
    pub title : String,
    pub desc  : String
}


#[packet( "minecraft:s2c/config/server_links" )]
pub struct ServerLinksS2CConfigPacket {
    pub links : LengthPrefixVec<Var32, ServerLink>
}
#[packet_part]
pub struct ServerLink {
    pub kind : Either<BuiltInServerLink, JsonText>
}
#[packet_part(Var32)]
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
