use crate::packet::PublicKey;

use std::thread::{ self, JoinHandle };
use std::net::SocketAddr;
use std::fmt;
use uuid::Uuid;
use serde::{ Deserialize as Deser, Deserializer as Deserer };
use openssl::sha::Sha1;
use reqwest;
use num_bigint::BigInt;


#[derive(Deser, Debug, Clone)]
pub struct MojAuth {
    #[serde(rename = "id", deserialize_with = "parse_uuid")]
    pub uuid  : Uuid,
    pub name  : String,
    #[serde(rename = "properties")]
    pub props : Vec<MojAuthProperty>
}
fn parse_uuid<'l, D : Deserer<'l>>(deser : D) -> Result<Uuid, D::Error> {
    let uuid = String::deserialize(deser)?;
    Uuid::parse_str(&uuid).map_err(|_| serde::de::Error::custom("Not a valid Uuid"))
}
#[derive(Deser, Debug, Clone)]
pub struct MojAuthProperty {
    pub name  : String,
    pub value : String,
    #[serde(rename = "signature")]
    pub sig   : String
}

impl MojAuth {

    /// `LighthouseMCRust`
    const NAMESPACE_LIGHTHOUSE : Uuid = Uuid::from_bytes([0x4c, 0x69, 0x67, 0x68, 0x74, 0x68, 0x6f, 0x75, 0x73, 0x65, 0x4d, 0x43, 0x52, 0x75, 0x73, 0x74]);

    pub async fn start<U : AsRef<str>, S : AsRef<str>>(block_proxies : Option<SocketAddr>, username : U, server_id : S, secret_cipher_key : &[u8], public_key : &PublicKey) -> Result<MojAuth, MojAuthError> {
        let mut sha = Sha1::new();
        sha.update(server_id.as_ref().as_ascii().unwrap().as_bytes());
        sha.update(secret_cipher_key);
        sha.update(&public_key.der_bytes());
        let sha = BigInt::from_signed_bytes_be(&sha.finish()).to_str_radix(16);
        let mut url = format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}", username.as_ref(), sha);
        if let Some(peer_addr) = block_proxies {
            let peer_addr = peer_addr.ip();
            if (peer_addr.is_global()) {
                url.push_str("&ip=");
                url.push_str(&peer_addr.to_string());
            }
        }
        let response = reqwest::get(&url).await.map_err(|_| MojAuthError::AuthServerDown)?;
        match (response.status().as_u16()) {
            200 => {
                let body = response.text().await.map_err(|_| MojAuthError::InvalidData)?;
                Ok(serde_json::from_str::<MojAuth>(&body).unwrap_or_else(|_| panic!("Body was {}", body)))
            },
            204 => Err(MojAuthError::Unverified),
            _   => Err(MojAuthError::InvalidData)
        }
    }

    pub fn start_non_blocking<U : AsRef<str>, S : AsRef<str>>(block_proxies : Option<SocketAddr>, username : U, server_id : S, secret_cipher_key : &[u8], public_key : &PublicKey) -> MojAuthHandle {
        let mut sha = Sha1::new();
        sha.update(server_id.as_ref().as_ascii().unwrap().as_bytes());
        sha.update(secret_cipher_key);
        sha.update(&public_key.der_bytes());
        let sha = BigInt::from_signed_bytes_be(&sha.finish()).to_str_radix(16);
        let mut url = format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}", username.as_ref(), sha);
        if let Some(peer_addr) = block_proxies {
            let peer_addr = peer_addr.ip();
            if (peer_addr.is_global()) {
                url.push_str("&ip=");
                url.push_str(&peer_addr.to_string());
            }
        }
        MojAuthHandle {
            already_done : None,
            handle : Some(thread::spawn(move || {
                let response = reqwest::blocking::get(&url).map_err(|_| MojAuthError::AuthServerDown)?;
                match (response.status().as_u16()) {
                    200 => {
                        let body = response.text().map_err(|_| MojAuthError::InvalidData)?;
                        Ok(serde_json::from_str::<MojAuth>(&body).unwrap_or_else(|_| panic!("Body was {}", body)))
                    },
                    204 => Err(MojAuthError::Unverified),
                    _   => Err(MojAuthError::InvalidData)
                }
            } ))
        }
    }

    pub fn start_blocking<U : AsRef<str>, S : AsRef<str>>(block_proxies : Option<SocketAddr>, username : U, server_id : S, secret_cipher_key : &[u8], public_key : &PublicKey) -> Result<MojAuth, MojAuthError> {
        Self::start_non_blocking(block_proxies, username, server_id, secret_cipher_key, public_key).wait_to_finish()
    }

    pub fn offline<S : Into<String>>(name : S) -> Self {
        let name = name.into();
        Self {
            uuid  : Uuid::new_v3(&Self::NAMESPACE_LIGHTHOUSE, name.as_bytes()),
            name,
            props : Vec::new(), // TODO: Offline skin
        }
    }

}


pub struct MojAuthHandle {
    already_done : Option<Result<MojAuth, MojAuthError>>,
    handle       : Option<JoinHandle<Result<MojAuth, MojAuthError>>>
}

impl MojAuthHandle {

    pub fn no_data() -> Self { Self {
        already_done : Some(Err(MojAuthError::InvalidData)),
        handle       : None
    } }

    pub fn already_finished<S : Into<String>>(name : S) -> Self {
        Self {
            already_done : Some(Ok(MojAuth::offline(name))),
            handle : None
        }
    }

    pub fn is_finished(&self) -> bool {
        self.already_done.is_some() || self.handle.as_ref().unwrap().is_finished()
    }

    pub fn wait_to_finish(self) -> Result<MojAuth, MojAuthError> {
        self.already_done.unwrap_or_else(|| self.handle.unwrap().join().unwrap())
    }

}
impl fmt::Debug for MojAuthHandle { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "MojAuthHandle")
} }


#[derive(Debug)]
pub enum MojAuthError {

    /// The authentication servers did not respond.
    AuthServerDown,

    /// The authentication servers responded with invalid data.
    InvalidData,

    /// The user could not be authenticated.
    Unverified

}
