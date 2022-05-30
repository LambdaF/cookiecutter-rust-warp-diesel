use crate::db_manager::DBManager;
use crate::model::NewItemModel;
use crate::AppError;
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct AddItem {
    pub name: String,
}

impl AddItem {
    pub fn to_model(&self) -> NewItemModel {
        NewItemModel {
            name : self.name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct IdResponse {
    pub id: i64,
}

impl IdResponse {
    pub fn new(id: i64) -> IdResponse {
        IdResponse { id }
    }
}

fn format_response<T: Serialize>(
    result: Result<T, AppError>,
    status: warp::http::StatusCode,
) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            status,
        )),
        Err(err) => {
            log::error!("Error while trying to format_response: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}

// GET all serializer
pub async fn list_items(db_manager: DBManager) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling list items");
    let result = db_manager.select_all();
    format_response(result, warp::http::StatusCode::OK)
}

//GET single item serializer
pub async fn list_item(
    item_id: i64,
    db_manager: DBManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling listing single item");
    let result = db_manager.select_single_item(item_id);
    format_response(result, warp::http::StatusCode::OK)
}

// POST create serializer
pub async fn add_item(
    db_manager: DBManager,
    new_item: AddItem,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling add item");
    let create_item = new_item.to_model();

    let id_response = db_manager
        .insert_item(create_item)
        .map(|item| IdResponse::new(item.id));

    format_response(id_response, warp::http::StatusCode::CREATED)
}

// DELETE serializer
pub async fn delete_item(
    item_id: i64,
    db_manager: DBManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling delete item");
    let result = db_manager.delete_item(item_id).map(|_| -> () { () });
    format_response(result, warp::http::StatusCode::NO_CONTENT)
}

