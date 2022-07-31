use crate::models::User;
use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn get() -> HttpResponse {
    HttpResponse::Ok().json(User {
        email: "test@example.com".to_string(),
        id: "randon-uuid".to_string(),
        name: "Max Mustermann".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(get)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let user: User = test::read_body_json(resp).await;

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.id, "randon-uuid");
        assert_eq!(user.name, "Max Mustermann");
    }
}
