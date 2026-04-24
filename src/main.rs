mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer, middleware};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Загружаем переменные окружения из .env
    dotenv().ok();
    env_logger::init();

    // Читаем URL базы данных
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL должна быть установлена в .env файле");

    // Инициализируем пул подключений и создаём таблицу
    let pool = db::init::init_db(&database_url).await;

    log::info!("Сервер запускается на http://127.0.0.1:8080");

    // Запускаем HTTP-сервер
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            // Маршруты для пользователей
            .route("/api/users", web::post().to(handlers::user_handlers::create_user))
            .route("/api/users", web::get().to(handlers::user_handlers::get_users))
            .route("/api/users/{id}", web::get().to(handlers::user_handlers::get_user_by_id))
            .route("/api/users/{id}", web::patch().to(handlers::user_handlers::update_user))
            .route("/api/users/{id}", web::delete().to(handlers::user_handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
