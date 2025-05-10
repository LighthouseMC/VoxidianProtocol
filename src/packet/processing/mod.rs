mod encrypt;
pub use encrypt::*;
mod compress;
pub use compress::*;

use crate::packet::*;
use crate::value::Var32;


pub struct PacketProcessing {
    pub secret_cipher : SecretCipher,
    pub compression   : CompressionMode
}
impl PacketProcessing {

    pub const NONE : Self = Self {
        secret_cipher : SecretCipher::NONE,
        compression   : CompressionMode::None
    };

    /// Includes full packet length.
    pub fn encode_encrypt(&mut self, plaintext : PacketWriter) -> Result<PacketWriter, EncodeError> {
        let mut smalltext = self.compression.compress(plaintext)?;
        let mut buf = [0; 5];
        let     n   = Var32::from(smalltext.len()).bytes_buf(&mut buf);
        #[allow(deprecated)]
        smalltext.insert(0, &buf[0..n]);
        let ciphertext = self.secret_cipher.encrypt(smalltext);
        Ok(ciphertext)
    }

    /// Queue must contain data that was **already encrypted**.
    ///
    /// Also returns the number of bytes that were consumed.
    pub fn decode_from_raw_queue(&mut self, queue : impl Iterator<Item = u8>) -> Result<(PacketReader<'static>, usize), DecodeError> {
        let (smalltext, consumed) = PacketReader::from_raw_queue(queue)?;
        let plaintext = self.compression.decompress(smalltext)?;
        Ok((plaintext, consumed))
    }

}
