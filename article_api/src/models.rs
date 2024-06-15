use super::schema::articles;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub published_at: NaiveDateTime,
    pub is_published: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub content: String,
    pub is_published: bool,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "articles"]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
    pub published_at: Option<NaiveDateTime>,
}
