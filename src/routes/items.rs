use crate::{
    actions,
    models::{Item, NewItem},
};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/items")]
pub fn get_all_items(db: crate::Database) -> Result<Json<Vec<Item>>, String> {
    let items = actions::get_all_items(&*db).map_err(|e| {
        error!("{}", e);
        Status::InternalServerError.to_string()
    })?;

    if let Some(items) = items {
        if items.is_empty() {
            info!("get_all_items returning \"no items\"");
            return Err("No items listed currently on the market".into());
        }

        info!("get_all_items returning {} item(s)", items.len());
        Ok(Json(items))
    } else {
        info!("get_all_items returning \"no items\"");
        Err("No items listed currently on the market".into())
    }
}

#[get("/item/<item_id>")]
pub fn get_item_by_id(db: crate::Database, item_id: i32) -> Result<Json<Item>, String> {
    let item = actions::get_item_by_id(&*db, item_id).map_err(|e| {
        error!("{}", e);
        Status::InternalServerError.to_string()
    })?;

    if let Some(item) = item {
        info!("get_item_by_id returning item with ID '{}'", item_id);
        Ok(Json(item))
    } else {
        info!(
            "get_item_by_id no item returned as an ID of '{}' does not exist",
            item_id
        );
        Err(format!("Item with an ID of '{}' does not exist", item_id))
    }
}

#[post("/add_item", format = "application/json", data = "<item>")]
pub fn add_item(db: crate::Database, item: NewItem) -> Result<Json<bool>, String> {
    let result = actions::add_item(&*db, item.clone()).map_err(|e| {
        error!("{}", e);
        Status::InternalServerError.to_string()
    })?;

    if let Some(result) = result {
        if result == 0 {
            info!("add_item item with ({:?}) not added", item);
            return Err(format!("add_item item with ({:?}) not added", item));
        }

        info!("add_item {} row(s) changed", result);
        Ok(Json(true))
    } else {
        info!("add_item item with ({:?}) not added", item);
        Err(format!("add_item item with ({:?}) not added", item))
    }
}

// #[delete("/remove_item", format = "application/json", data = "<item>")]
// pub fn remove_item(db: crate::Database, item: Item) {}
