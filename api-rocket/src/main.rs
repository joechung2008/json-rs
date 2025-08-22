#[macro_use]
extern crate rocket;

use rocket::data::{Data, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::serde::json::json;
use shared_lib::{parse, pretty_print_token};

#[post("/api/v1/parse", data = "<input>")]
async fn parse_endpoint(
    input: Data<'_>,
    content_type: Option<&ContentType>,
) -> Result<(ContentType, String), (Status, (ContentType, String))> {
    if content_type != Some(&ContentType::Plain) {
        let err_json = json!({ "code": 415, "message": "Unsupported Media Type" });
        return Err((
            Status::UnsupportedMediaType,
            (ContentType::JSON, err_json.to_string()),
        ));
    }

    let bytes = input.open(128.kibibytes()).into_bytes().await;

    let text = match bytes {
        Ok(b) => match String::from_utf8(b.value) {
            Ok(s) => s,
            Err(e) => {
                let err_json = json!({ "code": 400, "message": format!("Invalid UTF-8: {}", e) });
                return Err((
                    Status::BadRequest,
                    (ContentType::JSON, err_json.to_string()),
                ));
            }
        },
        Err(e) => {
            let err_json = json!({ "code": 400, "message": format!("Failed to read body: {}", e) });
            return Err((
                Status::BadRequest,
                (ContentType::JSON, err_json.to_string()),
            ));
        }
    };

    match parse(&text) {
        Ok(result) => Ok((ContentType::Plain, pretty_print_token(&result.token, 0))),
        Err(e) => {
            let err_json = json!({ "code": 400, "message": format!("{}", e) });
            Err((
                Status::BadRequest,
                (ContentType::JSON, err_json.to_string()),
            ))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![parse_endpoint])
}
