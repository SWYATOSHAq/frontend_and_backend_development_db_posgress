use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::user::{CreateUser, UpdateUser, User};

/// POST /api/users — Создание нового пользователя
pub async fn create_user(
    pool: web::Data<PgPool>,
    body: web::Json<CreateUser>,
) -> impl Responder {
    let result = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (first_name, last_name, age, created_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        RETURNING id, first_name, last_name, age, created_at, updated_at
        "#,
    )
    .bind(&body.first_name)
    .bind(&body.last_name)
    .bind(body.age)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            log::error!("Ошибка при создании пользователя: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Не удалось создать пользователя",
                "details": e.to_string()
            }))
        }
    }
}

/// GET /api/users — Получение списка всех пользователей
pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>(
        "SELECT id, first_name, last_name, age, created_at, updated_at FROM users ORDER BY id",
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            log::error!("Ошибка при получении списка пользователей: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Не удалось получить список пользователей",
                "details": e.to_string()
            }))
        }
    }
}

/// GET /api/users/{id} — Получение конкретного пользователя по ID
pub async fn get_user_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query_as::<_, User>(
        "SELECT id, first_name, last_name, age, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Пользователь с id={} не найден", user_id)
        })),
        Err(e) => {
            log::error!("Ошибка при получении пользователя id={}: {:?}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Не удалось получить пользователя",
                "details": e.to_string()
            }))
        }
    }
}

/// PATCH /api/users/{id} — Обновление информации пользователя
pub async fn update_user(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<UpdateUser>,
) -> impl Responder {
    let user_id = path.into_inner();

    // Сначала проверяем, существует ли пользователь
    let existing = sqlx::query_as::<_, User>(
        "SELECT id, first_name, last_name, age, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await;

    let existing_user = match existing {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("Пользователь с id={} не найден", user_id)
            }));
        }
        Err(e) => {
            log::error!("Ошибка при поиске пользователя id={}: {:?}", user_id, e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Ошибка базы данных",
                "details": e.to_string()
            }));
        }
    };

    // Обновляем только те поля, которые переданы в запросе
    let new_first_name = body.first_name.clone().unwrap_or(existing_user.first_name);
    let new_last_name = body.last_name.clone().unwrap_or(existing_user.last_name);
    let new_age = body.age.unwrap_or(existing_user.age);

    let result = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET first_name = $1, last_name = $2, age = $3, updated_at = NOW()
        WHERE id = $4
        RETURNING id, first_name, last_name, age, created_at, updated_at
        "#,
    )
    .bind(&new_first_name)
    .bind(&new_last_name)
    .bind(new_age)
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            log::error!("Ошибка при обновлении пользователя id={}: {:?}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Не удалось обновить пользователя",
                "details": e.to_string()
            }))
        }
    }
}

/// DELETE /api/users/{id} — Удаление пользователя
pub async fn delete_user(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query("DELETE FROM users WHERE id = $1 RETURNING id")
        .bind(user_id)
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(_)) => HttpResponse::Ok().json(serde_json::json!({
            "message": format!("Пользователь с id={} успешно удалён", user_id)
        })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Пользователь с id={} не найден", user_id)
        })),
        Err(e) => {
            log::error!("Ошибка при удалении пользователя id={}: {:?}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Не удалось удалить пользователя",
                "details": e.to_string()
            }))
        }
    }
}
