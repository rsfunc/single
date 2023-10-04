#[actix_web::main]
async fn main() -> std::io::Result<()> {
    single::startup::create_server().await
}
