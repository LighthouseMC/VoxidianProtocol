use super::*;
//use config::{ ReportDetail, ServerLink };


// TODO: S2CPackets


#[packet( "minecraft:s2c/play/disconnect" )]
pub struct DisconnectS2CPlayPacket {
    pub reason : NbtText
}


//packet_full_decode!{ S2C Play }
