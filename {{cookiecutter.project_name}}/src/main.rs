#[macro_use]
extern crate diesel;

mod api;
mod db_manager;
mod errors;
mod model;
mod schema;

use crate::api::AddItem;
use crate::db_manager::DBManager;
use crate::errors::{AppError, ErrorType};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv;
use log::info;
use pretty_env_logger;
use serde::de::DeserializeOwned;
use std::env;
use warp::{reject, Filter};

type PostgresConnectionPool = Pool<ConnectionManager<PgConnection>>;

fn create_db_connection_pool(db_url: &str) -> PostgresConnectionPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Could not create connection pool")
}

fn filter_db_access(
    pool: PostgresConnectionPool,
) -> impl Filter<Extract = (DBManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PostgresConnectionPool| async move {
            match pool.get() {
                Ok(conn) => Ok(DBManager::new(conn)),
                Err(err) => Err(reject::custom(AppError::new(
                    format!("Error getting connection from pool: {}", err.to_string()).as_str(),
                    ErrorType::Internal,
                ))),
            }
        })
}

fn filter_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

// GET /{{cookiecutter.initial_db_table_name}}
fn list_all_items(
    pool: PostgresConnectionPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("{{cookiecutter.initial_db_table_name}}")
        .and(warp::get())
        .and(filter_db_access(pool))
        .and_then(api::list_items)
}

// GET /{{cookiecutter.initial_db_table_name}}/:id
fn list_single_item(
    pool: PostgresConnectionPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("{{cookiecutter.initial_db_table_name}}" / i64)
        .and(warp::get())
        .and(filter_db_access(pool))
        .and_then(api::list_item)
}

// POST /{{cookiecutter.initial_db_table_name}}
fn add_new_item(
    pool: PostgresConnectionPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("{{cookiecutter.initial_db_table_name}}")
        .and(warp::post())
        .and(filter_db_access(pool))
        .and(filter_json_body::<AddItem>())
        .and_then(api::add_item)
}

// DELETE /{{cookiecutter.initial_db_table_name}}/:id
fn delete_item(
    pool: PostgresConnectionPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("{{cookiecutter.initial_db_table_name}}" / i64)
        .and(warp::delete())
        .and(filter_db_access(pool))
        .and_then(api::delete_item)
}

fn create_routes(
    pool: PostgresConnectionPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / ..).and(
        add_new_item(pool.clone())
            .or(delete_item(pool.clone()))
            .or(list_all_items(pool.clone()))
            .or(list_single_item(pool.clone())),
    )
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env not set");
    let port = env::var("DEFAULT_WARP_PORT").expect("DEFAULT_WARP_PORT not set");
    let create_db_connection_pool = create_db_connection_pool(database_url.as_str());
    let routes = create_routes(create_db_connection_pool).recover(errors::handle_rejection);

    info!("Starting server on port {}", port);

    let port = port.parse::<u16>().unwrap();
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
