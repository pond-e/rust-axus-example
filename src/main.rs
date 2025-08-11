use axum::{
    routing::get,
    Router,
    response::Html,
    extract::Form
};

use serde::Deserialize;

#[tokio::main]
async fn main() {
    // build our application with some routes
    let app = app();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(show_form).post(accept_form))
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <html>
            <body>
                <form action="/" method="post">
                    <input type="text" name="name" />
                </form>
            </body>
        </html>
        "#
    )
}

#[derive(Deserialize)]
struct FormData {
    name: String,
}

async fn accept_form(Form(form): Form<FormData>) -> Html<String> {
    Html(format!("Hello, {}!", form.name))
}