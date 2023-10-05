use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn info() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, App};

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(
            App::new() // App
                .service(info),
        )
        .await;
        let resp: actix_web::dev::ServiceResponse = test::TestRequest::get()
            .uri("/")
            .insert_header(ContentType::plaintext())
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());

        let body = resp.into_body();
        let resp_bytes = actix_http::body::to_bytes(body).await.unwrap();
        let resp_message = std::str::from_utf8(&resp_bytes).unwrap();
        pretty_assertions::assert_eq!(resp_message, "Hello world!")
    }
}
