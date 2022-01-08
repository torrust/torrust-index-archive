use std::{fs, error};
use serde_bencode::{de, Error};
use crate::models::torrent_file::Torrent;

pub fn read_torrent_from_file(path: &str) -> Result<Torrent, Box<dyn error::Error>> {
    let contents = match fs::read(path) {
        Ok(contents) => contents,
        Err(e) => return Err(e.into()),
    };

    match decode_torrent(&contents) {
        Ok(torrent) => Ok(torrent),
        Err(e) => Err(e.into()),
    }
}

pub fn decode_torrent(bytes: &[u8]) -> Result<Torrent, Box<dyn error::Error>> {
    match de::from_bytes::<Torrent>(&bytes) {
        Ok(torrent) => Ok(torrent),
        Err(e) => {
            println!("{:?}", e);
            Err(e.into())
        }
    }
}

pub fn encode_torrent(torrent: &Torrent) -> Result<Vec<u8>, Error> {
    match serde_bencode::to_bytes(torrent) {
        Ok(bencode_bytes) => Ok(bencode_bytes),
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
    }
}
