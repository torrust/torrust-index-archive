use crate::common::WebAppData;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::response::OkResponse;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{ Deserialize, Serialize };
use std::option::Option::Some;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/page")
           .service(web::resource("")
                .route(web::put().to(update_page))
                .route(web::delete().to(delete_page))
                .route(web::post().to(add_page)))
            .service(web::resource("/{route}")
                .route(web::get().to(get_page)))
    );
    cfg.service(web::scope("/pages").service(web::resource("").route(web::get().to(get_pages))));
}

#[derive(Debug, Deserialize)]
pub struct CreatePage {
    pub route: String,
    pub title: String,
    pub description: String,
}

impl CreatePage {
    pub fn verify(&self) -> Result<(), ServiceError> {
        if !self.title.is_empty() && !self.route.is_empty() {
            return Ok(());
        }

        Err(ServiceError::BadRequest)
    }
}

pub async fn get_page(
    req: HttpRequest,
    app_data: WebAppData,
) -> ServiceResult<impl Responder> {
    let route = match req.match_info().get("route") {
        None => return Err(ServiceError::BadRequest),
        Some(route) => route
    };
    let res = app_data.database.get_page_by_route(&route).await;
    if let Some(page) = res {
        Ok(HttpResponse::Ok().json(OkResponse { data: page }))
    } else {
        Err(ServiceError::RouteNotFound)
    }
}

pub async fn get_pages(app_data: WebAppData) -> ServiceResult<impl Responder> {
    let res = app_data.database.get_pages().await.unwrap();
    let pages = res;
    Ok(HttpResponse::Ok().json(OkResponse { data: pages }))
}

#[derive(Debug, Deserialize)]
pub struct PageUpdate {
    description: Option<String>,
    title: Option<String>,
    route: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageCreation {
    description: Option<String>,
    title: String,
    route: String,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    route: String,
}

// Update A route
// PUT /page
pub async fn update_page(
    req: HttpRequest,
    payload: web::Json<PageUpdate>,
    app_data: WebAppData,
) -> ServiceResult<impl Responder> {
    
    let user = app_data.auth.get_user_from_request(&req).await?;
    if !user.administrator {
        return Err(ServiceError::Unauthorized);
    }

    let route = &payload.route;
    let res = app_data.database.get_page_by_route(&route).await;

    let page = match res {
        Some(page) => page,
        None => return Err(ServiceError::RouteNotFound),
    };
    let description = match &payload.description {
        Some(_) => &payload.description,
        None => &page.description,
    };
    let title = payload.title.as_ref().unwrap_or(&page.title);
    let res = sqlx::query!(
        "UPDATE torrust_pages SET description = $1, title = $2 WHERE route=$3",
        description,
        title,
        route
    )
    .execute(&app_data.database.pool)
    .await;

    if let Err(_) = res {
        return Err(ServiceError::RouteNotFound);
    }

    if res.unwrap().rows_affected() == 0 {
        return Err(ServiceError::RouteNotFound);
    }

    Ok(HttpResponse::Ok())
}

pub async fn delete_page(
    req: HttpRequest,
    payload: web::Json<PageUpdate>,
    app_data: WebAppData,
) -> ServiceResult<impl Responder> {

    let user = app_data.auth.get_user_from_request(&req).await?;

    // check if user is administrator
    if !user.administrator {
        return Err(ServiceError::Unauthorized);
    }
    let route = &payload.route;
    let res = sqlx::query!("DELETE FROM torrust_pages WHERE route = ?", route)
        .execute(&app_data.database.pool)
        .await;

    if let Err(_) = res {
        return Err(ServiceError::RouteNotFound);
    }
    if res.unwrap().rows_affected() == 0 {
        return Err(ServiceError::RouteNotFound);
    }

    Ok(HttpResponse::Ok())
}

pub async fn add_page(
    req: HttpRequest,
    payload: web::Json<PageCreation>,
    app_data: WebAppData,
) -> ServiceResult<impl Responder> {

    let user = app_data.auth.get_user_from_request(&req).await?;
    // check if user is administrator
    if !user.administrator {
        return Err(ServiceError::Unauthorized);
    }

    let route = &payload.route;
    if !route.chars().all(char::is_alphanumeric) {
        return Err(ServiceError::BadRequest);
    }
    let title = &payload.title;
    let description = &payload.description;

    app_data
        .database
        .insert_page(&route, title, description)
        .await?;

    Ok(HttpResponse::Ok().json( OkResponse {
        data: payload
    }))
}
