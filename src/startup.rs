use actix_web::{web, App, HttpServer};

use slog;
use slog::{o, Drain, Logger};
use slog_async;
use slog_term;

fn configure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator)
        .use_file_location()
        .build()
        .fuse();

    // It is used for Synchronization
    let console_drain = slog_async::Async::new(console_drain).build().fuse();

    // Root logger
    slog::Logger::root(console_drain, o!("v"=>env!("CARGO_PKG_VERSION")))
}

pub async fn create_server() -> std::io::Result<()> {
    let log = configure_log();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(log.clone()))
            .service(crate::routes::info)
            .service(crate::routes::health_check)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
