use super::*;

use openssl::pkey::{ PKey, Private, Public };
use openssl::encrypt::{ Encrypter, Decrypter };
use openssl::rsa::{ Rsa, Padding };
use openssl::symm::{ Cipher, Crypter, Mode };


/// A public RSA key.
#[derive(Debug)]
pub struct PublicKey(PKey<Public>);
impl PublicKey {

    pub fn encrypt(&self, plaindata : &[u8]) -> Vec<u8> {
        let mut en  = Encrypter::new(&self.0).unwrap();
        en.set_rsa_padding(Padding::PKCS1).unwrap();
        let buf_len = en.encrypt_len(plaindata).unwrap();
        let mut out = vec![0; buf_len];
        let out_len = en.encrypt(plaindata, &mut out).unwrap();
        out.truncate(out_len);
        out
    }

    pub fn der_bytes(&self) -> Vec<u8> {
        self.0.public_key_to_der().unwrap()
    }

    pub fn from_der_bytes(key : &[u8]) -> Result<Self, ()> {
        Ok(Self(PKey::public_key_from_der(key).map_err(|_| ())?))
    }

}
impl PartialEq for PublicKey { fn eq(&self, other : &Self) -> bool {
    self.0.public_eq(&*other.0)
} }


/// A private RSA key.
pub struct PrivateKey(PKey<Private>);
impl PrivateKey {

    pub fn decrypt(&self, cipherdata : &[u8]) -> Result<Vec<u8>, ()> {
        let mut de  = Decrypter::new(&self.0).unwrap();
        de.set_rsa_padding(Padding::PKCS1).unwrap();
        let buf_len = de.decrypt_len(cipherdata).unwrap();
        let mut out = vec![0; buf_len];
        let out_len = de.decrypt(cipherdata, &mut out).map_err(|_| ())?;
        out.truncate(out_len);
        Ok(out)
    }

}


/// A secret AES key, shared between endpoints using the public key.
/// Can also be a no-cipher which will not encrypt anything.
pub struct SecretCipher(Option<SecretCipherInner>);
struct SecretCipherInner {
    en : Crypter,
    de : Crypter
}
impl SecretCipher {

    pub fn no_cipher() -> Self { Self(None) }

    pub fn is_no_cipher(&self) -> bool { matches!(self.0, None) } // `self.de` is guaranteed to have the same `is_some` as `self.en`.

    pub fn from_key_bytes(key_iv : &[u8]) -> Self { Self(Some(SecretCipherInner {
        en : Crypter::new(Cipher::aes_128_cfb8(), Mode::Encrypt, key_iv, Some(key_iv)).unwrap(),
        de : Crypter::new(Cipher::aes_128_cfb8(), Mode::Decrypt, key_iv, Some(key_iv)).unwrap()
    })) }

    pub fn encrypt(&mut self, plaindata : PacketBuf) -> PacketBuf {
        if let Some(SecretCipherInner { en, .. }) = &mut self.0 {
            let mut cipherdata = vec![0; plaindata.remaining()];
            en.update(plaindata.as_slice(), &mut cipherdata).unwrap();
            let mut out = PacketBuf::new();
            out.write_u8s(&cipherdata);
            out
        } else {
            plaindata
        }
    }

    pub fn encrypt_u8(&mut self, plainbyte : u8) -> Result<u8, DecodeError> {
        if let Some(SecretCipherInner { en, .. }) = &mut self.0 {
            let mut cipherbyte = [0];
            en.update(&[plainbyte], &mut cipherbyte).map_err(|_| DecodeError::InvalidData)?;
            Ok(cipherbyte[0])
        } else {
            Ok(plainbyte)
        }
    }

    pub fn decrypt(&mut self, cipherdata : PacketBuf) -> Result<PacketBuf, DecodeError> {
        if let Some(SecretCipherInner { de, .. }) = &mut self.0 {
            let mut plaindata = vec![0; cipherdata.remaining()];
            de.update(cipherdata.as_slice(), &mut plaindata).map_err(|_| DecodeError::InvalidData)?;
            let mut out = PacketBuf::new();
            out.write_u8s(&plaindata);
            Ok(out)
        } else {
            Ok(cipherdata)
        }
    }

    pub fn decrypt_u8(&mut self, cipherbyte : u8) -> Result<u8, DecodeError> {
        if let Some(SecretCipherInner { de, .. }) = &mut self.0 {
            let mut plainbyte = [0];
            de.update(&[cipherbyte], &mut plainbyte).map_err(|_| DecodeError::InvalidData)?;
            Ok(plainbyte[0])
        } else {
            Ok(cipherbyte)
        }
    }

}


/// Generate an RSA key pair.
pub fn generate_key_pair<const KEY_SIZE : u32>() -> (PrivateKey, PublicKey) {
    let private = PKey::from_rsa(Rsa::generate(KEY_SIZE).unwrap()).unwrap();
    let public  = PKey::from_rsa(Rsa::public_key_from_der(&private.public_key_to_der().unwrap()).unwrap()).unwrap();
    (PrivateKey(private), PublicKey(public))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsa_encrypt_decrypt() {
        let (real_private, public) = generate_key_pair::<2048>();
        let (attacker0_private, _) = generate_key_pair::<2048>();
        let (attacker1_private, _) = generate_key_pair::<1024>();
        let (attacker2_private, _) = generate_key_pair::<4096>();
        let data       = b"Hello, World!";
        let cipherdata = public.encrypt(data);
        let outdata = real_private.decrypt(&cipherdata);
        assert_eq!(data, outdata.unwrap().as_slice());
        let attacker0_outdata = attacker0_private.decrypt(&cipherdata);
        assert_ne!(data, attacker0_outdata.unwrap().as_slice());
        let attacker1_outdata = attacker1_private.decrypt(&cipherdata);
        assert_eq!(attacker1_outdata, Err(()));
        let attacker2_outdata = attacker2_private.decrypt(&cipherdata);
        assert_ne!(data, attacker2_outdata.unwrap().as_slice());
    }

    #[test]
    fn rsa_publickey_der() {
        let (_, public) = generate_key_pair::<4096>();
        let der      = public.der_bytes();
        let from_der = PublicKey::from_der_bytes(der.as_slice()).unwrap();
        assert_eq!(public, from_der);
    }

}
