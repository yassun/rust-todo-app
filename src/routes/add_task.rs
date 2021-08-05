use actix_web::{web, HttpResponse};
use crate::domain::{Todo};

pub async fn add_task(task: web::Json<Todo>) -> HttpResponse {
    println!("add_task #{:?}", task);
    HttpResponse::Ok().finish()
}
