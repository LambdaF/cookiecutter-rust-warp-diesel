use crate::errors::{AppError, ErrorType};
use crate::model::{ItemModel, NewItemModel};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::schema::{{cookiecutter.initial_db_table_name}};
use crate::schema::{{cookiecutter.initial_db_table_name}}::dsl::*;


type PooledPostgresConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBManager {
    connection: PooledPostgresConnection,
}

impl DBManager {
    pub fn new(connection: PooledPostgresConnection) -> DBManager {
        DBManager { connection }
    }

    // INSERT handler
    pub fn insert_item(&self, newmodel: NewItemModel) -> Result<ItemModel, AppError> {
        diesel::insert_into({{cookiecutter.initial_db_table_name}}::table)
            .values(&newmodel)
            .get_result(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while creating item"))
    }

    // SELECT * handler
    pub fn select_all(&self) -> Result<Vec<ItemModel>, AppError> {
        {{cookiecutter.initial_db_table_name}} 
            .load(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while listing item"))
    }

    // DELETE handler
    pub fn delete_item(&self, item_id: i64) -> Result<usize, AppError> {
        let deleted = diesel::delete({{cookiecutter.initial_db_table_name}}.filter(id.eq(item_id)))
            .execute(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while deleting item"))?;

        if deleted == 0 {
            return Err(AppError::new("Item not found", ErrorType::NotFound));
        }

        Ok(deleted)
    }

    // SELECT specific item handler
    pub fn select_single_item(&self, item_id: i64) -> Result<Vec<ItemModel>, AppError> {
        {{cookiecutter.initial_db_table_name}} 
            .filter(id.eq(item_id))
            .load::<ItemModel>(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while listing specific item"))
    }
}
