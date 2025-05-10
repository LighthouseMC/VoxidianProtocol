use super::*;


#[cfg(any(feature = "commands_clap", doc))]
mod clap;


#[packet("minecraft:s2c/play/commands")]
pub struct CommandsS2CPlayPacket {
    pub nodes      : LengthPrefixVec<Var32, CommandNode>,
    pub root_index : Var32
}


#[derive(Debug, Clone)]
pub struct CommandNode {
    pub kind           : CommandNodeKind,
    pub is_executable  : bool,
    pub child_indices  : LengthPrefixVec<Var32, Var32>,
    pub redirect_index : Option<Var32>
}
impl PacketEncode for CommandNode {
    fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {

        let mut flags = 0b00000000;
        if let CommandNodeKind::Literal { .. } = self.kind { flags |= 0b00000001; }
        if let CommandNodeKind::Argument { suggestions, .. } = &self.kind {
            flags |= 0b00000010;
            if suggestions.is_some() {
                flags |= 0b00010000;
            }
        }
        if (self.is_executable) { flags |= 0b00000100; }
        if self.redirect_index.is_some() { flags |= 0b00001000; }
        buf.write_u8(flags);

        buf.encode_write(&self.child_indices)?;

        if let Some(index) = self.redirect_index {
            buf.encode_write(index)?;
        }

        if let CommandNodeKind::Literal { name } = &self.kind {
            buf.encode_write(name)?;
        } else if let CommandNodeKind::Argument { name, parser, suggestions } = &self.kind {
            buf.encode_write(name)?;
            buf.encode_write(parser)?;
            if let Some(id) = suggestions {
                buf.encode_write(id)?;
            }
        }

        Ok(())
    }
}
impl PacketDecode for CommandNode {
    fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {

        let flags = buf.read_u8()?;

        let child_indices = buf.read_decode()?;

        let redirect_index = if (flags & 0b00001000 != 0) { Some(buf.read_decode()?) } else { None };

        let kind = match (flags & 0b00000011) {
            0 => CommandNodeKind::Root,
            1 => CommandNodeKind::Literal { name : buf.read_decode()? },
            2 => CommandNodeKind::Argument {
                name        : buf.read_decode()?,
                parser      : buf.read_decode()?,
                suggestions : if (flags & 0b00010000 != 0) { Some(buf.read_decode()?) } else { None }
            },
            kind => { return Err(DecodeError::InvalidData(Cow::Owned(format!("{} is not a valid command node kind", kind)))) }
        };

        Ok(Self {
            kind,
            child_indices,
            is_executable  : flags & 0b00000100 != 0,
            redirect_index
        })

    }
}


#[derive(Debug, Clone)]
pub enum CommandNodeKind {
    Root,
    Literal {
        name : String
    },
    Argument {
        name        : String,
        parser      : CommandNodeParser,
        suggestions : Option<Identifier>
    }
}


