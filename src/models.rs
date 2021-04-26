use crate::schema::items;
use chrono::NaiveDateTime;
use rocket::{
    data::{FromDataSimple, Outcome},
    http::{ContentType, Status},
    Data, Request,
};
use std::io::Read;

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

impl FromDataSimple for Item {
    type Error = String;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        if request.content_type() != Some(&ContentType::JSON) {
            debug!("{:#?}", request);
            return Outcome::Failure((
                Status::UnsupportedMediaType,
                "Data must be in JSON format".into(),
            ));
        }

        let mut string = String::new();
        if let Err(e) = data.open().take(1024).read_to_string(&mut string) {
            debug!("{:#?}", request);
            return Outcome::Failure((Status::BadRequest, format!("{}", e)));
        }

        let item: Self = match serde_json::from_str(&string) {
            Ok(i) => i,
            Err(e) => {
                debug!("{:#?}", request);
                return Outcome::Failure((Status::BadRequest, format!("{}", e)));
            },
        };

        Outcome::Success(item)
    }
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "items"]
pub struct NewItem {
    pub item_id: String,
    pub name: String,
    pub picture: String,
    pub price: i32,
    pub stock: i32,
    pub owner: String,
}

impl FromDataSimple for NewItem {
    type Error = String;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        if request.content_type() != Some(&ContentType::JSON) {
            debug!("{:#?}", request);
            return Outcome::Failure((
                Status::UnsupportedMediaType,
                "Data must be in JSON format".into(),
            ));
        }

        let mut string = String::new();
        if let Err(e) = data.open().take(1024).read_to_string(&mut string) {
            debug!("{:#?}", request);
            return Outcome::Failure((Status::BadRequest, format!("{}", e)));
        }

        let item: Self = match serde_json::from_str(&string) {
            Ok(i) => i,
            Err(e) => {
                debug!("{:#?}", request);
                return Outcome::Failure((Status::BadRequest, format!("{}", e)));
            },
        };

        Outcome::Success(item)
    }
}
