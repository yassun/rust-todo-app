use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    println!("helth-check");
    HttpResponse::Ok().finish()
}
