use serde::{Deserialize, Serialize};
use crate::models::torrent_file::Torrent;
use crate::handlers::torrent::CreateTorrent;
use sqlx::{FromRow};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TorrentListing {
    pub torrent_id: i64,
    pub uploader: String,
    pub info_hash: String,
    pub title: String,
    pub description: Option<String>,
    pub category_id: i64,
    pub upload_date: i64,
    pub file_size: i64,
    pub seeders: i64,
    pub leechers: i64,
    pub hidden: bool
}

#[derive(Debug)]
pub struct TorrentRequest {
    pub fields: CreateTorrent,
    pub torrent: Torrent,
}
