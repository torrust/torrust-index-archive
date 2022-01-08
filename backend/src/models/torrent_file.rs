use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use crate::config::TorrustConfig;
use serde_bencode::ser;
use sha1::{Digest, Sha1};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Node(String, i64);

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename="piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="root hash")]
    pub root_hash: Option<String>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Torrent {
    pub info: Info, //
    #[serde(default)]
    pub announce: Option<String>,
    #[serde(default)]
    pub nodes: Option<Vec<Node>>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename="creation date")]
    pub creation_date: Option<i64>,
    #[serde(rename="comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename="created by")]
    pub created_by: Option<String>,
}

impl Torrent {
    pub fn set_torrust_config(&mut self, cfg: &TorrustConfig) {
        self.announce = Some(cfg.tracker.url.clone());

        // if let Some(list) = &mut self.announce_list {
        //     let mut vec = Vec::new();
        //     vec.push(cfg.tracker.url.clone());
        //     list.insert(0, vec);
        // }

        // todo: config option to remove other trackers from uploaded torrent files
        self.announce_list = None;
    }

    pub fn calculate_info_hash_as_bytes(&self) -> [u8; 20] {
        let info_bencoded = ser::to_bytes(&self.info).unwrap();
        let mut hasher = Sha1::new();
        hasher.update(info_bencoded);
        let sum_hex = hasher.finalize();
        let mut sum_bytes: [u8; 20] = Default::default();
        sum_bytes.copy_from_slice(sum_hex.as_slice());
        sum_bytes
    }


    pub fn info_hash(&self) -> String {
        let mut buffer = [0u8; 40];
        let input = &self.calculate_info_hash_as_bytes();
        let bytes_out = binascii::bin2hex(input, &mut buffer).ok().unwrap();
        String::from(std::str::from_utf8(bytes_out).unwrap())
    }

    pub fn file_size(&self) -> i64 {
        if self.info.length.is_some() {
            return self.info.length.unwrap()
        } else {
            match &self.info.files {
                None => 0,
                Some(files) => {
                    let mut file_size = 0;
                    for file in files.iter() {
                        file_size += file.length;
                    }
                    file_size
                }
            }
        }
    }
}
