use actix_web::http::header::CONTENT_TYPE;
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::Serialize;
use shared_lib::{parse, pretty_print_token};

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

#[post("/api/v1/parse")]
async fn parse_endpoint(req_body: web::Bytes, req: actix_web::HttpRequest) -> impl Responder {
    let content_type = req
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok());

    if content_type != Some("text/plain") {
        let err = ErrorResponse {
            code: 415,
            message: "Unsupported Media Type".to_string(),
        };
        return HttpResponse::UnsupportedMediaType()
            .insert_header((CONTENT_TYPE, "application/json"))
            .json(err);
    }

    let text = match String::from_utf8(req_body.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            let err = ErrorResponse {
                code: 400,
                message: format!("Invalid UTF-8: {}", e),
            };
            return HttpResponse::BadRequest()
                .insert_header((CONTENT_TYPE, "application/json"))
                .json(err);
        }
    };

    match parse(&text) {
        Ok(result) => HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/plain"))
            .body(pretty_print_token(&result.token, 0)),
        Err(e) => {
            let err = ErrorResponse {
                code: 400,
                message: format!("{}", e),
            };
            HttpResponse::BadRequest()
                .insert_header((CONTENT_TYPE, "application/json"))
                .json(err)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:8000");
    HttpServer::new(|| App::new().service(parse_endpoint))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
