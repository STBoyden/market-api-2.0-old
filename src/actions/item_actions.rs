use crate::models::Item;
use diesel::prelude::*;

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
    id: i32,
) -> Result<Option<Item>, diesel::result::Error> {
    use crate::schema::items::dsl::*;

    let item = items
        .filter(item_id.eq(id))
        .first::<Item>(connection)
        .optional()?;

    Ok(item)
}
