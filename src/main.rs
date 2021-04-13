#[macro_use]
extern crate log;

mod routes;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;
use sqlx::PgPool;
use std::env;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        <h1>market-api index page</h1>
        Routes:
        <ul>
            <li>GET / -> shows this help page</li>
        </ul>
    "#,
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    let db_pool = PgPool::connect(&&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .route("/", web::get().to(index))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").unwrap_or(String::from("0.0.0.0"));
            let port = env::var("PORT").unwrap_or(String::from("8000"));
            server.bind(format!("{}:{}", host, port))?
        },
    };

    println!("Server is starting...");
    info!("Server is starting...");
    server.run().await?;

    Ok(())
}
