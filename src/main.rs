#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod actions;
mod models;
mod routes;
mod schema;
mod status_responders;

use anyhow::Result;
use dotenv::dotenv;
use rocket::config::{Config, Environment, Value};
use rocket_contrib::databases::diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use std::{collections::HashMap, env};

pub type DbConnection = MysqlConnection;

embed_migrations!();

#[database("db")]
pub struct Database(DbConnection);

fn main() -> Result<()> {
    dotenv().ok();

    let mut logger_build = env_logger::Builder::from_default_env();

    #[cfg(debug_assertions)]
    logger_build.filter_level(log::LevelFilter::Trace).init();

    #[cfg(not(debug_assertions))]
    logger_build
        .filter_module("market_api", log::LevelFilter::Debug)
        .filter_module("rocket", log::LevelFilter::Debug)
        .init();

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
    let secret_key =
        env::var("SECRET_KEY").expect("Rocket's SECRET_KEY environment variable not set");
    let database_url = format!(
        "mysql://{}:{}@{}:{}/market",
        database_user, database_password, database_host, database_port
    );

    let host = env::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port = env::var("PORT")
        .unwrap_or(String::from("8000"))
        .parse::<u16>()
        .expect("Could not parse supplied port as a valid u16");

    let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");

    diesel_migrations::run_pending_migrations(
        &pool.get().expect("Could not get DB connection"),
    )
    .expect("Could not run diesel migrations");

    let mut databases_config = HashMap::new();
    let mut databases = HashMap::new();

    databases_config.insert("url", Value::from(database_url));
    databases.insert("db", Value::from(databases_config));

    #[cfg(debug_assertions)]
    let environment = Environment::Development;
    #[cfg(not(debug_assertions))]
    let environment = Environment::Production;

    let config = Config::build(environment)
        .extra("databases", databases)
        .port(port)
        .address(host)
        .secret_key(secret_key)
        .finalize()
        .expect("Invalid config");

    rocket::custom(config)
        .attach(Database::fairing())
        .mount(
            "/",
            routes![
                // GET
                routes::index,
                routes::get_all_items,
                routes::get_item_by_id,
                // POST
                routes::add_item // DELETE
            ],
        )
        .register(catchers![
            status_responders::bad_request,
            status_responders::internal_server_error,
            status_responders::not_found,
        ])
        .launch();

    Ok(())
}
