mod config;
mod errors;

use crate::errors::CustomError;
use axum::{extract::Extension, response::{Html}, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(goals))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn goals(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::goals::get_goals()
        .bind(&client)
        .all()
        .await?;

    Ok(Html(ui_components::goals::users(users)))
}