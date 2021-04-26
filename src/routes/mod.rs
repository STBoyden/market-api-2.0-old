use rocket::{http::ContentType, Response};
use std::io::Cursor;

mod items;

pub use items::*;

#[get("/")]
pub fn index<'a>() -> Response<'a> {
    let mut response = Response::new();
    response.set_header(ContentType::HTML);
    response.set_sized_body(Cursor::new(
        r#"
        <h1>Market API</h1>
        Routes:
        <ul>
            <li>GET / -> shows this help page</li>
            <li>GET /items -> gets all items and returns in JSON</li>
            <li>GET /item/{item_id} -> gets the item with specified item_id and returns in JSON</li>
            <li>POST /add_item -> adds an item from a header supplied in JSON format</li>
        </ul>
    "#,
    ));

    response
}
