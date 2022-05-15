use crate::config::Configuration;
use std::sync::Arc;
use crate::database::Database;
use crate::models::tracker_key::TrackerKey;
use crate::errors::ServiceError;
use crate::models::user::User;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub info_hash: String,
    pub seeders: i64,
    pub completed: i64,
    pub leechers: i64,
    pub peers: Vec<Peer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    pub peer_id: Option<PeerId>,
    pub peer_addr: Option<String>,
    pub updated: Option<i64>,
    pub uploaded: Option<i64>,
    pub downloaded: Option<i64>,
    pub left: Option<i64>,
    pub event: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerId {
    pub id: Option<String>,
    pub client: Option<String>
}

pub struct TrackerService {
    cfg: Arc<Configuration>,
    database: Arc<Database>,
}

impl TrackerService {
    pub fn new(cfg: Arc<Configuration>, database: Arc<Database>) -> TrackerService {
        TrackerService {
            cfg,
            database
        }
    }

    pub async fn whitelist_info_hash(&self, info_hash: String) -> Result<(), ServiceError> {
        let settings = self.cfg.settings.read().await;

        let request_url =
            format!("{}/api/whitelist/{}?token={}", settings.tracker.api_url, info_hash, settings.tracker.token);

        drop(settings);

        let client = reqwest::Client::new();

        let response = match client.post(request_url).send().await {
            Ok(v) => Ok(v),
            Err(_) => Err(ServiceError::InternalServerError)
        }?;

        if response.status().is_success() {
            return Ok(())
        }

        Err(ServiceError::InternalServerError)
    }

    pub async fn remove_info_hash_from_whitelist(&self, info_hash: String) -> Result<(), ServiceError> {
        let settings = self.cfg.settings.read().await;

        let request_url =
            format!("{}/api/whitelist/{}?token={}", settings.tracker.api_url, info_hash, settings.tracker.token);

        drop(settings);

        let client = reqwest::Client::new();

        let response = match client.delete(request_url).send().await {
            Ok(v) => Ok(v),
            Err(_) => Err(ServiceError::InternalServerError)
        }?;

        if response.status().is_success() {
            return Ok(())
        }

        Err(ServiceError::InternalServerError)
    }

    pub async fn get_personal_announce_url(&self, user: &User) -> Result<String, ServiceError> {
        let settings = self.cfg.settings.read().await;

        let tracker_key = self.database.get_valid_tracker_key(user.user_id).await;

        match tracker_key {
            Some(v) => { Ok(format!("{}/{}", settings.tracker.url, v.key)) }
            None => {
                match self.retrieve_new_tracker_key(user.user_id).await {
                    Ok(v) => { Ok(format!("{}/{}", settings.tracker.url, v.key)) },
                    Err(_) => { Err(ServiceError::TrackerOffline) }
                }
            }
        }
    }

    pub async fn retrieve_new_tracker_key(&self, user_id: i64) -> Result<TrackerKey, ServiceError> {
        let settings = self.cfg.settings.read().await;

        let request_url =
            format!("{}/api/key/{}?token={}", settings.tracker.api_url, settings.tracker.token_valid_seconds, settings.tracker.token);

        drop(settings);

        let client = reqwest::Client::new();
        let response = match client.post(request_url)
            .send()
            .await {
            Ok(v) => Ok(v),
            Err(_) => Err(ServiceError::InternalServerError)
        }?;

        let tracker_key: TrackerKey = match response.json::<TrackerKey>().await {
            Ok(v) => Ok(v),
            Err(_) => Err(ServiceError::InternalServerError)
        }?;

        println!("{:?}", tracker_key);

        self.database.issue_tracker_key(&tracker_key, user_id).await?;

        Ok(tracker_key)
    }

    // get torrent info from tracker api
    pub async fn get_torrent_info(&self, info_hash: &str) -> Result<TorrentInfo, ServiceError> {
        let settings = self.cfg.settings.read().await;

        let request_url =
            format!("{}/api/torrent/{}?token={}", settings.tracker.api_url, info_hash, settings.tracker.token);

        drop(settings);

        let client = reqwest::Client::new();
        let response = match client.get(request_url)
            .send()
            .await {
            Ok(v) => Ok(v),
            Err(_) => Err(ServiceError::InternalServerError)
        }?;

        let torrent_info = match response.json::<TorrentInfo>().await {
            Ok(torrent_info) => {
                let _ = self.database.update_tracker_info(info_hash, torrent_info.seeders, torrent_info.leechers).await;
                Ok(torrent_info)
            },
            Err(e) => {
                eprintln!("{:?}", e);
                let _ = self.database.update_tracker_info(info_hash, 0, 0).await;
                Err(ServiceError::TorrentNotFound)
            }
        }?;

        Ok(torrent_info)
    }

    pub async fn update_torrents(&self) -> Result<(), ()> {
        println!("Updating torrents..");
        let torrents = self.database.get_all_torrent_ids().await?;

        for torrent in torrents {
            let _ = self.get_torrent_info(&torrent.info_hash).await;
        }

        Ok(())
    }
}
