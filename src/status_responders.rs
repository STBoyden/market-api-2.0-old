use rocket::Request;
use rocket_contrib::json::Json;

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    code: u32,
    message: String,
}

#[catch(400)]
pub fn bad_request(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code: 400,
        message: format!(
            "Recieved bad request: URI {}, Method {},",
            req.uri(),
            req.method(),
        ),
    })
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code: 404,
        message: format!("'{}' not found", req.uri()),
    })
}

#[catch(500)]
pub fn internal_server_error(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code: 500,
        message: format!(
            "Internal sever error occurred: URI {}, Method {}",
            req.uri(),
            req.method(),
        ),
    })
}
