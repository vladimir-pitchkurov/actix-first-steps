use actix_web::{get, web, HttpResponse, Responder, Scope, Result};
use serde::Serialize;


pub async fn api_test(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Serialize)]
struct SuccessResponseApi<T> {
    success: bool,
    data: T,
    error: Option<String>,
}

impl<T> SuccessResponseApi<T> {
    fn new(data: T) -> Self {
        SuccessResponseApi {
            success: true,
            data,
            error: None,
        }
    }
}

#[get("/name/{name}")]
async fn index(name: web::Path<String>) -> Result<impl Responder> {
    let obj = SuccessResponseApi::new(name.to_string());
    Ok(web::Json(obj))
}

#[get("/")]
pub async fn hello_api() -> String {
    "hello api".to_string()
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(index)
        .service(hello_api)
        // ...so this handles requests for `POST /api/test`
        .route("/test", web::post().to(api_test))
}
