#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;

mod actions;
mod models;
mod routes;
mod schema;

use {
    actix_web::{web, App, HttpResponse, HttpServer, Responder},
    anyhow::Result,
    diesel::{
        mysql::MysqlConnection,
        r2d2::{self, ConnectionManager},
    },
    dotenv::dotenv,
    listenfd::ListenFd,
    routes::*,
    std::env,
};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

embed_migrations!();

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        <h1>market-api index page</h1>
        Routes:
        <ul>
            <li>GET / -> shows this help page</li>
            <li>GET /items -> gets all items in JSON format</li>
        </ul>
    "#,
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut logger_build = env_logger::Builder::from_default_env();

    logger_build.filter_level(log::LevelFilter::Debug).init();

    let mut listenfd = ListenFd::from_env();

    let database_host =
        env::var("DATABASE_HOST").expect("DATABASE_HOST environment variable not set");
    let database_port = env::var("DATABASE_PORT")
        .expect("DATABASE_PORT environment variable not set")
        .parse::<u16>()
        .expect("Could not tranform value for DATABASE_PORT into valid u16");
    let database_user =
        env::var("MYSQL_USER").expect("MYSQL_USER environment variable not set");
    let database_password = env::var("MYSQL_ROOT_PASSWORD")
        .expect("MYSQL_ROOT_PASSWORD environment variable not set");
    let database_url = format!(
        "mysql://{}:{}@{}:{}/market",
        database_user, database_password, database_host, database_port
    );

    let host = env::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port = env::var("PORT").unwrap_or(String::from("8000"));

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");

    diesel_migrations::run_pending_migrations(
        &pool.get().expect("Could not get DB connection"),
    )
    .expect("Could not run diesel migrations");

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(index))
            .route("/items", web::get().to(get_all_items))
            .route("/item/{item_id}", web::get().to(get_item_by_id))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", host, port))?,
    };

    info!("Server is starting...");
    info!("Server (HOST: {}, PORT: {})", host, port);
    server.run().await?;

    Ok(())
}
