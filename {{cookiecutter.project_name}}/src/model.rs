use crate::schema::{{cookiecutter.initial_db_table_name}};
use diesel::deserialize::FromSql;
use diesel::{deserialize, serialize};
use diesel::pg::Pg;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Queryable)]
pub struct ItemModel {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "{{cookiecutter.initial_db_table_name}}"]
pub struct NewItemModel {
    pub name: String,
}
