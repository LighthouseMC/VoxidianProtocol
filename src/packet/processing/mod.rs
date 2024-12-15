mod encrypt;
pub use encrypt::*;
mod compress;
pub use compress::*;

use crate::packet::*;
use crate::value::VarInt;


pub struct PacketProcessing {
    pub secret_cipher : SecretCipher,
    pub compression   : CompressionMode
}
impl PacketProcessing {

    /// Includes full packet length.
    pub fn encode_encrypt(&mut self, plaintext : PacketBuf) -> Result<PacketBuf, EncodeError> {
        let mut smalltext  = self.compression.compress(plaintext)?;
        #[allow(deprecated)]
        smalltext.insert(0, &VarInt::from(smalltext.remaining() as i32).as_bytes());
        let ciphertext = self.secret_cipher.encrypt(smalltext);
        Ok(ciphertext)
    }

    /// Queue must contain data that was **already encrypted**.
    /// 
    /// Also returns the number of bytes that were consumed.
    pub fn decode_from_raw_queue(&mut self, queue : impl Iterator<Item = u8>) -> Result<(PacketBuf, usize), DecodeError> {
        let (smalltext, consumed) = PacketBuf::from_raw_queue(queue)?;
        let plaintext = self.compression.decompress(smalltext)?;
        Ok((plaintext, consumed))
    }

}
