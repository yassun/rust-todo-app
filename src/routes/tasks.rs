use actix_web::{web, HttpResponse};

pub async fn tasks(id: web::Path<u32>) -> HttpResponse {
    println!("get_tasks id:{}", id);
    HttpResponse::Ok().finish()
}
