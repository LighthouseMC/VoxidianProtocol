use super::*;

use std::io::Write;
use std::borrow::Cow;
use flate2::Compression;
use flate2::write::{ ZlibEncoder, ZlibDecoder };


#[derive(Debug, Clone, Copy)]
pub enum CompressionMode {
    None,
    ZLib {
        /// In bytes
        threshold : usize
    }
}
impl CompressionMode {

    pub fn threshold(&self) -> Var32 { Var32::from(match (self) {
        CompressionMode::None               => -1,
        CompressionMode::ZLib { threshold } => *threshold as i32,
    }) }

}
impl CompressionMode {


    /// `plaintext` should include the packet ID and packet data, but **should not include the full packet length**.
    ///
    /// The output **will not** contain the full packet length.
    pub fn compress(&self, plaintext : PacketWriter) -> Result<PacketWriter, EncodeError> { match (self) {

        Self::None => Ok(plaintext),

        Self::ZLib { threshold } => {
            let (data_length, smalltext) = if (plaintext.len() < *threshold) {
                (0, plaintext.into_inner())
            } else {
                let mut en = ZlibEncoder::new(Vec::new(), Compression::default());
                en.write_all(plaintext.as_slice()).unwrap();
                (plaintext.len(), en.finish().unwrap())
            };
            let mut out = PacketWriter::new();
            out.encode_write(Var32::from(data_length))?;
            out.write_u8s(smalltext.as_slice());
            Ok(out)
        }

    } }


    /// `smalltext` should include the data length, compressed packet ID, and compressed packet data, but **should not include the full packet length**.
    ///
    /// The output **will not** contain the full packet length.
    pub fn decompress<'l>(&self, mut smalltext : PacketReader<'l>) -> Result<PacketReader<'l>, DecodeError> { match (self) {

        Self::None => Ok(smalltext),

        Self::ZLib { .. } => {
            let data_length = smalltext.read_decode::<Var32>()?.as_i32();
            if (data_length <= 0) {
                Ok(smalltext)
            } else {
                let smalltext = smalltext.read_u8s(smalltext.remaining())?;
                let mut de = ZlibDecoder::new(Vec::new());
                de.write_all(&smalltext).map_err(|_| DecodeError::InvalidData(Cow::Borrowed("Compressed smalltext is not valid ZLib data")))?;
                Ok(PacketReader::from(de.finish().map_err(|_| DecodeError::InvalidData(Cow::Borrowed("Compressed smalltext is not valid ZLib data")))?))
            }
        }

    } }


}
