# DB Postgres API — Управление пользователями

REST API для управления списком пользователей, написанное на **Rust** с использованием фреймворка **Actix-Web** и базы данных **PostgreSQL**.

---

## 📋 Содержание

- [Технологии](#-технологии)
- [Требования](#-требования)
- [Установка и настройка PostgreSQL](#-установка-и-настройка-postgresql)
- [Подключение базы данных к проекту](#-подключение-базы-данных-к-проекту)
- [Запуск приложения](#-запуск-приложения)
- [Структура проекта](#-структура-проекта)
- [Модель данных](#-модель-данных)
- [API Эндпоинты](#-api-эндпоинты)
- [Примеры запросов](#-примеры-запросов-curl)

---

## 🛠 Технологии

| Технология | Версия | Назначение |
|---|---|---|
| Rust | 1.75+ | Язык программирования |
| Actix-Web | 4.x | HTTP-фреймворк |
| SQLx | 0.7.x | Асинхронный драйвер PostgreSQL |
| PostgreSQL | 14+ | Реляционная база данных |
| Serde | 1.x | Сериализация / десериализация JSON |
| Chrono | 0.4.x | Работа с датами и временем |

---

## 📦 Требования

Перед началом работы убедитесь, что на вашем компьютере установлены:

1. **Rust** (включая `cargo`) — [https://rustup.rs](https://rustup.rs)
   ```bash
   # Проверка установки
   rustc --version
   cargo --version
   ```

2. **PostgreSQL** (сервер базы данных) — [https://www.postgresql.org/download/](https://www.postgresql.org/download/)
   ```bash
   # Проверка установки
   psql --version
   ```

---

## 🐘 Установка и настройка PostgreSQL

### Шаг 1. Установка PostgreSQL

**macOS (через Homebrew):**
```bash
brew install postgresql@16
brew services start postgresql@16
```

**Ubuntu / Debian:**
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

**Windows:**
Скачайте установщик с [официального сайта](https://www.postgresql.org/download/windows/) и следуйте инструкции мастера установки.

### Шаг 2. Создание базы данных

Подключитесь к PostgreSQL и создайте базу данных:

```bash
# Вход в консоль PostgreSQL (от имени пользователя postgres)
psql -U postgres
```

Внутри консоли `psql` выполните:

```sql
-- Создаём базу данных для проекта
CREATE DATABASE online_banking;

-- (Опционально) Создаём отдельного пользователя
CREATE USER banking_user WITH PASSWORD 'my_secure_password';

-- Даём права на базу данных
GRANT ALL PRIVILEGES ON DATABASE online_banking TO banking_user;

-- Выход из psql
\q
```

### Шаг 3. Проверка подключения

Убедитесь, что можете подключиться к созданной базе:

```bash
psql -U postgres -d online_banking
```

Если подключение прошло успешно, вы увидите приглашение `online_banking=#`.

---

## 🔗 Подключение базы данных к проекту

### Как это работает

Проект использует библиотеку **SQLx** для асинхронного взаимодействия с PostgreSQL. Подключение настраивается через переменную окружения `DATABASE_URL`.

### Шаг 1. Настройка файла `.env`

В корне проекта находится файл `.env`. Откройте его и укажите параметры подключения к вашей базе данных:

```env
DATABASE_URL=postgres://<пользователь>:<пароль>@<хост>:<порт>/<имя_базы>
```

**Примеры:**

```env
# Стандартное подключение (пользователь postgres, без пароля, локальный сервер)
DATABASE_URL=postgres://postgres@localhost:5432/online_banking

# С паролем
DATABASE_URL=postgres://postgres:postgres@localhost:5432/online_banking

# С отдельным пользователем
DATABASE_URL=postgres://banking_user:my_secure_password@localhost:5432/online_banking
```

### Шаг 2. Формат DATABASE_URL

```
postgres://<USER>:<PASSWORD>@<HOST>:<PORT>/<DATABASE>
```

| Параметр | Описание | Пример |
|---|---|---|
| `USER` | Имя пользователя PostgreSQL | `postgres` |
| `PASSWORD` | Пароль пользователя | `postgres` |
| `HOST` | Адрес сервера БД | `localhost` или `127.0.0.1` |
| `PORT` | Порт PostgreSQL (по умолчанию 5432) | `5432` |
| `DATABASE` | Имя базы данных | `online_banking` |

### Шаг 3. Автоматическая миграция

При запуске приложение **автоматически создаёт таблицу** `users`, если она ещё не существует. Вам не нужно вручную выполнять SQL-скрипты — всё происходит в файле `src/db/init.rs`:

```sql
CREATE TABLE IF NOT EXISTS users (
    id          SERIAL PRIMARY KEY,
    first_name  VARCHAR(100) NOT NULL,
    last_name   VARCHAR(100) NOT NULL,
    age         INTEGER NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP NOT NULL DEFAULT NOW()
);
```

### Шаг 4. Пул подключений

Приложение создаёт **пул из 5 подключений** к базе данных (настраивается в `src/db/init.rs`). Это позволяет эффективно обрабатывать несколько запросов одновременно без создания нового подключения для каждого запроса.

---

## 🚀 Запуск приложения

### 1. Клонируйте / откройте проект

```bash
cd ~/Desktop/19-20
```

### 2. Убедитесь, что PostgreSQL запущен

```bash
# macOS
brew services list | grep postgresql

# Linux
sudo systemctl status postgresql
```

### 3. Запустите приложение

```bash
# Обычный запуск
cargo run

# С логированием (рекомендуется)
RUST_LOG=info cargo run
```

При успешном запуске вы увидите в консоли:

```
[INFO] База данных инициализирована, таблица users готова
[INFO] Сервер запускается на http://127.0.0.1:8080
```

Сервер будет доступен по адресу: **http://127.0.0.1:8080**

---

## 📁 Структура проекта

```
19-20/
├── .env                          # Переменные окружения (DATABASE_URL)
├── Cargo.toml                    # Зависимости проекта
├── README.md                     # Документация (этот файл)
└── src/
    ├── main.rs                   # Точка входа: инициализация БД и маршрутов
    ├── db/
    │   ├── mod.rs                # Модуль базы данных
    │   └── init.rs               # Подключение к PostgreSQL и миграция
    ├── handlers/
    │   ├── mod.rs                # Модуль обработчиков
    │   └── user_handlers.rs      # CRUD-обработчики для пользователей
    └── models/
        ├── mod.rs                # Модуль моделей
        └── user.rs               # Модели User, CreateUser, UpdateUser
```

---

## 📊 Модель данных

### Таблица `users`

| Поле | Тип данных | Описание |
|---|---|---|
| `id` | `SERIAL (Integer)` | Уникальный идентификатор (автоинкремент) |
| `first_name` | `VARCHAR(100)` | Имя пользователя |
| `last_name` | `VARCHAR(100)` | Фамилия пользователя |
| `age` | `INTEGER` | Возраст пользователя |
| `created_at` | `TIMESTAMP` | Время создания записи |
| `updated_at` | `TIMESTAMP` | Время последнего обновления |

---

## 📡 API Эндпоинты

| Метод | Адрес | Описание |
|---|---|---|
| `POST` | `/api/users` | Создание нового пользователя |
| `GET` | `/api/users` | Получение списка всех пользователей |
| `GET` | `/api/users/{id}` | Получение конкретного пользователя по ID |
| `PATCH` | `/api/users/{id}` | Обновление информации пользователя |
| `DELETE` | `/api/users/{id}` | Удаление пользователя |

---

## 📝 Примеры запросов (curl)

### 1. Создание пользователя

```bash
curl -X POST http://127.0.0.1:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Иван",
    "last_name": "Петров",
    "age": 25
  }'
```

**Ответ (201 Created):**
```json
{
  "id": 1,
  "first_name": "Иван",
  "last_name": "Петров",
  "age": 25,
  "created_at": "2026-04-24T00:15:00",
  "updated_at": "2026-04-24T00:15:00"
}
```

### 2. Получение списка всех пользователей

```bash
curl http://127.0.0.1:8080/api/users
```

**Ответ (200 OK):**
```json
[
  {
    "id": 1,
    "first_name": "Иван",
    "last_name": "Петров",
    "age": 25,
    "created_at": "2026-04-24T00:15:00",
    "updated_at": "2026-04-24T00:15:00"
  },
  {
    "id": 2,
    "first_name": "Мария",
    "last_name": "Сидорова",
    "age": 30,
    "created_at": "2026-04-24T00:16:00",
    "updated_at": "2026-04-24T00:16:00"
  }
]
```

### 3. Получение пользователя по ID

```bash
curl http://127.0.0.1:8080/api/users/1
```

**Ответ (200 OK):**
```json
{
  "id": 1,
  "first_name": "Иван",
  "last_name": "Петров",
  "age": 25,
  "created_at": "2026-04-24T00:15:00",
  "updated_at": "2026-04-24T00:15:00"
}
```

**Если пользователь не найден (404 Not Found):**
```json
{
  "error": "Пользователь с id=999 не найден"
}
```

### 4. Обновление пользователя

Можно обновить **одно, несколько или все** поля — PATCH-запрос принимает только те поля, которые нужно изменить:

```bash
# Обновить только имя
curl -X PATCH http://127.0.0.1:8080/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Алексей"
  }'

# Обновить имя и возраст
curl -X PATCH http://127.0.0.1:8080/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Алексей",
    "age": 26
  }'
```

**Ответ (200 OK):**
```json
{
  "id": 1,
  "first_name": "Алексей",
  "last_name": "Петров",
  "age": 26,
  "created_at": "2026-04-24T00:15:00",
  "updated_at": "2026-04-24T00:20:00"
}
```

### 5. Удаление пользователя

```bash
curl -X DELETE http://127.0.0.1:8080/api/users/1
```

**Ответ (200 OK):**
```json
{
  "message": "Пользователь с id=1 успешно удалён"
}
```

**Если пользователь не найден (404 Not Found):**
```json
{
  "error": "Пользователь с id=1 не найден"
}
```

---

## ⚠️ Возможные ошибки

| Ошибка | Причина | Решение |
|---|---|---|
| `Не удалось подключиться к базе данных PostgreSQL` | PostgreSQL не запущен или неверный `DATABASE_URL` | Проверьте, что PostgreSQL работает и `.env` настроен правильно |
| `connection refused` | Сервер БД недоступен | Запустите PostgreSQL: `brew services start postgresql` |
| `password authentication failed` | Неверный пароль в `DATABASE_URL` | Исправьте пароль в файле `.env` |
| `database "online_banking" does not exist` | База данных не создана | Выполните `createdb online_banking` или создайте через `psql` |
