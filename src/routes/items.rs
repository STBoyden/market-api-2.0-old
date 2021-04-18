use crate::actions;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};

pub async fn get_all_items(
    _: HttpRequest,
    pool: web::Data<crate::DbPool>,
) -> Result<impl Responder, Error> {
    #[cfg(debug_assertions)]
    info!("get_all_items called");

    let connection = pool.get().expect("Could not get DB connection");

    let items = web::block(move || actions::get_all_items(&connection))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(items) = items {
        if items.is_empty() {
            info!("get_all_items returning \"no items\"");
            return Ok(
                HttpResponse::NotFound().body("No items currently listed on the market")
            );
        }

        info!("get_all_items returning {:#?}", items);
        Ok(HttpResponse::Ok().json(items))
    } else {
        info!("get_all_items returning \"no items\"");
        Ok(HttpResponse::NotFound().body("No items currently listed on the market"))
    }
}

pub async fn get_item_by_id(
    req: HttpRequest,
    pool: web::Data<crate::DbPool>,
) -> Result<impl Responder, Error> {
    let item_id: i32 = if let Ok(id) = req
        .match_info()
        .get("item_id")
        .expect("Did not supply item_id")
        .parse()
    {
        id
    } else {
        return Ok(
            HttpResponse::Ok().body("Cannot parse supplied item_id as a valid i32")
        );
    };

    info!("get_item_by_id called with an item_id of {}", item_id);

    let connection = pool.get().expect("Could not get DB connection");

    let item = web::block(move || actions::get_item_by_id(&connection, item_id))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(item) = item {
        info!("get_item_by_id returning {:?}", item);
        Ok(HttpResponse::Ok().json(item))
    } else {
        info!("get_item_by_id returning \"does not exists\"");
        Ok(HttpResponse::Ok().body(&format!(
            "Item with ID \"{}\" does not exist on the market",
            item_id
        )))
    }
}
