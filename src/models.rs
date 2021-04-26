use crate::schema::items;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Item {
    pub id: i32,
    pub item_id: String,
    pub name: String,
    pub picture: Option<String>,
    pub price: i32,
    pub stock: i32,
    pub owner: String,
    pub posted_timestamp: NaiveDateTime,
}

