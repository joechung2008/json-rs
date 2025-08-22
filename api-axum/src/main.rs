use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};
use axum_extra::{TypedHeader, headers::ContentType};
use serde_json::json;
use shared_lib::{parse, pretty_print_token};

async fn parse_endpoint(
    (content_type, body): (Option<TypedHeader<ContentType>>, axum::body::Bytes),
) -> impl IntoResponse {
    match content_type {
        Some(TypedHeader(ct)) if ct == ContentType::from(mime::TEXT_PLAIN) => {}
        _ => {
            let err_json = json!({ "code": 415, "message": "Unsupported Media Type" });
            return (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                [("content-type", "application/json")],
                err_json.to_string(),
            );
        }
    }

    // Read body as string
    let text = match String::from_utf8(body.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            let err_json = json!({ "code": 400, "message": format!("Invalid UTF-8: {}", e) });
            return (
                StatusCode::BAD_REQUEST,
                [("content-type", "application/json")],
                err_json.to_string(),
            );
        }
    };

    match parse(&text) {
        Ok(result) => (
            StatusCode::OK,
            [("content-type", "text/plain")],
            pretty_print_token(&result.token, 0),
        ),
        Err(e) => {
            let err_json = json!({ "code": 400, "message": format!("{}", e) });
            (
                StatusCode::BAD_REQUEST,
                [("content-type", "application/json")],
                err_json.to_string(),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/v1/parse", post(parse_endpoint));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let service = app.into_make_service();
    let server = axum::serve(listener, service);
    println!("Server running on http://localhost:8000");
    server.await.unwrap();
}
