use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::web::{Query};
use futures::{AsyncWriteExt, StreamExt, TryStreamExt};
use serde::{Deserialize};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::response::{NewTorrentResponse, OkResponse, TorrentResponse, TorrentsResponse};
use crate::models::torrent::{TorrentListing, TorrentRequest};
use crate::utils::parse_torrent;
use crate::common::{WebAppData};
use std::io::Cursor;
use std::io::{Write};
use crate::models::torrent_file::{Torrent, File};
use crate::AsCSV;
use std::option::Option::Some;
use sqlx::{FromRow};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/torrent")
            .service(web::resource("/upload")
                .route(web::post().to(upload_torrent)))
            .service(web::resource("/download/{id}")
                .route(web::get().to(download_torrent)))
            .service(web::resource("/{id}")
                .route(web::get().to(get_torrent))
                .route(web::delete().to(delete_torrent)))
    );
    cfg.service(
        web::scope("/torrents")
            .service(web::resource("")
                .route(web::get().to(get_torrents)))
    );
}

#[derive(Debug, Deserialize)]
pub struct DisplayInfo {
    page_size: Option<i32>,
    page: Option<i32>,
    sort: Option<String>,
    // expects comma separated string, eg: "?categories=movie,other,app"
    categories: Option<String>,
    search: Option<String>,
}

