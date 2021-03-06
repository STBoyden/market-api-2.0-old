use crate::models::{Item, NewItem};
use rocket_contrib::databases::diesel::prelude::*;

pub fn get_all_items(
    connection: &MysqlConnection,
) -> Result<Option<Vec<Item>>, diesel::result::Error> {
    use crate::schema::items::dsl::*;

    let market = items
        .order(item_id.asc())
        .load::<Item>(connection)
        .optional()?;

    Ok(market)
}

pub fn get_item_by_id(
    connection: &MysqlConnection,
    record_id: i32,
) -> Result<Option<Item>, diesel::result::Error> {
    use crate::schema::items::dsl::*;

    let item = items
        .filter(id.eq(record_id))
        .first::<Item>(connection)
        .optional()?;

    Ok(item)
}

pub fn add_item(
    connection: &MysqlConnection,
    item: NewItem,
) -> Result<Option<usize>, diesel::result::Error> {
    use crate::schema::items::dsl::*;

    let rows_affected = diesel::insert_into(items)
        .values(&item)
        .execute(connection)
        .optional()?;

    Ok(rows_affected)
}

pub fn remove_item(
    connection: &MysqlConnection,
    record_id: i32,
) -> Result<Option<usize>, diesel::result::Error> {
    use crate::schema::items::dsl::*;

    let rows_affected = diesel::delete(items.filter(id.eq(record_id)))
        .execute(connection)
        .optional()?;

    Ok(rows_affected)
}
