use super::*;


#[packet( prefix = 0x00, bound = S2C, stage = Play )]
pub struct BundleDelimiterS2CPacket;


#[packet( prefix = 0x01, bound = S2C, stage = Play )]
pub struct SpawnEntityS2CPacket {
    pub id       : VarInt,
    pub uuid     : Uuid,
    pub kind     : VarInt,
    pub x        : f64,
    pub y        : f64,
    pub z        : f64,
    pub pitch    : f64,
    pub yaw      : f64,
    pub head_yaw : f64,
    pub data     : VarInt,
    pub vel_x    : f64,
    pub vel_y    : f64,
    pub vel_z    : f64
}


#[packet( prefix = 0x02, bound = S2C, stage = Play )]
pub struct SpawnXpOrbS2CPacket {
    pub id    : VarInt,
    pub x     : f64,
    pub y     : f64,
    pub z     : f64,
    pub count : i16
}


#[packet( prefix = 0x03, bound = S2C, stage = Play )]
pub struct EntityAnimationS2CPacket {
    pub id   : VarInt,
    pub anim : EntityAnimation
}
#[packet_part(u8)]
pub enum EntityAnimation {
    SwingMainHand   = 0,
    LeaveBed        = 2,
    SwingOffHand    = 3,
    CritEffect      = 4,
    MagicCritEffect = 5
}


#[packet( prefix = 0x04, bound = S2C, stage = Play )]
pub struct AwardStatS2CPacket {
    pub stats : LengthPrefixVec<VarInt, Stat>
}
#[packet_part]
pub struct Stat {
    pub category : StatCategory,
    pub kind     : VarInt,
    pub value    : VarInt
}
#[packet_part(VarInt)]
pub enum StatCategory {
    Mined    = 0,
    Crafted  = 1,
    Used     = 2,
    Broken   = 3,
    PickedUp = 4,
    Dropped  = 5,
    Killed   = 6,
    KilledBy = 7,
    Custom   = 8
}


#[packet( prefix = 0x05, bound = S2C, stage = Play )]
pub struct AcknowledgeBlockChangeS2CPacket(pub VarInt);


#[packet( prefix = 0x06, bound = S2C, stage = Play )]
pub struct SetBlockDestroyStageS2CPacket {
    pub entity_id : VarInt,
    pub pos       : BlockPos,
    pub stage     : u8
}


#[packet( prefix = 0x07, bound = S2C, stage = Play )]
pub struct BlockEntityDataS2CPacket {
    pub pos  : BlockPos,
    pub kind : VarInt,
    pub data : Nbt
}


#[packet( prefix = 0x08, bound = S2C, stage = Play )]
pub struct BlockActionS2CPacket {
    pub pos    : BlockPos,
    pub action : u8,
    pub param  : u8,
    pub block  : RegEntry<Block>
}


#[packet( prefix = 0x09, bound = S2C, stage = Play )]
pub struct BlockUpdateS2CPacket {
    pub pos   : BlockPos,
    pub block : RegEntry<BlockState>
}


#[packet( prefix = 0x0A, bound = S2C, stage = Play )]
pub struct BossBarS2CPacket {
    pub uuid   : Uuid,
    pub action : BossBarAction
}
#[packet_part(VarInt)]
pub enum BossBarAction {
    Add {
        title    : Text,
        health   : f32,
        colour   : BossBarColour,
        division : BossBarDivision,
        flags    : u8
    },
    Remove,
    UpdateHealth {
        health : f32
    },
    UpdateTitle {
        title : Text
    },
    UpdateStyle {
        colour   : BossBarColour,
        division : BossBarDivision
    },
    UpdateFlags {
        flags : u8
    }
}
#[packet_part(VarInt)]
pub enum BossBarColour {
    Pink   = 0,
    Blue   = 1,
    Red    = 2,
    Green  = 3,
    Yellow = 4,
    Purple = 5,
    White  = 6
}
#[packet_part(VarInt)]
pub enum BossBarDivision {
    None    = 0,
    Notch6  = 1,
    Notch10 = 2,
    Notch12 = 3,
    Notch20 = 4
}


#[packet( prefix = 0x0B, bound = S2C, stage = Play )]
pub struct ChangeDifficultyS2CPacket {
    pub difficulty : Difficulty,
    pub locked     : bool
}
#[packet_part(u8)]
pub enum Difficulty {
    Peaceful = 0,
    Easy     = 1,
    Normal   = 2,
    Hard     = 3
}


#[packet( prefix = 0x0C, bound = S2C, stage = Play )]
pub struct ChunkBatchFinishS2CPacket {
    pub size : VarInt
}


#[packet( prefix = 0x0D, bound = S2C, stage = Play )]
pub struct ChunkBatchStartS2CPacket;


#[packet( prefix = 0x0E, bound = S2C, stage = Play )]
pub struct ChunkBiomesS2CPacket {
    pub chunks : LengthPrefixVec<VarInt, ChunkBiomeData>
}
#[packet_part]
pub struct ChunkBiomeData {
    pub x    : i32,
    pub z    : i32,
    pub data : LengthPrefixVec<VarInt, u8>
}


#[packet( prefix = 0x0F, bound = S2C, stage = Play )]
pub struct ClearTitlesS2CPacket {
    pub reset : bool
}


#[packet( prefix = 0x10, bound = S2C, stage = Play )]
pub struct CommandSuggestionsResponseS2CPacket {
    pub id          : VarInt,
    pub start       : VarInt,
    pub len         : VarInt,
    pub suggestions : LengthPrefixVec<VarInt, CommandSuggestion>
}
#[packet_part]
pub struct CommandSuggestion {
    pub matched : String,
    pub tooltip : Option<Text>
}


// TODO: CommandsS2CPacket


#[packet( prefix = 0x12, bound = S2C, stage = Play )]
pub struct CloseContainerS2CPacket {
    pub window : u8
}


// TODO: SetContainerContentS2CPacket


#[packet( prefix = 0x14, bound = S2C, stage = Play )]
pub struct SetContainerPropertyS2CPacket {
    pub window : u8,
    pub prop   : u16,
    pub value  : u16
}


// TODO: SetContainerSlotS2CPacket


#[packet( prefix = 0x16, bound = S2C, stage = Play )]
pub struct CookieRequestS2CPacket {
    pub key : Identifier
}


#[packet( prefix = 0x17, bound = S2C, stage = Play )]
pub struct SetCooldownS2CPacket {
    pub item  : RegEntry<Item>,
    pub ticks : VarInt
}


#[packet( prefix = 0x18, bound = S2C, stage = Play )]
pub struct ChatSuggestionsS2CPacket {
    pub action : ChatSuggestionAction
}
#[packet_part(VarInt)]
pub enum ChatSuggestionAction {
    Add    = 0,
    Remove = 1,
    Set    = 2
}


#[packet( prefix = 0x19, bound = S2C, stage = Play )]
pub struct PluginMessageS2CPacket {
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


// TODO: DamageEventS2CPacket


packet_full_decode!( bound = S2C, stage = Play );
