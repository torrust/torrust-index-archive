use std::sync::Arc;
use actix_web::{App, HttpServer, middleware, web};
use actix_cors::Cors;
use torrust::database::Database;
use torrust::{handlers};
use torrust::config::TorrustConfig;
use torrust::common::AppData;
use torrust::auth::AuthorizationService;
use torrust::tracker::TrackerService;
use torrust::mailer::MailerService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = match TorrustConfig::load_from_file() {
        Ok(config) => Arc::new(config),
        Err(error) => {
            panic!("{}", error)
        }
    };

    let database = Arc::new(Database::new(&cfg.database.connect_url).await);
    let auth = Arc::new(AuthorizationService::new(cfg.clone(), database.clone()));
    let tracker_service = Arc::new(TrackerService::new(cfg.clone(), database.clone()));
    let mailer_service = Arc::new(MailerService::new(cfg.clone()));
    let app_data = Arc::new(
        AppData::new(
            cfg.clone(),
            database.clone(),
            auth.clone(),
            tracker_service.clone(),
            mailer_service.clone(),
        )
    );

    // create/update database tables
    sqlx::migrate!().run(&database.pool).await.unwrap();

    // create torrent upload folder
    async_std::fs::create_dir_all(&cfg.storage.upload_path).await?;

    let interval = cfg.database.torrent_info_update_interval;
    let weak_tracker_service = std::sync::Arc::downgrade(&tracker_service);

    // repeating task, update all seeders and leechers info
    tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(interval);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await; // first tick is immediate...
        loop {
            interval.tick().await;
            if let Some(tracker) = weak_tracker_service.upgrade() {
                let _ = tracker.update_torrents().await;
            } else {
                break;
            }
        }
    });

    println!("Listening on 0.0.0.0:{}", cfg.net.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(app_data.clone()))
            .wrap(middleware::Logger::default())
            .configure(handlers::init_routes)
    })
        .bind(("0.0.0.0", cfg.net.port))?
        .run()
        .await
}
