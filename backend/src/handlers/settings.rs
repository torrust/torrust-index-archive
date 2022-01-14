use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::common::WebAppData;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::response::{CategoryResponse, OkResponse};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/settings")
            .service(web::resource("")
                .route(web::get().to(get_settings))
            )
            .service(web::resource("/name")
                .route(web::get().to(get_site_name))
                //.route(web::post().to(update_site_name))
            )
    );
}

pub async fn get_settings(req: HttpRequest, app_data: WebAppData) -> ServiceResult<impl Responder> {
    // check for user
    let user = app_data.auth.get_user_from_request(&req).await?;

    // check if user is administrator
    if !user.administrator { return Err(ServiceError::Unauthorized) }

    Ok(HttpResponse::Ok().json(OkResponse {
        data: &app_data.cfg
    }))
}

pub async fn get_site_name(app_data: WebAppData) -> ServiceResult<impl Responder> {
    Ok(HttpResponse::Ok().json(OkResponse {
        data: &app_data.cfg.website.name
    }))
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SiteName {
//     pub name: String
// }
//
// pub async fn update_site_name(req: HttpRequest, payload: web::Json<SiteName>, app_data: WebAppData) -> ServiceResult<impl Responder> {
//     // check for user
//     let user = app_data.auth.get_user_from_request(&req).await?;
//
//     // check if user is administrator
//     if !user.administrator { return Err(ServiceError::Unauthorized) }
//
//     let update =
//
//     if let Err(sqlx::Error::Database(err)) = res {
//         return if err.message().contains("UNIQUE") {
//             Err(ServiceError::CategoryExists)
//         } else {
//             Err(ServiceError::InternalServerError)
//         }
//     }
//
//     Ok(HttpResponse::Ok().json(OkResponse {
//         data: payload.name.clone()
//     }))
// }
