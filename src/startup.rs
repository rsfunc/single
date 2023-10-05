use actix_web::{App, HttpServer};

pub async fn create_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(crate::routes::info)
            .service(crate::routes::health_check)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
