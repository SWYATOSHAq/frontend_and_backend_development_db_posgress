use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Сущность «Пользователь» — полная модель из БД
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// DTO для создания нового пользователя (POST /api/users)
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

/// DTO для обновления пользователя (PATCH /api/users/:id)
/// Все поля опциональны — обновляем только то, что передано
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<i32>,
}