#[derive(FromRow)]
pub struct TorrentCount {
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateTorrent {
    pub title: String,
    pub description: String,
    pub category: String,
}

impl CreateTorrent {
    pub fn verify(&self) -> Result<(), ServiceError>{
        if !self.title.is_empty() && !self.category.is_empty() {
            return Ok(())
        }

        Err(ServiceError::BadRequest)
    }
}

// eg: /torrents?categories=music,other,movie&search=bunny&sort=size_DESC
pub async fn get_torrents(params: Query<DisplayInfo>, app_data: WebAppData) -> ServiceResult<impl Responder> {
    let page = params.page.unwrap_or(0);
    let page_size = params.page_size.unwrap_or(30);
    let offset = page * page_size;
    let categories = params.categories.as_csv::<String>().unwrap_or(None);
    let search = match &params.search {
        None => "%".to_string(),
        Some(v) => format!("%{}%", v)
    };

    let sort_query: String = match &params.sort {
        Some(sort) => {
            match sort.as_str() {
                "uploaded_ASC" => "upload_date ASC".to_string(),
                "uploaded_DESC" => "upload_date DESC".to_string(),
                "seeders_ASC" => "seeders ASC".to_string(),
                "seeders_DESC" => "seeders DESC".to_string(),
                "leechers_ASC" => "leechers ASC".to_string(),
                "leechers_DESC" => "leechers DESC".to_string(),
                "name_ASC" => "title ASC".to_string(),
                "name_DESC" => "title DESC".to_string(),
                "size_ASC" => "file_size ASC".to_string(),
                "size_DESC" => "file_size DESC".to_string(),
                _ => "upload_date DESC".to_string()
            }
        }
        None => "upload_date DESC".to_string()
    };

    let category_filter_query = if let Some(c) = categories {
        let mut i = 0;
        let mut category_filters = String::new();
        for category in c.iter() {
            // don't take user input in the db query
            if let Some(sanitized_category) = &app_data.database.verify_category(category).await {
                let mut str = format!("tc.name = '{}'", sanitized_category);
                if i > 0 { str = format!(" OR {}", str); }
                category_filters.push_str(&str);
                i += 1;
            }
        }
        if category_filters.len() > 0 {
            format!("INNER JOIN torrust_categories tc ON tt.category_id = tc.category_id AND ({})", category_filters)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let mut query_string = format!("SELECT tt.* FROM torrust_torrents tt {} WHERE hidden = false AND title LIKE ?", category_filter_query);
    let count_query_string = format!("SELECT COUNT(torrent_id) as count FROM ({})", query_string);

    let count: TorrentCount = sqlx::query_as::<_, TorrentCount>(&count_query_string)
        .bind(search.clone())
        .fetch_one(&app_data.database.pool)
        .await?;

    query_string = format!("{} ORDER BY {} LIMIT ?, ?", query_string, sort_query);

    let res: Vec<TorrentListing> = sqlx::query_as::<_, TorrentListing>(&query_string)
        .bind(search)
        .bind(offset)
        .bind(page_size)
        .fetch_all(&app_data.database.pool).await?;

    let torrents_response = TorrentsResponse {
        total: count.count as u32,
        results: res
    };

    Ok(HttpResponse::Ok().json(OkResponse {
        data: torrents_response
    }))
}


pub async fn get_torrent(req: HttpRequest, app_data: WebAppData) -> ServiceResult<impl Responder> {
    let settings = app_data.cfg.settings.read().await;

    let torrent_id = get_torrent_id_from_request(&req)?;

    let torrent_listing = app_data.database.get_torrent_by_id(torrent_id).await?;
    let mut torrent_response = TorrentResponse::from_listing(torrent_listing);

    let filepath = format!("{}/{}", settings.storage.upload_path, torrent_response.torrent_id.to_string() + ".torrent");

    drop(settings);

    if let Ok(torrent) = parse_torrent::read_torrent_from_file(&filepath) {
        if let Some(files) = torrent.info.files {
            torrent_response.files = Some(files);
        } else {
            // todo: tidy up this code, it's error prone
            let file = File {
                path: vec![torrent.info.name],
                length: torrent.info.length.unwrap_or(0),
                md5sum: None
            };

            torrent_response.files = Some(vec![file]);
        }
    }

    // get realtime seeders and leechers
    if let Ok(torrent_info) = app_data.tracker.get_torrent_info(&torrent_response.info_hash).await {
        torrent_response.seeders = torrent_info.seeders;
        torrent_response.leechers = torrent_info.leechers;
    }

    Ok(HttpResponse::Ok().json(OkResponse {
        data: torrent_response
    }))
}

pub async fn delete_torrent(req: HttpRequest, app_data: WebAppData) -> ServiceResult<impl Responder> {
    let user = app_data.auth.get_user_from_request(&req).await?;

    // check if user is administrator
    if !user.administrator { return Err(ServiceError::Unauthorized) }

    let torrent_id = get_torrent_id_from_request(&req)?;

    let res = sqlx::query!(
        "UPDATE torrust_torrents SET hidden = true WHERE torrent_id = ?",
        torrent_id
    )
        .execute(&app_data.database.pool)
        .await;

    match res {
        Ok(_) => (),
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    Ok(HttpResponse::Ok().json(OkResponse {
        data: NewTorrentResponse {
            torrent_id
        }
    }))
}

pub async fn upload_torrent(req: HttpRequest, payload: Multipart, app_data: WebAppData) -> ServiceResult<impl Responder> {
    let user = app_data.auth.get_user_from_request(&req).await?;

    let mut torrent_request = get_torrent_request_from_payload(payload).await?;

    // update announce url to our own tracker url
    torrent_request.torrent.set_torrust_config(&app_data.cfg).await;

    let res = sqlx::query!(
        "SELECT category_id FROM torrust_categories WHERE name = ?",
        torrent_request.fields.category
    )
        .fetch_one(&app_data.database.pool)
        .await;

    let row = match res {
        Ok(row) => row,
        Err(_) => return Err(ServiceError::InvalidCategory),
    };

    let username = user.username;
    let info_hash = torrent_request.torrent.info_hash();
    let title = torrent_request.fields.title;
    //let category = torrent_request.fields.category;
    let description = torrent_request.fields.description;
    //let current_time = current_time() as i64;
    let file_size = torrent_request.torrent.file_size();
    let mut seeders = 0;
    let mut leechers = 0;

    if let Ok(torrent_info) = app_data.tracker.get_torrent_info(&info_hash).await {
        seeders = torrent_info.seeders;
        leechers = torrent_info.leechers;
    }

    // println!("{:?}", (&username, &info_hash, &title, &category, &description, &current_time, &file_size));

    let torrent_id = app_data.database.insert_torrent_and_get_id(username, info_hash, title, row.category_id, description, file_size, seeders, leechers).await?;

    // whitelist info hash on tracker
    let _r = app_data.tracker.whitelist_info_hash(torrent_request.torrent.info_hash()).await;

    let settings = app_data.cfg.settings.read().await;

    let filepath = format!("{}/{}", settings.storage.upload_path, torrent_id.to_string() + ".torrent");

    drop(settings);

    save_torrent_file(&filepath, &torrent_request.torrent).await?;

    Ok(HttpResponse::Ok().json(OkResponse {
        data: NewTorrentResponse {
            torrent_id
        }
    }))
}

pub async fn download_torrent(req: HttpRequest, app_data: WebAppData) -> ServiceResult<impl Responder> {
    let torrent_id = get_torrent_id_from_request(&req)?;

    let settings = app_data.cfg.settings.read().await;

    // optional
    let user = app_data.auth.get_user_from_request(&req).await;

    let filepath = format!("{}/{}", settings.storage.upload_path, torrent_id.to_string() + ".torrent");

    let mut torrent = match parse_torrent::read_torrent_from_file(&filepath) {
        Ok(torrent) => Ok(torrent),
        Err(e) => {
            println!("{:?}", e);
            Err(ServiceError::InternalServerError)
        }
    }?;

    if user.is_ok() {
        let unwrapped_user = user.unwrap();
        let personal_announce_url = app_data.tracker.get_personal_announce_url(&unwrapped_user).await?;
        torrent.announce = Some(personal_announce_url.clone());
        if let Some(list) = &mut torrent.announce_list {
            let mut vec = Vec::new();
            vec.push(personal_announce_url);
            list.insert(0, vec);
        }
    } else {
        torrent.announce = Some(settings.tracker.url.clone());
    }

    drop(settings);

    let buffer = match parse_torrent::encode_torrent(&torrent) {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{:?}", e);
            Err(ServiceError::InternalServerError)
        }
    }?;

    Ok(HttpResponse::Ok()
        .content_type("application/x-bittorrent")
        .body(buffer)
    )
}

// async fn verify_torrent_ownership(user: &User, torrent_listing: &TorrentListing) -> Result<(), ServiceError> {
//     match torrent_listing.uploader == user.username {
//         true => Ok(()),
//         false => Err(ServiceError::BadRequest)
//     }
// }

async fn save_torrent_file(filepath: &str, torrent: &Torrent) -> Result<(), ServiceError> {
    let torrent_bytes = match parse_torrent::encode_torrent(torrent) {
        Ok(v) => Ok(v),
        Err(_) => Err(ServiceError::InternalServerError)
    }?;

    let mut f = match async_std::fs::File::create(&filepath).await {
        Ok(v) => Ok(v),
        Err(_) => Err(ServiceError::InternalServerError)
    }?;

    match AsyncWriteExt::write_all(&mut f, &torrent_bytes.as_slice()).await {
        Ok(v) => Ok(v),
        Err(_) => Err(ServiceError::InternalServerError)
    }?;

    Ok(())
}

fn get_torrent_id_from_request(req: &HttpRequest) -> Result<i64, ServiceError> {
    match req.match_info().get("id") {
        None => Err(ServiceError::BadRequest),
        Some(torrent_id) => {
            match torrent_id.parse() {
                Err(_) => Err(ServiceError::BadRequest),
                Ok(v) => Ok(v)
            }
        }
    }
}

async fn get_torrent_request_from_payload(mut payload: Multipart) -> Result<TorrentRequest, ServiceError> {
    let torrent_buffer = vec![0u8];
    let mut torrent_cursor = Cursor::new(torrent_buffer);

    let mut title = "".to_string();
    let mut description = "".to_string();
    let mut category = "".to_string();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap();

        match name {
            "title" | "description" | "category" => {
                let data = field.next().await;
                if data.is_none() { continue }
                let wrapped_data = &data.unwrap().unwrap();
                let parsed_data = std::str::from_utf8(&wrapped_data).unwrap();

                match name {
                    "title" => { title = parsed_data.to_string() }
                    "description" => { description = parsed_data.to_string() }
                    "category" => { category = parsed_data.to_string() }
                    _ => {}
                }
            }
            "torrent" => {
                if field.content_type().to_string() != "application/x-bittorrent" {
                    return Err(ServiceError::InvalidFileType)
                }

                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    torrent_cursor.write_all(&data)?;
                }
            }
            _ => {}
        }
    }

    let fields = CreateTorrent {
        title,
        description,
        category,
    };

    fields.verify()?;

    let position = torrent_cursor.position() as usize;
    let inner = torrent_cursor.get_ref();

    let torrent = match parse_torrent::decode_torrent(&inner[..position]) {
        Ok(torrent) => Ok(torrent),
        Err(_) => Err(ServiceError::InvalidTorrentFile)
    }?;

    Ok(TorrentRequest {
        fields,
        torrent,
    })
}
