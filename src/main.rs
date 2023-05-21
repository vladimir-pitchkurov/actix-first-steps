mod api;

use actix_web::{App, HttpServer};
use actix_web_lab::web::spa;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(api::api_scope())
            .service(
                spa()
                    .index_file("./public/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./public")
                    .finish()
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
