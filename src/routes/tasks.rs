use actix_web::HttpResponse;

pub async fn tasks() -> HttpResponse {
    println!("get_tasks");
    HttpResponse::Ok().finish()
}
