use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::users;

#[derive(Queryable, Serialize)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub avatar: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub guid: String,
    pub name: String,
    pub email: String,
    pub avatar: String,
}

#[derive(AsChangeset, Queryable, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub avatar: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}