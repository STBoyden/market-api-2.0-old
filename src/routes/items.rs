use crate::{actions, models::Item};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/items")]
pub fn get_all_items(db: crate::Database) -> Result<Json<Vec<Item>>, String> {
    info!("get_all_items called");

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
        return Err("No items listed currently on the market".into());
    }
}

// pub async fn get_item_by_id(
//     req: HttpRequest,
//     pool: web::Data<crate::DbPool>,
// ) -> Result<impl Responder, Error> {
//     let item_id: i32 = if let Ok(id) = req
//         .match_info()
//         .get("item_id")
//         .expect("Did not supply item_id")
//         .parse()
//     {
//         id
//     } else {
//         return Ok(
//             HttpResponse::Ok().body("Cannot parse supplied item_id as a valid
// i32")         );
//     };

//     info!("get_item_by_id called with an item_id of {}", item_id);

//     let connection = pool.get().expect("Could not get DB connection");

//     let item = web::block(move || actions::get_item_by_id(&connection,
// item_id))         .await
//         .map_err(|e| {
//             error!("{}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;

//     if let Some(item) = item {
//         info!("get_item_by_id returning {:?}", item);
//         Ok(HttpResponse::Ok().json(item))
//     } else {
//         info!("get_item_by_id returning \"does not exists\"");
//         Ok(HttpResponse::Ok().body(&format!(
//             "Item with ID \"{}\" does not exist on the market",
//             item_id
//         )))
//     }
// }
