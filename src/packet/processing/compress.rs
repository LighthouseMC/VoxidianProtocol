use super::*;

use std::io::Write;
use flate2::Compression;
use flate2::write::{ ZlibEncoder, ZlibDecoder };


#[derive(Debug)]
pub enum CompressionMode {
    None,
    ZLib {
        /// In bytes
        threshold : usize
    }
}
impl CompressionMode {


    /// `plaintext` should include the packet ID and packet data, but **should not include the full packet length**.
    /// 
    /// The output **will not** contain the full packet length.
    pub fn compress(&self, plaintext : PacketBuf) -> Result<PacketBuf, EncodeError> { match (self) {

        Self::None => Ok(plaintext),

        Self::ZLib { threshold } => {
            let (data_length, smalltext) = if (plaintext.remaining() < *threshold) {
                (VarInt::from(0), plaintext)
            } else {
                let mut en = ZlibEncoder::new(Vec::new(), Compression::default());
                en.write_all(plaintext.as_slice()).unwrap();
                let mut out = PacketBuf::new();
                out.write_u8s(&en.finish().unwrap());
                (VarInt::from(plaintext.remaining()), out)
            };
            let mut out = PacketBuf::new();
            out.encode_write(data_length)?;
            out.write_u8s(smalltext.as_slice());
            Ok(out)
        }

    } }


    /// `smalltext` should include the data length, compressed packet ID, and compressed packet data, but **should not include the full packet length**.
    /// 
    /// The output **will not** contain the full packet length.
    pub fn decompress(&self, mut smalltext : PacketBuf) -> Result<PacketBuf, DecodeError> { match (self) {

        Self::None => Ok(smalltext),

        Self::ZLib { .. } => {
            let data_length = smalltext.read_decode::<VarInt>()?.as_i32();
            if (data_length < 0) {
                Ok(smalltext)
            } else {
                let smalltext = smalltext.read_u8s(smalltext.remaining())?;
                let mut de = ZlibDecoder::new(Vec::new());
                de.write_all(&smalltext).map_err(|_| DecodeError::InvalidData)?;
                let mut out = PacketBuf::new();
                out.write_u8s(&de.finish().map_err(|_| DecodeError::InvalidData)?);
                Ok(out)
            }
        }

    } }


}
