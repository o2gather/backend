use crate::schema::{events, users, event_members, event_comments};
use chrono::naive::serde::{ts_seconds, ts_seconds_option};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
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

#[derive(Deserialize, Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub name: String,
    pub description: String,
    pub category: String,
    #[serde(with = "ts_seconds")]
    pub start_time: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub end_time: NaiveDateTime,
    pub min_amount: i64,
    pub max_amount: i64,
    #[serde(skip)]
    pub user_id: Uuid,
}

#[derive(AsChangeset, Queryable, Deserialize)]
#[diesel(table_name = events)]
pub struct UpdateEvent {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    #[serde(with = "ts_seconds_option")]
    #[serde(default)]
    pub start_time: Option<NaiveDateTime>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub end_time: Option<NaiveDateTime>,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = events)]
#[diesel(primary_key(id))]
pub struct Event {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: String,
    #[serde(with = "ts_seconds")]
    pub start_time: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub end_time: NaiveDateTime,
    pub min_amount: i64,
    pub max_amount: i64,
}

#[derive(Queryable, Serialize)]
pub struct EventMember {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub amount: i64,
}

#[derive(Serialize)]
pub struct EventWithMembers {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: String,
    #[serde(with = "ts_seconds")]
    pub start_time: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub end_time: NaiveDateTime,
    pub min_amount: i64,
    pub max_amount: i64,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<EventMember>>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = event_members)]
pub struct NewEventMember {
    #[serde(skip)]
    pub event_id: Uuid,
    #[serde(skip)]
    pub user_id: Uuid,
    pub amount: i64,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = event_comments)]
pub struct NewEventMsg {
    #[serde(skip)]
    pub event_id: Uuid,
    #[serde(skip)]
    pub user_id: Uuid,
    pub content: String,
}

#[derive(Serialize, Queryable)]
#[diesel(table_name = users)]
pub struct EventMsgUser {
    pub name: String,
    pub avatar: String,
}
#[derive(Serialize, Queryable)]
#[diesel(table_name = event_comments)]
pub struct EventMsg {
    pub user: EventMsgUser,
    pub content: String,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
}
