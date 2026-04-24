use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Создаёт пул подключений к PostgreSQL и выполняет миграцию (создание таблицы)
pub async fn init_db(database_url: &str) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Не удалось подключиться к базе данных PostgreSQL");

    // Создаём таблицу users, если она ещё не существует
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id          SERIAL PRIMARY KEY,
            first_name  VARCHAR(100) NOT NULL,
            last_name   VARCHAR(100) NOT NULL,
            age         INTEGER NOT NULL,
            created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at  TIMESTAMP NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("Не удалось создать таблицу users");

    log::info!("База данных инициализирована, таблица users готова");
    pool
}
