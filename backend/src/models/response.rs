use serde::{Deserialize, Serialize};
use crate::database::Category;
use crate::models::torrent::TorrentListing;
use crate::models::torrent_file::File;

pub enum OkResponses {
    TokenResponse(TokenResponse)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OkResponse<T> {
    pub data: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse<T> {
    pub errors: Vec<T>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub token: String,
    pub username: String,
    pub admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTorrentResponse {
    pub torrent_id: i64,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct CategoryResponse {
    pub category_id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub num_torrents: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TorrentResponse {
    pub torrent_id: i64,
    pub uploader: String,
    pub info_hash: String,
    pub title: String,
    pub description: Option<String>,
    pub category: Category,
    pub upload_date: i64,
    pub file_size: i64,
    pub seeders: i64,
    pub leechers: i64,
    pub files: Option<Vec<File>>,
    pub trackers: Vec<String>,
    pub magnet_link: String,
}

impl TorrentResponse {
    pub fn from_listing(torrent_listing: TorrentListing) -> TorrentResponse {
        TorrentResponse {
            torrent_id: torrent_listing.torrent_id,
            uploader: torrent_listing.uploader,
            info_hash: torrent_listing.info_hash,
            title: torrent_listing.title,
            description: torrent_listing.description,
            category: Category { name: "".to_string(), icon: None, num_torrents: 0 },
            upload_date: torrent_listing.upload_date,
            file_size: torrent_listing.file_size,
            seeders: torrent_listing.seeders,
            leechers: torrent_listing.leechers,
            files: None,
            trackers: vec![],
            magnet_link: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct TorrentsResponse {
    pub total: u32,
    pub results: Vec<TorrentListing>,
}
