use crate::schema::items;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Item {
    pub item_id: i32,
    pub name: String,
    pub price: i32,
    pub stock: i32,
    pub owner: String,
    pub posted_timestamp: NaiveDateTime,
}