#[derive(Debug, Clone)]
pub enum CommandNodeParser {
    Bool,
    Float {
        min : Option<f32>,
        max : Option<f32>
    },
    Double {
        min : Option<f64>,
        max : Option<f64>
    },
    Integer {
        min : Option<i32>,
        max : Option<i32>
    },
    Long {
        min : Option<i64>,
        max : Option<i64>
    },
    String {
        behaviour : StringCommandNode
    },
    Entity {
        single       : bool,
        players_only : bool
    },
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Colour,
    Component,
    Style,
    Message,
    Nbt,
    NbtTag,
    NbtPath,
    Objective,
    ObjectiveCriteria,
    Operation,
    Particle,
    Angle,
    Rotation,
    ScoreboardSlot,
    ScoreHolder {
        allow_multiple : bool
    },
    Swizzle,
    Team,
    ItemSlot,
    Identifier,
    Function,
    EntityAnchor,
    IntRange,
    FloatRange,
    Dimension,
    Gamemode,
    Time {
        min_ticks : i32
    },
    ResourceOrTag {
        registry : Identifier
    },
    ResourceOrTagKey {
        registry : Identifier
    },
    Resource {
        registry : Identifier
    },
    ResourceKey {
        registry : Identifier
    },
    TemplateMirror,
    TemplateRotation,
    Heightmap,
    Uuid
}
impl PacketEncode for CommandNodeParser {
    fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
        match (self) {
            Self::Bool => buf.encode_write(Var32::from(0)),
            Self::Float { min, max } => {
                buf.encode_write(Var32::from(1))?;
                let mut flags = 0b00000000;
                if min.is_some() { flags |= 0b00000001; }
                if max.is_some() { flags |= 0b00000010; }
                buf.write_u8(flags);
                if let Some(min) = min { buf.encode_write(min)?; }
                if let Some(max) = max { buf.encode_write(max)?; }
                Ok(())
            },
            Self::Double { min, max } => {
                buf.encode_write(Var32::from(2))?;
                let mut flags = 0b00000000;
                if min.is_some() { flags |= 0b00000001; }
                if max.is_some() { flags |= 0b00000010; }
                buf.write_u8(flags);
                if let Some(min) = min { buf.encode_write(min)?; }
                if let Some(max) = max { buf.encode_write(max)?; }
                Ok(())
            },
            Self::Integer { min, max } => {
                buf.encode_write(Var32::from(3))?;
                let mut flags = 0b00000000;
                if min.is_some() { flags |= 0b00000001; }
                if max.is_some() { flags |= 0b00000010; }
                buf.write_u8(flags);
                if let Some(min) = min { buf.encode_write(min)?; }
                if let Some(max) = max { buf.encode_write(max)?; }
                Ok(())
            },
            Self::Long { min, max } => {
                buf.encode_write(Var32::from(4))?;
                let mut flags = 0b00000000;
                if min.is_some() { flags |= 0b00000001; }
                if max.is_some() { flags |= 0b00000010; }
                buf.write_u8(flags);
                if let Some(min) = min { buf.encode_write(min)?; }
                if let Some(max) = max { buf.encode_write(max)?; }
                Ok(())
            },
            Self::String { behaviour } => {
                buf.encode_write(Var32::from(5))?;
                buf.encode_write(behaviour)?;
                Ok(())
            },
            Self::Entity { single, players_only } => {
                buf.encode_write(Var32::from(6))?;
                let mut flags = 0b00000000;
                if (*single       ) { flags |= 0b00000001; }
                if (*players_only ) { flags |= 0b00000010; }
                buf.write_u8(flags);
                Ok(())
            },
            Self::GameProfile => buf.encode_write(Var32::from(7)),
            Self::BlockPos => buf.encode_write(Var32::from(8)),
            Self::ColumnPos => buf.encode_write(Var32::from(9)),
            Self::Vec3 => buf.encode_write(Var32::from(10)),
            Self::Vec2 => buf.encode_write(Var32::from(11)),
            Self::BlockState => buf.encode_write(Var32::from(12)),
            Self::BlockPredicate => buf.encode_write(Var32::from(13)),
            Self::ItemStack => buf.encode_write(Var32::from(14)),
            Self::ItemPredicate => buf.encode_write(Var32::from(15)),
            Self::Colour => buf.encode_write(Var32::from(16)),
            Self::Component => buf.encode_write(Var32::from(17)),
            Self::Style => buf.encode_write(Var32::from(18)),
            Self::Message => buf.encode_write(Var32::from(19)),
            Self::Nbt => buf.encode_write(Var32::from(20)),
            Self::NbtTag => buf.encode_write(Var32::from(21)),
            Self::NbtPath => buf.encode_write(Var32::from(22)),
            Self::Objective => buf.encode_write(Var32::from(23)),
            Self::ObjectiveCriteria => buf.encode_write(Var32::from(24)),
            Self::Operation => buf.encode_write(Var32::from(25)),
            Self::Particle => buf.encode_write(Var32::from(26)),
            Self::Angle => buf.encode_write(Var32::from(27)),
            Self::Rotation => buf.encode_write(Var32::from(28)),
            Self::ScoreboardSlot => buf.encode_write(Var32::from(29)),
            Self::ScoreHolder { allow_multiple } => {
                buf.encode_write(Var32::from(30))?;
                let mut flags = 0b00000000;
                if (*allow_multiple) { flags |= 0b00000001; }
                buf.write_u8(flags);
                Ok(())
            },
            Self::Swizzle => buf.encode_write(Var32::from(31)),
            Self::Team => buf.encode_write(Var32::from(32)),
            Self::ItemSlot => buf.encode_write(Var32::from(33)),
            Self::Identifier => buf.encode_write(Var32::from(34)),
            Self::Function => buf.encode_write(Var32::from(35)),
            Self::EntityAnchor => buf.encode_write(Var32::from(36)),
            Self::IntRange => buf.encode_write(Var32::from(37)),
            Self::FloatRange => buf.encode_write(Var32::from(38)),
            Self::Dimension => buf.encode_write(Var32::from(39)),
            Self::Gamemode => buf.encode_write(Var32::from(40)),
            Self::Time { min_ticks } => {
                buf.encode_write(Var32::from(41))?;
                buf.encode_write(min_ticks)?;
                Ok(())
            },
            Self::ResourceOrTag { registry } => {
                buf.encode_write(Var32::from(42))?;
                buf.encode_write(registry)?;
                Ok(())
            },
            Self::ResourceOrTagKey { registry } => {
                buf.encode_write(Var32::from(43))?;
                buf.encode_write(registry)?;
                Ok(())
            },
            Self::Resource { registry } => {
                buf.encode_write(Var32::from(44))?;
                buf.encode_write(registry)?;
                Ok(())
            },
            Self::ResourceKey { registry } => {
                buf.encode_write(Var32::from(45))?;
                buf.encode_write(registry)?;
                Ok(())
            },
            Self::TemplateMirror => buf.encode_write(Var32::from(46)),
            Self::TemplateRotation => buf.encode_write(Var32::from(47)),
            Self::Heightmap => buf.encode_write(Var32::from(48)),
            Self::Uuid => buf.encode_write(Var32::from(49)),
        }
    }
}
impl PacketDecode for CommandNodeParser {
    fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        match (buf.read_decode::<Var32>()?.as_i32()) {
            0 => Ok(Self::Bool),
            1 => {
                let flags = buf.read_u8()?;
                Ok(Self::Float {
                    min : if (flags & 0b00000001 != 0) { Some(buf.read_decode()?) } else { None },
                    max : if (flags & 0b00000010 != 0) { Some(buf.read_decode()?) } else { None }
                })
            },
            2 => {
                let flags = buf.read_u8()?;
                Ok(Self::Double {
                    min : if (flags & 0b00000001 != 0) { Some(buf.read_decode()?) } else { None },
                    max : if (flags & 0b00000010 != 0) { Some(buf.read_decode()?) } else { None }
                })
            },
            3 => {
                let flags = buf.read_u8()?;
                Ok(Self::Integer {
                    min : if (flags & 0b00000001 != 0) { Some(buf.read_decode()?) } else { None },
                    max : if (flags & 0b00000010 != 0) { Some(buf.read_decode()?) } else { None }
                })
            },
            4 => {
                let flags = buf.read_u8()?;
                Ok(Self::Long {
                    min : if (flags & 0b00000001 != 0) { Some(buf.read_decode()?) } else { None },
                    max : if (flags & 0b00000010 != 0) { Some(buf.read_decode()?) } else { None }
                })
            },
            5 => Ok(Self::String { behaviour : buf.read_decode()? }),
            6 => {
                let flags = buf.read_u8()?;
                Ok(Self::Entity {
                    single       : flags & 0b00000001 != 0,
                    players_only : flags & 0b00000010 != 0
                })
            },
            7 => Ok(Self::GameProfile),
            8 => Ok(Self::BlockPos),
            9 => Ok(Self::ColumnPos),
            10 => Ok(Self::Vec3),
            11 => Ok(Self::Vec2),
            12 => Ok(Self::BlockState),
            13 => Ok(Self::BlockPredicate),
            14 => Ok(Self::ItemStack),
            15 => Ok(Self::ItemPredicate),
            16 => Ok(Self::Colour),
            17 => Ok(Self::Component),
            18 => Ok(Self::Style),
            19 => Ok(Self::Message),
            20 => Ok(Self::Nbt),
            21 => Ok(Self::NbtTag),
            22 => Ok(Self::NbtPath),
            23 => Ok(Self::Objective),
            24 => Ok(Self::ObjectiveCriteria),
            25 => Ok(Self::Operation),
            26 => Ok(Self::Particle),
            27 => Ok(Self::Angle),
            28 => Ok(Self::Rotation),
            29 => Ok(Self::ScoreboardSlot),
            30 => {
                let flags = buf.read_u8()?;
                Ok(Self::ScoreHolder {
                    allow_multiple : flags & 0b00000001 != 0
                })
            },
            31 => Ok(Self::Swizzle),
            32 => Ok(Self::Team),
            33 => Ok(Self::ItemSlot),
            34 => Ok(Self::Identifier),
            35 => Ok(Self::Function),
            36 => Ok(Self::EntityAnchor),
            37 => Ok(Self::IntRange),
            38 => Ok(Self::FloatRange),
            39 => Ok(Self::Dimension),
            40 => Ok(Self::Gamemode),
            41 => Ok(Self::Time { min_ticks : buf.read_decode()? }),
            42 => Ok(Self::ResourceOrTag { registry : buf.read_decode()? }),
            43 => Ok(Self::ResourceOrTagKey { registry : buf.read_decode()? }),
            44 => Ok(Self::Resource { registry : buf.read_decode()? }),
            45 => Ok(Self::ResourceKey { registry : buf.read_decode()? }),
            46 => Ok(Self::TemplateMirror),
            47 => Ok(Self::TemplateRotation),
            48 => Ok(Self::Heightmap),
            49 => Ok(Self::Uuid),
            id => Err(DecodeError::InvalidData(Cow::Owned(format!("Unknown command node parser type {}", id))))
        }
    }
}


#[packet_part(Var32)]
pub enum StringCommandNode {
    SingleWord     = 0,
    QuotablePhrase = 1,
    GreedyPhrase   = 2
}
