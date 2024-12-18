#![feature(
    iter_array_chunks,
    decl_macro,
    ascii_char,
    debug_closure_helpers,
    ip
)]

pub mod packet;
pub mod value;
pub mod registry;
pub mod mojang;


pub const MINECRAFT_VERSION : &'static str = "1.21.4";
//pub const PROTOCOL_VERSION  : usize        = ;


// TODO: Current packet uses protocol 767 (1.21.1). At some point, follow https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Pre-release_protocol to update to protocol 769 (1.21.4).
