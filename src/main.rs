use axum::{
    routing::get,
    Router,
    response::Html,
    extract::Form,
    extract::Path,
};
use serde::Deserialize;
use dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
// use sqlx::types::chrono;

#[tokio::main]
async fn main() {
    // build our application with some routes
    let app = app();

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(show_form).post(accept_form))
        .route("/users/{email}", get(show_user_name))
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <html>
            <body>
                <form action="/" method="post">
                    <input type="text" name="name" />
                    <input type="email" name="email" />
                    <button type="submit">Submit</button>
                </form>
            </body>
        </html>
        "#
    )
}

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

// DB
// #[derive(Debug)]
// struct User {
//     id: Option<i64>,
//     name: String,
//     email: String,
//     address: Option<String>,
//     created_at: chrono::NaiveDateTime,
// }

async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    // .env fileがない場合はスキップ
    dotenv::dotenv().ok();
    
    // DATABASE_URLが設定されていない場合はデフォルト値を使用
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./database.db".to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}

#[derive(Debug)]
struct CreateUserRequest {
    name: String,
    email: String,
    address: Option<String>,
}

async fn create_user(
    pool: &sqlx::SqlitePool,
    request: CreateUserRequest,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "insert into users (name, email, address) values (?, ?, ?)",
        request.name,
        request.email,
        request.address
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

async fn accept_form(Form(form): Form<FormData>) -> Html<String> {
    let pool = create_pool().await.unwrap();
    let user_id = create_user(&pool, CreateUserRequest {
        name: form.name,
        email: form.email,
        address: None,
    }).await.unwrap();

    Html(format!("Hello, {}!", user_id))
}

async fn show_user_name(Path(email): Path<String>) -> Html<String> {
    let pool = create_pool().await.unwrap();
    let user_name: String = sqlx::query_scalar!(
        "select name from users where email = ?",
        email
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Html(format!("Hello, {}!", user_name))
}
